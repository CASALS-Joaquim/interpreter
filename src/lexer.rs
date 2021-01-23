use super::token;
use super::token::tokens;
pub use self::tokens::*;

pub struct Lexer {
	input: String,
	current: usize
}

impl Lexer {
	pub fn new(input: String) -> Self {
		Lexer {
			input: input,
			current: 0
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
		while match self.get_char(0) {
			Some(' ')
			| Some('\t')
			| Some('\n')
			| Some('\r') => true,
			_ => false
		} {
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

	pub fn read_number(&mut self) -> String {
		let mut buf: String = String::new();
		while match self.get_char(0) {
			Some(ch) => ch.is_digit(10),
			None => false
		} {
			match self.get_char(0) {
				Some(ch) => buf.push_str(ch.to_string().as_str()),
				_ => {}
			};
			self.read_char();
		}
		self.current -= 1;
		buf
	}
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
						Some(token::Token {
							token_type: String::from(token::EQ),
							literal: String::from("==")
						})
					}
					_ => {
						Some(token::Token {
							token_type: String::from(token::ASSIGN),
							literal: String::from("=")
						})
					}
				}
			}
			
			Some('+') => {
				Some(token::Token {
					token_type: String::from(token::PLUS),
					literal: String::from("+")
				})
			}

			Some('-') => {
				Some(token::Token {
					token_type: String::from(token::MINUS),
					literal: String::from("-")
				})
			}

			Some('!') => {
				match self.get_char(1) {
					Some('=') => {
						Some(token::Token {
							token_type: String::from(token::NOT_EQ),
							literal: String::from("!=")
						})
					}
					_ => {
						Some(token::Token{
							token_type: String::from(token::BANG),
							literal: String::from("!")
						})
					}
				}
			}

			Some('/') => {
				Some(token::Token {
					token_type: String::from(token::SLASH),
					literal: String::from("/")
				})
			}

			Some('*') => {
				Some(token::Token {
					token_type: String::from(token::ASTERISK),
					literal: String::from("*")
				})
			}

			Some('<') => {
				Some(token::Token {
					token_type: String::from(token::LT),
					literal: String::from("<")
				})
			}

			Some('>') => {
				Some(token::Token {
					token_type: String::from(token::GT),
					literal: String::from(">")
				})
			}
			
			Some(';') => {
				Some(token::Token {
					token_type: String::from(token::SEMICOLON),
					literal: String::from(";")
				})
			}
			
			Some(',') => {
				Some(token::Token {
					token_type: String::from(token::COMMA),
					literal: String::from(",")
				})
			}

			Some('{') => {
				Some(token::Token {
					token_type: String::from(token::LBRACE),
					literal: String::from("{")
				})
			}

			Some('}') => {
				Some(token::Token {
					token_type: String::from(token::RBRACE),
					literal: String::from("}")
				})
			}

			Some('(') => {
				Some(token::Token {
					token_type: String::from(token::LPAREN),
					literal: String::from("(")
				})
			}

			Some(')') => {
				Some(token::Token {
					token_type: String::from(token::RPAREN),
					literal: String::from(")")
				})
			}

			None => {
				Some(token::Token {
					token_type: String::from(token::EOF),
					literal: String::from("")
				})
			}
			
			Some(ch) => {
				if ch.is_alphabetic() {
					let ident = self.read_identifier();
					Some(token::Token {
						token_type: String::from(token::lookup_indent(ident.as_str())),
						literal: ident
					})
				} else if ch.is_digit(10) {
					let num = self.read_number();
					Some(token::Token {
						token_type: String::from(token::INT),
						literal: num
					})
				} else {
					Some(token::Token {
						token_type: String::from(token::ILLEGAL),
						literal: ch.to_string()
					})
				}
			}
		};
		self.read_char();
		match tok {
			Some(tok) if tok.token_type == String::from(token::EOF) => None,
			Some(tok) => Some(tok),
			None => None
		}
	}
}

#[cfg(test)]
pub mod test {
    use super::token;
    #[derive(Debug)]
    struct ExpectedToken {
        pub expected_type: &'static str,
        pub expected_literal: &'static str,
    }
    
