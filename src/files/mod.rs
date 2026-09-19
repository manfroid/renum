use crate::cli::Cli;
use crate::file_entry::{capture_pattern_string, format_file_name};
use regex::Regex;

pub fn rename(from: &str, cli: &Cli) -> Option<String> {
    if let Ok(re) = Regex::new(&capture_pattern_string(cli)) {
        println!("{re:#?}");

        if let Some(captures) = re.captures(from) {
            println!("{captures:#?}");
            // according to regex pattern, this capture must be numeric => unwrap it!
            let number = captures["number"].to_string().parse::<u32>().unwrap();
            let tail = captures["tail"].to_string();
            return Some(format_file_name(&tail, number, cli));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rename_can_return_renamed_string() {
        const FILENAME: &str = "[001] test_file.dat";
        let cli = Cli::new();

        let result = rename(FILENAME, &cli);
        assert_eq!(result, Some(String::from("[1] test_file.dat")));
    }
}
