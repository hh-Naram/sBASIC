#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    Input,
    Let,
    Rem,
    If,
    Then,
    Goto,
    End,

    Screen,
    Clear,
    Line,
    To,
    Dot,
    Circle,

    Comment(String),
    Variable(String),
    Number(i32),
    Text(String),

    Plus,
    Minus,
    Divide,
    Multiply,

    Equals,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,

    Lparen,
    Rparen,
    Comma,

    Bang,
    UnaryMinus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

impl Token {
    pub fn to_token(symbol: &str) -> Option<Token> {
        match symbol {
            "PRINT" => Some(Token::Print),
            "INPUT" => Some(Token::Input),
            "LET" => Some(Token::Let),
            "REM" => Some(Token::Rem),
            "IF" => Some(Token::If),
            "THEN" => Some(Token::Then),
            "GOTO" => Some(Token::Goto),
            "END" => Some(Token::End),

            "SCREEN" => Some(Token::Screen),
            "CLEAR" => Some(Token::Clear),
            "LINE" => Some(Token::Line),
            "To" => Some(Token::To),
            "DOT" => Some(Token::Dot),
            "CIRCLE" => Some(Token::Circle),

            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "/" => Some(Token::Divide),
            "*" => Some(Token::Multiply),

            "=" => Some(Token::Equals),
            "<>" => Some(Token::NotEqual),
            "<" => Some(Token::LessThan),
            ">" => Some(Token::GreaterThan),
            "<=" => Some(Token::LessThanEqual),
            ">=" => Some(Token::GreaterThanEqual),

            "(" => Some(Token::Lparen),
            ")" => Some(Token::Rparen),
            "," => Some(Token::Comma),
            "!" => Some(Token::Bang),

            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        match *self {
            Token::Equals
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual
            | Token::Plus
            | Token::Minus
            | Token::Divide
            | Token::Multiply
            | Token::Bang
            | Token::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_comparison_operator(&self) -> bool {
        match *self {
            Token::Equals
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match *self {
            Token::Bang | Token::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_binary_operator(&self) -> bool {
        self.is_operator() && !self.is_unary_operator()
    }

    pub fn is_value(&self) -> bool {
        match *self {
            Token::Variable(_) | Token::Number(_) | Token::Text(_) => true,
            _ => false,
        }
    }

    pub fn operator_precedence(&self) -> Result<u8, String> {
        if !self.is_operator() {
            return Err("ERR: Not an operator!".to_string());
        }

        match *self {
            Token::UnaryMinus | Token::Bang => Ok(1 << 5),
            Token::Multiply | Token::Divide => Ok(1 << 4),
            Token::Minus | Token::Plus => Ok(1 << 3),
            _ => Ok(1 << 2),
        }
    }

    pub fn operator_associativity(&self) -> Result<Associativity, String> {
        match *self {
            Token::UnaryMinus | Token::Bang => Ok(Associativity::Right),
            _ => Ok(Associativity::Left),
        }
    }
}
