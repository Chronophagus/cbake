use super::version::Version;

pub fn from_version(ver: Version) -> Box<dyn Generator> {
    if ver < Version::new(3, 0) {
        Box::new(CMake_2_8)
    } else {
        Box::new(CMake_3_x)
    }
}

// Version independent commands

pub fn min_ver(version: &Version) -> String {
    format!(
        "cmake_minimum_required(VERSION {})\n\n",
        version.to_string()
    )
}

pub fn set_var(var_name: &str, var_val: &str) -> String {
    format!("set({} {})\n", var_name, var_val)
}

pub fn project(name: &str) -> String {
    format!("project({})\n\n", name)
}

pub fn comment(s: &str) -> String {
    format!("#{}\n", s)
}

pub fn default_build_type(build_type: &str) -> String {
    format!(
        r#"
if(NOT CMAKE_BUILD_TYPE) 
set(CMAKE_BUILD_TYPE, "{}")
endif()

"#,
        build_type
    )
}

pub fn include_dirs(dir_names: &[String]) -> String {
    let dir_list = dir_names.join("\n");
    format!("include_directories(\n    {}\n)\n\n", dir_list)
}

pub fn add_executable(sources: &[String]) -> String {
    let source_list = sources.join("\n    ");
    format!(
        "\nadd_executable(${{PROJECT_NAME}}\n    {}\n)\n\n",
        source_list
    )
}

pub trait Generator {
    fn version(&self) -> Version;

    fn min_ver(&self) -> String {
        min_ver(&self.version())
    }

    // TODO: Add version dependent commands
}

pub struct CMake_2_8;

impl Generator for CMake_2_8 {
    fn version(&self) -> Version {
        Version::new(2, 8)
    }
}

pub struct CMake_3_x;

impl Generator for CMake_3_x {
    fn version(&self) -> Version {
        Version::new(3, 1)
    }
}
