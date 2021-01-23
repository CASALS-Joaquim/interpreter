mod lexer;
mod token;

fn main() {
	repl::start();
}

mod repl {
	const PROMPT: &'static str = ">>> ";
	pub fn start() {
		use std::io::Write;
		use std::io::stdin as get_stdin;
		use std::io::stdout as get_stdout;
		let stdin = get_stdin();
		let handle = stdin.lock();
		use std::io::BufRead;
		let mut stdout = get_stdout();

		print!("{}", PROMPT); stdout.flush().unwrap();
		//let mut line = String::new();
		for line in handle.lines() {
			let line = line.unwrap();
			stdout.flush().unwrap();
			let lex = crate::lexer::Lexer::new(line);
			for token in lex {
				println!("{{ Type: {}    Literal: {} }}", token.token_type, token.literal);
				stdout.flush().unwrap();
			}
			println!("");
			print!("{}", PROMPT);
			stdout.flush().unwrap();
		}
	}
}