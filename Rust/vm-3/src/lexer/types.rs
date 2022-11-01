#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    INT,
    VOID,
    CHAR,
    POINTER(Box<Type>),
}

#[derive(Debug, PartialEq)]
pub enum SeparatorState {
    OPEN,
    CLOSE,
}

#[derive(Debug, PartialEq)]
pub enum Builtin {
    PRINTF,
    FREE,
    RETURN,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    SLASH,
    EQUALS,
    ASTERISK,
    AMPERSAND,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    EOL,
    DOT,
    QUOTE,
    COMMA,
    VALUE(i32),
    TYPE(Type),
    BULTIN(Builtin),
    OPERATOR(Operator),
    IDENTIFIER(String),
    BLOCK(SeparatorState),
    PAREN(SeparatorState),
}

pub type TypedIdentifier = (Type, String);
