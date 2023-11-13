use std::fs::{self};
use std::path::Path;
use std::process::Command;

fn convert_m4a_to_mp3_and_delete_original(input_file_path: &Path, output_file_path: &Path) {
    match Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file_path)
        .arg(output_file_path)
        .status()
    {
        Ok(_) => {
            println!("Converted: {:?} to {:?}", input_file_path, output_file_path);

            match fs::remove_file(input_file_path) {
                Ok(_) => println!("Deleted original file: {:?}", input_file_path),
                Err(e) => eprintln!("Error deleting file {:?}: {}", input_file_path, e),
            }
        }
        Err(e) => eprintln!("Error converting file {:?}: {}", input_file_path, e),
    }
}

fn search_directory(dir: &str) {
    let paths = match fs::read_dir(dir) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading directory {}: {}", dir, e);
            return;
        }
    };

    for path in paths {
        let entry = match path {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading path: {}", e);
                continue;
            }
        };

        let full_path = entry.path();
        let metadata = match fs::metadata(&full_path) {
            Ok(meta) => meta,
            Err(e) => {
                eprintln!("Error reading metadata for {:?}: {}", full_path, e);
                continue;
            }
        };

        if metadata.is_dir() {
            search_directory(full_path.to_str().unwrap());
        } else if let Some(ext) = full_path.extension() {
            if ext == "m4a" {
                let mp3_path = full_path.with_extension("mp3");
                convert_m4a_to_mp3_and_delete_original(&full_path, &mp3_path);
            }
        }
    }
}

fn main() {
    let main_directory = "./files/";
    search_directory(main_directory);
}