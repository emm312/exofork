lalrpop_mod!(parser);

use std::process::exit;

use super::ast::Toplvl;

pub fn parse(src: &String) -> Vec<Toplvl> {
    let parse_res = parser::ExoforkParser::new().parse(src);
    match parse_res {
        Ok(nodes) => nodes,
        Err(e) => {
            println!("{}", e);
            exit(-1)
        }
    }
}
