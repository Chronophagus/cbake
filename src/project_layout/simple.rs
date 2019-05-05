// ProjectRoot/
// --- build/
// ------ Debug/
// ------ Release/
// ------ build files...
// --- source files...
// --- CMakeLists.txt

use super::ProjectLayout;
use super::MAIN_CPP_CONTENTS;

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

const BUILD_PATH: &str = "build";
const DEBUG_TARGET_PATH: &str = "build/Debug";
const RELEASE_TARGET_PATH: &str = "build/Release";

pub struct Simple {
    project_root: PathBuf,
}

impl Simple {
    pub fn new(path: PathBuf) -> Self {
        Simple { project_root: path }
    }
}

impl ProjectLayout for Simple {
    fn generate(&self) -> io::Result<()> {
        fs::create_dir(&self.project_root)?;
        let _dir_guard = change_dir(&self.project_root)?;

        fs::create_dir_all(DEBUG_TARGET_PATH)?;
        fs::create_dir_all(RELEASE_TARGET_PATH)?;

        let mut f = fs::File::create("main.cpp")?;

        f.write(MAIN_CPP_CONTENTS.as_bytes())?;

        Ok(())
    }

    fn collect_include_dirs(&self) -> io::Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn collect_sources(&self) -> io::Result<Vec<String>> {
        let mut sources = Vec::new();
        let _dir_guard = change_dir(&self.project_root)?;

        for entry in fs::read_dir(".")? {
            let entry = entry?;

            let entry_path = entry.path();
            if entry.file_type()?.is_file() {
                if let Some(extension) = entry_path.extension() {
                    if extension == "cpp"
                        || extension == "h"
                        || extension == "hpp"
                        || extension == "c"
                    {
                        sources.push(entry_path.into_os_string().into_string().unwrap());
                    }
                }
            }
        }

        Ok(sources)
    }

    fn get_build_path(&self) -> PathBuf {
        let mut ret = self.project_root.clone();
        ret.push(BUILD_PATH);

        ret
    }

    fn get_project_path(&self) -> PathBuf {
        self.project_root.clone()
    }

    fn write_file(&mut self, file_name: &str, contents: &[u8]) -> io::Result<()> {
        let _dir_guard = change_dir(&self.project_root)?;

        let mut f = fs::File::create(file_name)?;
        f.write(contents)?;

        Ok(())
    }

    fn open_file(&self, file_name: &str) -> io::Result<fs::File> {
        let _dir_guard = change_dir(&self.project_root)?;

        fs::File::open(file_name)
    }

    fn create_dir(&mut self, dir_name: &str) -> io::Result<()> {
        let _dir_guard = change_dir(&self.project_root)?;

        fs::create_dir(dir_name)?;

        Ok(())
    }
}

// This type allows to change working directory temporary until instance of this type exists
struct ChangeDirGuard {
    prev_dir: PathBuf,
}

impl Drop for ChangeDirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.prev_dir);
    }
}

fn change_dir(new_dir: &PathBuf) -> io::Result<ChangeDirGuard> {
    let prev_dir = std::env::current_dir()?;
    std::env::set_current_dir(new_dir)?;

    Ok(ChangeDirGuard { prev_dir })
}
