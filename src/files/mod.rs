use std::collections::HashMap;

use crate::cli::Cli;
use crate::file_entry::FileEntry;

pub fn rename_by_file_entries(
    file_entries: &Vec<FileEntry>,
    number_map: &HashMap<u32, u32>,
    cli: &Cli,
) {
    file_entries.into_iter().for_each(|file_entry| {
        let new_file_name =
            format_file_name(&file_entry.name_part, number_map[&file_entry.number], cli);
        println!("{} -> {}", file_entry.full_name, new_file_name);
        let _ = std::fs::rename(&file_entry.full_name, &new_file_name);
    });
}

/// Return the String that represents a file name adhering tot the numbering scheme
/// required by this app; to be used to create a Regex from
pub fn capture_pattern_string(cli: &Cli) -> String {
    format!(
        "^\\{}(?<number>\\d+)\\{}(?<tail>.*)$",
        cli.l_delim(),
        cli.r_delim()
    )
}

fn format_file_name(file_name_part: &str, number: u32, cli: &Cli) -> String {
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
    fn capture_pattern_string_works_with_defaults() {
        let cli = Cli::new();

        let capture_pattern = capture_pattern_string(&cli);
        assert_eq!(
            capture_pattern,
            String::from("^\\[(?<number>\\d+)\\](?<tail>.*)$")
        );
    }

    #[test]
    fn capture_pattern_string_works_with_single_delimiter() {
        let cli = Cli {
            delimiters: "#".to_string(),
            ..Cli::new()
        };
        let capture_pattern = capture_pattern_string(&cli);
        assert_eq!(
            capture_pattern,
            String::from("^\\#(?<number>\\d+)\\#(?<tail>.*)$")
        );
    }

    #[test]
    fn capture_pattern_string_works_with_empty_delimiter() {
        let cli = Cli {
            delimiters: String::default(),
            ..Cli::new()
        };
        let capture_pattern = capture_pattern_string(&cli);
        assert_eq!(
            capture_pattern,
            String::from("^\\[(?<number>\\d+)\\](?<tail>.*)$")
        );
    }

    #[test]
    fn format_file_name_keeps_number_width_if_necessary() {
        let cli = Cli::from_string("-w 1");
        let file_entry = FileEntry {
            number: 42,
            name_part: " A file named file.dat".to_string(),
            full_name: "[42] A file named file.dat".to_string(),
        };
        let file_name = format_file_name(&file_entry.name_part, file_entry.number, &cli);

        assert_eq!(file_name, "[42] A file named file.dat".to_string());
    }

    #[test]
    fn format_file_name_widens_number_width() {
        let cli = Cli::from_string("--width 4");
        let file_entry = FileEntry {
            number: 3,
            name_part: " A file named file.dat".to_string(),
            full_name: "[3] A file named file.dat".to_string(),
        };
        let file_name = format_file_name(&file_entry.name_part, file_entry.number, &cli);

        assert_eq!(file_name, "[0003] A file named file.dat".to_string());
    }

    #[test]
    fn format_file_name_changes_delimiter_pair() {
        let cli = Cli::from_string("--delimiters ..");
        let file_entry = FileEntry {
            number: 3,
            name_part: " A file named file.dat".to_string(),
            full_name: "[3] A file named file.dat".to_string(),
        };
        let file_name = format_file_name(&file_entry.name_part, file_entry.number, &cli);

        assert_eq!(file_name, ".3. A file named file.dat".to_string());
    }

    #[test]
    fn format_file_name_changes_single_delimiter() {
        let cli = Cli::from_string("--delimiters |");
        let file_entry = FileEntry {
            number: 3,
            name_part: " A file named file.dat".to_string(),
            full_name: "[3] A file named file.dat".to_string(),
        };
        let file_name = format_file_name(&file_entry.name_part, file_entry.number, &cli);

        assert_eq!(file_name, "|3| A file named file.dat".to_string());
    }
}
