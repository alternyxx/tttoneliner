use std::fs::write;
use serde_json::to_string_pretty;
mod dataset;
mod tictactoe;

fn main() -> std::io::Result<()> {
    let ds = dataset::generate_dataset();

    let json = to_string_pretty(&ds).unwrap();

    write("dataset.json", json)?;

    Ok(())
}