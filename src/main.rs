use std::{
    collections::HashMap,
    process::{self},
};

struct BUILTINS {
    commands: HashMap<String, fn(Vec<String>)>,
}

impl BUILTINS {
    fn new() -> BUILTINS {
        let mut commands: HashMap<String, fn(Vec<String>)> = HashMap::new();

        commands.insert(String::from("exit"), builtin_exit);
        commands.insert(String::from("help"), builtin_help);

        BUILTINS { commands }
    }
}

fn builtin_exit(_args: Vec<String>) {
    println!("Goodbye! :)");
    process::exit(0)
}

fn builtin_help(_args: Vec<String>) {
    println!("Help!")
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
        println!("Running shell");
        loop {
            self.prompt();
            let line = self.read_line();
            let args = self.split_line(line);
            let status = self.execute(args);
            println!("What is my status: {}", status);
            if !status {
                println!("Exiting shell...");
                break;
            }
        }
    }

    fn prompt(&self) {
        println!("> ")
    }

    fn read_line(&self) -> String {
        println!("reading line");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {}", input);
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

    fn check_for_builtins(&self, args: Vec<String>) {
        for (key, value) in self.builtins.commands.iter() {
            if args[0].trim() == key.to_string().trim() {
                value(args.clone())
            }
        }
    }

    fn execute(&self, args: Vec<String>) -> bool {
        if args.is_empty() {
            return true;
        }

        // check for builtin commands
        self.check_for_builtins(args);
        //
        // save_history

        // idk i think this needs to be something else
        return self.launch() || true;
    }
}

fn main() {
    println!("Hello, world!");
    Shell::new().run();
}
