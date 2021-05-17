# The Parser Explanation

```rust
/// An expression is
/// either a standalone value,
/// either a prefix expression with one operator, and the `b` expression, but no the `a` expr but no following operation,
/// either an infix expression with one operator, and the `a` and `b` expression, but no following operation,
/// either one of the above with a following application
fn expression(&self, paren: Paren, left: Option<Expression>, op: Option<Op>, right: Option<Expression>, next_op: Option<Op>) -> Expression {
	match a {
		None => self.parse_prefix(b, next_op), // prefix
		Some(left) => match op {
			None => left,
			Some(op) => match next_op {
				None => {
					let right = right.unxrap();
					ast::Expression::Infix {
						left: left,
						operator: op.0,
						right: right
					}
				},
				Some(op) => 
	
    
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
