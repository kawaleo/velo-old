use crate::syntax::ast::Expression;

pub fn console_printstr(input: Expression) {
    match input {
        Expression::StringLiteral(string) => {
            println!("{}", string)
        }
        _ => std::process::exit(1),
    }
}
