pub mod simple;

pub use simple::Simple;

use std::{fs, io, path::PathBuf};

const MAIN_CPP_CONTENTS: &str = include_str!("../../resources/main.cpp");

pub trait ProjectLayout {
    // Generate project files
    fn generate(&self) -> io::Result<()>;
    fn collect_include_dirs(&self) -> io::Result<Vec<String>>;
    fn collect_sources(&self) -> io::Result<Vec<String>>;

    fn get_build_path(&self) -> PathBuf;

    // Must return path where uppermost CMakeLists.txt is located
    fn get_project_path(&self) -> PathBuf;

    // Add custom file to layout
    fn write_file(&mut self, file_name: &str, contents: &[u8]) -> io::Result<()>;

    fn open_file(&self, file_name: &str) -> io::Result<fs::File>;

    // Add custom directory to layout
    fn create_dir(&mut self, dir_name: &str) -> io::Result<()>;
}
