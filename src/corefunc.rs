use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str;

//=====================================
//=== Function for getting ModIds =====
//=====================================
//hello world
pub fn idscollect(source: String) -> io::Result<String> {
    let mut text_file = File::open(source)?;
    let mut strbuf = Vec::new();
    let _loading_strbuf = text_file.read_to_end(&mut strbuf);
    let mut content = match str::from_utf8(&strbuf) {
        Ok(content) => content.to_string(),
        Err(_err) => {
            println!("Error reading bytes as utf8");
            "Error".to_string()
        }
    };
    loop {
        if content.contains("id=") {
            let offset = content.find('=').unwrap() + 1;
            content.replace_range(..offset, "");
        } else if !content.contains("id=") {
            break;
        }
    }

    loop {
        if content.contains("\n") {
            let offset = content.find('\n').unwrap();
            let _ = content.split_off(offset);
        }

        if content.contains("\r") {
            let offset = content.find('\r').unwrap();
            let _ = content.split_off(offset);
        }

        if !content.contains("\n") || !content.contains("\r") {
            break;
        }
    }
    return Ok(content);
}

//=====================================
//== Function for getting Mod Paths ===
//=====================================

pub fn pathcollect(source: &str) -> io::Result<Vec<String>> {
    let mut paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(source)? {
        let dir = entry?;
        let dirpath = dir.path().to_str().unwrap().to_string();
        paths.push(dirpath);
    }
    return Ok(paths);
}

//===========================================
//=== Function for getting workshop ids =====
//===========================================

pub fn workidbuild(source: &str) -> io::Result<Vec<String>> {
    let mut workids: Vec<String> = Vec::new();
    for entry in fs::read_dir(source)? {
        let dir = entry?;
        let wid = dir
            .path()
            .to_str()
            .unwrap()
            .to_string()
            .replace(&source, "");
        if wid.contains("/") {
            workids.push(wid.replace("/", ""));
        }

        if wid.contains("\\") {
            workids.push(wid.replace("\\", ""));
        }
    }
    return Ok(workids);
}

//=================================================================
//=== Functions for recursively locating mod.info directories =====
//=================================================================

pub fn modidpathcollecter(source: Vec<String>) -> std::io::Result<Vec<String>> {
    let mut modinfos: Vec<String> = Vec::new();

    for val in source {
        let _ = collect_modids(&Path::new(&val), &mut modinfos);
    }
    return Ok(modinfos);
}

pub fn collect_modids(path: &Path, modinfos: &mut Vec<String>) -> std::io::Result<()> {
    if !path.is_file() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_str().unwrap().contains("mod.info") {
                modinfos.push(path.to_str().unwrap().to_string());
            } else if path.is_dir() {
                let _ = collect_modids(&path, modinfos);
            }
        }
    }
    Ok(())
}

//==========================================================================
//=== Functions for recursively locating map names and collecting them =====
//==========================================================================

pub fn mapnamecollect(source: Vec<String>) -> std::io::Result<Vec<String>> {
    let mut mapnames: Vec<String> = Vec::new();

    for val in source {
        collect_mapnames(&Path::new(&val), &mut mapnames)?;
    }

    return Ok(mapnames);
}

pub fn collect_mapnames(path: &Path, mapnames: &mut Vec<String>) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if path.to_str().unwrap().contains("maps") {
                    for sub_entry in fs::read_dir(&path)? {
                        let place: String = path.to_str().unwrap().to_string() + "/";
                        let sub_entry = sub_entry?;
                        let sub_path = sub_entry.path();
                        if sub_path.is_dir() {
                            if sub_path.to_str().unwrap().contains("\\") {
                                let place: String = place.replace("/", "\\");
                                mapnames.push(sub_path.to_str().unwrap().to_string().replace(&place, ""));
                            }
                            else if !sub_path.to_str().unwrap().contains("\\") {
                                mapnames
                                    .push(sub_path.to_str().unwrap().to_string().replace(&place, ""));
                            }
                        }
                    }
                } else {
                    collect_mapnames(&path, mapnames)?;
                }
            }
        }
    }
    return Ok(());
}
