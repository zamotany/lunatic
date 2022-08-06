use parser::Parser;
use visitor::debug_visitor;

mod ast;
mod parser;
mod scanner;
mod token;
mod visitor;

// const INPUT_SOURCE: &str = "
// function fact (n)
// 	if n == 0 then
// 		return 1
// 	else
// 		return n * fact(n-1)
// 	end
// end

// function test()
// 	if true then
// 		return 0
// 	else if whatever then
// 		return -1
// 	else
// 		return 1
// 	end
// end
// ";

fn main() {
    let mut scanner = scanner::Scanner::new("{ ['fo'..'o'] = 'bar' }");
    match scanner.scan_tokens() {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(Some(expression)) => {
                let debug_visitor = debug_visitor::DebugVisitor::new();
                let output = expression.visit(&debug_visitor);
                println!("{}", output);
            }
            Ok(None) => {
                println!("Parsing results were None");
            }
            Err(error) => {
                println!("Error parsing: {}", error);
            }
        },
        Err(error) => {
            println!("Error scanning: {}", error);
        }
    }
}
