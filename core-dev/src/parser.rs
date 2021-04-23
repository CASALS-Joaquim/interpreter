use std::any::Any;
use std::rc::Rc;

use super::ast;
use core_stable::token;
use core_stable::lexer;

pub struct Parser {
    tokens: Vec<token::Token>,
    cur_token: usize,
    counter: (isize, isize)
}


#[allow(dead_code)]
impl Parser {
    pub fn new(string: String) -> Self {
        Self {
            tokens: lexer::Lexer::new(string).collect(),
            cur_token: 0,
            counter: (0, 0),
        }
    }

    fn get(&self, delta: isize) -> Option<token::Token> {
        match self.tokens.get((self.cur_token as isize + delta) as usize).clone() {
            Some(token) => Some(token.clone()),
            None => None
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let ast = self.parse();
        ast::Program::from(ast)
    }

    pub fn parse(&mut self) -> ast::BlockStatement {
        let statements = ast::BlockStatement::new();
        statements
    }

    fn parse_statement(&mut self) -> ast::Statement {
        let ret = match self.tokens.get(self.cur_token) {
            Some(token::Let) => self.parse_let_statement(),
            Some(token::Return) => self.parse_return_statement(),
            Some(_) => self.parse_expression().into(),
            _ => panic!()
        };
        match ret.clone() {
            ast::Statement::Expr(expr) => {
                match expr {
                    ast::Expression::BlockExpression(_) => (),
                    ast::Expression::IfExpression{ .. } => (),
                    _ => {
                        assert_eq!(self.get(0), Some(token::Semicolon));
                        self.cur_token += 1;
                    }
                }
            },
            _ => {
                assert_eq!(self.get(0), Some(token::Semicolon));
                self.cur_token += 1;
            }
        };
        ret
    }

    fn parse_expression(&mut self) -> ast::Expression {
        let ret = match self.get(0) {
            Some(token::Token::LeftBrace) => self.parse_block_statement(),

            Some(token::Token::Int(_))
            | Some(token::Token::String(_))
            | Some(token::Boolean(_))
            | Some(token::Token::Unit)
            | Some(token::Token::Float(_)) => self.parse_literal(),

            Some(token::Token::Function) => self.parse_function(),

            Some(token::Token::Ident(ident)) => {
                self.cur_token += 1;
                ast::Expression::Ident(ident)
            },
            Some(token::Token::LeftParen) => self.parse_grouping_expression(),
            Some(some) => panic!("{:?}", some),
            None => panic!()
        };
        fn returning(this: &mut Parser, ret: ast::Expression) -> ast::Expression {
            if this.get(0) == Some(token::Token::LeftParen) {
                let ret = this.parse_call_expression(ret);
                returning(this, ret)
            } else {
                ret
            }
        }
        returning(self, ret)
    }

    fn parse_block_statement(&mut self) -> ast::Expression {
        assert_eq!(self.get(0), Some(token::LeftBrace));
        let count = self.counter.0;
        self.counter.0 += 1;
        self.cur_token += 1;
        let mut statements = ast::BlockStatement::new();
        for _ in self.cur_token..self.tokens.len() {
            if let Some(token::RightBrace) = self.get(0) {
                self.counter.0 -= 1;
                self.cur_token += 1;
                if count-self.counter.0 == 0 {
                    break;
                }
            }
            statements.push(self.parse_statement());
        }
        ast::Expression::BlockExpression(statements)
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        assert_eq!(self.get(0), Some(token::Let));
        self.cur_token += 1;
        if let Some(token::Token::Ident(ident)) = self.get(0) {
            self.cur_token += 1;
            assert_eq!(self.get(0), Some(token::Assign));
            self.cur_token += 1;
            let value = self.parse_expression();
            assert_eq!(self.get(0), Some(token::Semicolon));
            ast::Statement::Let {
                name: ident,
                value: value
            }
        } else {
            panic!()
        }
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        assert_eq!(self.get(0), Some(token::Return));
        self.cur_token += 1;
        ast::Statement::Return(self.parse_expression())
    }

    fn parse_call_expression(&mut self, lambda: ast::Expression) -> ast::Expression {
        assert_eq!(self.get(0), Some(token::Token::LeftParen));
        let mut expressions = Vec::new();
        if self.get(1) != Some(token::RightParen) {
            while self.get(0) != Some(token::Token::RightParen) {
                self.cur_token += 1;
                expressions.push(self.parse_expression());
                assert!(match self.get(0) {
                    Some(token::Token::Comma) | Some(token::RightParen) => true,
                    _ => false
                })
            }
        } else {
            self.cur_token += 1;
        }
        assert_eq!(self.get(0), Some(token::RightParen));
        self.cur_token += 1;
        ast::Expression::CallExpression {
            parameters: expressions,
            lambda: Box::new(lambda)
        }
    }

    fn parse_grouping_expression(&mut self) -> ast::Expression {
        assert_eq!(self.get(0), Some(token::LeftParen));
        self.cur_token += 1;
        let count = self.counter.1;
        self.counter.1 += 1;        
        let ret = self.parse_expression();
        assert_eq!(self.get(0), Some(token::RightParen));
        self.counter.1 -= 1;
        assert_eq!(self.counter.1, count);
        ret
    }

    fn parse_operator_precedance(&mut self) -> ast::Expression {
        todo!()
    }

    fn parse_literal(&mut self) -> ast::Expression {
        let temp = self.get(0);
        self.cur_token += 1;
        match temp {
            Some(token::Int(v)) => ast::Expression::Int(v),
            Some(token::Float(v)) => ast::Expression::Float(v),
            Some(token::String(v)) => ast::Expression::String(v),
            Some(token::Boolean(v)) => ast::Expression::Boolean(v),
            _ => panic!()
        }
    }

    fn parse_function(&mut self) -> ast::Expression{
        assert_eq!(self.get(0), Some(token::Token::Function));
        self.cur_token += 1;
        assert_eq!(self.get(0), Some(token::Token::LeftParen));
        
        let mut params = ast::Parameters::new();
        while self.get(0) != Some(token::Token::RightParen) {
            self.cur_token += 1;
            params.push(
                if let Some(token::Ident(ident)) = self.get(0) {
                    ident
                } else {
                    match self.get(0) {
                        Some(token::Token::Comma)=> continue,
                        Some(token::RightParen) => break,
                        _ => panic!()
                    };
                });
        }
        self.cur_token += 1;

        ast::Expression::Function {
            params: params,
            body: Box::new(self.parse_expression())
        }
    }
}

impl From<lexer::Lexer> for Parser {
    fn from(lex: lexer::Lexer) -> Parser {
        Self {
            tokens: lex.collect(),
            cur_token: 0,
            counter: (0, 0)
        }
    }
}

impl From<Vec<token::Token>> for Parser {
    fn from(tokens: Vec<token::Token>) -> Parser {
        Self {
            tokens: tokens,
            cur_token: 0,
            counter: (0, 0)
        }
    }
}


#[cfg(test)]
pub mod test {
    use std::any::Any;
    use std::rc::Rc;

