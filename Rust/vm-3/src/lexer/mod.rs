pub mod types;

pub mod lexer {

    use super::types::*;

    pub fn tokenize(lang: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = lang.chars().peekable();

        while let Some(chr) = chars.next() {
            match chr {
                ' ' | '\n' => (),
                '.' => tokens.push(Token::DOT),
                ';' => tokens.push(Token::EOL),
                '{' => tokens.push(Token::BLOCK(SeparatorState::OPEN)),
                '}' => tokens.push(Token::BLOCK(SeparatorState::CLOSE)),
                '"' => tokens.push(Token::QUOTE),
                '=' => tokens.push(Token::OPERATOR(Operator::EQUALS)),
                '+' => tokens.push(Token::OPERATOR(Operator::PLUS)),
                '-' => tokens.push(Token::OPERATOR(Operator::MINUS)),
                '/' => tokens.push(Token::OPERATOR(Operator::SLASH)),
                '*' => tokens.push(Token::OPERATOR(Operator::ASTERISK)),
                '&' => tokens.push(Token::OPERATOR(Operator::AMPERSAND)),
                '(' => tokens.push(Token::PAREN(SeparatorState::OPEN)),
                ')' => tokens.push(Token::PAREN(SeparatorState::CLOSE)),
                ',' => tokens.push(Token::COMMA),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    tokens.push(build_num(chr, &mut chars))
                }
                _ => {
                    let tok_str = build_identifier_string(chr, &mut chars);
                    tokens.push(match_type(&tok_str))
                }
            }
        }
        tokens
    }

    fn match_type(s: &str) -> Token {
        // if s.chars().nth(0).unwrap() == '*' {
        //     let remaining_tok = s.chars().enumerate().filter(|&(i, _)| i == 1).map(|(_, c)| c).collect::<String>().as_str();
        //     if let Token::TYPE(t) = match_type(remaining_tok) {
        //         Token::TYPE(Type::POINTER(Box::new(t)))
        //     }
        // } else {
        match s {
            "void" => Token::TYPE(Type::VOID),
            "int" => Token::TYPE(Type::INT),
            "char" => Token::TYPE(Type::CHAR),
            "free" => Token::BULTIN(Builtin::FREE),
            "return" => Token::BULTIN(Builtin::RETURN),
            "printf" => Token::BULTIN(Builtin::PRINTF),
            _ => Token::IDENTIFIER(s.to_string()),
        }
        // }
    }

    fn is_identifier_char(chr: &char) -> bool {
        ![
            ' ', '\n', ';', '{', '}', '=', '+', '-', '/', '*', '&', '(', ')', '"', ',', '.',
        ]
        .contains(&chr)
    }

    fn is_num(chr: &char) -> bool {
        match chr {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
            _ => false,
        }
    }

    fn build_identifier_string(
        chr: char,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> String {
        let mut id: String = chr.to_string();
        while let Some(ch) = chars.peek() {
            if is_identifier_char(&ch) {
                id.push(chars.next().unwrap())
            } else {
                return id;
            }
        }
        id
    }

    fn build_num(chr: char, chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
        let mut id: String = chr.to_string();
        while let Some(ch) = chars.peek() {
            if is_num(&ch) {
                id.push(chars.next().unwrap())
            } else {
                return Token::VALUE(str::parse::<i32>(&id).unwrap());
            }
        }
        Token::VALUE(str::parse::<i32>(&id).unwrap())
    }
}
