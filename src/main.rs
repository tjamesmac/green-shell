mod parser;
mod shell;

use shell::Shell;

fn main() {
    Shell::new().run();
}
