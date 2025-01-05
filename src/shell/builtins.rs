use crate::shell::{get_home_dir, ShellStatus};
use std::{
    collections::HashMap,
    env::{self},
    path::Path,
};

pub struct Builtins {
    pub commands: HashMap<String, fn(Vec<String>) -> ShellStatus>,
}

impl Builtins {
    pub fn new() -> Self {
        let mut commands: HashMap<String, fn(Vec<String>) -> ShellStatus> = HashMap::new();

        commands.insert(String::from("exit"), builtin_exit);
        commands.insert(String::from("help"), builtin_help);
        commands.insert(String::from("cd"), builtin_cd);

        Self { commands }
    }
}

fn builtin_cd(args: Vec<String>) -> ShellStatus {
    if args.len() < 2 {
        let home_dir = match get_home_dir() {
            Ok(path) => path,
            Err(error) => {
                eprintln!("Failed to get $HOME: {}", error);
                return ShellStatus::Exit;
            }
        };
        // TODO: remove assert because it blows up when incorrect path is given
        assert!(env::set_current_dir(Path::new(&home_dir)).is_ok());
    } else {
        let (_command, destination_path) = args.split_first().unwrap();

        assert!(env::set_current_dir(Path::new(&destination_path[0])).is_ok());
    }
    ShellStatus::Running
}

fn builtin_exit(_args: Vec<String>) -> ShellStatus {
    ShellStatus::Exit
}

fn builtin_help(_args: Vec<String>) -> ShellStatus {
    println!("Help!");

    ShellStatus::Running
}
