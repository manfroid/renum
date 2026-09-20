use renum::cli::Cli;
use renum::files::rename_in_folder;

use std::io;

/// Renumbers files in a directory
fn main() -> io::Result<()> {
    let cli = Cli::from_args();
    rename_in_folder(&cli.path, &cli)
}
