use tracing::info;

use crate::{args::NewFlags, constants::DIR_PATH};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, absolute},
};

pub struct NewCommand {
    flags: NewFlags,
    dir_path: PathBuf,
}
impl NewCommand {
    pub fn new(flags: NewFlags, dir_path: PathBuf) -> Self {
        Self { flags, dir_path }
    }
    fn get_path(&self) -> PathBuf {
        if self.flags.default || self.dir_path == Path::new(".") {
            match env::current_dir() {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("Failed to get current directory!");
                    std::process::exit(1);
                }
            }
        } else {
            match absolute(&self.dir_path) {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("Failed to get absolute path!");
                    std::process::exit(1);
                }
            }
        }
    }
    pub fn init(&self) {
        println!();
        RackFolder::new(self.get_path().join(DIR_PATH)).init();
    }
}

struct RackFolder {
    path: PathBuf,
}
impl RackFolder {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
    fn init(&self) {
        self.create_rack_dir();
        self.create_hump_file();
        self.create_hump_dir();
    }
    fn create_rack_dir(&self) {
        let path = self.path.clone();
        let exist_status = fs::exists(&path).unwrap_or(false) && path.is_dir();

        match fs::create_dir_all(&path) {
            Ok(_) => {
                if exist_status {
                    println!("Reinitialized a rack repository in {}", path.display());
                } else {
                    println!("Initialized a new rack repository in {}", path.display());
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("Reinitialized a rack repository in {}", path.display())
                }
                _ => {
                    eprintln!("Failed to create rack directory!");
                    std::process::exit(1);
                }
            },
        }
    }
    fn create_hump_file(&self) {
        let path = self.path.clone();
        match File::create(path.join("HUMP")) {
            Ok(mut f) => {
                info!("Created hump file in {}", path.display());
                match f.write_all(b"hump: hump/baby/bonkers") {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Failed to write the hump file in {}", e);
                        eprintln!("Failed to create hump file!");
                        std::process::exit(1);
                    }
                }
                info!("Wrote the hump file in {}", path.display())
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("Reinitialized a hump file in {}", path.display())
                }
                e => {
                    eprintln!("Failed to create hump file! {}", e);
                    std::process::exit(1);
                }
            },
        }
    }
    fn create_hump_dir(&self) {
        let path = self.path.clone();
        if fs::create_dir(path.join("hump")).is_ok() {
            info!("Created hump directory in {}", path.display());
            match fs::create_dir(path.join("hump/baby")) {
                Ok(_) => {
                    info!("Created hump/baby directory in {}", path.display());
                }
                Err(e) => {
                    eprintln!("Failed to create hump/baby directory! {}", e);
                    std::process::exit(1);
                }
            }
            if let Ok(mut f) = fs::File::create(path.join("hump/baby/bonkers"))
                && let Ok(_) = f.write_all(b"bonkers")
            {
                info!("Created hump/baby/bonkers in {}", path.display());
            }
        }
    }
}
