use crate::lexer::Token;

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

struct ScopeNode {
    commands: Vec<CommandNode>,
}

struct VariableNode {
    name: String,
}

struct AssignmentNode {
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
    AssignmentNode(AssignmentNode),
    FunctionCallNode(FunctionCallNode),
}

enum ExpressionNode {
    BinaryOperationNode(BinaryOperationNode),
    UnaryOperationNode(UnaryOperationNode),
    IntLiteralNode(IntLiteralNode),
    FloatLiteralNode(FloatLiteralNode),
    StringLiteralNode(StringLiteralNode),
    CharLiteralNode(CharLiteralNode),
    FunctionCallNode(FunctionCallNode),
}

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, idx: 0 }
    }

    fn get(&self, relative_idx: usize) -> Option<&Token> {
        self.tokens.get(self.idx + relative_idx)
    }

    fn update_idx(&mut self, relative_idx: usize) {
        self.idx += relative_idx;
    }

    fn parse_command(&mut self) -> Result<Node, String> {
        // it's ok to unwrap since this function will only get called when there are tokens left
        let first = self.get(0).unwrap();

    }

    fn parse(&mut self) -> Result<Node, String> {
        let scope_node = Node::ScopeNode { commands: vec!() }; 
        while self.idx != self.tokens.len() {
            scope_node.commands.push(self.parse_command()?);
        } 
        Ok(scope_node)
    }
}

