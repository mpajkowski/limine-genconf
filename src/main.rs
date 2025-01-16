mod cli;
mod entry;
mod format;

use std::io;

use clap::Parser;
use cli::Cli;

fn main() -> io::Result<()> {
    let config = Cli::parse();
    eprintln!("Config: {config:?}");

    let entries = entry::load_entries(&config)?;
    eprintln!("Found following entries: {entries:?}");

    let out = format::limine(entries, &config);
    println!("{out}");

    Ok(())
}
