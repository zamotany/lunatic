mod token;
mod scanner;

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
    let mut scanner = scanner::Scanner::new(INPUT_SOURCE);
    println!("{:#?}", scanner.scan_tokens());
}
