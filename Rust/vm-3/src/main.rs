mod lexer;
mod parser;

use crate::lexer::lexer::*;
use crate::lexer::types::*;
use crate::parser::parser::*;

pub enum AST {}

pub struct Expr<T> {
    val: T,
}

pub struct Binop {
    op: Operator,
    lhs: Expr<i32>,
    rhs: Expr<i32>,
}

fn split_by_eol(tok: &[Token]) -> Vec<Vec<&Token>> {
    let mut v = Vec::new();
    let mut vv = Vec::new();
    for item in tok {
        if is_eol(item) {
            v.push(vv);
            vv = Vec::new();
        } else {
            vv.push(item);
        }
    }
    v.push(vv);
    v
}

fn main() {
    let lang = std::fs::read_to_string("./lang.c").unwrap();
    let toks = &tokenize(lang);
    let funcs = parse_to_func(&toks);
    let body = funcs[0].body;
    let tokens = split_by_eol(&body);

    for i in 0..tokens.len() {
        println!("{:?}", tokens[i]);
        println!("-----------")
        if is_type(&tokens[i][i]) && is_identifier(&tokens[i + 1][i + 1]) && is_eq(&tokens[i + 2][i + 2]) {
            let mut j = i;
            while !is_eol(&tokens[i][j]) {
                j += 1;
            }
            println!("{:?}", &tokens[i + 3..j]);
        }
    }

    // println!("{:#?}", tokens);
}
