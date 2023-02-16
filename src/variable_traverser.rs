use std::collections::HashMap;

use crate::nodes::*;

pub struct VariableTraverser {
    // later there will be multiple variable tables for each scope or something like that
    pub variable_table: HashMap<String, Type> 
}

impl VariableTraverser {
    pub fn new() -> VariableTraverser {
        VariableTraverser {
            variable_table: HashMap::new()
        }
    }

    pub fn traverse(&mut self, scope_node: &mut ScopeNode) -> Result<(), String> {
        for command in &mut scope_node.commands {
            match command {
                CommandNode::VariableDefinitionNode(var_def_node) => {
                    let vtype;
                    // check for left type definition (e.g. let uint x = 'a'; => uint)
                    if let Some(vtype_str) = &var_def_node.vtype  {
                        vtype = Type::from(vtype_str); 
                        // check for right type definition (e.g. let uint x = 'a'; => char) and see if they conflict
                        if let Some(right_expr) = &mut var_def_node.expression {
                            self.assign_expression_type(right_expr)?;
                            let right_type = right_expr.t.as_ref().unwrap();
                            if vtype != *right_type {
                                return Err(format!("Mismatching variable types in variable definition: \"{}\" (left) and \"{}\" (right)", vtype_str, right_type.to_str()));
                            }
                        }
                    } else {
                        // check for right type, if the left type is not there, it has to be there
                        if let Some(right_expr) = &mut var_def_node.expression {
                            self.assign_expression_type(right_expr)?;
                            vtype = right_expr.t.as_ref().unwrap().clone();
                        } else {
                            return Err(format!("Undefined type for variable definition!"));
                        }
                    }
                    self.variable_table.insert(var_def_node.variable.name.clone(), vtype);
                } 
                
                CommandNode::VariableAssignmentNode(var_assign_node) => {
                    // test if the variable even exists
                    let vtype = match self.variable_table.get(&var_assign_node.variable.name) {
                        Some(t) => t,
                        None => { return Err(format!("Assigning to undefined variable \"{}\"", &var_assign_node.variable.name)); }
                    };

                    let right_expr = var_assign_node.expression.as_mut();
                    self.assign_expression_type(right_expr)?;
                    // check if the types match
                    let right_type = right_expr.t.as_ref().unwrap();
                    if *vtype != *right_type {
                        return Err(format!("Cannot assign expression of type \"{}\" to variable of type \"{}\"", right_type.to_str(), vtype.to_str()));
                    }
                }

                CommandNode::FunctionCallNode(func_call_node) => {
                    // only the print function exists atm, so this is hardcoded
                    if func_call_node.function.name != "print" {
                        return Err(format!("Undefined function \"{}\" (only the print function is implemented yet)", func_call_node.function.name));
                    }
                    if func_call_node.args.len() != 1 {
                        return Err(format!("Invalid number of elements for print function! (You have to supply exactly one element)"));
                    }
                    let arg_expr: &mut TExpressionNode = &mut func_call_node.args[0];
                    self.assign_expression_type(arg_expr)?;
                    // for now, every type is allowed for the print function
                }
            }
        } 
        return Ok(());
    }


    // determine the "t" (type) field for an expression node (and also for the child nodes, if they exist)
    fn assign_expression_type(&self, expression_node: &mut TExpressionNode) -> Result<(), String> {
        let expression_type: Type = match &mut expression_node.node {
            ExpressionNode::VariableNode(var_node) => {
                // check if the variable exists -> if yes, return type of the variable
                let type_result = self.variable_table.get(&var_node.name);
                match type_result {
                    Some(t) => *t,
                    None => { return Err(format!("Usage of undefined variable \"{}\" in expression!", var_node.name)); }
                }
            },
        
            ExpressionNode::UnaryOperationNode(unary_op_node) => {
                // inherit type of the child expression node, but the type has to be numeric (int/float)
                let sub_expression_node = &mut unary_op_node.expression;
                self.assign_expression_type(sub_expression_node)?;
                let sub_expression_type = sub_expression_node.t.as_ref().unwrap();
                match sub_expression_type {
                    Type::Int | Type::Float => {},
                    _ => {
                        return Err(format!("Invalid type \"{}\" for unary operation (must be either int or float)", sub_expression_type.to_str()));
                    }
                }
                *sub_expression_type
            }
            
            ExpressionNode::BinaryOperationNode(binary_op_node) => {
                let left_expr_node = &mut binary_op_node.left_expr;
                let right_expr_node = &mut binary_op_node.right_expr;
                self.assign_expression_type(left_expr_node)?;
                self.assign_expression_type(right_expr_node)?;
                let left_expr_type = left_expr_node.t.as_ref().unwrap();
                let right_expr_type = right_expr_node.t.as_ref().unwrap();
                // both types have to be numeric (int/float)
                if (*left_expr_type != Type::Int && *left_expr_type != Type::Float) || (*right_expr_type != Type::Int && *right_expr_type != Type::Float) {
                    return Err(format!("Invalid types \"{}\" and \"{}\" for binary operation!", left_expr_type.to_str(), right_expr_type.to_str()))
                }
                // if at least one of them is float, then the parent type is also float
                if *right_expr_type == Type::Float || *left_expr_type == Type::Float {
                    Type::Float
                } else {
                    Type::Int
                }
            }

            ExpressionNode::FunctionCallNode(func_call_node) => {
                return Err(format!("Function calls in expressions aren't supported yet! (Tried to call \"{}()\" in expression)", func_call_node.function.name))
            }

            // so complicated...
            ExpressionNode::IntLiteralNode(_) => Type::Int,
            ExpressionNode::FloatLiteralNode(_) => Type::Float,
            ExpressionNode::BoolLiteralNode(_) => Type::Bool,
            ExpressionNode::CharLiteralNode(_) => Type::Char,
            ExpressionNode::StringLiteralNode(_) => Type::Str,
        };
        expression_node.t = Some(expression_type);
        return Ok(());
    }
}

