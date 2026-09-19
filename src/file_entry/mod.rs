use crate::cli::Cli;
use regex::Regex;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::{io, u32};

#[derive(Debug)]
pub struct FileEntry {
    pub number: u32,
    pub name_part: String,
    pub full_name: String,
    pub new_name: Option<String>,
}

impl FileEntry {
    /// Creates a FileEntry from a dir entry according to the given Cli
    pub fn from(dir_entry: &DirEntry, cli: &Cli) -> Option<Self> {
        let file_path = dir_entry.path().display().to_string();
        let filename = dir_entry.file_name().display().to_string();
        if let Some((number, name_part)) = split_file_name(&filename, cli) {
            return Some(Self {
                number,
                name_part,
                full_name: file_path,
                new_name: None,
            });
        }
        None
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
        let start = cli.base.unwrap_or(new_numbers[0]);
        let step_size = cli.step.unwrap_or(1) as usize;
        let end = start + (new_numbers.len() * step_size) as u32;
        dbg!(&start, &end, &step_size);
        new_numbers = (start..end).step_by(step_size).collect();
        dbg!(&new_numbers);
    } else {
        // adjust new starting value
        if let Some(start) = cli.base
            && start != original_numbers[0]
        {
            new_numbers = new_numbers
                .iter()
                .map(|n| n + start - original_numbers[0])
                .collect();
        }

        // apply new step width
        if let Some(delta) = cli.step
            && delta > 1
        {
            new_numbers = new_numbers
                .iter()
                .enumerate()
                .map(|(i, n)| n + i as u32 * delta)
                .collect::<Vec<u32>>();
        }
    }

    if cli.mirror {
        dbg!(&new_numbers);
        let start = new_numbers[0];
        let end = new_numbers[new_numbers.len() - 1];
        // dbg!(&start);
        // dbg!(&end);
        new_numbers = new_numbers.into_iter().map(|n| end - n + start).collect();
        dbg!(&new_numbers);
    }

    let mut number_map = HashMap::new();
    for i in 0..original_numbers.len() {
        number_map.insert(original_numbers[i], new_numbers[i]);
    }

    Some(number_map)
}

pub fn capture_pattern_string(cli: &Cli) -> String {
    format!(
        "^\\{}(?<number>\\d+)\\{}(?<tail>.*)$",
        cli.l_delim(),
        cli.r_delim()
    )
}

pub fn format_file_name(file_name_part: &str, number: u32, cli: &Cli) -> String {
    format!(
        "{}{:0width$}{}{}",
        cli.l_delim(),
        number,
        cli.r_delim(),
        file_name_part,
        width = cli.width.unwrap_or(0)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn format_file_name_changing_number_width() {
        let cli = Cli {
            width: Some(4),
            ..Cli::new()
        };
        let file_entry = FileEntry {
            number: 3,
            name_part: " A file named file.dat".to_string(),
            full_name: "[3] A file named file.dat".to_string(),
            new_name: None,
        };
        let file_name = format_file_name(&file_entry.name_part, file_entry.number, &cli);

        assert_eq!(file_name, "[0003] A file named file.dat".to_string());
    }

    #[test]
    fn generate_numbers_works_with_default_settings() {
        let cli = Cli { ..Cli::new() };
        let original_numbers = vec![1, 2, 3, 4, 5, 6];
        let number_map = generate_new_numbers(original_numbers.clone(), &cli);
        dbg!(&number_map);
        assert!(number_map.is_some());
    }
}
