use std::{fs, path::Path};

pub fn cleanup(path: impl AsRef<Path>, exclude: &'static [&'static str]) -> anyhow::Result<()> {
    if !path.as_ref().exists() {
        return fs::create_dir(path).map_err(Into::into);
    }

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        match path.components().next_back().and_then(|c| c.as_os_str().to_str()) {
            Some(path) if exclude.contains(&path) => {}
            _ => {
                if path.metadata()?.file_type().is_dir() {
                    fs::remove_dir_all(path)?
                } else {
                    fs::remove_file(path)?
                }
            }
        }
    }

    Ok(())
}
