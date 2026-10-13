use crate::parser::ast::{
    BinaryOperator as CBinaryOperator, BlockItem as CBlockItem, Declaration as CDeclaration,
    Expression as CExpression, Function as CFunction, Identifier as CIdentifier,
    Program as CProgram, Statement as CStatement, UnaryOperator as CUnaryOperator,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct SemanticAnalysis {
    pub tmp_var: i32,
    pub var_map: HashMap<String, String>,
}

impl SemanticAnalysis {
    pub fn resolve_all(&mut self, program: CProgram) -> CProgram {
        let CProgram(f) = program;
        let res_f = self.resolve_function(f);
        return CProgram(res_f);
    }

    fn resolve_function(&mut self, function: CFunction) -> CFunction {
        let CFunction(identifier, block_items) = function;
        let res_block_items = self.resolve_block_items(block_items);
        return CFunction(identifier.clone(), res_block_items);
    }

    fn resolve_block_items(&mut self, block_items: Vec<CBlockItem>) -> Vec<CBlockItem> {
        let mut new_block_items = Vec::new();
        for block_item in block_items {
            match block_item {
                CBlockItem::S(statement) => {
                    new_block_items.push(CBlockItem::S(self.resolve_statement(statement)));
                }
                CBlockItem::D(declaration) => {
                    new_block_items.push(CBlockItem::D(self.resolve_declaration(declaration)));
                }
            }
        }
        return new_block_items;
    }

    fn resolve_declaration(&mut self, declaration: CDeclaration) -> CDeclaration {
        let CDeclaration(identifier, init) = declaration;
        let CIdentifier(id) = identifier;
        if self.var_map.contains_key(&id) {
            panic!("Duplicate variable declaration: {}", id);
        }
        let unique_name = format!("{}.{}", id, self.tmp_var);
        self.tmp_var += 1;
        self.var_map.insert(id.clone(), unique_name.clone());
        let new_init: Option<CExpression> = match init {
            Some(expression) => Some(self.resolve_expression(expression)),
            None => None,
        };
        return CDeclaration(CIdentifier(unique_name), new_init);
    }

    fn resolve_statement(&mut self, statement: CStatement) -> CStatement {
        match statement {
            CStatement::Return(expression) => {
                CStatement::Return(self.resolve_expression(expression))
            }
            CStatement::Exp(expression) => {
                return CStatement::Exp(self.resolve_expression(expression));
            }
            CStatement::Null => CStatement::Null,
        }
    }

    fn resolve_expression(&mut self, expression: CExpression) -> CExpression {
        match expression {
            CExpression::Constant(c) => CExpression::Constant(c),
            CExpression::Var(CIdentifier(id)) => {
                if self.var_map.contains_key(&id) {
                    return CExpression::Var(CIdentifier(self.var_map.get(&id).expect("").clone()));
                } else {
                    panic!("Undeclared variable");
                }
            }
            CExpression::Unary(unary_operator, expression) => {
                return CExpression::Unary(
                    unary_operator.clone(),
                    Box::new(self.resolve_expression(*expression)),
                );
            }
            CExpression::Binary(binary_operator, expression, expression1) => {
                return CExpression::Binary(
                    binary_operator.clone(),
                    Box::new(self.resolve_expression(*expression)),
                    Box::new(self.resolve_expression(*expression1)),
                );
            }
            CExpression::Assignment(expression, expression1) => {
                let lexpr = *expression;
                let rexpr = *expression1;
                match lexpr {
                    CExpression::Var(_) => {
                        return CExpression::Assignment(
                            Box::new(self.resolve_expression(lexpr)),
                            Box::new(self.resolve_expression(rexpr)),
                        );
                    }
                    _ => panic!("Left side of assignment invalid"),
                }
            }
        }
    }
}
