use log::{error, info};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
enum FileType {
    Image,
    Document,
    Music,
    Video,
    Unknown,
}

fn get_file_type_mapping() -> HashMap<&'static str, FileType> {
    let mut mapping = HashMap::new();
    for ext in ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"] {
        mapping.insert(ext, FileType::Image);
    }
    for ext in [
        "txt", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "md", "odt", "rtf",
    ] {
        mapping.insert(ext, FileType::Document);
    }
    for ext in ["mp3", "wav", "ogg", "flac", "aac", "m4a"] {
        mapping.insert(ext, FileType::Music);
    }
    for ext in ["mp4", "mkv", "avi", "mov", "wmv", "flv", "mpeg"] {
        mapping.insert(ext, FileType::Video);
    }
    mapping
}

fn get_files_in_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }
    Ok(files)
}

fn get_file_type(path: &Path, mapping: &HashMap<&str, FileType>) -> FileType {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            mapping
                .get(&ext.to_lowercase()[..])
                .cloned()
                .unwrap_or(FileType::Unknown)
        })
        .unwrap_or(FileType::Unknown)
}

fn get_folder_name(file_type: &FileType) -> &'static str {
    match file_type {
        FileType::Image => "images",
        FileType::Document => "documents",
        FileType::Music => "music",
        FileType::Video => "videos",
        FileType::Unknown => "others",
    }
}

fn move_file_to_folder(file: &Path, folder: &Path) -> io::Result<()> {
    if !folder.exists() {
        fs::create_dir_all(folder)?;
    }
    let file_name = file.file_name().unwrap();
    let target_path = folder.join(file_name);
    if target_path.exists() {
        error!("File already exists: {}", target_path.display());
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Target file already exists",
        ));
    }

    match fs::rename(file, &target_path) {
        Ok(_) => {
            info!("Moved {} to {}", file.display(), target_path.display());
            Ok(())
        }
        Err(e) => {
            error!("Rename failed, trying copy/delete: {}", e);
            fs::copy(file, &target_path)?;
            fs::remove_file(file)?;
            info!("Copied and deleted original file: {}", file.display());
            Ok(())
        }
    }
}

fn main() -> Result<(), io::Error> {
    let directory_path =
        PathBuf::from(r"C:\Users\akkum\OneDrive\Desktop\Code\project\rust\testfolder");
    let mapping = get_file_type_mapping();

    let files = get_files_in_directory(&directory_path)?;

    for file in files {
        let file_type = get_file_type(&file, &mapping);
        let folder_name = get_folder_name(&file_type);
        let target_folder = directory_path.join("organized").join(folder_name);

        println!(
            "Moving {:?} to {:?}",
            file.file_name().unwrap(),
            target_folder
        );

        if let Err(e) = move_file_to_folder(&file, &target_folder) {
            error!("Failed to move file {}: {}", file.display(), e);
        }
    }

    println!("All files have been organized.");
    Ok(())
}
