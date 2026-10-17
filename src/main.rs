use crate::args::Cli;
use clap::Parser;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::to_str;
use std::fs;
use std::path::PathBuf;

mod args;

fn main() {
    let args = Cli::parse();
    // Since the file path is a required argument, just use it
    let file_path = PathBuf::from(args.file);
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
    file_header.push_str(&format!("Data: {:?} Endian\n", file.ehdr.endianness));
    file_header.push_str(&format!(
        "OS/ABI: {}\n",
        to_str::e_osabi_to_str(file.ehdr.osabi)?
    ));
    file_header.push_str(&format!("ABI Version: {}\n", file.ehdr.abiversion));
    file_header.push_str(&format!(
        "Type: {}\n",
        to_str::e_type_to_human_str(file.ehdr.e_type)?
    ));
    file_header.push_str(&format!("Machine: {}\n", file.ehdr.e_machine));
    file_header.push_str(&format!("ELF Version: 0x{:x}\n", file.ehdr.version));
    file_header.push_str(&format!("Entry point address: 0x{:x}\n", file.ehdr.e_entry));
    file_header.push_str(&format!(
        "Start of program headers: {} (bytes into file)\n",
        file.ehdr.e_phoff
    ));
    file_header.push_str(&format!(
        "Start of section headers: {} (bytes into file)\n",
        file.ehdr.e_shoff
    ));
    file_header.push_str(&format!("Flags: 0x{:x}\n", file.ehdr.e_flags));
    file_header.push_str(&format!(
        "Size of this header: {} (bytes)\n",
        file.ehdr.e_ehsize
    ));
    file_header.push_str(&format!(
        "Size of program headers: {} (bytes)\n",
        file.ehdr.e_phentsize
    ));
    file_header.push_str(&format!(
        "Number of program headers: {}\n",
        file.ehdr.e_phnum
    ));
    file_header.push_str(&format!(
        "Size of section headers: {} (bytes)\n",
        file.ehdr.e_shentsize
    ));
    file_header.push_str(&format!(
        "Number of section headers: {}\n",
        file.ehdr.e_shnum
    ));
    file_header.push_str(&format!(
        "Section header string table index: {}",
        file.ehdr.e_shstrndx
    ));
    // Return the completed file header string
    Some(file_header)
}
