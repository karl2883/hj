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

    fn to_str(&self) -> String {
        String::from(
            match self {
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Modulo => "%",
            }
        )
    }

    fn priority_score(&self) -> u32 {
        match self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Multiply | Operator::Divide | Operator::Modulo => 2,
        }
    }
}

pub struct ScopeNode {
    commands: Vec<CommandNode>,
}

struct VariableNode {
    name: String,
}

struct VariableDefinitionNode {
    vtype: Option<String>, // temporary
    variable: VariableNode,
    expression: Option<Box<ExpressionNode>>,
}

struct VariableAssignmentNode {
    variable: VariableNode,
    expression: Box<ExpressionNode>,
}

struct BinaryOperationNode {
    left_expr: Box<ExpressionNode>,
    operator: Operator,
    right_expr: Box<ExpressionNode>,
}

struct UnaryOperationNode {
    operator: Operator,
    expression: Box<ExpressionNode>,
}

struct IntLiteralNode {
    value: i64,
}

struct FloatLiteralNode {
    value: f64,
}

struct BoolLiteralNode {
    value: bool,
}

struct StringLiteralNode {
    value: String,
}

struct CharLiteralNode {
    value: char,
}

struct FunctionCallNode {
    function: FunctionNode,
    args: Vec<ExpressionNode>,
}

struct FunctionNode {
    name: String,
}

enum CommandNode {
    VariableDefinitionNode(VariableDefinitionNode),
    VariableAssignmentNode(VariableAssignmentNode),
    FunctionCallNode(FunctionCallNode),
}

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

fn get_tab_str(tab_lvl: usize) -> String {
    String::from("\t").repeat(tab_lvl) 
}

impl ExpressionNode {
    fn debug_str(&self, tab_lvl: usize) -> String {
        match self {
            ExpressionNode::BinaryOperationNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Binary operation:\n";
                s += &node.left_expr.debug_str(tab_lvl+1);
                s += &format!("{}Operator: {}\n", get_tab_str(tab_lvl+1), node.operator.to_str());
                s += &node.right_expr.debug_str(tab_lvl+1);
                s
            }

            ExpressionNode::UnaryOperationNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Unary operation:\n";
                s += &format!("{}Operator: {}\n", get_tab_str(tab_lvl+1), node.operator.to_str());
                s += &node.expression.debug_str(tab_lvl+1);
                s
            }

            ExpressionNode::VariableNode(node) => {
                format!("{}Variable with name {}\n", get_tab_str(tab_lvl), node.name)
            }

            ExpressionNode::FunctionCallNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Function call";
                s += &format!("calling to function {}\n", node.function.name);
                if node.args.is_empty() {
                    s += &format!("{}without arguments\n", get_tab_str(tab_lvl));
                } else {
                    let mut i = 1;
                    for arg in &node.args {
                        s += &format!("{}Argument {}:\n", get_tab_str(tab_lvl), i);
                        s += &arg.debug_str(tab_lvl+1);
                        i += 1;
                    }
                }
                s
            }
            
            ExpressionNode::IntLiteralNode(node) => {
                format!("{}Int literal with value {}\n", get_tab_str(tab_lvl), node.value)
            }

            ExpressionNode::FloatLiteralNode(node) => {
                format!("{}Float literal with value {}\n", get_tab_str(tab_lvl), node.value)
            }

            ExpressionNode::CharLiteralNode(node) => {
                format!("{}Char literal with value '{}'\n", get_tab_str(tab_lvl), node.value)
            }

            ExpressionNode::StringLiteralNode(node) => {
                format!("{}String literal with value \"{}\"\n", get_tab_str(tab_lvl), node.value)
            }

            ExpressionNode::BoolLiteralNode(node) => {
                format!("{}Bool literal with value {}\n", get_tab_str(tab_lvl), node.value)
            }
        }
    }
}

impl CommandNode {
    fn debug_str(&self, tab_lvl: usize) -> String {
        match self {
            CommandNode::VariableDefinitionNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Variable definition";
                if let Some(vtype) = &node.vtype {
                    s += &format!(" with explicit type {}", vtype);
                }
                s += &format!(" defining variable {}\n", node.variable.name);
                if let Some(expr) = &node.expression {
                    s += &get_tab_str(tab_lvl+1);
                    s += "with expression:\n";
                    s += &expr.as_ref().debug_str(tab_lvl+1);   
                }
                s
            }

            CommandNode::VariableAssignmentNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Variable assignment ";
                s += &format!("assigning to variable {}\n", node.variable.name);
                s += &format!("{}with expression:\n", get_tab_str(tab_lvl+1));
                s += &node.expression.as_ref().debug_str(tab_lvl+1);   
                s
            }

            CommandNode::FunctionCallNode(node) => {
                let mut s = get_tab_str(tab_lvl) + "Function call command ";
                s += &format!("calling to function {}\n", node.function.name);
                if node.args.is_empty() {
                    s += &format!("{}without arguments\n", get_tab_str(tab_lvl));
                } else {
                    let mut i = 1;
                    for arg in &node.args {
                        s += &format!("{}Argument {}:\n", get_tab_str(tab_lvl), i);
                        s += &arg.debug_str(tab_lvl+1);
                        i += 1;
                    }
                }
                s
            }
        }
    }
}

impl ScopeNode {
    pub fn debug_str(&self) -> String {
        let mut s = String::from("Outer scope node with commands:\n");
        for command in &self.commands {
            s += &command.debug_str(1);
        }
        s
    }
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
                let node = StringLiteralNode { value: next_token.value.clone().trim_matches('"').to_string() };
                Ok(ExpressionNode::StringLiteralNode(node))
            }

            TokenType::CharLiteral => {
                let node = CharLiteralNode { value: next_token.value.chars().nth(1).unwrap() };
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

    fn parse_binary_expression(&mut self, left_expr: ExpressionNode) -> Result<ExpressionNode, String> {
        // we will assume the next token is an operator
        let op_token = self.next().unwrap();
        let op = Operator::from(op_token.value.as_str());
        let mut right_expr = self.parse_single_value()?;
        
        loop {
            let token_after = self.get_or_err(0, "Unexpected EOF when trying to parsing expression (missed a semicolon?)")?;
            if let TokenType::Operator = token_after.kind {
                let next_op = Operator::from(token_after.value.as_str());
                if next_op.priority_score() > op.priority_score() {
                    right_expr = self.parse_binary_expression(right_expr)?;
                    continue;
                }
            }
            break;
        }

        let node = BinaryOperationNode {
            left_expr: Box::new(left_expr),
            operator: op,
            right_expr: Box::new(right_expr),
        };

        Ok(ExpressionNode::BinaryOperationNode(node))
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
                    // let the binary expression parsing handle it
                    self.idx -= 1;
                    current_expression = self.parse_binary_expression(current_expression)?; 
                }


                // part of the expression
                _ => {return Err(format!("Unexpected token \"{}\" while parsing expression (forgot a semicolon?)", next_token.value))},
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
