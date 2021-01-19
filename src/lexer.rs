use std::str::Chars;
use std::iter::Enumerate;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use super::token;
use super::token::tokens;
pub use tokens::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    input: Enumerate<Chars<'static>>,
    current: Option<(usize, char)>,
    passed: bool,
}

impl Lexer {
    pub fn new(input: (*const u8, usize)) -> Self {
        let input = match from_utf8(unsafe {from_raw_parts(input.0, input.1)}){
            Ok(v) => v,
            Err(_) => panic!()
        };
        let mut lex = Lexer {
            input: input.chars().enumerate(),
            current: None,
            passed: false,
        };
        lex.current = lex.input.next();
        lex
    }

    pub fn read_char(&mut self) {
        if !self.passed {
            self.current = match self.input.next() {
                Some(something) => {
                    Some(something)
                }
                None => {
                    None
                }
            };
        }
        self.passed = false;
    }

    pub fn skip_whitespaces(&mut self) {
        while match self.current {
            Some((_, ' ')) | Some((_, '\t')) | Some((_, '\n')) | Some((_, '\r')) => {
                true
            }
            _ => false
        } {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self)-> (*const u8, usize) {
        let mut buf = String::new();
        while match self.current {
            Some((_, ch)) => {
                ch.is_alphanumeric()
            }
            None => { false }
        } {
            buf = format!("{}{}", buf, match self.current {
                Some((_, value)) => { value }
                None => { panic!(1) }
            });
            self.read_char()
        }
        self.passed = true;
        (buf.as_str().as_ptr(), buf.as_str().len())
    }

    pub fn read_number(&mut self) -> (*const u8, usize) {
        let mut buf: String = String::new();
        while match self.current {
            Some((_, ch)) => ch.is_digit(10),
            _ => false
        } {
            match self.current {
                Some((_, ch)) => {
                    buf.push_str(ch.to_string().as_str())
                }_ => {}
            };
            self.read_char();
            
        }
        self.passed = true;
        (buf.as_ptr(), buf.len())
    }
}


impl Iterator for Lexer {
    type Item = token::Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        let tok = match self.current {
            Some((_, '=')) => {
                self.read_char();
                self.passed = true;
                match self.current {
                    Some((_, '=')) => {
                        self.passed = false;
                        Some(token::Token {
                            token_type: token::EQ,
                            literal: "=="
                        })
                    }
                    _ => {
                        Some(token::Token {
                            token_type: token::ASSIGN,
                            literal: "="
                        })
                    }
                }
            }

            Some((_, '+')) => {
                Some(token::Token {
                    token_type: token::PLUS,
                    literal: "+"
                })
            }

            Some((_, '-')) => {
                Some(token::Token {
                    token_type: token::MINUS,
                    literal: "-"
                })
            }

            Some((_, '!')) => {
                self.read_char();
                self.passed = true;
                match self.current {
                    Some((_, '=')) => {
                        self.passed = false;
                        Some(token::Token {
                            token_type: token::NOT_EQ,
                            literal: "!="
                        })
                    }
                    _ => {
                        Some(token::Token{
                            token_type: token::BANG,
                            literal: "!"
                        })
                    }
                }
            }

            Some((_, '/')) => {
                Some(token::Token {
                    token_type: token::SLASH,
                    literal: "/"
                })
            }

            Some((_, '*')) => {
                Some(token::Token {
                    token_type: token::ASTERISK,
                    literal: "*"
                })
            }

            Some((_, '<')) => {
                Some(token::Token {
                    token_type: token::LT,
                    literal: "<"
                })
            }

            Some((_, '>')) => {
                Some(token::Token {
                    token_type: token::GT,
                    literal: ">"
                })
            }

            Some((_, ';')) => {
                Some(token::Token {
                    token_type: token::SEMICOLON,
                    literal: ";"
                })
            }

            Some((_, ',')) => {
                Some(token::Token {
                    token_type: token::COMMA,
                    literal: ","
                })
            }

            Some((_, '{')) => {
                Some(token::Token {
                    token_type: token::LBRACE,
                    literal: "{"
                })
            }

            Some((_, '}')) => {
                Some(token::Token {
                    token_type: token::RBRACE,
                    literal: "}"
                })
            }

            Some((_, '(')) => {
                Some(token::Token {
                    token_type: token::LPAREN,
                    literal: "("
                })
            }

            Some((_, ')')) => {
                Some(token::Token {
                    token_type: token::RPAREN,
                    literal: ")"
                })
            }

            None => {
                Some(token::Token {
                    token_type: token::EOF,
                    literal: ""
                })
            }

            Some((_, ch)) => {
                if ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    let literal: &'static str = match from_utf8(unsafe {from_raw_parts(ident.0, ident.1)}) {
                        Ok(x) => x,
                        Err(_) => panic!("Non valid UTF8")
                    };
                    Some(token::Token {
                        token_type: token::lookup_indent(literal.clone().to_owned().as_str()),
                        literal: literal
                    })
                } else if ch.is_digit(10) {
                    let num = self.read_number();
                    Some(token::Token {
                        token_type: token::INT,
                        literal: match from_utf8(unsafe {from_raw_parts(num.0, num.1)}) {
                            Ok(x) => x,
                            Err(_) => panic!("Non valid UTF8")
                        }
                    })
                } else {
                    Some(token::Token {
                        token_type: token::ILLEGAL,
                        literal: match from_utf8(unsafe {
                            from_raw_parts(ch.to_string().as_str().as_ptr(), ch.to_string().as_str().len())
                        }) {
                            Ok(v) => v,
                            Err(_) => panic!()
                        }
                    })
                }
            }
        };
        self.read_char();
        match tok {
            Some(tok) if tok.token_type == token::EOF => {
                None
            }
            Some(tok) => {
                Some(tok)
            }
            None=> panic!()
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
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

        let lex: crate::lexer::Lexer = crate::lexer::Lexer::new((input.as_ptr(), input.len()));
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
            println!("\t\"Type\": {{\"{}\": \"{}\"}},", test_tok.expected_type, tok.token_type);
            assert_eq!(test_tok.expected_type, tok.token_type);
            println!("\t\"Literal\": {{\"{}\": \"{}\"}},", test_tok.expected_literal, tok.literal);
            assert_eq!(test_tok.expected_literal, tok.literal);
        }
        println!("}}");
    }
}