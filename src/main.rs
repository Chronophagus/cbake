mod arg_parser;
mod cmake;
mod error;
mod project_layout;

use error::{ExecutionError, Failure, FatalError};
use project_layout::ProjectLayout;

use ansi_term::Colour;

use std::{error::Error, fmt, io::Read, path::PathBuf};

type ExecutionResult = Result<(), ExecutionError>;

fn main() {
    let command = arg_parser::parse_command();

    if let Err(e) = exec_command(command) {
        eprintln!("{}", e);
    } else {
        println!("  {}", Colour::Green.bold().paint("Success"));
    }
}

fn exec_command(cmd: arg_parser::Command) -> ExecutionResult {
    use arg_parser::Command::*;

    match cmd {
        Init {} => exec_init(),
        New { path } => exec_new(path),
        Build { release } => exec_build(release),
        Run { release } => exec_run(release),
        Clean {} => exec_clean(),
    }
}

fn exec_init() -> ExecutionResult {
    unimplemented!()
}

fn exec_new(project_path: PathBuf) -> ExecutionResult {
    let project_name = parse_project_name(&project_path)?;

    let cmake_ver = cmake::exec::version().map_err(error::version_err)?;

    println!(
        "  {} {}",
        Colour::Green.bold().paint("Creating"),
        Colour::White.bold().paint(&project_name)
    );

    let project_root = std::path::PathBuf::from(&project_path);
    let mut layout = project_layout::Simple::new(project_root);

    layout.generate().map_err(error::layout_gen_err)?;
    let source_list = layout.collect_sources().map_err(error::collect_sources_err)?;

    let cmake_lists = cmake::Builder::new(project_name, cmake::generator::from_version(cmake_ver))
        .cpp_standard(11)
        .sources(source_list)
        .build();

    layout.write_file("CMakeLists.txt", cmake_lists.as_bytes()).map_err(error::cmake_write_err)?;

    let project_path = layout.get_project_path();
    let build_dir = layout.get_build_path();

    cmake::exec::init(&project_path, &build_dir).map_err(error::init_err)?;

    Ok(())
}

fn exec_build(release: bool) -> ExecutionResult {
    let project_path = std::env::current_dir().map_err(error::dir_access_err)?;

    let project_name = parse_project_name(&project_path)?;

    println!(
        "  {} {} ",
        Colour::Green.bold().paint("Building"),
        Colour::White.bold().paint(project_name)
    );

    let mut layout = project_layout::Simple::new(project_path);

    let mut sources = layout.collect_sources().map_err(error::collect_sources_err)?;
    sources.sort();

    let mut cmake_file = layout.open_file("CMakeLists.txt").map_err(error::cmake_read_err)?;

    let mut cmake_lists = String::new();
    cmake_file.read_to_string(&mut cmake_lists).map_err(error::cmake_read_err)?;
    
    let add_executable_command = cmake::generator::add_executable(&sources);

    if !replace_command(&mut cmake_lists, "add_executable", &add_executable_command) {
        cmake_lists.push_str(&add_executable_command);    
    }

    layout.write_file("CMakeLists.txt", cmake_lists.as_bytes()).map_err(error::cmake_write_err)?;
    let build_dir = layout.get_build_path();
    let source_dir = layout.get_project_path();

    let build_type = if release {"Release"} else {"Debug"};
    let output_dir = format!("./{}", build_type);

    cmake::exec::InitExtBuilder::new()
        .set_var("CMAKE_BUILD_TYPE", build_type)
        .set_var("CMAKE_RUNTIME_OUTPUT_DIRECTORY", &output_dir)
        .execute(&source_dir, &build_dir).map_err(error::init_err)?;

    cmake::exec::build(&build_dir).map_err(|e| {
        match e {
            cmake::exec::ExecutionError::CMake(_) => ExecutionError::from(Failure),
            cmake::exec::ExecutionError::IO(err) => ExecutionError::from(FatalError::new(Box::new(err), "Cannot run build command"))
        }
    })?;

    Ok(())
}

fn exec_run(release: bool) -> ExecutionResult {
    exec_build(release)?;
    println!("  {}", Colour::Green.bold().paint("Success"));

    let project_path = std::env::current_dir().map_err(error::dir_access_err)?;
    let project_name = parse_project_name(&project_path)?;

    println!("  {} {}", 
        Colour::Green.bold().paint("Running"), 
        Colour::White.bold().paint(&project_name));

    let mut executable_path = project_layout::Simple::new(project_path).get_build_path();

    if release == false { executable_path.push("Debug") } else { executable_path.push("Release"); }
    executable_path.push(project_name);

    std::process::Command::new(executable_path)
        .stdout(std::process::Stdio::inherit())
        .output().map_err(|e| FatalError::new(Box::new(e), "Cannot run executable"))?;

    std::process::exit(0);
}

fn exec_clean() -> ExecutionResult {
    unimplemented!()
}

fn parse_project_name(project_path: &PathBuf) -> Result<String, FatalError> {
    let project_name = project_path
        .file_name()
        .ok_or(FatalError::with_help(
            Box::new(InvalidPath), 
            "Cannot parse a project name", 
            "This command creates a new directory with project contents. The path must end with a non existing directory name"
        ))?
        .to_str()
        .ok_or(FatalError::with_help(
            Box::new(InvalidProjectName), 
            "Cannot parse a project name",
            "Project name must consist only of ASCII characters excluding whitespaces"
        ))?
        .to_string();

    if !project_name.is_ascii() || project_name.contains(" ") 
    {
        Err(FatalError::with_help(
            Box::new(InvalidProjectName), 
            "Project name verification failed",
            "Project name must consist only of ASCII characters excluding whitespaces"
        ))
    }
    else {
        Ok(project_name)
    }
}

// Returns true if command was replaced
fn replace_command(buf: &mut String, command_name: &str, new_command_text: &str) -> bool {
    let command_range = buf
        .find(command_name)
        .and_then(|start_pos| {
            buf[start_pos..]
                .find(')')
                .map(|i| (start_pos, i + start_pos + 1))
        });


    if let Some((start, end)) = command_range {
        buf.replace_range(start..end, new_command_text.trim());
        true
    }
    else {
        false
    }
}

#[derive(Debug)]
struct InvalidPath;

impl fmt::Display for InvalidPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "invalid path")
    }
}

impl Error for InvalidPath {
    fn description(&self) -> &str {
        "invalid path"
    }
}

#[derive(Debug)]
struct InvalidProjectName;

impl fmt::Display for InvalidProjectName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Invalid project name")
    }
}

impl Error for InvalidProjectName {
    fn description(&self) -> &str {
        "Invalid project name"
    }
}

