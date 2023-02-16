
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

impl Operator {
    pub fn from(op: &str) -> Operator {
        match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "%" => Operator::Modulo,
            _ => panic!("Internal compiler error (unknown operator)")
        }
    }

    pub fn to_str(&self) -> String {
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

    pub fn priority_score(&self) -> u32 {
        match self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Multiply | Operator::Divide | Operator::Modulo => 2,
        }
    }
}


#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Char,
    Str
}

impl Type {
    pub fn from(typename: &str) -> Type {
        match typename {
            "int" => Type::Int,
            "float" => Type::Float,
            "bool" => Type::Bool,
            "char" => Type::Char,
            "str" => Type::Str,
            _ => panic!("Interal compiler error (invalid type)")
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Type::Int => "int",
            Type::Float => "float",
            Type::Bool => "bool",
            Type::Char => "char",
            Type::Str => "str",
        }
    }
}


pub struct ScopeNode {
    pub commands: Vec<CommandNode>,
}

pub struct VariableNode {
    pub name: String,
}

pub struct VariableDefinitionNode {
    pub vtype: Option<String>, // temporary
    pub variable: VariableNode,
    pub expression: Option<Box<TExpressionNode>>,
}

pub struct VariableAssignmentNode {
    pub variable: VariableNode,
    pub expression: Box<TExpressionNode>,
}

pub struct BinaryOperationNode {
    pub left_expr: Box<TExpressionNode>,
    pub operator: Operator,
    pub right_expr: Box<TExpressionNode>,
}

pub struct UnaryOperationNode {
    pub operator: Operator,
    pub expression: Box<TExpressionNode>,
}

pub struct IntLiteralNode {
    pub value: i64,
}

pub struct FloatLiteralNode {
    pub value: f64,
}

pub struct BoolLiteralNode {
    pub value: bool,
}

pub struct StringLiteralNode {
    pub value: String,
}

pub struct CharLiteralNode {
    pub value: char,
}

pub struct FunctionCallNode {
    pub function: FunctionNode,
    pub args: Vec<TExpressionNode>,
}

pub struct FunctionNode {
    pub name: String,
}

pub enum CommandNode {
    VariableDefinitionNode(VariableDefinitionNode),
    VariableAssignmentNode(VariableAssignmentNode),
    FunctionCallNode(FunctionCallNode),
}

pub struct TExpressionNode {
    pub node: ExpressionNode,
    pub t: Option<Type>,
}

pub enum ExpressionNode {
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

pub fn get_tab_str(tab_lvl: usize) -> String {
    String::from("\t").repeat(tab_lvl) 
}

impl TExpressionNode {
    pub fn debug_str(&self, tab_lvl: usize) -> String {
        match &self.node {
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
    pub fn debug_str(&self, tab_lvl: usize) -> String {
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

