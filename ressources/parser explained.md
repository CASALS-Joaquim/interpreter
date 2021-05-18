# The Parser Explanation

```rust
/// An expression is
/// either a standalone value,
/// either a prefix expression with one operator, and the `right` expression, but no a `left` expr, neither a following operation,
/// either an infix expression with one operator, and the `left` and `right` expressions, but no following operation,
/// either one of the above with a following application
fn expression(&mut self, start_del: Delimiter, left: Option<ast::Expression>, op: Option<operator::Op>, right: Option<ast::Expression>, next_op: Option<operator::Op>) -> Expression {
	let ret = match a {
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
	};
	if !start_del.closes(self.get(0).unwrap()) {
		panic!()
	}
	expr
}


#[derive(PartialEq)]
enum Delimiter {
    None,
    Paren,
    Brace,
    Bracket
}

impl Delimiter {
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
|  Operators  |  Precedence  |  Associativity |   Name   |
|  ---------  |  ----------  |  ------------- |  ------  |
|   ';', ∅    |      0       |  Left to Right |  LOWEST  |
|     ','     |      1       |  Left to Right |  COMMA   |
|     '='     |      2       |  Right to Left |  ASSIGN  |
| '==', '!='  |      3       |  Left to Right | EQUALITY |
|'<', '>', '<=', '>='|   4   |  Left to Right |COMPARISON|
|   '+', '-'  |      5       |  Left to Right |    SUM   |
|  '\*', '/'  |      6       |  Left to Right |  PRODUCT |
|   '\*\*'    |      7       |  Right to Left |    POW   |
|'+', '-', '!'|      8       |  Right to Left |  PREFIX  |
|  '()', '[]' |      9       |  Left to Right |  SUFFIX  |
