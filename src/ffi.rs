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