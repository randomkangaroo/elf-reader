use crate::args::Cli;
use crate::headers::{format_file_header, format_program_headers, format_section_headers};
use clap::Parser;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use std::fs;
use std::path::PathBuf;

mod args;
mod headers;

fn main() {
    let args = Cli::parse();
    // Since the file path is a required argument, just use it
    let file_path = PathBuf::from(args.file);
    let file_data = match fs::read(&file_path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: Could not open file '{}'", file_path.display());
            return;
        }
    };
    let slice = file_data.as_slice();
    // Try to parse this file as an ELF file
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice);
    let file_data = match file {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            return;
        }
    };
    // TODO: depending on args, print different parts of the ELF file
    if args.all {
        // TODO!
    } else if args.file_header {
        if let Some(header_string) = format_file_header(file_data) {
            println!("{}", header_string);
        } else {
            eprintln!("Error: could not print the file header")
        }
    } else if args.program_headers {
        if let Some(program_header_string) = format_program_headers(file_data) {
            println!("{}", program_header_string);
        }
    } else if args.section_headers {
        if let Some(section_header_string) = format_section_headers(file_data) {
            println!("{}", section_header_string);
        }
    } else {
        println!("you shouldn't see this");
    }
    // TODO: better problem handling instead of just an unwrap
}
