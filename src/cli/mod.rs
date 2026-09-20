use clap::Parser;

/// A container for command-line arguments parsed by the clap crate
#[derive(Debug, PartialEq, Eq, Parser)]
// #[command(version, no_binary_name = true)]
#[command(version, no_binary_name = false)]
pub struct Cli {
    /// The folder with the file(s) to be renumbered (default: current directory)
    #[arg(default_value_t = String::from("."))]
    pub path: String,
    /// Starting value of numbers (optional)
    #[arg(short, long)]
    pub base: Option<u32>,
    /// Consecutive numbers = eliminate gaps (default: false)
    #[arg(short, long, default_value_t = false)]
    pub consecutive: bool,
    /// delimiters around original number; if left and right
    /// are the same, onlx one can be given
    #[arg(short, long, default_value_t = String::from("[]"))]
    pub delimiters: String,
    /// Mirror numbering = reverse number order
    #[arg(short, long, default_value_t = false)]
    pub mirror: bool,
    /// Recurse into subdirectories
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,
    /// Step width of numbers (optional)
    #[arg(short, long, requires("consecutive"))]
    pub step: Option<u32>,
    /// width of number, left-padded with zeroes (optional)
    #[arg(short, long)]
    pub width: Option<usize>,
}

impl Cli {
    /// Creates a Cli struct set to its default values
    pub fn new() -> Self {
        Cli {
            path: ".".to_string(),
            base: None,
            consecutive: false,
            delimiters: "[]".to_string(),
            mirror: false,
            recursive: false,
            step: None,
            width: None,
        }
    }

    /// Creates a Cli struct from the program's command-line arguments
    pub fn from_args() -> Self {
        Self::parse()
    }

    /// Creates a Cli struct from a string containing the arguments (w/o binary file name)
    pub fn from_string(line: &str) -> Self {
        let binary_name = module_path!().split("::").take(1).next().unwrap_or("crate");
        let line = format!("{binary_name} {line}");
        let args = line.split_whitespace().collect::<Vec<&str>>();
        Self::parse_from(args)
    }

    /// Returns the left delimiter of a number in a file name
    pub fn l_delim(&self) -> String {
        if self.delimiters.len() < 1 {
            "[".to_string()
        } else {
            self.delimiters[..1].to_string()
        }
    }

    /// Returns the right delimiter of a number in a file name
    pub fn r_delim(&self) -> String {
        if self.delimiters.len() < 1 {
            "]".to_string()
        } else {
            self.delimiters
                .get(1..2)
                .unwrap_or(&self.l_delim())
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_creates_default_cli() {
        let cli = Cli::new();
        assert_eq!(cli.path, ".".to_string());
        assert_eq!(cli.base, None);
        assert_eq!(cli.consecutive, false);
        assert_eq!(cli.delimiters, "[]".to_string());
        assert_eq!(cli.mirror, false);
        assert_eq!(cli.recursive, false);
        assert_eq!(cli.step, None);
        assert_eq!(cli.width, None);
    }

    #[test]
    fn from_string_with_args_creates_appropriate_cli() {
        let cli_from_string = Cli::from_string("-m -b 2 --width 4");
        let expected_cli = Cli {
            base: Some(2),
            mirror: true,
            width: Some(4),
            ..Cli::new()
        };
        assert_eq!(cli_from_string, expected_cli);
    }
}
