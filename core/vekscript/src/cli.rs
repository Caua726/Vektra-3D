use std::io;
use std::io::Write;


pub fn main() {
    let input = io::stdin();
    let mut output = io::stdout();
    println!("Interactive mode started. Type 'exit' to quit.");

    loop {
        output.write_all(b"> ").unwrap();
        output.flush().unwrap();

        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();

        let trimmed = buffer.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "exit" {
            break;
        }

        // A very basic check. A full REPL would require parsing logic.
        if trimmed.starts_with("import") || trimmed.starts_with("object") || trimmed.starts_with("keyframe") || trimmed.starts_with("interpolate") {
             println!("(Command recognized, but REPL execution is not fully implemented yet)");
        } else {
             println!("Error: command not recognized: '{}'", trimmed);
        }
    }
}