use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process::Command,
};

struct Builtins {
    commands: HashMap<String, fn(Vec<String>) -> bool>,
}

impl Builtins {
    fn new() -> Self {
        let mut commands: HashMap<String, fn(Vec<String>) -> bool> = HashMap::new();

        commands.insert(String::from("exit"), builtin_exit);
        commands.insert(String::from("help"), builtin_help);
        commands.insert(String::from("cd"), builtin_cd);

        Self { commands }
    }
}

fn builtin_cd(_args: Vec<String>) -> bool {
    println!("implement cd!");
    true
}

fn builtin_exit(_args: Vec<String>) -> bool {
    println!("Goodbye! :)");
    false
}

fn builtin_help(_args: Vec<String>) -> bool {
    println!("Help!");

    true
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
        loop {
            self.prompt();
            let line = self.read_line();
            let args = self.split_line(line);
            let status = self.execute(args);
            if !status {
                println!("Exiting green-shell...");
                break;
            }
            println!("")
        }
    }

    fn get_current_working_directory(&self) -> std::io::Result<String> {
        match env::current_dir() {
            Ok(path) => Ok(path.display().to_string()),
            Err(error) => {
                eprintln!("Error getting current directory: {}", error);
                Err(error)
            }
        }
    }

    fn prompt(&self) {
        match self.get_current_working_directory() {
            Ok(cwd) => print!("{}\n> ", cwd),
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

    fn launch(&self, command: &str, args: &[String]) -> bool {
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

        true
    }

    fn check_for_builtins(&self, arg: &str) -> Option<fn(Vec<String>) -> bool> {
        self.builtins.commands.get(arg).copied()
    }

    fn execute(&self, args: Vec<String>) -> bool {
        if args.is_empty() {
            return true;
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
