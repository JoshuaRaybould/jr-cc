use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, Expression as CExpression, Function as CFunction,
    Identifier as CIdentifier, Program as CProgram, Statement as CStatement,
    UnaryOperator as CUnaryOperator,
};

fn get_indentation(indent: usize) -> String {
    " ".repeat(4).repeat(indent)
}

pub fn pretty_print(program: CProgram) {
    let indent = 0;
    let CProgram(func) = program;

    println!("Program(");
    pretty_print_function(func, indent + 1);
    println!(")")
}

fn pretty_print_function(func: CFunction, indent: usize) {
    let indentation = get_indentation(indent);
    let CFunction(identifier, statement) = func;

    println!("{}Function(", indentation);
    pretty_print_identifier(identifier, indent + 1);
    pretty_print_statement(statement, indent + 1);
    println!("{})", indentation);
}

fn pretty_print_identifier(identifier: CIdentifier, indent: usize) {
    let indentation = get_indentation(indent);
    let CIdentifier(id_string) = identifier;
    println!("{}name=\"{}\",", indentation, id_string);
}

fn pretty_print_statement(statement: CStatement, indent: usize) {
    let indentation = get_indentation(indent);

    match statement {
        CStatement::Return(expression) => {
            println!("{}body=Return(", indentation);
            pretty_print_expression(expression, indent + 1);
            println!("{})", indentation);
        }
    }
}

fn pretty_print_expression(expression: CExpression, indent: usize) {
    let indentation = get_indentation(indent);
    let expr_str = get_expression_str(expression);

    println!("{}{}", indentation, expr_str);
}

fn get_expression_str(expression: CExpression) -> String {
    match expression {
        CExpression::Constant(val) => format!("Constant({})", val),
        CExpression::Unary(unary_operator, expression) => {
            let operator = get_unary_op(unary_operator);
            let expr_str = get_expression_str(*expression);
            format!("{}({})", operator, expr_str)
        }
        CExpression::Binary(binary_operator, expression, expression1) => {
            let operator = get_binary_op(binary_operator);
            let expr_str = get_expression_str(*expression);
            let expr_str1 = get_expression_str(*expression1);
            format!("{}({}, {})", operator, expr_str, expr_str1)
        }
    }
}

fn get_unary_op(operator: CUnaryOperator) -> String {
    match operator {
        CUnaryOperator::Complement => "~".to_string(),
        CUnaryOperator::Negate => "-".to_string(),
    }
}

fn get_binary_op(operator: CBinaryOperator) -> String {
    match operator {
        CBinaryOperator::Add => "Add".to_string(),
        CBinaryOperator::Subtract => "Subtract".to_string(),
        CBinaryOperator::Multiply => "Multiply".to_string(),
        CBinaryOperator::Divide => "Divide".to_string(),
        CBinaryOperator::Remainder => "Remainder".to_string(),
        CBinaryOperator::BitAnd => "BitAnd".to_string(),
        CBinaryOperator::BitOr => "BitOr".to_string(),
        CBinaryOperator::Xor => "Xor".to_string(),
        CBinaryOperator::LeftShift => "LShift".to_string(),
        CBinaryOperator::RightShift => "RShift".to_string(),
    }
}
