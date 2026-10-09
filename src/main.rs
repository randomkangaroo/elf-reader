use elf::ElfBytes;
use elf::endian::AnyEndian;

fn main() {
    // TODO: get a file to parse from args using clap
    let file_path = "/usr/bin/bash";
    // TODO: make this error handling robust
    let file_data = std::fs::read(file_path).expect("Could not read the file");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Could not parse");
    // TODO: depending on args, print different parts of the ELF file
    print_file_header(file);
}

fn print_file_header(file: ElfBytes<'_, AnyEndian>) {
    // TODO: make this function more robust
    // maybe move the printing to a separate function and just return a string?
    println!("{:?}", file.ehdr.class);
    println!("{:?}", file.ehdr.abiversion);
}
