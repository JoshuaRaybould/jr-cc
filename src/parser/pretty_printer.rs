use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, BlockItem as CBlockItem, Declaration as CDeclaration,
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};

fn get_indentation(indent: usize) -> String {
    " ".repeat(4).repeat(indent)
}

pub fn pretty_print(program: &CProgram) {
    let indent = 0;
    let CProgram(func) = program;

    println!("Program(");
    pretty_print_function(func, indent + 1);
    println!(")")
}

fn pretty_print_function(func: &CFunction, indent: usize) {
    let indentation = get_indentation(indent);
    let CFunction(identifier, block_items) = func;

    println!("{}Function(", indentation);
    pretty_print_identifier(identifier, indent + 1);
    pretty_print_block_items(block_items, indent + 1);
    println!("{})", indentation);
}

fn pretty_print_block_items(block_items: &Vec<CBlockItem>, indent: usize) {
    for block_item in block_items {
        match block_item {
            CBlockItem::S(statement) => pretty_print_statement(statement, indent),
            CBlockItem::D(declaration) => pretty_print_declaration(declaration, indent),
        }
    }
}

fn pretty_print_identifier(identifier: &CIdentifier, indent: usize) {
    let indentation = get_indentation(indent);
    let CIdentifier(id_string) = identifier;
    println!("{}name=\"{}\",", indentation, id_string);
}

fn pretty_print_statement(statement: &CStatement, indent: usize) {
    let indentation = get_indentation(indent);

    match statement {
        CStatement::Return(expression) => {
            println!("{}Return(", indentation);
            pretty_print_expression(expression, indent + 1);
            println!("{})", indentation);
        }
        CStatement::Exp(expression) => {
            println!("{}Expression(", indentation);
            pretty_print_expression(expression, indent + 1);
            println!("{})", indentation);
        }
        CStatement::Null => println!("Null"),
    }
}

fn pretty_print_declaration(declaration: &CDeclaration, indent: usize) {
    let indentation = get_indentation(indent);
    let CDeclaration(identifier, optional_expr) = declaration;
    let CIdentifier(id) = identifier;
    match optional_expr {
        Some(expression) => {
            let expr_str = get_expression_str(expression);
            println!("{}int({} = ({}))", indentation, id, expr_str);
        }
        None => println!("{}int({})", indentation, id),
    };
}

fn pretty_print_expression(expression: &CExpression, indent: usize) {
    let indentation = get_indentation(indent);
    let expr_str = get_expression_str(expression);

    println!("{}{}", indentation, expr_str);
}

fn get_expression_str(expression: &CExpression) -> String {
    match expression {
        CExpression::Constant(val) => format!("Constant({})", val),
        CExpression::Unary(unary_operator, expression) => {
            let operator = get_unary_op(unary_operator);
            let expr_str = get_expression_str(expression);
            format!("{}({})", operator, expr_str)
        }
        CExpression::Binary(binary_operator, expression, expression1) => {
            let operator = get_binary_op(binary_operator);
            let expr_str = get_expression_str(expression);
            let expr_str1 = get_expression_str(expression1);
            format!("{}({}, {})", operator, expr_str, expr_str1)
        }
        CExpression::Var(identifier) => {
            let CIdentifier(id) = identifier;
            format!("{}", id)
        }
        CExpression::Assignment(expression, expression1) => {
            let expr_str_1 = get_expression_str(expression);
            let expr_str_2 = get_expression_str(expression1);
            format!("{}=({})", expr_str_1, expr_str_2)
        }
    }
}

fn get_unary_op(operator: &CUnaryOperator) -> String {
    match operator {
        CUnaryOperator::Complement => "~".to_string(),
        CUnaryOperator::Negate => "-".to_string(),
        CUnaryOperator::Not => "!".to_string(),
    }
}

fn get_binary_op(operator: &CBinaryOperator) -> String {
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
        CBinaryOperator::And => "And".to_string(),
        CBinaryOperator::Or => "Or".to_string(),
        CBinaryOperator::Equal => "equal".to_string(),
        CBinaryOperator::Neq => "NotEqual".to_string(),
        CBinaryOperator::Le => "LessThan".to_string(),
        CBinaryOperator::Leq => "Leq".to_string(),
        CBinaryOperator::Ge => "GreaterThan".to_string(),
        CBinaryOperator::Geq => "Geq".to_string(),
        CBinaryOperator::Assign => "Assign".to_string(),
    }
}
