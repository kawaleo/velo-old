use std::io::{self, Write};
use std::process;
use std::{env, fs};

use syntax::lexer::Lexer;
use syntax::parse::Parser;

mod syntax;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl()
    } else if args.len() == 2 {
        let path = &args[1];
        if let Ok(contents) = fs::read_to_string(path) {
            parse_file(contents)
        } else {
            eprintln!("Error reading file: {}", path);
        }
    }
}

fn repl() {
    println!("Velo REPL [beta]\nUse `quit` to exit safely\n");
    println!("NOTES TO SELF:");
    println!(
        "Implement function body parsing\nRefactors\nWarning Emission\nTuple Types in functions"
    );

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line :(");
        let input = input.trim();

        if input == "quit" {
            process::exit(0);
        }

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        let tokens = tokens.tokens;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        println!("{:#?}", ast);
    }
}

fn parse_file(contents: String) {
    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize();
    let tokens = tokens.tokens;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
