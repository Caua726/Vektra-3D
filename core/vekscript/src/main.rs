use std::fs;

pub mod cli;
pub mod interpreter;
pub mod parser;
pub mod lexer;
pub mod grammar;
pub mod renderer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
            if !args[1].starts_with("--") {
            let file_path = &args[1];
            process_file(file_path);
        }
        else if args[1] == "--file" && args.len() > 2 {
            let file_path = &args[2];
            process_file(file_path);
        }
        else if args[1] == "--terminal" {
            println!("Starting interactive mode");
            cli::main();
        }
        else {
            println!("Erro, argumentos inválidos, possiveis argumentos: vekscript [file.vks] | --file [file.vks] | --terminal");
            println!("Usando modo interativo");
            cli::main();
        }
    } else {
        println!("Iniciando modo interativo");
        cli::main();
    }
}

fn process_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("--- Lexer Output ---");
            let tokens = lexer::tokenize(&content);
            println!("{:?}\n", tokens);

            println!("--- Parser Output ---");
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    println!("{:#?}\n", ast);
                    
                    println!("--- Interpreter Output ---");
                    let mut interpreter = interpreter::Interpreter::new();
                    match interpreter.interpret(&ast) {
                        Ok(world) => {
                            println!("{:#?}\n", world);
                            println!("--- Starting Renderer ---");
                            renderer::run(world);
                        },
                        Err(e) => eprintln!("Interpreter Error: {}", e),
                    }
                },
                Err(e) => eprintln!("Parsing Error: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Erro ao abrir arquivo: {}", e);
        }
    }
}