use caso::solve_str;

fn main() {
    println!("=== Caso 0.2 ===");
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
            x if x.starts_with("echo ") => {
                match caso::parsing::parse_str(x[5..].trim()) {
                    Ok(x) => {
                        println!("{}", x);
                        println!("{:?}", x);
                        continue;
                    }
                    Err(err) => {
                        println!("ERROR:\n{}", err);
                        continue;
                    }
                }
            }
            "" => {
                // Print separator for readability.
                print!("\n------------------------------------<o=o");
                println!("o=o>------------------------------------\n");
                continue;
            }
            x => {
                match solve_str(x) {
                    Ok(y) => println!("{}", y),
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
    }
}

fn print_help() {print!("{}", include_str!("../assets/help/help.txt"))}
