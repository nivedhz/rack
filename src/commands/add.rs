use std::{
    fs,
    io::Write,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process,
};

use crate::constants::DIR_PATH;

pub struct AddCommand {
    path: PathBuf,
}

impl Default for AddCommand {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
        }
    }
}
impl AddCommand {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn init(&self) {
        if !Path::new(DIR_PATH).exists() {
            eprintln!("fatal: not a rack repository");
            process::exit(1);
        }
        self.create_stage_file();
        self.create_goods_dir();
    }
    fn create_goods_dir(&self) {
        let added_file_path = self.path.clone();
        let goods_dir_path = Path::new(DIR_PATH).join("goods");
        if !goods_dir_path.exists() {
            fs::create_dir_all(&goods_dir_path).unwrap();
        }
        let file_content = fs::read_to_string(&added_file_path).unwrap();
        let mut file = fs::File::options()
            .create(true)
            .append(true)
            .open(goods_dir_path.join(format!("{}.goods", added_file_path.display())))
            .unwrap();
        file.write_all(file_content.as_bytes()).unwrap();
    }
    fn create_stage_file(&self) {
        let added_file_path = self.path.clone();
        let stage_file_path = Path::new(DIR_PATH).join("STAGE");
        let mut file = fs::File::options()
            .create(true)
            .append(true)
            .open(stage_file_path)
            .unwrap();
        file.write_all(
            format!(
                "STAGED:{} {}\n",
                self.path.display(),
                added_file_path.metadata().unwrap().mtime()
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
