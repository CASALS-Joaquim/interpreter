mod lexer;
mod token;
use std::slice::from_raw_parts;
use std::str::from_utf8;

fn main() {
	repl::start();
}


#[no_mangle]
pub extern fn ffitokenizer(tokenized: *const u8, length: usize) -> ffi::TokenArray {
	let tokens = tokenizer((tokenized, length));

	let mut c_tokens: Vec<ffi::Token> = Vec::new();
	for tok in tokens {
			c_tokens.push(ffi::cast_rust_token_into_c_token(&tok));
	}
	ffi::TokenArray {
			tokens: c_tokens.as_ptr(),
			length: c_tokens.len()
	}
}

mod ffi {
    #[repr(C)]
    pub struct Token {
        token_type: *const u8,
        token_type_length: usize,
        literal: *const u8,
        literal_length: usize
    }

    pub fn cast_rust_token_into_c_token(token: &crate::token::Token) -> Token {
        Token {
            token_type: token.token_type.as_ptr(),
            token_type_length: token.token_type.len(),
            literal: token.literal.as_ptr(),
            literal_length: token.literal.len()
        }
    }
    #[repr(C)]
    pub struct TokenArray {
        pub tokens: *const Token,
        pub length: usize
    }
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