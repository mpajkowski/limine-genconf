use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(long, default_value = "Linux")]
    pub title: String,

    #[clap(long, default_value = "/boot")]
    pub scan_path: PathBuf,

    #[clap(long)]
    pub cmdline: String,

    #[clap(long, default_value_t = 5)]
    pub timeout: u32,
}
