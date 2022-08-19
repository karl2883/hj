use crate::lexer::{Token, TokenType};

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

impl Operator {
    fn from(op: &str) -> Operator {
        match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "%" => Operator::Modulo,
            _ => panic!("Internal compiler error (unknown operator)")
        }
    }

    fn priority_score(&self) -> u32 {
        match self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Multiply | Operator::Divide | Operator::Modulo => 2,
        }
    }
}

#[derive(Debug)]
pub struct ScopeNode {
    commands: Vec<CommandNode>,
}

#[derive(Debug)]
struct VariableNode {
    name: String,
}

#[derive(Debug)]
struct VariableDefinitionNode {
    vtype: Option<String>, // temporary
    variable: VariableNode,
    expression: Option<Box<ExpressionNode>>,
}

#[derive(Debug)]
struct VariableAssignmentNode {
    variable: VariableNode,
    expression: Box<ExpressionNode>,
}

#[derive(Debug)]
struct BinaryOperationNode {
    left_expr: Box<ExpressionNode>,
    operator: Operator,
    right_expr: Box<ExpressionNode>,
}

#[derive(Debug)]
struct UnaryOperationNode {
    operator: Operator,
    expression: Box<ExpressionNode>,
}

#[derive(Debug)]
struct IntLiteralNode {
    value: i64,
}

#[derive(Debug)]
struct FloatLiteralNode {
    value: f64,
}

#[derive(Debug)]
struct BoolLiteralNode {
    value: bool,
}

#[derive(Debug)]
struct StringLiteralNode {
    value: String,
}

#[derive(Debug)]
struct CharLiteralNode {
    value: char,
}

#[derive(Debug)]
struct FunctionCallNode {
    function: FunctionNode,
    args: Vec<ExpressionNode>,
}

#[derive(Debug)]
struct FunctionNode {
    name: String,
}

#[derive(Debug)]
enum CommandNode {
    VariableDefinitionNode(VariableDefinitionNode),
    VariableAssignmentNode(VariableAssignmentNode),
    FunctionCallNode(FunctionCallNode),
}

