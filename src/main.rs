use std::{
    collections::HashMap,
    env::{self},
    io::{self, Write},
    path::Path,
    process::Command,
};

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
        eprintln!("TODO: implement cd without destination to go back to $HOME")
    } else {
        let (_command, destination_path) = args.split_first().unwrap();

        let destination = &destination_path[0];

        assert!(env::set_current_dir(Path::new(destination)).is_ok());
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
        println!("Welcome to green-shell");

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
            Ok(cwd) => print!("{}\n> ", self.get_shortened_path(cwd)),
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
        let output = Command::new(command).args(args).output();

        match output {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stdout().write_all(&output.stderr).unwrap();
            }
            Err(err) => {
                eprintln!("failed to execute process - {}: {}", command, err);
            }
        }

        ShellStatus::Running
    }

    fn check_for_builtins(&self, arg: &str) -> Option<fn(Vec<String>) -> ShellStatus> {
        self.builtins.commands.get(arg).copied()
    }

    fn execute(&self, args: Vec<String>) -> ShellStatus {
        if args.is_empty() {
            return ShellStatus::Running;
        }

        // implement save_history
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
