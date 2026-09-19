use renum::cli::Cli;
// use renum::file_entry::collect_file_entries;
use renum::file_entry::generate_new_numbers;

use std::io;

/// Renumbers files in a directory
fn main() -> io::Result<()> {
    let cli = Cli::from_args();
    // println!("{cli:#?}");

    // let (file_entries, dirs) = collect_file_entries(&cli.path, &cli)?;
    // dbg!(&file_entries);
    // dbg!(&dirs);

    let original_numbers = vec![1, 2, 4];
    let number_map = generate_new_numbers(original_numbers.clone(), &cli);
    dbg!(&number_map);

    let original_numbers = vec![9, 17, 3, 7, 1];
    let number_map = generate_new_numbers(original_numbers.clone(), &cli);
    dbg!(&number_map);

    Ok(())
}