#[derive(Debug)]
enum ExpressionNode {
    BinaryOperationNode(BinaryOperationNode),
    UnaryOperationNode(UnaryOperationNode),
    VariableNode(VariableNode),
    IntLiteralNode(IntLiteralNode),
    FloatLiteralNode(FloatLiteralNode),
    BoolLiteralNode(BoolLiteralNode),
    StringLiteralNode(StringLiteralNode),
    CharLiteralNode(CharLiteralNode),
    FunctionCallNode(FunctionCallNode),
}

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, idx: 0 }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.idx);
        self.idx += 1;
        return token;
    }

    fn next_or_err(&mut self, msg: &str) -> Result<&Token, String> {
        match self.tokens.get(self.idx) {
            Some(token) => {
                self.idx += 1;
                Ok(token)
            }
            None => Err(String::from(msg))
        }
    }

    fn get(&self, relative_idx: usize) -> Option<&Token> {
        self.tokens.get(self.idx + relative_idx)
    }

    fn get_or_err(&self, relative_idx: usize, msg: &str) -> Result<&Token, String> {
        match self.tokens.get(self.idx + relative_idx) {
            Some(token) => Ok(token),
            None => Err(String::from(msg))
        }
    }

    fn parse_single_value(&mut self) -> Result<ExpressionNode, String> {
        let next_token = self.next_or_err("Unexpected EOF when trying to parse expression (missing value)")?;
        match next_token.kind {
            // parenthesis -> nested
            TokenType::OpenParen => {
                let value = self.parse_expression()?;
                // safe because otherwise parse_expression would've thrown an error
                let next = self.next().unwrap();
                match next.kind {
                    TokenType::CloseParen => {
                        Ok(value)
                    }
                    _ => {
                        Err(format!("Unexpected token \"{}\" after expression (expected closing parenthesis)", next.value))
                    }
                }
            }

            // operator -> unary operator (either + or -)
            TokenType::Operator => {
                let operator = Operator::from(next_token.value.as_str());
                match operator {
                    Operator::Plus | Operator::Minus => {
                        let value = self.parse_single_value()?;
                        let node = UnaryOperationNode {
                            operator,
                            expression: Box::new(value),
                        };
                        Ok(ExpressionNode::UnaryOperationNode(node))
                    }
                    _ => {
                        Err(format!("The operator \"{}\" can't be used as a unary operator (expected value before it)", next_token.value))
                    }
                }
            }

            // literals (nice and easy)
            TokenType::NumberLiteral => {
                if next_token.value.contains(".") {
                    let node = FloatLiteralNode { value: next_token.value.parse::<f64>().unwrap() };
                    Ok(ExpressionNode::FloatLiteralNode(node))
                } else {
                    let node = IntLiteralNode { value: next_token.value.parse::<i64>().unwrap() };
                    Ok(ExpressionNode::IntLiteralNode(node))
                }
            }

            TokenType::BoolLiteral => {
                let mut value = false;
                if next_token.value == "true" {
                    value = true; 
                }
                let node = BoolLiteralNode { value };
                Ok(ExpressionNode::BoolLiteralNode(node))
            }

            TokenType::StringLiteral => {
                let node = StringLiteralNode { value: next_token.value.clone() };
                Ok(ExpressionNode::StringLiteralNode(node))
            }

            TokenType::CharLiteral => {
                let node = CharLiteralNode { value: next_token.value.chars().next().unwrap() };
                Ok(ExpressionNode::CharLiteralNode(node))
            }
            
            // variables and function calls
            TokenType::Name => {
                // so we avoid borrowing errors in case it's a variable
                let name = next_token.value.clone();
                let second = self.get_or_err(0, "Unexpected EOF while parsing expression (missed a semicolon?)")?;
                match second.kind {
                    // function call
                    TokenType::OpenParen => {
                        self.idx -= 1;
                        let node = self.parse_function_call()?;
                        Ok(ExpressionNode::FunctionCallNode(node))
                    }
                    // variable
                    _ => {
                        let node = VariableNode { name };
                        Ok(ExpressionNode::VariableNode(node))
                    }
                }
            }

            _ => {
                Err(format!("Unexpected token \"{}\" while parsing expression, expected value", next_token.value))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<ExpressionNode, String> {
        // we need a basis node for the expression, so we parse the first token(s)
        let mut current_expression = self.parse_single_value()?;
        loop {
            let next_token = self.next_or_err("Unexpected EOF when trying to parse expression (missed a semicolon?)")?;
            match next_token.kind {
                // stop tokens
                TokenType::Comma | TokenType::CloseParen | TokenType::Semicolon => {
                    // expression over, caller of expression function should deal with stop tokens
                    self.idx -= 1;
                    break;
                },

                TokenType::Operator => {
                    let op = Operator::from(next_token.value.as_str());
                    match current_expression {
                        ExpressionNode::BinaryOperationNode(ref mut node) => {
                            let mut lvl = 0;
                            let mut current_top_lvl = node;
                            while current_top_lvl.operator.priority_score() < op.priority_score() {
                                lvl += 1;
                                if let ExpressionNode::BinaryOperationNode(ref mut child_node) = *current_top_lvl.right_expr {
                                    current_top_lvl = child_node;
                                } else {
                                    break;
                                }
                            }
                            
                            if lvl == 0 {
                                let node = BinaryOperationNode {
                                    left_expr: Box::new(current_expression),
                                    operator: op,
                                    right_expr: Box::new(self.parse_single_value()?),
                                };
                                current_expression = ExpressionNode::BinaryOperationNode(node);
                            } else {
                                let mut current_top_lvl_expr = &mut current_expression;
                                for _ in 0..lvl-1 {
                                    current_top_lvl_expr = match current_top_lvl_expr {
                                        ExpressionNode::BinaryOperationNode(ref mut node) => {
                                            node.right_expr.as_mut()
                                        }
                                        _ => {return Err(String::from("Internal compiler error while parsing expression"))}
                                    } 
                                }
                                if let ExpressionNode::BinaryOperationNode(node) = current_top_lvl_expr {
                                    let mut expression_buffer = ExpressionNode::IntLiteralNode(IntLiteralNode{ value: 42 });
                                    std::mem::swap(node.right_expr.as_mut(), &mut expression_buffer);
                                    node.right_expr = Box::new(ExpressionNode::BinaryOperationNode(BinaryOperationNode{
                                        left_expr: Box::from(expression_buffer),
                                        operator: op,
                                        right_expr: Box::new(self.parse_single_value()?),
                                    }));
                                }
                            }
                        }
                        _ => {
                            let node = BinaryOperationNode {
                                left_expr: Box::new(current_expression),
                                operator: op,
                                right_expr: Box::new(self.parse_single_value()?),
                            };
                            current_expression = ExpressionNode::BinaryOperationNode(node);
                        }
                    }
                }


                // part of the expression
                _ => (),
            }
        }
        return Ok(current_expression);
    }

    fn parse_variable_definition(&mut self) -> Result<VariableDefinitionNode, String> {
        // we can assume the "let" is there because the method got called
        self.idx += 1;

        let mut vtype: Option<String> = None;
        let mut first = self.next_or_err("Unexpected EOF when trying to parse a variable definition (expected variable name or type)")?;
        if let TokenType::InbuiltType = first.kind {
            vtype = Some(first.value.clone());
            first = self.next_or_err("Unexpected EOF when trying to parse a variable definition (expected variable name)")?;
        }

        let var_name = match first.kind {
            TokenType::Name => first.value.clone(),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable definition (expected variable name)", first.value))}
        };
        let var_node = VariableNode {name: var_name};

        let assignment_operator = self.next_or_err("Unexpected EOF when trying to parse a variable definition (expected equal sign)")?;
        let assignment_operator = match assignment_operator.kind {
            TokenType::AssignmentOperator => assignment_operator.value.clone(),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable definition (expected variable name)", assignment_operator.value))}
        };

        let expression = match assignment_operator.as_str() {
            "=" => self.parse_expression()?,
            _ => {return Err(format!("Can't use special assignment operator \"{}\" for a variable definition", assignment_operator))}
        };

        let semicolon = self.next_or_err("Unexpected EOF when trying to parse a variable assignment (expected semicolon)")?;

        match semicolon.kind {
            TokenType::Semicolon => (),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable assignment (expected semicolon)", semicolon.value))}
        }

        return Ok(VariableDefinitionNode {vtype, variable: var_node, expression: Some(Box::new(expression))})
    }

    fn parse_variable_assignment(&mut self) -> Result<VariableAssignmentNode, String> {
        let var_name = self.next_or_err("Unexpected EOF when trying to parse a variable assignment (expected variable name)")?;
        let var_name = match var_name.kind {
            TokenType::Name => var_name.value.clone(),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable assignment (expected variable name)", var_name.value))}
        };
        let var_node = VariableNode {name: var_name.clone()};

        let assignment_operator = self.next_or_err("Unexpected EOF when trying to parse a variable assignment (expected equal sign)")?;
        let assignment_operator = match assignment_operator.kind {
            TokenType::AssignmentOperator => assignment_operator.value.clone(),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable assignment (expected variable name)", assignment_operator.value))}
        };

        let expression = match assignment_operator.as_str() {
            "=" => self.parse_expression()?,
            _ => {
                let operator = assignment_operator.get(..1).unwrap();
                let operator = Operator::from(operator);
                let op_node = BinaryOperationNode {
                    left_expr: Box::new(ExpressionNode::VariableNode(VariableNode {name: var_name})),
                    operator,
                    right_expr: Box::new(self.parse_expression()?)
                };
                ExpressionNode::BinaryOperationNode(op_node)
            }
        };

        let semicolon = self.next_or_err("Unexpected EOF when trying to parse a variable assignment (expected semicolon)")?;

        match semicolon.kind {
            TokenType::Semicolon => (),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing variable assignment (expected semicolon)", semicolon.value))}
        }

        return Ok(VariableAssignmentNode {variable: var_node, expression: Box::new(expression)})
    }

    fn parse_function_call(&mut self) -> Result<FunctionCallNode, String> {
        // we can assume it's a function name because that's when this function gets called
        let function_name = self.next().unwrap().value.clone();
        let function_node = FunctionNode {
            name: function_name,
        };
        // we can also assume that the opening parenthesis is there for the same reason
        self.idx += 1;

        let mut args: Vec<ExpressionNode> = vec!();
        let next_token = self.get_or_err(0, "Unexpected EOF when trying to parse function call (expected closing parenthesis)")?;
        match next_token.kind {
            TokenType::CloseParen => {
                self.idx += 1;
            },
            _ => {
                loop {
                    let expression = self.parse_expression()?;
                    args.push(expression);
                    let next_token = self.next_or_err("Unexpected EOF when trying to parse function call (expected closing parenthesis)")?;
                    match next_token.kind {
                        TokenType::CloseParen => {break;}
                        TokenType::Comma => (),
                        _ => {return Err(format!("Unexpected token \"{}\" in function parameters", next_token.value))}
                    }
                }
            }
        }

        let semicolon = self.next_or_err("Unexpected EOF when trying to parse function call (expected semicolon)")?;

        match semicolon.kind {
            TokenType::Semicolon => (),
            _ => {return Err(format!("Unexpected token \"{}\" while parsing function call (expected semicolon)", semicolon.value))}
        }

        Ok(FunctionCallNode {
            function: function_node,
            args,
        })
    }

    fn parse_command(&mut self) -> Result<CommandNode, String> {
        // it's ok to unwrap since this function will only get called when there are tokens left
        let first = self.get(0).unwrap();
        match first.kind {
            TokenType::Keyword => {
                if first.value == "let" {
                    let definition_node = self.parse_variable_definition()?;
                    Ok(CommandNode::VariableDefinitionNode(definition_node))
                } else {
                    Err(format!("Unexpected keyword \"{}\", expected a command (either a variable assignment or a function call)", first.value))
                }
            } 
            TokenType::Name => {
                let second = self.get_or_err(1, "Unexpected EOF when trying to parse a command")?;
                match second.kind {
                    TokenType::OpenParen => {
                        let function_call_node = self.parse_function_call()?;
                        Ok(CommandNode::FunctionCallNode(function_call_node))
                    }
                    TokenType::AssignmentOperator => {
                        let assignment_node = self.parse_variable_assignment()?;
                        Ok(CommandNode::VariableAssignmentNode(assignment_node))
                    }
                    _ => Err(format!("Unexpected token \"{}\" after custom name, expected a command (either a variable assignment or a function call)", second.value))
                }
            }
            _ => Err(format!("Unexpected token \"{}\", expected a command (either a variable assignment or a function call)", first.value))
        }
    }

    pub fn parse(&mut self) -> Result<ScopeNode, String> {
        let mut scope_node = ScopeNode { commands: vec!() }; 
        while self.idx < self.tokens.len() {
            scope_node.commands.push(self.parse_command()?);
        } 
        Ok(scope_node)
    }
}
