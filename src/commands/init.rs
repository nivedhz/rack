use std::{
    env,
    path::{Path, PathBuf},
};

pub fn initialize_rev_dir(dir_path: PathBuf) {
    if dir_path == Path::new(".") {
        println!("Initializing {}", env::current_dir().unwrap().display());
        return;
    }
    println!(
        "Initializing {}",
        std::path::absolute(&dir_path).unwrap().display()
    );
}
