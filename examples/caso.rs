use caso::solve_str;

fn main() {
    println!("=== Caso 0.1 ===");
    println!("Type `help` for more information.");
    loop {
        use std::io::{self, Write};

        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("ERROR: Could not read input");
                continue;
            }
        };

        match &*input.trim() {
            "bye" => break,
            "help" => {print_help(); continue}
            "" => {
                // Print separator for readability.
                print!("\n------------------------------------<o=o");
                println!("o=o>------------------------------------\n");
                continue;
            }
            x => {
                if let Some(y) = solve_str(x) {
                    println!("{}", y);
                }
            }
        }
    }
}

fn print_help() {print!("{}", include_str!("../assets/help/help.txt"))}