    use super::*;
    use ast::{ Statement, Expression };
    use core_stable::lexer;

    struct TestLetStatement {
        pub input: String,
        pub expected_ast: ast::Statement
    }

    #[test]
    pub fn test_let_statement() {
        let tests = [
            TestLetStatement {
                input: String::from("let x = 5;"),
                expected_ast: ast::Statement::Let {
                    name: String::from("x"),
                    value: ast::Expression::Int(5)
                }
            },

            TestLetStatement {
                input: String::from("let y = true;"),
                expected_ast: ast::Statement::Let {
                    name: ast::Identifier::from("y"),
                    value: ast::Expression::Boolean(true)
                }
            },

            TestLetStatement {
                input: String::from("let foobar = y;"),
                expected_ast: ast::Statement::Let {
                    name: ast::Identifier::from("foobar"),
                    value: ast::Expression::Ident(ast::Identifier::from("y"))
                }
            }
        ];

        for test in tests.iter() {
            let statement = Parser::new(test.input.clone()).parse_statement();
            println!("Test: {:?}\n\nGot: {:?}\n\n\n", test.expected_ast, statement);
            assert!(match statement {
                ast::Statement::Let{ .. } => true,
                _ => false
            });

            assert_eq!(statement, test.expected_ast);
        }
    }

    struct TestReturnStatements {
        pub input: String,
        pub expected_value: ast::Expression,
    }

