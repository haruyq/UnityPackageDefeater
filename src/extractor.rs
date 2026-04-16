use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tar::Archive;
use anyhow::Result;
use flate2::read::GzDecoder;

use crate::model::GuidEntry;

fn write(entries: HashMap<String, GuidEntry>, output_dir: &Path, meta: &bool) -> io::Result<()> {
    for (_, entry_data) in entries {
        let pathname = match entry_data.pathname {
            Some(p) => p,
            None => continue,
        };

        let target_path = output_dir.join(&pathname);

        if let Some(asset) = entry_data.asset {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
                println!("Create: {}", parent.display());
            }
            fs::write(&target_path, asset)?;
            println!("Extract: {}", target_path.display());
        } else {
            fs::create_dir_all(&target_path)?;
            println!("Create: {}", target_path.display());
        }

        if *meta &&
            let Some(asset_meta) = entry_data.asset_meta {
                let mut meta_os = target_path.clone().into_os_string();
                meta_os.push(".meta");
                let meta_path = PathBuf::from(meta_os);

                if let Some(parent) = meta_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                println!("Extract: {}", meta_path.display());
                fs::write(meta_path, asset_meta)?;
        }
    };

    Ok(())
}

fn get_entries(archive: &mut Archive<GzDecoder<File>>) -> Result<HashMap<String, GuidEntry>> {
    let mut entries = HashMap::new();

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();

        let components: Vec<_> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .filter(|s| s != ".")
            .collect();

        if components.len() != 2 {
            continue;
        }

        let guid = components[0].clone();
        let filename = components[1].clone();
        let entry_data: &mut GuidEntry = entries.entry(guid.clone()).or_default();
        
        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer)?;

        match filename.as_str() {
            "pathname" => {
                let text = String::from_utf8_lossy(&buffer).trim().to_string();
                entry_data.pathname = Some(text);
            }
            "asset" => {
                entry_data.asset = Some(buffer);
            }
            "asset.meta" => {
                entry_data.asset_meta = Some(buffer);
            }
            _ => {}
        }
    }
    Ok(entries)
}

fn decompress(path: &str) -> Result<Archive<GzDecoder<File>>> {
    let f = File::open(path)?;
    let gz = GzDecoder::new(f);
    let data = Archive::new(gz);
    Ok(data)
}

pub fn extract(path: &str, output_dir: &Path, meta: &bool) -> Result<()> {
    let mut archive = decompress(path)?;
    let entries = get_entries(&mut archive)?;

    write(entries, output_dir, meta)?;
    Ok(())
}