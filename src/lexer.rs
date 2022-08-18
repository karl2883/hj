use std::str::Chars;

pub enum TokenType {
    Operator, // +-*/ and so on
    AssignmentOperator, // = += -= *= /= ++ -- and so on
    OpenParen, // (
    CloseParen, // )
    OpenBracket, // [
    CloseBracket, // ]
    OpenBrace, // {
    CloseBrace, // }
    Comma,
    InbuiltType, // int, uint, float, ufloat, bool, char, str
    Name, // custom names, e.g. for variables
    NumberLiteral, // all 4 number types (int/uint/float/ufloat)
    StringLiteral, // any text ("foo")
    CharLiteral, // any character ('r')
    BoolLiteral, // true/false
    Semicolon,
    Keyword // let, if, else, while, ...
}

impl TokenType {
    fn debug_str(&self) -> &str {
        match &self {
            TokenType::Operator => "Operator",
            TokenType::AssignmentOperator => "Assignment operator",
            TokenType::OpenParen => "Opening parenthesis",
            TokenType::CloseParen => "Closing parenthesis",
            TokenType::OpenBracket => "Opening bracket",
            TokenType::CloseBracket => "Closing bracket",
            TokenType::OpenBrace => "Opening brace",
            TokenType::CloseBrace => "Closing brace",
            TokenType::Comma => "Comma",
            TokenType::InbuiltType => "Inbuilt type",
            TokenType::Name => "Custom name",
            TokenType::NumberLiteral => "Number literal",
            TokenType::StringLiteral => "String literal",
            TokenType::CharLiteral => "Char literal",
            TokenType::BoolLiteral => "Bool literal",
            TokenType::Semicolon => "Semicolon",
            TokenType::Keyword => "Keyword"
        }
    }
}

const INBUILT_TYPES: [&str; 7] = ["int", "uint", "float", "ufloat", "bool", "char", "str"];
const KEYWORDS: [&str; 4] = ["let", "if", "else", "while"];
const BOOL_LITERALS: [&str; 2] = ["true", "false"];

pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    fn new(kind: TokenType, contents: String) -> Token {
        Token { kind, value: contents }
    }

    pub fn debug_str(&self) -> String {
        format!("{} (\"{}\")", self.kind.debug_str(), self.value)
    }
}

struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a String) -> Tokenizer<'a> {
        Tokenizer {
            chars: source.chars(),
        }
    }

    fn is_empty(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&self, n: usize) -> Option<char> {
        let mut cloned = self.chars.clone();
        let mut value = ' ';
        for _ in 0..n {
            value = cloned.next()?;
        }
        Some(value)
    }

    fn peek_string(&self, n: usize) -> Option<String> {
        let mut string = String::new();
        let mut cloned = self.chars.clone();
        for _ in 0..n {
            string.push(cloned.next()?);
        }
        Some(string)
    }

    fn peek_name(&self) -> String {
        let mut string = String::new();
        let mut cloned = self.chars.clone();
        loop {
            match cloned.next() {
                Some(c) => {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        string.push(c);
                    } else {
                        break;
                    }
                }
                None => { break; }
            }
        }
        string
    }

    fn peek_number(&self) -> String {
        let mut string = String::new();
        let mut cloned = self.chars.clone();
        loop {
            match cloned.next() {
                Some(c) => {
                    if c.is_ascii_digit() || c == '.' {
                        string.push(c);
                    } else {
                        break;
                    }
                }
                None => { break; }
            }
        }
        string
    }

    fn peek_until(&self, searched: char, offset: usize, or_eof: bool) -> Option<String> {
        let mut string = String::new();
        let mut cloned = self.chars.clone();
        for _ in 0..offset {
            cloned.next()?;
        }
        loop {
            match cloned.next() {
                Some(c) => {
                    if c == searched {
                        break;
                    } else {
                        string.push(c);
                    }
                }
                None => {
                    if or_eof {
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(string)
    }

    fn advance(&mut self, n: usize) -> Result<(), ()> {
        for _ in 0..n {
            match self.chars.next() {
                None => return Err(()),
                _ => ()
            };
        }
        Ok(())
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        let first_char = self.peek(1).unwrap();

        let token = match first_char {
            ' ' | '\n' | '\t' =>  {
                self.chars.next();
                return Ok(None)
            }

            // simple one-character tokens
            '(' => Token::new(TokenType::OpenParen, first_char.to_string()),
            ')' => Token::new(TokenType::CloseParen, first_char.to_string()),
            '[' => Token::new(TokenType::OpenBracket, first_char.to_string()),
            ']' => Token::new(TokenType::CloseBracket, first_char.to_string()),
            '{' => Token::new(TokenType::OpenBrace, first_char.to_string()),
            '}' => Token::new(TokenType::CloseBrace, first_char.to_string()),
            ',' => Token::new(TokenType::Comma, first_char.to_string()),
            ';' => Token::new(TokenType::Semicolon, first_char.to_string()),
            '=' => Token::new(TokenType::AssignmentOperator, first_char.to_string()),

            '+' | '-' | '*' | '/' | '%' => {
                if first_char == '/' && self.peek(2).unwrap_or(' ') == '/' {
                    let comment = self.peek_until('\n', 2, true).unwrap();
                    self.advance(comment.len()+2).unwrap();
                    return Ok(None);
                }
                if self.peek(2).unwrap_or(' ') == '=' {
                    Token::new(TokenType::AssignmentOperator, self.peek_string(2).unwrap())
                } else {
                    Token::new(TokenType::Operator, first_char.to_string())
                }  
            },

            '\'' => {
                if let Some(mut contents) = self.peek_until('\'', 1, false) {
                    contents.push('\'');
                    contents.insert(0, '\'');
                    Token::new(TokenType::CharLiteral, contents)
                } else {
                    return Err("Unexpected EOF (you have to close the \' character literal!)".to_owned()) 
                }
            }

            '"' => {
                if let Some(mut contents) = self.peek_until('"', 1, false) {
                    contents.push('"');
                    contents.insert(0, '"');
                    Token::new(TokenType::StringLiteral, contents)
                } else {
                    return Err("Unexpected EOF (you have to close the \" string literal!)".to_owned()) 
                }
            }
            
            _ => {
                if first_char.is_ascii_digit() {
                    let num = self.peek_number();
                    if num.chars().filter(|c| *c == '.').count() <= 1 && !num.starts_with('.') && !num.ends_with('.') {
                        Token::new(TokenType::NumberLiteral, self.peek_number())
                    } else {
                        return Err("Invalid number syntax!".to_owned())
                    }
                } else if first_char.is_ascii_alphabetic() {
                    let name: String = self.peek_name();
                    if KEYWORDS.contains(&name.as_str()) {
                        Token::new(TokenType::Keyword, name)
                    } else if INBUILT_TYPES.contains(&name.as_str()) {
                        Token::new(TokenType::InbuiltType, name)
                    } else if BOOL_LITERALS.contains(&name.as_str()) {
                        Token::new(TokenType::BoolLiteral, name)
                    } else {
                        Token::new(TokenType::Name, name)
                    }
                } else {
                    return Err(format!("Unexpected character '{}'!", first_char));
                }
            }

        };
        Ok(Some(token))
    }
}


pub fn create_tokens(source: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    
    let mut tokenizer = Tokenizer::new(&source);

    while !tokenizer.is_empty() {
        let token = tokenizer.next_token()?;
        if let Some(token) = token {
            tokenizer.advance(token.value.len()).unwrap();
            tokens.push(token);
        } 
    }

    Ok(tokens)
}