    #[test]
    pub fn test_return_statements() {
        let tests = vec![
            TestReturnStatements {
                input: String::from("return 5;"),
                expected_value: ast::Expression::Int(5),
            },

            TestReturnStatements {
                input: String::from("return true;"),
                expected_value: ast::Expression::Boolean(true)
            },
/*
            TestReturnStatements {
                input: String::from("return foobar;"),
                expected_value: ast::Expression::String(String::from("foobar")),
            },*/
        ];

        for test in tests.iter() {
            let statement = Parser::new(test.input.clone()).parse_statement();
            assert!(match statement {
                ast::Statement::Return(value) => {
                    assert_eq!(test.expected_value, value);
                    true
                },
                _ => false
            });
            
        }
    }
/*
    struct TestParsingPrefixExpressions<'a> {
        input: String,
        operator: String,
        value: Box<dyn Any>,
    }

    #[test]
    pub fn test_parsing_prefix_expressions() {
        let tests = vec![
            TestParsingPrefixExpressions {
                input: String::from("!5;"),
                operator: String::from("!"),
                value: Box::new(5),
            },

            TestParsingPrefixExpressions {
                input: String::from("-15;"),
                operator: String::from("-"),
                value: Box::new(15),
            },

            TestParsingPrefixExpressions {
                input: String::from("!foobar"),
                operator: String::from("!"),
                value: Box::new(String::from("foobar")),
            },

            TestParsingPrefixExpressions {
                input: String::from("-foobar;"),
                operator: String::from("-"),
                value: Box::new(String::from("foobar")),
            },

            TestParsingPrefixExpressions {
                input: String::from("!true;"),
                operator: String:from("!"),
                value: Box::new(true),
            },

            TestParsingPrefixExpressions {
                input: String::from("!false;"),
                operator: String::from("!"),
                value: Box::new(false),
            },
        ];

        for test in tests.iter() {
            let program = setup_program(test.input);
            
            assert_eq!(program.statements.len(), 1);

            let statement = program.statements[0] as ast::ExpressionStatement;

            let prefix_expression = statement.expression as ast::PrefixExpression;
            assert_eq!(prefix_expression.operator, test.operator);
            assert!(test_literal_expression(prefix_expression.right, test.value));
        }
    }

    struct TestInfixExpression {
        input: String,
        left_value: Box<dyn Any>,
        operator: String,
        right_value: Box<dyn Any>,
    }

    #[test]
    pub fn test_parsing_infix_expressions() {
        let tests_infix = vec![
            TestInfixExpression {
                input: String::from("5 + 5;"),
                left_value: Box::new(5),
                operator: String::from("+"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 - 5;"),
                left_value: Box::new(5),
                operator: String::from("-"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 * 5;"),
                left_value: Box::new(5),
                operator: String::from("*"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 / 5;"),
                left_value: Box::new(5),
                operator: String::from("/"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 > 5;"),
                left_value: Box::new(5),
                operator: String::from(">"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 < 5;"),
                left_value: Box::new(5),
                operator: String::from("<"),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 == 5;"),
                left_value: Box::new(5),
                operator: String::from("=="),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("5 != 5"),
                left_value: Box::new(5),
                operator: String::from("!="),
                right_value: Box::new(5),
            },

            TestInfixExpression {
                input: String::from("foobar + barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("+"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar - barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("-"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar * barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("*"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar / barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("/"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar > barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from(">"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar < barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("<"),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar == barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("=="),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("foobar != barfoo;"),
                left_value: Box::new(String::new("foobar")),
                operator: String::from("!="),
                right_value: Box::new(String::from("barfoo")),
            },

            TestInfixExpression {
                input: String::from("true == true;"),
                left_value: Box::new(true),
                operator: String::from("=="),
                right_value: Box::new(true),
            },

            TestInfixExpression {
                input: String::from("true != false;"),
                left_value: Box::new(true),
                operator: String::from("!="),
                right_value: Box::new(false),
            },

            TestInfixExpression {
                input: String::from("false == false;"),
                left_value: Box::new(false),
                operator: String::from("=="),
                right_value: Box::new(false),
            },
        ];

        for test in tests_infix.iter() {
            let program = setup_program(test.input);

            assert_eq!(program.statements.len(), 1);
            let statement = program.statements[0] as ast::ExpressionStatement;

            if (!test_infix_expressions(statement.expression, test.left_value, test.operator, test.right_value)) {
                return ();
            }
        }
    }

    struct TestOperatorPrecedence {
        input: String,
        expected: String,
    }

    #[test]
    pub fn test_operator_precedence_parsing() {
        let tests = vec![
            TestOperatorPrecedence {
                input: String::from("-a * b"),
                expected: String::from("((-a) * b)"),
            },

            TestOperatorPrecedence {
                input: String::from("!-a"),
                expected: String::from("(!(-a)"),
            },

            TestOperatorPrecedence {
                input: String::from("a + b + c"),
                expected: String::from("((a + b) + c)"),
            },

            TestOperatorPrecedence {
                input: String::from("a + b - c"),
                expected: String::from("((a + b) - c)"),
            },

            TestOperatorPrecedence {
                input: String::from("a * b * c"),
                expected: String::from("((a * b) * c)"),
            },

            TestOperatorPrecedence {
                input: String::from("a * b / c"),
                expected: String::from("((a * b) / c)"),
            },

            TestOperatorPrecedence {
                input: String::from("a + b / c"),
                expected: String::from("(a + (b / c)"),
            },

            TestOperatorPrecedence {
                input: String::from("a + b * c + d / e - f"),
                expected: String::from("(((a + (b / c)) + (d / e)) -f)"),
            },

            TestOperatorPrecedence {
                input: String::from("3 + 4; -5 * 5"),
                expected: String::from("(3 + 4)((-5) * 5)"),
            },

            TestOperatorPrecedence {
                input: String::from("5 > 4 == 3 < 4"),
                expected: String::from("((5 > 4) == (3 < 4)))"),
            },

            TestOperatorPrecedence {
                input: String::from("5 < 4 != 3 > 4"),
                expected: String::from("((5 < 4) != (3 > 4)"),
            },

            TestOperatorPrecedence {
                input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
                expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            },

            TestOperatorPrecedence {
                input: String::from("true"),
                expected: String::from("true"),
            },

            TestOperatorPrecedence {
                input: String::from("false"),
                expected: String::from("false"),
            },

            TestOperatorPrecedence {
                input: String::from("3 > 5 == false"),
                expected: String::from("((3 > 5) == false)"),
            },

            TestOperatorPrecedence {
                input: String::from("3 < 5 == true"),
                expected: String::from("((3 < 5) == true)"),
            },

            TestOperatorPrecedence {
                input: String::from("1 + (2 + 3) + 4"),
                expected: String::from("((1 + (2 + 3)) + 4)"),
            },

            TestOperatorPrecedence {
                input: String::from("(5 + 5) * 2"),
                expected: String::from("((5 + 4) * 2"),
            },

            TestOperatorPrecedence {
                input: String::from("2 / (5 + 5"),
                expected: String::from("(2 / (5 + 5))"),
            },

            TestOperatorPrecedence {
                input: String::from("-(5 + 5)"),
                expected: String::from("(-(5 + 5))"),
            },

            TestOperatorPrecedence {
                input: String::from("!(true == true)"),
                expected: String::from("(!(true == true))"),
            },

            TestOperatorPrecedence {
                input: String::from("a + add(b * c) + d"),
                expected: String::from("((a + add((b * c))) + d)"),
            },

            TestOperatorPrecedence {
                input: String::from("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))"),
                expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
            },

            TestOperatorPrecedence {
                input: String::from("add(a + b + c * d / f + g)"),
                expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
            },
        ];

        for test in tests {
            let program = setup_program(test.input);

            let actual = program.string();
            assert_eq!(actual, test.expected);
        }
    }

    struct TestBoolean {
        input: String,
        expected_boolean: bool,
    }

    #[test]
    pub fn TestIfExpression() {
        let input = String::from("if (w < y) { x }");

        let program = setup_program(input);

        assert_eq!(program.statements.len(), 1);
        let statement = program.statements[0] as ast::ExpressionStatement;
        let expression = statement.expression as ast::IfExpression;
        if (!test_infix_expression(expression.condition, "x", "<", "y")) {
            return ();
        }
        assert_eq!(expression.consequence.statements.len(), 1);

        if (!test_identifier(consequence.expression, "x")) {
            return ();
        }

        assert_ne!(expression.alternative, None);
    }

    #[test]
    pub fn test_if_else_expression() {
        let input = String::from("if (x<y) { x } else { y }");

        let program = setup_program(String::from("input"));

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0] as ast::ExpressionStatement<_>;

        let expression = statement.expression as ast::IfExpression<_>;

        if !test_infix_expression(expression.condition, "x", "<", "y") {
            return ();
        }

        assert_eq!(expression.consequence.statements.len(), 1),

        let consequence = expression.consequence.statements[0] as ast::ExpressionStatement<_>;

        if (!test_identifier(consequence.expression, "x") {
            return ();
        }

        assert_eq!(expression.alternative.statements.len(), 1);

        let alternative = expression.alternative.statements[0] as ast::ExpressionStatement<_>;
        if (!test_identifier(alternative.expression, "y") {
            return ();
        }
    }

    #[test]
    pub fn test_function_literal_parsing() {
        let input = String::from("fn(x, y) { x + y; }");

        let program = setup_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0] as ast::ExpressionStatement;

        let function = statement.expression as ast::FunctionLiteral;

        assert_eq!(function.paramters.len(), 2);

        test_literal_expression(function.parameters[0], "x");
        test_literal_expression(function.paramters[1], "y");

        assert_eq!(function.body.statements.len(), 1);

        let body_statement = function.body.statements[0] as ast::ExpressionStatement;

        test_infix_expression(body_statement.expression, "x", "+", "y");
    }
*/
    struct TestParametersParsing {
        input: String,
        expected_parameters: Vec<ast::Identifier>,
    }

