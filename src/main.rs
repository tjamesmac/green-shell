use std::{
    collections::HashMap,
    io::Write,
    process::{self},
};

struct BUILTINS {
    commands: HashMap<String, fn(Vec<String>) -> bool>,
}

impl BUILTINS {
    fn new() -> BUILTINS {
        let mut commands: HashMap<String, fn(Vec<String>) -> bool> = HashMap::new();

        commands.insert(String::from("exit"), builtin_exit);
        commands.insert(String::from("help"), builtin_help);

        BUILTINS { commands }
    }
}

fn builtin_exit(_args: Vec<String>) -> bool {
    println!("Goodbye! :)");
    return false;
}

fn builtin_help(_args: Vec<String>) -> bool {
    println!("Help!");

    return true;
}

struct Shell {
    builtins: BUILTINS,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            builtins: BUILTINS::new(),
        }
    }

    fn run(&self) {
        println!("Welcome to green-shell");
        loop {
            self.prompt();
            let line = self.read_line();
            let args = self.split_line(line);
            let status = self.execute(args);
            println!("What is my status: {}", status);
            if !status {
                println!("Exiting green-shell...");
                break;
            }
        }
    }

    fn prompt(&self) {
        print!("> ");
        std::io::stdout().flush().unwrap()
    }

    fn read_line(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }

    fn split_line(&self, line: String) -> Vec<String> {
        let line_to_split = line.trim();

        if line_to_split.is_empty() {
            return Vec::new();
        }

        line.split(" ").map(str::to_string).collect()
    }

    fn launch(&self) -> bool {
        println!("Implement the launch!");
        // temporary bool return
        false
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
            None => self.launch() || true,
        }
    }
}

fn main() {
    println!("Hello, world!");
    Shell::new().run();
}
