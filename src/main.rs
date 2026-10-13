use crate::args::Cli;
use clap::Parser;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::to_str;
use std::fs;
use std::path::PathBuf;

mod args;

fn main() {
    // TODO: get a file to parse from args using clap
    let args = Cli::parse();
    println!("File name: {:?}", args.file.unwrap());
    let file_path = PathBuf::from("/usr/bin/sh");
    // TODO: make this error handling robust
    let file_data = fs::read(file_path).expect("Could not read the file");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Could not parse");
    // TODO: depending on args, print different parts of the ELF file
    // TODO: better problem handling instead of just an unwrap
    println!("{}", format_file_header(file).unwrap());
}

fn format_file_header(file: ElfBytes<'_, AnyEndian>) -> Option<String> {
    let mut file_header = String::new();
    file_header.push_str("----ELF File Header-----\n");
    file_header.push_str(&format!("Class: {:?}\n", file.ehdr.class));
    file_header.push_str(&format!(
        "OS/ABI: {}\n",
        to_str::e_osabi_to_str(file.ehdr.osabi)?
    ));
    file_header.push_str(&format!("ABI Version: {}\n", file.ehdr.abiversion));
    file_header.push_str(&format!(
        "Type: {}\n",
        to_str::e_type_to_human_str(file.ehdr.e_type)?
    ));
    // TODO: add more fields and change formatting
    // Return the completed file header string
    Some(file_header)
}
