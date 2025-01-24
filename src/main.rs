// use std::{path::PathBuf, thread::panicking};

use clap::Parser;
use ruff_python_ast::Expr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    code: Option<String>,
}


fn get_expr_name(expr: &Expr) -> String {
    let name = match expr {
        ruff_python_ast::Expr::BoolOp(_) => "bool_op",
        ruff_python_ast::Expr::Named(_) => "named",
        ruff_python_ast::Expr::BinOp(_) => "bin_op",
        ruff_python_ast::Expr::UnaryOp(_) => "unary_op",
        ruff_python_ast::Expr::Lambda(_) => "lambda",
        ruff_python_ast::Expr::If(_) => "if",
        ruff_python_ast::Expr::Dict(_) => "dict",
        ruff_python_ast::Expr::Set(_) => "set",
        ruff_python_ast::Expr::ListComp(_) => "list_comp",
        ruff_python_ast::Expr::SetComp(_) => "set_comp",
        ruff_python_ast::Expr::DictComp(_) => "dict_comp",
        ruff_python_ast::Expr::Generator(_) => "generator",
        ruff_python_ast::Expr::Await(_) => "await",
        ruff_python_ast::Expr::Yield(_) => "yield",
        ruff_python_ast::Expr::YieldFrom(_) => "yield_from",
        ruff_python_ast::Expr::Compare(_) => "compare",
        ruff_python_ast::Expr::Call(_) => "call",
        ruff_python_ast::Expr::FString(_) => "f_string",
        ruff_python_ast::Expr::StringLiteral(_) => "string_literal",
        ruff_python_ast::Expr::BytesLiteral(_) => "bytes_literal",
        ruff_python_ast::Expr::NumberLiteral(_) => "number_literal",
        ruff_python_ast::Expr::BooleanLiteral(_) => "boolean_literal",
        ruff_python_ast::Expr::NoneLiteral(_) => "none_literal",
        ruff_python_ast::Expr::EllipsisLiteral(_) => "ellipsis_literal",
        ruff_python_ast::Expr::Attribute(_) => "attribute",
        ruff_python_ast::Expr::Subscript(_) => "subscript",
        ruff_python_ast::Expr::Starred(_) => "starred",
        ruff_python_ast::Expr::Name(_) => "name",
        ruff_python_ast::Expr::List(_) => "list",
        ruff_python_ast::Expr::Tuple(_) => "tuple",
        ruff_python_ast::Expr::Slice(_) => "slice",
        ruff_python_ast::Expr::IpyEscapeCommand(_) => "ipy_escape_command",
    };
    name.to_string()
}


fn main() {
    let cli = Cli::parse();
    if let Some(code) = cli.code {
        let parser = ruff_python_parser::parse_expression(&code);
        if parser.is_ok() {
            let expr_name = get_expr_name(parser.unwrap().as_result().unwrap().expr());
            println!("{}", expr_name)
        }
        else {
            println!("error")
        }
    } 
}
