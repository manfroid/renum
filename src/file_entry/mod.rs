use crate::cli::Cli;
use crate::files::capture_pattern_string;
use regex::Regex;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::{io, u32};

#[derive(Debug)]
pub struct FileEntry {
    pub number: u32,
    pub name_part: String,
    pub full_name: String,
}

impl FileEntry {
    /// Creates a FileEntry from a dir entry according to the given Cli
    pub fn from(dir_entry: &DirEntry, cli: &Cli) -> Option<Self> {
        let file_path = dir_entry.path().display().to_string();
        let filename = dir_entry.file_name().display().to_string();
        if let Some((number, name_part)) = Self::split_file_name(&filename, cli) {
            return Some(Self {
                number,
                name_part,
                full_name: file_path,
            });
        }
        None
    }

    /// Split a file name matching the numbered file pattern into the number and
    /// the rest of the file name after the matched pattern as a heap String
    fn split_file_name(filename: &str, cli: &Cli) -> Option<(u32, String)> {
        // build regular expression from pattern
        if let Ok(re) = Regex::new(&capture_pattern_string(cli)) {
            // collect captures from filename as haystack
            if let Some(captures) = re.captures(filename) {
                // according to regex pattern, this capture must be numeric => unwrap it!
                let number = captures["number"].to_string().parse::<u32>().unwrap();
                let tail = captures["tail"].to_string();
                return Some((number, tail));
            }
        }

        return None;
    }
}

pub fn collect_file_entries(
    folder: &str,
    cli: &Cli,
) -> io::Result<(Vec<FileEntry>, Vec<DirEntry>)> {
    let dir = std::fs::read_dir(folder)?;

    // N.B. cannot get partition() to work, so let's build it ourselves:
    let mut files = vec![];
    let mut dirs = vec![]; // holds all non-file entries
    for dir_entry in dir {
        let dir_entry = dir_entry?;
        if dir_entry.path().is_file() {
            files.push(dir_entry);
        } else {
            dirs.push(dir_entry);
        }
    }

    // create a collection of FileEntry objects from DirEntries
    let mut file_entries = files
        .iter()
        // keep only files whose name starts with the l_delim
        .filter(|dir_entry| {
            dir_entry
                .file_name()
                .display()
                .to_string()
                .starts_with(&cli.l_delim())
        })
        // map DirEntries for matching files to FileEntries
        .filter_map(|dir_entry| FileEntry::from(&dir_entry, cli))
        .collect::<Vec<FileEntry>>();

    // sort FileEntries by number
    file_entries
        .sort_by(|file_entry_1, file_entry_2| file_entry_1.number.cmp(&file_entry_2.number));

    Ok((file_entries, dirs))
}

pub fn generate_new_numbers_for_file_entries(
    file_entries: &Vec<FileEntry>,
    cli: &Cli,
) -> Option<HashMap<u32, u32>> {
    let original_numbers = file_entries
        .iter()
        .map(|file_entry| file_entry.number)
        .collect::<Vec<u32>>();

    if original_numbers.len() < 1 {
        return None;
    }

    generate_new_numbers(original_numbers.clone(), cli)
}

pub fn generate_new_numbers(original_numbers: Vec<u32>, cli: &Cli) -> Option<HashMap<u32, u32>> {
    let mut new_numbers = Vec::with_capacity(original_numbers.len());

    for number in original_numbers {
        if !new_numbers.contains(&number) {
            new_numbers.push(number);
        }
    }

    if new_numbers.len() < 1 {
        return None;
    }

    // make sure numbers are in ascending order
    new_numbers.sort();
    let original_numbers = new_numbers.clone();

    if cli.consecutive {
        // handle -s (and -s <step size>) options
        // take optional -b <base number> as start value for numbering into account
        let start = cli.base.unwrap_or(new_numbers[0]);
        // if present, use optional -s <step size> for incremtns
        let step_size = cli.step.unwrap_or(1) as usize;
        // calculate end number for generating a range of numbers
        let end = start + (new_numbers.len() * step_size) as u32;
        // replace new_numbers with newly generated range
        new_numbers = (start..end).step_by(step_size).collect();
    } else {
        // handle -b <base number> option only if given and different from
        // original first value
        if let Some(start) = cli.base
            && start != original_numbers[0]
        {
            let apply_start = |n: u32| n + start - original_numbers[0];
            // use into_oter to work with u32 values; new_numbers will then be reassigned
            new_numbers = new_numbers.into_iter().map(apply_start).collect();
        }
    }

    if cli.mirror {
        // handle -m option
        // get start value from current values
        let start = new_numbers[0];
        // get end value from current values
        let end = new_numbers[new_numbers.len() - 1];
        // the mirror closure reverses the values honouring gaps
        let mirror = |n: u32| end - n + start;
        // replace new_numbers with mirrored values
        // use into_oter to work with u32 values; new_numbers will then be reassigned
        new_numbers = new_numbers.into_iter().map(mirror).collect();
    }

    let mut number_map = HashMap::new();
    for i in 0..original_numbers.len() {
        number_map.insert(original_numbers[i], new_numbers[i]);
    }

    Some(number_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn generate_numbers_works_with_default_settings() {
        let cli = Cli { ..Cli::new() };
        let original_numbers = vec![1, 2, 3, 4, 5, 6];
        let number_map = generate_new_numbers(original_numbers.clone(), &cli);
        dbg!(&number_map);
        assert!(number_map.is_some());
    }
}
