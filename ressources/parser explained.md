# The Parser Explanation

```rust
/// An expression is
/// either a standalone value,
/// either a prefix expression with one operator, and the `right` expression, but no a `left` expr, neither a following operation,
/// either an infix expression with one operator, and the `left` and `right` expressions, but no following operation,
/// either one of the above with a following application
fn expression(&mut self, paren: Paren, left: Option<ast::Expression>, op: Option<operator::Op>, right: Option<ast::Expression>, next_op: Option<operator::Op>) -> Expression {
	match a {
		None => self.parse_prefix(b, next_op), // prefix
		Some(left) => match op {
			None => left,
			Some(op) => {
				let right = match next_op {
					None | (Some(nop) if op > nop) => right.unwrap(),
					Some(nop) => expression(right, next_op, self.raw(), match OpPrec::try_from(self.get(0)) {
						Err(_) => None,
						Ok(op) => Some(op)
					}
				};
				ast::Expression::Infix {
					left: left,
					op: op.0,
					right: right
				}
			}
		}
	}
}


#[derive(PartialEq)]
enum Paren {
    None,
    Paren,
    Brace,
    Bracket
}

impl Paren {
    pub fn closes(&self, tok: &token::Token) -> bool {
        match self {
            Paren => tok == tok::Token::RightParen,
            Brace => tok == token::Token::RightBrace,
            Bracket => panic!() // tok == token::Token::RightBracket
            _ => true,
		}
	}
}
        
```
Operator precedence and associativity:
|  Operators  |  Precedence  |  Associativity |
|  ---------  |  ----------  |  ------------- |
|   ';', ∅    |      0       |  Left to Right |
|     ','     |      1       |  Left to Right |
|   '+', '-'  |      2       |  Left to Right |
