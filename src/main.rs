use clap::ArgMatches;
use inventory_processor::process_inventory;
use parse_inventory::write_inventory;
use std::error::Error;
use std::path::Path;

mod cli_parsing;
mod file_len_inventory;
mod inventory_processor;
mod parse_inventory;

fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = cli_parsing::parse_cli();
    let dir_path_str: &str = matches
        .get_one::<String>("dir-path")
        .expect("The directory path is required but was not provided");
    let header: bool = matches.get_flag("header");
    let max_depth: usize = matches
        .get_one::<String>("max-depth")
        .unwrap()
        .parse()
        .unwrap();

    let dir_path: &Path = Path::new(dir_path_str);

    let inventory = process_inventory(dir_path, max_depth, header);

    write_inventory(inventory, dir_path)?;
    Ok(())
}
