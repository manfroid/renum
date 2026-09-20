pub mod cli;

mod file_entry;
mod files;

use cli::Cli;
use file_entry::{collect_file_entries, generate_new_numbers_for_file_entries};
use files::rename_by_file_entries;
use std::io;

pub fn rename_in_folder(folder: &str, cli: &Cli) -> io::Result<()> {
    let (file_entries, _) = collect_file_entries(folder, &cli)?;

    if let Some(number_map) = generate_new_numbers_for_file_entries(&file_entries, cli) {
        rename_by_file_entries(&file_entries, &number_map, cli);
    }

    Ok(())
}