    #[test]
    pub fn test_function_parameter_parsing() {
        let tests = vec![
            TestParametersParsing {
                input: String::from("fn() {};"),
                expected_parameters: Vec::new(),
            },

            TestParametersParsing {
                input: String::from("fn(x) {};"),
                expected_parameters: vec![
                    String::from("x"),
                ],
            },

            TestParametersParsing {
                input: String::from("fn(x, y, z) {};"),
                expected_parameters: vec![
                    String::from("x"),
                    String::from("y"),
                    String::from("z"),
                ],
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            println!("{}", i);
            let expression = Parser::new(test.input.clone()).parse_expression();
            match expression {
                ast::Expression::Function{ ref params, .. } => {
                    println!("Expected length {}, got {} !", test.expected_parameters.len(), params.len());
                    assert_eq!(params.len(), test.expected_parameters.len());
                    test.expected_parameters
                        .iter()
                        .zip(params.iter())
                        .for_each(|(expected, got)| {
                            println!("Expected: {}\nGot: {}\n\n", expected, got);
                            assert_eq!(expected, got);
                        });
                },
                _ => panic!()
            }
        }
    }

    #[test]
    pub fn test_call_expression_parsing() {
        let input = String::from("add(1, foobar, { 45; }); rec()(1);");
        let mut pars = Parser::new(input);
        let stmnt_1 = pars.parse_statement();
        match stmnt_1 {
            ast::Statement::Expr(ast::Expression::CallExpression{ parameters, lambda }) => {
                println!("Parameters: {:?}", parameters);
                println!("Lambda: {:?}", *lambda);
                assert_eq!(parameters, vec![
                    ast::Expression::Int(1),
                    ast::Expression::Ident(String::from("foobar")),
                    ast::Expression::BlockExpression(vec![ast::Statement::Expr(ast::Expression::Int(45))])
                ]);
                assert_eq!(*lambda, ast::Expression::Ident(String::from("add")));
            },
            _ => panic!()
        };
        let stmnt_2 = pars.parse_statement();
        match stmnt_2 {
            ast::Statement::Expr(ast::Expression::CallExpression{ parameters, lambda }) => {
                println!("Parameters: {:?}", parameters);
                println!("Lambda: {:?}", *lambda);
                assert_eq!(parameters, vec![ast::Expression::Int(1)]);
                match *lambda {
                    ast::Expression::CallExpression{ parameters, lambda } => {
                        assert_eq!(parameters, vec![]);
                        match *lambda {
                            ast::Expression::Ident(ident) => {
                                assert_eq!(ident, String::from("rec"));
                            },
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
    }
/*
    pub fn test_infix_expression<T>(expression: Box<dyn ast::Expression<T = T>>, left: Box<dyn Any>, operator: String, right: Box<dyn Any>) -> bool {
        let op_exp = expression as ast::InfixExpression<_, _>;

        if (!test_literal_expression(op_exp.left, left) {
            return false;
        }

        assert_eq!(opExp.operator, operator);

        if (!test_literal_expression(op_exp.right, right) {
            return false;
        }
        true
    }

    pub fn test_literal_expression<T>(expression: Box<dyn ast::Expression<T = T>>, expected: Box<dyn Any>) -> bool {
        match expected.type_id() {
            TypeId::of::<isize>() => return test_integer_literal(expression, expected.downcast_ref::<isize>()),
            TypeId::of::<String>() => return test_identifier(expression, expected.downcast_ref::<String>()),
            TypeId::of::<bool>() => return test_boolean_literal(expression, expected.downcast_ref::<bool>()),
        }
        false
    }

    pub fn test_identifier(expression: Box<ast::Expression>, value: String) -> bool {
        let ident = (*expression) as ast::Identifier
    }*/
}
