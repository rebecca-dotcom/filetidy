
use std::fs;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("❌ Failed to read current directory.");

    println!("📁 Organizing files in: {}", current_dir.display());

    for entry in fs::read_dir(&current_dir).expect("❌ Failed to read directory contents.") {
        let entry = entry.expect("❌ Failed to read entry.");
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_str().unwrap();
                let folder_path = current_dir.join(ext_str);

                if !folder_path.exists() {
                    fs::create_dir(&folder_path).expect("❌ Failed to create folder.");
                }

                let file_name = path.file_name().unwrap();
                let new_path = folder_path.join(file_name);
                fs::rename(&path, &new_path).expect("❌ Failed to move file.");

                println!("✅ Moved {} to folder: {}", file_name.to_string_lossy(), ext_str);
            }
        }
    }

    println!("🎉 Done! All files sorted by extension.");
}
