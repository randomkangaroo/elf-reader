use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ELF Reader")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The file to get the ELF information about
    pub file: Option<PathBuf>,
    /// Flag to print all information about this ELF file
    #[arg(short, long)]
    pub all: bool,
    /// Flag to print the file header (on by default)
    #[arg(short, long, default_value_t = true)]
    pub file_header: bool,
    ///Flag to print the section headers
    #[arg(short, long)]
    pub section_headers: bool,
    /// Flag to print the program headers
    #[arg(short, long)]
    pub program_headers: bool,
}
