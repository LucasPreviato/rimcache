use std::fs;
use std::path::Path;


pub fn list_xml_files(mods_dir: &str) -> Vec<String> {
    let mut xml_files = Vec::new();

    visit_dirs(Path::new(mods_dir), &mut |entry|{
        if let Some(ext) = entry.path().extension(){
            if ext == "xml"{
                if let Some(path_str) = entry.path().to_str(){
                    xml_files.push(path_str.to_string());
                }
            }
        }
    });

    xml_files
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&fs::DirEntry)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb);
            } else {
                cb(&entry);
            }
        }
    }
}