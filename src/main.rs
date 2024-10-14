use clap::ArgMatches;
use std::error::Error;

use inventory_processor::process_inventory;
use parse_inventory::write_inventory;

mod cli_parser;
mod file_len_inventory;
mod inventory_processor;
mod parse_inventory;

fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = cli_parser::parse_cli();
    let dir_path: &str = matches
        .get_one::<String>("dir-path")
        .expect("The directory path is required but was not provided");
    let header: bool = matches.get_flag("header");
    let max_depth: usize = matches
        .get_one::<String>("max-depth")
        .unwrap()
        .parse()
        .unwrap();

    let inventory = process_inventory(dir_path, max_depth, header);

    write_inventory(inventory, dir_path)?;
    Ok(())
}
