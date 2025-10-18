use std::{env, fs, io, path::PathBuf};

pub fn parse_path(path_string: &String) -> Result<PathBuf, String> {
    let expanded_path: PathBuf = if path_string.starts_with("~/") {
        let home_dir = env::home_dir().ok_or("Could not determine home directory")?;
        let relative_path = &path_string[2..];
        home_dir.join(relative_path)
    } else {
        PathBuf::from(path_string)
    };

    if !expanded_path.exists() {
        return Err(format!(
            "Error: Specified path ({}) does not exist",
            expanded_path.display()
        ));
    }

    return Ok(expanded_path);
}

pub fn input_path() -> Result<PathBuf, String> {
    let mut input_buffer = String::new();
    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: {}", err));
        }
    };

    input_buffer = input_buffer.trim().to_string();

    return parse_path(&input_buffer);
}

pub fn get_all_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    if path.is_file() {
        files.push(path);
        return files;
    }

    let dir_entries = match fs::read_dir(&path) {
        Ok(rd) => rd,
        Err(err) => {
            eprintln!("Error reading dir {}: {}", path.display(), err);
            return files;
        }
    };

    for entry in dir_entries {
        match entry {
            Ok(entry) => {
                files.append(&mut get_all_files(entry.path()));
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    return files;
}

pub fn get_all_files_with_extension(path: PathBuf, extension: &String) -> Vec<PathBuf> {
    let ext_str = if extension.starts_with('.') {
        &extension[1..]
    } else {
        extension.as_str()
    };

    return get_all_files(path)
        .into_iter()
        .filter(|path| match path.extension() {
            Some(ext) => ext == ext_str,
            None => false,
        })
        .collect();
}
