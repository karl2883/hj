enum Node {
    // list of commands to execute in scope (assignment node/function call node)
    ScopeNode(Vec<Node>),
    // name of variable
    VariableNode(String),
    // variable node, expression node
    AssignmentNode(Box<Node>, Box<Node>),
    // literal nodes, operator nodes
    ExpressionNode(Vec<Node>),
    // value
    LiteralNode(String),
    // operator
    OperatorNode(String),
    // function node, list of expression nodes (parameters)
    FunctionCallNode(Box<Node>, Vec<Node>),
    // function name
    FunctionNode(String),
}
