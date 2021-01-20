mod lexer;
mod token;
use std::slice::from_raw_parts;
use std::str::from_utf8;

fn main() {
	repl::start();
}

fn tokenizer(tokenized: (*const u8, usize)) -> Vec<token::Token> {
	let lexer = lexer::Lexer::new((tokenized.0, tokenized.1));
	let mut tokens = Vec::new();
	for tok in lexer {
		tokens.push(tok);
	}
	tokens
}

mod repl {
	const PROMPT: &'static str = ">>> ";
	pub fn start() {
		use std::io::stdin as get_stdin;
		use std::io::Read;
		let mut stdin = get_stdin();
		use std::io::BufRead;
		let mut line = String::new();
		loop {
			print!("{}", PROMPT);
			stdin.read_to_string(&mut line);
			if line.contains(&String::from("\\q\n")) { break; }
			let tokens = crate::tokenizer((
				line
					.as_str()
					.as_ptr(),
				line
					.as_str()
					.len()));
			for token in tokens.iter() {
				println!("{{ Type: {}    Literal: {} }}",
					token.token_type, token.literal);
			}
			println!("");
		}
	}
}