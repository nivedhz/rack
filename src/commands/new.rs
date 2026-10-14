use tracing::info;

use crate::{args::NewFlags, constants::DIR_PATH};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf, absolute},
    process,
};

pub struct DeflowerCommand {
    flags: NewFlags,
    dir_path: PathBuf,
}
impl DeflowerCommand {
    pub fn new(flags: NewFlags, dir_path: PathBuf) -> Self {
        Self { flags, dir_path }
    }
    fn get_path(&self) -> PathBuf {
        if self.flags.default || self.dir_path == Path::new(".") {
            match env::current_dir() {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("fatal: failed to get current directory!");
                    process::exit(1);
                }
            }
        } else {
            match absolute(&self.dir_path) {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("fatal: failed to get absolute path!");
                    process::exit(1);
                }
            }
        }
    }
    pub fn run(&self) {
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
        self.create_baby_file();
        self.create_hump_dir().unwrap_or_else(|e| {
            eprintln!("fatal: failed to create hump directory! {}", e);
            process::exit(1);
        });
        self.create_goods_dir();
        self.create_rack_ignore_file();
    }
    fn create_rack_dir(&self) {
        let path = &self.path;
        let exist_status = fs::exists(path).unwrap_or(false) && path.is_dir();

        match fs::create_dir_all(path) {
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
                    eprintln!("fatal: failed to create rack directory!");
                    process::exit(1);
                }
            },
        }
    }
    fn create_baby_file(&self) {
        let path = &self.path;
        match File::create(path.join("BABY")) {
            Ok(mut f) => {
                info!("Created baby file in {}", path.display());
                f.write_all(b"hump: hump/baby/bonkers").unwrap_or_else(|e| {
                    eprintln!("fatal: failed to create baby file! {}", e);
                    process::exit(1);
                });
                info!("Wrote the baby file in {}", path.display())
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("Reinitialized a baby file in {}", path.display())
                }
                e => {
                    eprintln!("fatal: failed to create baby file! {}", e);
                    process::exit(1);
                }
            },
        }
    }
    fn create_hump_dir(&self) -> std::io::Result<()> {
        let path = &self.path;
        if fs::create_dir_all(path.join("hump/baby")).is_ok() {
            info!("Created hump directory in {}", path.display());
            let mut file = File::create(path.join("hump/baby/bonkers"))?;
            file.write_all(b"bonkers")?;
            info!("Created hump/baby/bonkers in {}", path.display());
        }
        Ok(())
    }
    fn create_goods_dir(&self) {
        let path = &self.path;
        if fs::create_dir_all(path.join("goods")).is_ok() {
            info!("Created goods directory in {}", path.display());
        } else {
            eprintln!("fatal: failed to create goods directory!");
            process::exit(1);
        }
    }
    fn create_rack_ignore_file(&self) {
        let path = match self.path.parent() {
            Some(p) => p,
            None => {
                eprintln!("fatal: failed to get parent directory!");
                process::exit(1);
            }
        };
        let mut file = File::create(path.join(".rackignore")).unwrap_or_else(|e| {
            eprintln!("fatal: failed to create .rackignore file! {}", e);
            process::exit(1);
        });
        file.write_all(b"# Enter the relative or absolute paths of the files or directories you want to ignore.\n")
            .unwrap_or_else(|e| {
                eprintln!("fatal: failed to write to .gitignore file! {}", e);
                process::exit(1);
            });
    }
}
