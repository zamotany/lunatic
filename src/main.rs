use parser::Parser;

mod token;
mod scanner;
mod parser;
mod ast;

const INPUT_SOURCE: &str = "
function fact (n)
	if n == 0 then
		return 1
	else
		return n * fact(n-1)
	end
end

function test()
	if true then
		return 0
	else if whatever then
		return -1
	else
		return 1
	end
end
";

fn main() {
    let mut scanner = scanner::Scanner::new("not (not true)");
		match scanner.scan_tokens() {
			Ok(tokens) => {
				let mut parser = Parser::new(tokens);
                println!("{:#?}", parser.parse())
			},
			Err(error) => {
				println!("{}", error);
			}
		}
}
