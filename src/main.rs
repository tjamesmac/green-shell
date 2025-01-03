use std::{
    collections::HashMap,
    env::{self},
    fs::OpenOptions,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use owo_colors::OwoColorize;

enum ShellStatus {
    Running,
    Exit,
}

struct Builtins {
    commands: HashMap<String, fn(Vec<String>) -> ShellStatus>,
}

impl Builtins {
    fn new() -> Self {
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
        assert!(env::set_current_dir(Path::new(&home_dir)).is_ok());
    } else {
        let (_command, destination_path) = args.split_first().unwrap();

        assert!(env::set_current_dir(Path::new(&destination_path[0])).is_ok());
    }
    ShellStatus::Running
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

fn builtin_exit(_args: Vec<String>) -> ShellStatus {
    println!("Goodbye! :)");
    ShellStatus::Exit
}

fn builtin_help(_args: Vec<String>) -> ShellStatus {
    println!("Help!");

    ShellStatus::Running
}

struct Shell {
    builtins: Builtins,
}

impl Shell {
    fn new() -> Self {
        Self {
            builtins: Builtins::new(),
        }
    }

    fn run(&self) {
        println!("Welcome to {}!", "green-shell".green());

        let mut status = ShellStatus::Running;
        while let ShellStatus::Running = status {
            self.prompt();
            let line = self.read_line();
            let args = self.split_line(line);
            status = self.execute(args);
            println!("")
        }
        println!("Exiting green-shell...");
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
        line.split_whitespace().map(str::to_string).collect()
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
            let (command, args) = args.split_first().unwrap();
            self.launch(command, args)
        }
    }
}

fn main() {
    Shell::new().run();
}
