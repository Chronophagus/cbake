use std::error::Error;
use std::fmt;

use ansi_term::Colour;

// Errors to display to the user
pub fn version_err(err: Box<dyn Error + 'static>) -> FatalError {
    FatalError::with_help(
            err,
            "Cannot obtain a cmake version",
            "Make sure cmake is installed and present in PATH environment variable. Run `cmake --version` to verify cmake installation",
        )
}

pub fn init_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::new(Box::new(err), "Cannot initialize cmake build directory")
}

pub fn layout_gen_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::new(Box::new(err), "Cannot generate a project layout")
}

pub fn collect_sources_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::new(Box::new(err), "Cannot collect project sources")
}

pub fn dir_access_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::new(Box::new(err), "Cannot access project directory")
}

pub fn cmake_write_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::new(Box::new(err), "Cannot write CMakeLists.txt")
}

pub fn cmake_read_err<E>(err: E) -> FatalError
where
    E: Error + 'static,
{
    FatalError::with_help(
        Box::new(err),
        "Cannot read CMakeLists.txt",
        "Make sure you are located within a project directory",
    )
}

// Any error that prevents command execution
#[derive(Debug)]
pub struct FatalError {
    error: Box<dyn Error>,
    what: String,
    help: Option<String>,
}

impl FatalError {
    pub fn new<S: AsRef<str>>(error: Box<dyn Error>, what: S) -> Self {
        FatalError {
            error,
            what: String::from(what.as_ref()),
            help: None,
        }
    }

    pub fn with_help<S: AsRef<str>>(error: Box<dyn Error>, what: S, help: S) -> Self {
        FatalError {
            error,
            what: String::from(what.as_ref()),
            help: Some(String::from(help.as_ref())),
        }
    }
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "  {} {}({}) ",
            Colour::Red.bold().paint("Error:"),
            self.what,
            self.error.description()
        )?;

        if let Some(help) = &self.help {
            writeln!(f, "  {} {}", Colour::Yellow.bold().paint("Help:"), help)?;
        }

        Ok(())
    }
}

impl Error for FatalError {
    fn description(&self) -> &str {
        self.error.description()
    }
}

// Indicates that command executed but without success
#[derive(Debug)]
pub struct Failure;

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  {}", Colour::Red.bold().paint("Failure"))
    }
}

impl Error for Failure {
    fn description(&self) -> &str {
        "Failure"
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    Fatal(FatalError),
    Failure(Failure),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ExecutionError::*;
        match self {
            Fatal(err) => write!(f, "{}", err),
            Failure(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ExecutionError {
    fn description(&self) -> &str {
        use ExecutionError::*;
        match self {
            Fatal(err) => err.description(),
            Failure(err) => err.description(),
        }
    }
}

impl From<FatalError> for ExecutionError {
    fn from(err: FatalError) -> Self {
        ExecutionError::Fatal(err)
    }
}

impl From<Failure> for ExecutionError {
    fn from(err: Failure) -> Self {
        ExecutionError::Failure(err)
    }
}
