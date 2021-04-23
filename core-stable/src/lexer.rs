use super::token;

pub struct Lexer {
	input: String,
	current: usize
}

enum IntPrefix {
	Hexadecimal,
	Binary,
	Octal,
	None
}

impl IntPrefix {
	pub fn is_digit(&self, digit: char) -> bool {
		match self {
			Self::Hexadecimal => digit.is_digit(16),
			Self::Binary => digit.is_digit(2),
			Self::Octal => digit.is_digit(8),
			Self::None => digit.is_digit(10)
		}
	}

	pub fn parse_number(&self, number: &str) -> isize {
		isize::from_str_radix(number, match self {
			Self::None => 10,
			Self::Hexadecimal => 16,
			Self::Binary => 2,
			Self::Octal => 8,
		}).unwrap()
	}
}

impl From<&str> for IntPrefix {
	fn from(prefix: &str) -> IntPrefix {
		match prefix {
			"0x" => IntPrefix::Hexadecimal,
			"0b" => IntPrefix::Binary,
			"0o" => IntPrefix::Octal,
			_ => IntPrefix::None
		}
	}
}
impl Lexer {
	pub fn new(input: String) -> Self {
		Lexer {
			input: input,
			current: 0,
		}
	}

	fn get_char(&self, distance_from_current: isize) -> Option<char> {
		self.input.chars().nth((self.current as isize + distance_from_current) as usize)
	}

	pub fn read_char(&mut self) {
		match self.input.chars().nth(1) {
			Some(_) => self.current += 1,
			None => return ()
		}
	}

	pub fn skip_whitespaces(&mut self) {
		while matches!{ self.get_char(0), Some(' ') | Some('\t') | Some('\n') | Some('\r') } {
			self.read_char();
		}
	}

	pub fn read_identifier(&mut self) -> String {
		let mut buf = String::new();
		while match self.get_char(0) {
			Some(ch) => ch.is_alphanumeric(),
			None => false
		} {
			buf = format!("{}{}", buf, match self.get_char(0) {
				Some(ch) => ch,
				None => panic!("Corrupted data!")
			});
			self.read_char()
        }
        self.current -= 1;
		buf
	}
	
	pub fn read_number(&mut self) -> isize {
		let mut buf: String = String::new();
		let prefix = match IntPrefix::from(&self.input[self.current..self.current+2]) {
			IntPrefix::None => IntPrefix::None,
			prefix => {
				self.current += 2;
				prefix
			}
		};
		while match self.get_char(0) {
			Some(ch) => prefix.is_digit(ch),
			None => false
		} {
			match self.get_char(0) {
				Some(ch) => buf.push_str(ch.to_string().as_str()),
				_ => {}
			};
			self.read_char();
		}
		self.current -= 1;
		prefix.parse_number(&buf)
	}

	/*	pub fn read_string(&mut self) -> String {

	}*/
}

impl Iterator for Lexer {
	type Item = token::Token;
	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespaces();
		let tok = match self.get_char(0) {
			//check the equality or assignment case
			Some('=') => {
				//which one it  is? equality or assignment?
				match self.get_char(1) {
					Some('=') => {
                        self.current += 1;
						token::Eq
					}
					_ => token::Assign
				}
			}
			
			Some('+') => token::Plus,

			Some('-') => token::Minus,

			Some('!') => match self.get_char(1) {
                Some('=') => {
					self.current += 1;
					token::NotEq
				},
                _ => token::Bang
            }

			Some('/') => token::Slash,

			Some('*') => token::Asterisk,

			Some('<') => match self.get_char(1) {
				Some('=') => {
					self.current += 1;
					token::LowerThanOrEqualTo
				},
				_ => token::LowerThan
			},

			Some('>') => match self.get_char(1) {
				Some('=') => {
					self.current += 1;
					token::GreaterThanOrEqualTo
				},
				_ => token::LowerThan
			},

			Some(';') => token::Semicolon,

			Some(',') => token::Comma,

			Some('{') => token::LeftBrace,

			Some('}') => token::RightBrace,

			Some('(') => token::LeftParen,

			Some(')') => token::RightParen,

			None => token::EndOfFile,

			Some(ch) => {
				if ch.is_alphabetic() {
					let ident = self.read_identifier();
					token::lookup_indent(ident.as_str())
				} else if ch.is_digit(10) {
					let num = self.read_number();
					token::Int(num)
				} else {
					token::Illegal(ch)
                }
            }
        };
		self.read_char();
		match tok {
			tok if tok == token::EndOfFile => None,
			tok => Some(tok),
		}
	}
}

