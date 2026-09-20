use clap::Parser;

/// A container for command-line arguments parsed by the clap crate
#[derive(Debug, Parser)]
#[command(version)]
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
    /// A Cli struct set to its default values
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

    pub fn from_args() -> Self {
        Self::parse()
    }

    pub fn l_delim(&self) -> String {
        if self.delimiters.len() < 1 {
            "[".to_string()
        } else {
            self.delimiters[..1].to_string()
        }
    }

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
