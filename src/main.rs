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

    fn launch(&self, args: Vec<String>) -> bool {
        if let Some((first, rest)) = args.split_first() {
            let output = Command::new(first)
                .args(rest)
                .output()
                .expect(format!("failed to execute process - {}", first).as_str());

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        } else {
            return false;
        }

        return true;
    }

    fn check_for_builtins(&self, arg: String) -> Option<(&String, &fn(Vec<String>) -> bool)> {
        self.builtins
            .commands
            .iter()
            .find(|x| arg.trim() == x.0.to_string().trim())
    }

    fn execute(&self, args: Vec<String>) -> bool {
        if args.is_empty() {
            return true;
        }

        let has_builtin = self.check_for_builtins(args[0].clone());

        // implement save_history
        match has_builtin {
            Some(built) => built.1(args),
            None => self.launch(args) || true,
        }
    }
}

fn main() {
    Shell::new().run();
}
