use elf::ElfBytes;
use elf::endian::AnyEndian;
use elf::to_str;

pub fn format_file_header(file: ElfBytes<'_, AnyEndian>) -> Option<String> {
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

pub fn format_program_headers(file: ElfBytes<'_, AnyEndian>) -> Option<String> {
    // TODO!
    None
}

pub fn format_section_headers(file: ElfBytes<'_, AnyEndian>) -> Option<String> {
    // TODO!
    None
}
