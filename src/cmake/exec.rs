/// This module executes cmake commands
use super::version::Version;

use regex::Regex;

use std::{
    error::Error,
    ffi::OsString,
    fmt, io,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn version() -> Result<Version, Box<dyn std::error::Error>> {
    let exec_status = Command::new("cmake").arg("--version").output()?;

    let command_output = String::from_utf8(exec_status.stdout)?;

    // Pattern to search version numbers like 1.2 or 1.2.3
    let ver_pattern = Regex::new(r"(\d+\.){1,2}(\d+)").unwrap();

    let version: Version = ver_pattern
        .find_iter(&command_output)
        .next() // Get first match
        .unwrap() // If command executed without errors the version string must be in the command output
        .as_str() // Convert regex::Match to str
        .parse()?; // Convert to Version type

    Ok(version)
}

pub fn init(project_dir: &PathBuf, build_dir: &PathBuf) -> Result<(), ExecutionError> {
    let mut arg1 = OsString::from("-B");
    let mut arg2 = OsString::from("-H");

    arg1.push(build_dir.as_os_str());
    arg2.push(project_dir.as_os_str());

    let output = Command::new("cmake")
        .arg(arg1)
        .arg(arg2)
        .stdout(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let exit_code = output.status.code().unwrap_or(-1);
        Err(ExecutionError::from(CMakeFailure(exit_code)))
    }
}

pub fn build(build_dir: &PathBuf) -> Result<(), ExecutionError> {
    let output = Command::new("cmake")
        .arg("--build")
        .arg(build_dir.as_os_str())
        .stdout(Stdio::inherit())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        let exit_code = output.status.code().unwrap_or(-1);
        Err(ExecutionError::from(CMakeFailure(exit_code)))
    }
}

pub struct InitExtBuilder {
    variables: Vec<String>,
}

impl InitExtBuilder {
    pub fn new() -> Self {
        InitExtBuilder {
            variables: Vec::new(),
        }
    }

    pub fn set_var(mut self, name: &str, value: &str) -> Self {
        self.variables.push(format!("-D{}={}", name, value));
        self
    }

    pub fn execute(self, source_dir: &PathBuf, build_dir: &PathBuf) -> Result<(), ExecutionError> {
        let optional_args = self.variables;
        let mut arg1 = OsString::from("-B");
        let mut arg2 = OsString::from("-H");

        arg1.push(build_dir.as_os_str());
        arg2.push(source_dir.as_os_str());

        let output = Command::new("cmake")
            .args(optional_args)
            .arg(arg1)
            .arg(arg2)
            .stdout(Stdio::inherit())
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let exit_code = output.status.code().unwrap_or(-1);
            Err(ExecutionError::from(CMakeFailure(exit_code)))
        }
    }
}

#[derive(Debug)]
pub struct CMakeFailure(i32);

impl Error for CMakeFailure {
    fn description(&self) -> &str {
        "CMake failure"
    }
}

impl fmt::Display for CMakeFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("cmake failed with exit code: {}", self.0))
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    IO(io::Error),
    CMake(CMakeFailure),
}

impl Error for ExecutionError {
    fn description(&self) -> &str {
        "Error while executing a CMake command"
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::IO(err) => write!(f, "{}", err),
            ExecutionError::CMake(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for ExecutionError {
    fn from(err: io::Error) -> Self {
        ExecutionError::IO(err)
    }
}

impl From<CMakeFailure> for ExecutionError {
    fn from(err: CMakeFailure) -> Self {
        ExecutionError::CMake(err)
    }
}
