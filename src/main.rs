use std::process::exit;

use carbonasmpp::{lowerer, grammar};

fn main() {
    let ast = grammar::CarbonPPProgramParser::new().parse(&std::fs::read_to_string("./test.carbon++").unwrap()).unwrap_or_else(|e| { println!("{}", e); exit(-1); });
    let label_map = ast.generate_label_map();
    lowerer::lower(ast, label_map)
}
