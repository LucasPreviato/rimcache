use crate::infra::fs_reader;
use crate::infra::xml_parser;
/// Futuramente aqui vamos:
/// - Ler a pasta de mods
/// - Buscar arquivos XML
/// - Gerar uma estrutura de cache otimizada

pub fn run() {
    println!("Executing build command");

    let mods_path = "./mods";
    println!("Reading xml archives from: {}", mods_path);

    let xml_files = fs_reader::list_xml_files(mods_path);

    println!("Found {} xml files", xml_files.len());

    for file in xml_files {
        println!("Processing file: {}", file);

        if file.ends_with("About.xml") {
            println!("read metadados from: {}", file);

            if let Some(metadata) = xml_parser::parse_about_xml(&file) {
                println!("🧩 Mod encontrado:");
                println!("  📛 Nome:         {:?}", metadata.name);
                println!("  👤 Autor:        {:?}", metadata.author);
                println!("  📝 Descrição:    {:?}", metadata.description);
                println!("  🧾 Package ID:   {:?}", metadata.packageId);
                println!("find mod: {:?}", metadata.name);

                if let Some(versions) = metadata.supportedVersions {
                    println!("  ✅ Versões:      {:?}", versions.versions);
                }
            } else {
                println!("error parsing metadata from: {}", file);
            }
        }
    }
}
