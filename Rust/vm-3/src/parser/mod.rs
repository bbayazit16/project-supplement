pub mod AST;
pub mod function;

pub mod parser {

    use super::super::lexer::types::*;
    use super::function::Function;

    pub fn parse_to_func(tokens: &[Token]) -> Vec<Function> {
        let mut func_vec = vec![];
        for i in 0..tokens.len() {
            if (is_type(&tokens[i])
                && is_identifier(&tokens[i + 1])
                && is_paren_open(&tokens[i + 2]))
                || (is_type(&tokens[i])
                    && is_operator(&tokens[i + 1])
                    && is_identifier(&tokens[i + 2]))
                    && is_paren_open(&tokens[i + 3])
            {
                let mut j = i;
                while !is_paren_close(&tokens[j]) {
                    j += 1;
                }
                let header = &tokens[i..j + 1];

                let mut s = 1;
                let mut k = j;
                while !is_block_close(&tokens[k]) && s != 0 {
                    if is_block_open(&tokens[k]) {
                        s += 1;
                    } else if is_block_close(&tokens[k]) {
                        s -= 1;
                    }
                    k += 1;
                }

                let body = &tokens[j + 2..k];

                func_vec.push(Function::new(header, body))
            }
        }
        func_vec
    }

    pub fn is_type(tok: &Token) -> bool {
        match tok {
            Token::TYPE(_) => true,
            _ => false,
        }
    }

    fn is_paren_open(tok: &Token) -> bool {
        match tok {
            Token::PAREN(SeparatorState::OPEN) => true,
            _ => false,
        }
    }

    pub fn is_paren_close(tok: &Token) -> bool {
        match tok {
            Token::PAREN(SeparatorState::CLOSE) => true,
            _ => false,
        }
    }

    fn is_block_open(tok: &Token) -> bool {
        match tok {
            Token::BLOCK(SeparatorState::CLOSE) => true,
            _ => false,
        }
    }

    fn is_block_close(tok: &Token) -> bool {
        match tok {
            Token::BLOCK(SeparatorState::CLOSE) => true,
            _ => false,
        }
    }

    pub fn is_identifier(tok: &Token) -> bool {
        match tok {
            Token::IDENTIFIER(_) => true,
            _ => false,
        }
    }

    pub fn is_eq(tok: &Token) -> bool {
        match tok {
            Token::OPERATOR(Operator::EQUALS) => true,
            _ => false,
        }
    }

    pub fn is_eol(tok: &Token) -> bool {
        match tok {
            Token::EOL => true,
            _ => false,
        }
    }

    fn is_operator(tok: &Token) -> bool {
        match tok {
            Token::OPERATOR(_) => true,
            _ => false,
        }
    }

    pub fn is_asterisk(tok: &Token) -> bool {
        match tok {
            Token::OPERATOR(Operator::ASTERISK) => true,
            _ => false,
        }
    }

    pub fn is_comma(tok: &Token) -> bool {
        match tok {
            Token::COMMA => true,
            _ => false,
        }
    }
}
