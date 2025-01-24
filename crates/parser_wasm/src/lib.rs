use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parser_expr(code: &str) -> String {
    parser_expr::parser_expr(code)
}