    #[test]
    pub fn test_next_token() {
        let input = "let five = 5;\n\nlet ten = 10;\n\nlet add = fn(x, y) {\n\tx + y;\n};\n\nlet result = add(five, ten);\n";
        let tests = vec![
            //begin 1
            ExpectedToken {
                expected_type: token::LET,
                expected_literal: "let"
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "five"
            },

            ExpectedToken {
                expected_type: token::ASSIGN,
                expected_literal: "="
            },

            ExpectedToken {
                expected_type: token::INT,
                expected_literal: "5"
            },

            ExpectedToken {
                expected_type: token::SEMICOLON,
                expected_literal: ";"
            },
            //end 1

            //begin 2
            ExpectedToken {
                expected_type: token::LET,
                expected_literal: "let"
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "ten"
            },

            ExpectedToken {
                expected_type: token::ASSIGN,
                expected_literal: "="
            },

            ExpectedToken {
                expected_type: token::INT,
                expected_literal: "10"
            },

            ExpectedToken {
                expected_type: token::SEMICOLON,
                expected_literal: ";"
            },
            //end 2

            //begin 3
            ExpectedToken {
                expected_type: token::LET,
                expected_literal: "let"
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "add"
            },

            ExpectedToken {
                expected_type: token::ASSIGN,
                expected_literal: "="
            },

            ExpectedToken {
                expected_type: token::FUNCTION,
                expected_literal: "fn"
            },

            ExpectedToken {
                expected_type: token::LPAREN,
                expected_literal: "("
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "x"
            },
            
            ExpectedToken {
                expected_type: token::COMMA,
                expected_literal: ","
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "y"
            },

            ExpectedToken {
                expected_type: token::RPAREN,
                expected_literal: ")"
            },

            //open block 3.x
            ExpectedToken {
                expected_type: token::LBRACE,
                expected_literal: "{"
            },

                //begin 3.1
                ExpectedToken {
                    expected_type: token::IDENT,
                    expected_literal: "x"
                },

                ExpectedToken {
                    expected_type: token::PLUS,
                    expected_literal: "+"
                },

                ExpectedToken {
                    expected_type: token::IDENT,
                    expected_literal: "y"
                },

                ExpectedToken {
                    expected_type: token::SEMICOLON,
                    expected_literal: ";"
                },
                //end 3.1

            //close block 3.x
            ExpectedToken {
                expected_type: token::RBRACE,
                expected_literal: "}"
            },

            ExpectedToken {
                expected_type: token::SEMICOLON,
                expected_literal: ";"
            },
            //end 3

            //begin 4
            ExpectedToken {
                expected_type: token::LET,
                expected_literal: "let"
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "result"
            },

            ExpectedToken {
                expected_type: token::ASSIGN,
                expected_literal: "="
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "add"
            },

            ExpectedToken {
                expected_type: token::LPAREN,
                expected_literal: "("
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "five"
            },

            ExpectedToken {
                expected_type: token::COMMA,
                expected_literal: ","
            },

            ExpectedToken {
                expected_type: token::IDENT,
                expected_literal: "ten"
            },

            ExpectedToken {
                expected_type: token::RPAREN,
                expected_literal: ")"
            },

            ExpectedToken {
                expected_type: token::SEMICOLON,
                expected_literal: ";"
            },
            //end 4

            //begin 5
            ExpectedToken {
                expected_type: token::BANG,
                expected_literal: "!"
            },

            ExpectedToken {
                expected_type: token::MINUS,
                expected_literal: "-"
            },

            ExpectedToken {
                expected_type: token::SLASH,
                expected_literal: "/"
            },

            ExpectedToken {
                expected_type: token::ASTERISK,
                expected_literal: "*"
            },

            ExpectedToken {
                expected_type: token::INT,
                expected_literal: "5"
            },

            ExpectedToken {
                expected_type: token::SEMICOLON,
                expected_literal: ";"
            },
            //end 5
        ];

        let lex: crate::lexer::Lexer = crate::lexer::Lexer::new(String::from(input));
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
            println!("\t\"Type\": {{expected: {{\"{}\"    found: \"{}\"}}}},", test_tok.expected_type, tok.token_type);
            println!("\t\"Literal\": {{expected: {{\"{}\": found: \"{}\"}}}},", test_tok.expected_literal, tok.literal);
            assert_eq!(test_tok.expected_type, tok.token_type);
            assert_eq!(test_tok.expected_literal, tok.literal);
        }
        println!("}}");
    }
}