use std::fs;
use std::fs::File;
use std::io;
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::str;

//=====================================
//=== Function for getting ModIds =====
//=====================================

fn idcollect(source: String) -> io::Result<String> {
    let text_file = File::open(source)?;
    let mut strbuf = [0u8; 8];
    let mut output: String = String::new();
    let mut bytecount: u64 = 0;

    loop {
        let _bytes_read = text_file.read_at(&mut strbuf, bytecount)?;
        let container: String = str::from_utf8(&strbuf).unwrap().to_string();

        if !output.is_empty() {
            if container.contains("\n") {
                let index = strbuf.iter().rposition(|x| *x == 10);
                match index {
                    Some(index) => {
                        output.insert_str(output.len(), &container[..index]);
                    }
                    None => println!("Byte 10 representing newline characer not found"),
                }
                break;
            } else {
                output.push_str(&container);
            }
        }

        if container.contains("id=") {
            let mut tempcont = container;
            let offset = tempcont.find('=').unwrap();
            tempcont.replace_range(..offset, "");
            output = tempcont.to_string();
        } else {
            println!("still searching for line containing 'id='")
        }
        bytecount += 8;
    }
    return Ok(output);
}

//=====================================
//== Function for getting Mod Paths ===
//=====================================

fn pathcollect(source: &str) -> io::Result<Vec<String>> {
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

fn workidbuild(source: &str) -> io::Result<Vec<String>> {
    let mut workids: Vec<String> = Vec::new();
    for entry in fs::read_dir(source)? {
        let dir = entry?;
        let wid = dir
            .path()
            .to_str()
            .unwrap()
            .to_string()
            .replace(&source, "");
        workids.push(wid);
    }
    return Ok(workids);
}

//=================================================================
//=== Functions for recursively locating mod.info directories =====
//=================================================================

fn modidpathcollecter(source: Vec<String>) -> std::io::Result<Vec<String>> {
    let mut modinfos: Vec<String> = Vec::new();

    for val in source {
        collect_modids(&Path::new(&val), &mut modinfos);
    }
    return Ok(modinfos);
}

fn collect_modids(path: &Path, modinfos: &mut Vec<String>) -> std::io::Result<()> {
    if !path.is_file() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if path.to_str().unwrap().contains("mod.info") {
                modinfos.push(path.to_str().unwrap().to_string());
            } else if path.is_dir() {
                collect_modids(&path, modinfos);
            }
        }
    }
    Ok(())
}

//==========================================================================
//=== Functions for recursively locating map names and collecting them =====
//==========================================================================

fn mapnamecollect(source: Vec<String>) -> std::io::Result<Vec<String>> {
    let mut mapnames: Vec<String> = Vec::new();

    for val in source {
        collect_mapnames(&Path::new(&val), &mut mapnames)?;
    }

    return Ok(mapnames);
}

fn collect_mapnames(path: &Path, mapnames: &mut Vec<String>) -> std::io::Result<()> {
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
                            mapnames
                                .push(sub_path.to_str().unwrap().to_string().replace(&place, ""));
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

//=================================================================================================
//=== Functions for creating and writing to text file that contains all collected information =====
//=================================================================================================
