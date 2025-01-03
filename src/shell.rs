mod builtins;

use builtins::*;

use std::{
    collections::HashMap,
    env::{self},
    fs::OpenOptions,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use owo_colors::OwoColorize;

pub enum ShellStatus {
    Running,
    Exit,
}

fn get_home_dir() -> Result<String, String> {
    env::var("HOME").map_err(|error| format!("Failed to get $HOME: {}", error))
}

fn save_history(args: &Vec<String>) -> ShellStatus {
    let home_dir = match get_home_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Failed to get $HOME: {}", error);
            return ShellStatus::Exit;
        }
    };

    let default_history_path = Path::new(&home_dir).join(".gsh_history");

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(default_history_path);

    match file {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", args.join(" ")) {
                eprintln!("Couldn't save command to the history file - {}", e)
            }
        }
        _ => eprintln!("oh no!"),
    }

    ShellStatus::Running
}


fn split_by_whitespace(to_split: String) -> Vec<String> {
    to_split.split_whitespace().map(str::to_string).collect()
}

pub struct Shell {
    builtins: Builtins,
    aliases: HashMap<String, Vec<String>>,
}

impl Shell {
    pub fn new() -> Self {
        let mut aliases = HashMap::new();

        aliases.insert(
            String::from("lg"),
            split_by_whitespace(String::from("lazygit")),
        );
        aliases.insert(
            String::from("gs"),
            split_by_whitespace(String::from("git status -s -b")),
        );

        Self {
            builtins: Builtins::new(),
            aliases,
        }
    }

    pub fn run(&self) {
        println!("Welcome to {}!", "green-shell".green());

        let mut status = ShellStatus::Running;
        while let ShellStatus::Running = status {
            self.prompt();
            let line = self.read_line();
            let args = self.split_line(line);
            status = self.execute(args);
            println!("")
        }
        println!("Exiting {}...", "green-shell".green());
    }

    fn get_current_working_directory(&self) -> std::io::Result<String> {
        env::current_dir()
            .map(|path| path.display().to_string())
            .map_err(|error| {
                eprintln!("Error getting current directory: {}", error);
                error
            })
    }

    fn get_shortened_path(&self, cwd: String) -> String {
        let home_dir = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => return cwd.to_string(),
        };

        if cwd.starts_with(&home_dir) {
            format!("~{}", &cwd[home_dir.len()..])
        } else {
            cwd.to_string()
        }
    }

    fn prompt(&self) {
        match self.get_current_working_directory() {
            Ok(cwd) => print!("{}\n{} ", self.get_shortened_path(cwd).green(), ">".green()),
            Err(_) => print!("> "),
        }
        std::io::stdout().flush().unwrap()
    }

    fn read_line(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn split_line(&self, line: String) -> Vec<String> {
        split_by_whitespace(line)
    }

    fn launch(&self, command: &str, args: &[String]) -> ShellStatus {
        let result = Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match result {
            Ok(status) if status.success() => ShellStatus::Running,
            Ok(status) => {
                eprintln!("failed to execute process - {}: {}", command, status);
                ShellStatus::Running
            }
            Err(err) => {
                eprintln!("failed to execute process - {}: {}", command, err);
                ShellStatus::Running
            }
        }
    }

    fn check_for_builtins(&self, arg: &str) -> Option<fn(Vec<String>) -> ShellStatus> {
        self.builtins.commands.get(arg).copied()
    }

    fn execute(&self, args: Vec<String>) -> ShellStatus {
        if args.is_empty() {
            return ShellStatus::Running;
        }

        save_history(&args);

        if let Some(builtin) = self.check_for_builtins(&args[0]) {
            builtin(args)
        } else {
            let check_if_alias_args = match self.aliases.get(&args[0]) {
                Some(alias_args) => alias_args.clone(),
                None => args,
            };

            let (command, args) = check_if_alias_args.split_first().unwrap();

            self.launch(&command, args)
        }
    }
}
