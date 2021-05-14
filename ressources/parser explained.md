# The Parser Explanation

```rust
expression(sign: Sign, paren: Paren, a: Expression, op: Option<Op>, b: Option<Expression>, next_op: Option<Op>)

enum Sign {
    None,
    Plus,
    Minus
}
```
Operator precedence and associativity:
|  Operators  |  Precedence  |  Associativity |
|  ---------  |  ----------  |  ------------- |
|   ';', ∅    |      0       |  Left to Right |
|     ','     |      1       |  Left to Right |
|   '+', '-'  |      2       |  Left to Right |
