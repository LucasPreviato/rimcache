use crate::infra::fs_reader;

/// Futuramente aqui vamos:
/// - Ler a pasta de mods
/// - Buscar arquivos XML
/// - Gerar uma estrutura de cache otimizada

pub fn run(){
    println!("Executing build command");
    
    let mods_path = "./mods";
    println!("Reading xml archives from: {}", mods_path);

    let xml_files = fs_reader::list_xml_files(mods_path);

    println!("Found {} xml files", xml_files.len());
    for file in xml_files{
        println!("Processing file: {}", file);
    }
}