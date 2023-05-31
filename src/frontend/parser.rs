lalrpop_mod!(parser);

use std::process::exit;

use super::ast::Node;

pub fn parse(src: &String) -> Vec<Node> {
    let parse_res = parser::ExoforkParser::new().parse(src);
    match parse_res {
        Ok(nodes) => nodes,
        Err(e) => { println!("{}", e); exit(-1) }
    }
}