use super::generator::{self, Generator};

pub struct Builder {
    generator: Box<dyn Generator>,

    project_name: String,
    sources: Vec<String>,
    include_dirs: Vec<String>, // or PathBuf?
    default_build_type: Option<String>,
    cpp_standard: Option<String>,
}

impl Builder {
    pub fn new(project_name: String, generator: Box<dyn Generator>) -> Self {
        Builder {
            generator,

            project_name,
            sources: Vec::new(),
            include_dirs: Vec::new(),
            default_build_type: None,
            cpp_standard: None,
        }
    }

    pub fn default_build_type(mut self, build_type: String) -> Self {
        self.default_build_type = Some(build_type);
        self
    }

    pub fn cpp_standard(mut self, standard: u8) -> Self {
        self.cpp_standard = Some(standard.to_string());
        self
    }

    pub fn include_dirs(mut self, mut dirs: Vec<String>) -> Self {
        self.include_dirs.append(&mut dirs);
        self
    }

    pub fn sources(mut self, mut sources: Vec<String>) -> Self {
        self.sources.append(&mut sources);
        self
    }

    pub fn build(self) -> String {
        let mut cmake_lists = String::new();

        cmake_lists += &self.generator.min_ver();
        cmake_lists += &generator::project(&self.project_name);

        if let Some(build_type) = self.default_build_type {
            cmake_lists += &generator::default_build_type(&build_type);
        }

        if let Some(standard) = self.cpp_standard {
            cmake_lists += &generator::set_var("CMAKE_CXX_STANDARD", &standard);
            cmake_lists.push('\n');
        }

        if !self.include_dirs.is_empty() {
            cmake_lists += &generator::include_dirs(&self.include_dirs);
        }

        if !self.sources.is_empty() {
            cmake_lists += &generator::comment("-------- Warning: This section will be overwritten by cbake utility. Don't change it manually if you will use it --------");
            cmake_lists += &generator::add_executable(&self.sources);
            cmake_lists += &generator::comment("--------------------------------------------------------------------------------------------------------------------------");
        }

        cmake_lists
    }
}