#[cfg(test)]
pub mod test {
    use super::token;
    #[derive(Debug)]
    struct ExpectedToken(token::Token);

    #[test]
    pub fn test_next_token() {
        let input = String::from("let five = 5;\n\nlet ten = 10;\n\nlet add = fn(x, y) {\n\tx + y;\n};\n\nlet result = add(five, ten);\n!-/*5; let True = true; let False = !True");
        let tests = vec![
            //begin 1
            ExpectedToken(token::Let),
            ExpectedToken(token::Ident(String::from("five"))),
            ExpectedToken(token::Assign),
            ExpectedToken(token::Int(5)),
            ExpectedToken(token::Semicolon),
            //end 1

            //begin 2
            ExpectedToken(token::Let),
            ExpectedToken(token::Ident(String::from("ten"))),
            ExpectedToken(token::Assign),
            ExpectedToken(token::Int(10)),
            ExpectedToken(token::Semicolon),
            //end 2

            //begin 3
            ExpectedToken(token::Let),
            ExpectedToken(token::Ident(String::from("add"))),
            ExpectedToken(token::Assign),
            ExpectedToken(token::Function),
            ExpectedToken(token::LeftParen),
            ExpectedToken(token::Ident(String::from("x"))),
			ExpectedToken(token::Comma),
            ExpectedToken(token::Ident(String::from("y"))),
            ExpectedToken(token::RightParen),

            //open block 3.x
            ExpectedToken(token::LeftBrace),

                //begin 3.1
                ExpectedToken(token::Ident(String::from("x"))),
                ExpectedToken(token::Plus),
                ExpectedToken(token::Ident(String::from("y"))),
                ExpectedToken(token::Semicolon),
                //end 3.1

            //close block 3.x
            ExpectedToken(token::RightBrace),
            ExpectedToken(token::Semicolon),
            //end 3

            //begin 4
            ExpectedToken(token::Let),
            ExpectedToken(token::Ident(String::from("result"))),
            ExpectedToken(token::Assign),
            ExpectedToken(token::Ident(String::from("add"))),
            ExpectedToken(token::LeftParen),
            ExpectedToken(token::Ident(String::from("five"))),
            ExpectedToken(token::Comma),
            ExpectedToken(token::Ident(String::from("ten"))),
            ExpectedToken(token::RightParen),
            ExpectedToken(token::Semicolon),
            //end 4

            //begin 5
            ExpectedToken(token::Bang),
            ExpectedToken(token::Minus),
            ExpectedToken(token::Slash),
            ExpectedToken(token::Asterisk),
            ExpectedToken(token::Int(5)),
            ExpectedToken(token::Semicolon),
            //end 5

			//begin 6
			ExpectedToken(token::Let),
			ExpectedToken(token::Ident(String::from("True"))),
			ExpectedToken(token::Assign),
			ExpectedToken(token::Boolean(true)),
			ExpectedToken(token::Semicolon),
			//end 6

			//begin 7
			ExpectedToken(token::Let),
			ExpectedToken(token::Ident(String::from("False"))),
			ExpectedToken(token::Assign),
			ExpectedToken(token::Bang),
			ExpectedToken(token::Ident(String::from("True"))),
			ExpectedToken(token::Semicolon)
        ];

        let lex: crate::lexer::Lexer = crate::lexer::Lexer::new(input);
        println!("{{");
        let mut tests_iter = tests.iter();
        for tok in lex {
            let test_tok = match tests_iter.next(){
                Some(tok) => {
                    tok
                }
                None => {
                    break;
                }
            };
            println!("\t\"Token\": {{expected: {{\"{:?}\"    found: \"{:?}\"}}}},", test_tok.0, tok);
            assert_eq!(test_tok.0, tok);
        }
        println!("}}");
    }
}