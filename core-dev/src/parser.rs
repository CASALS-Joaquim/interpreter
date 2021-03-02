use std::any::Any;

use core_stable::ast;
use core_stable::lexer;

#[cfg(test)]
pub mod test {
    use super as parser;
    use super::lexer;
    use super::ast;
    use std::any::Any;

    pub fn setup_program(input: String) -> ast::Program {
        let mut lex = lexer::Lexer::new(test.input);
        let mut pars = parser::Parser::new(lex);
        let program = pars.parse_program();
        check_parser_error(&test, &pars);
        program
    }
    struct TestLetStatement {
        pub input: String,
        pub expected_identifier: String,
        pub expected_value: Box<dyn Any>,
    }

    #[test]
    pub fn test_let_statements() {
        let tests = vec![
            TestLetStatement {
                input: String::from("let x = 5;"),
                expected_identifier: String::from("x"),
                expected_value: &(5 as isize)
            },

            TestLetStatement {
                input: String::from("let y = true"),
                expected_identifier: String::from("y"),
                expected_value: &true
            },

            TestLetStatement {
                input: String::from("let foobar = y"),
                expected_identifier: String::from("foobar"),
                expected_value: &String::from("y")
            }
        ];

        for test in tests.iter() {
            let program = setup_program(test.input);

            println!("Expected 1 statement, if not got, going to panic!, here got {}", program.statements.len());
            assert_eq!(program.statements.len(), 1);

            let statement = program.statements[0];
            assert!(test_let_statement(&statement, &test.expected_identifier));

            let value = (statement as ast::LetStatement).value;
            assert!(test_literal_expression(value, &test.expected_value));
        }
    }

    struct TestReturnStatements {
        pub input: String,
        pub expected_value: Box<dyn Any>,
    }
    #[test]
    pub fn test_return_statements() {
        let tests = vec![
            TestReturnStatements {
                input: String::from("return 5;"),
                expected_value: 5
            },

            TestReturnStatements {
                input: String::from("return true;"),
                expected_value: true
            },

            TestReturnStatements {
                input: String::from("return foobar;"),
                expected_value: &String::from("foobar"),
            },
        ];

        for test in tests.iter() {
            let program = setup_program(test.input);

            assert_eq!(program.statements.len(), 1);

            let statement = program.statements[0];
            let return_statement = statement as ast::ReturnStatement;

            assert_eq!(return_statement.token_literal(), String::from("return"));

            if test_literal_expression(return_statement.return_value, test.expected_value) {
                return ();
            }
        }
    }

    #[test]
    pub fn test_identifier_expression() {
        let input = String::from("foobar");

        let program = setup_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0] as ast::ExpressionStatement;

        let ident = statement.expression as ast::Identifier;

        assert_eq!(ident.value, String::from("foobar"));
        asser_eq!(ident.token_literal(), String::from("foobar"));
    }

    #[test]
    pub fn test_integer_literal_expression() {
        let input = String::from("5;");

        let program = setup_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = program.statements[0] as ast::ExpressionStatement;

        let literal = statement.expression as ast::IntegerLiteral;

        assert_eq!(literal.value, 5);
        assert_eq!(literal.token_literal, String::from("5"))
    }

    struct TestParsingPrefixExpressions<'a> {
        input: String,
        operator: String,
        value: Box<dyn Any>,
    }
    #[test]
    pub fn test_parsing_prefix_expressions() {
        let tests = vec![
            TestParsingPrefix {
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
            assert!(test_literal_expression(prefix_expression.right, test.value))
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
    pub fn test_boolean_expression() {
        let tests = vec![
            TestBoolean {
                input: String::from("true;"),
                expected_boolean: true,
            },

            TestBoolean {
                input: String::from("false;"),
                expected_boolean: false,
            },
        ];

        for test in tests.iter() {
            let program = setup_program(test.input);

            assert_eq!(program.statements.len(), 1);
            let statement = program.statements[0] as ast::ExpressionStatement;
            let boolean = statement.expression as ast::Boolean;
            assert_eq!(boolean.value, test.expected_boolean);
        }
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
}
