use super::super::lexer::types::*;
use super::parser::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Function<'a> {
    return_type: Type,
    name: String,
    params: Option<Vec<TypedIdentifier>>,
    pub body: &'a [Token],
}

impl<'a> Function<'a> {
    pub fn new(header: &[Token], body: &'a [Token]) -> Self {
        let mut start_idx = 1;
        let return_type: Type;
        if let Token::TYPE(t) = &header[0] {
            if is_asterisk(&header[1]) {
                return_type = Type::POINTER(Box::new(t.clone()));
                start_idx = 2;
            } else {
                return_type = t.clone();
            }
        } else {
            panic!("Expected return type");
        }

        let name: String;
        if let Token::IDENTIFIER(s) = &header[start_idx] {
            name = s.to_string();
        } else {
            panic!("Expected function identifier");
        }

        let mut i = start_idx;
        while !is_paren_close(&header[i]) {
            i += 1;
        }

        let params = Self::build_types(&header[start_idx + 2..i]);

        Self {
            return_type,
            name,
            params,
            body,
        }
    }

    fn build_types(tok: &[Token]) -> Option<Vec<TypedIdentifier>> {
        let mut types = Vec::new();
        let all_tokens = Self::split_commas(tok);
        for tokens in all_tokens {
            if tokens.len() == 0 {
                return None;
            }
            if is_asterisk(&tokens[1]) {
                if let Token::TYPE(t) = &tokens[0] {
                    let tp = Type::POINTER(Box::new(t.clone()));
                    if let Token::IDENTIFIER(n) = &tokens[2] {
                        types.push((tp, n.to_string()))
                    }
                }
            } else {
                if let Token::TYPE(t) = &tokens[0] {
                    if let Token::IDENTIFIER(n) = &tokens[1] {
                        types.push((t.clone(), n.to_string()))
                    }
                }
            }
        }
        Some(types)
    }

    fn split_commas(tok: &[Token]) -> Vec<Vec<&Token>> {
        let mut v = Vec::new();
        let mut vv = Vec::new();
        for item in tok {
            if is_comma(item) {
                v.push(vv);
                vv = Vec::new();
            } else {
                vv.push(item);
            }
        }
        v.push(vv);
        v
    }
}
