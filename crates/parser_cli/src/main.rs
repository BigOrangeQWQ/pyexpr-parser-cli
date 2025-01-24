use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    let res = parser_expr::parser_expr(&args.join(""));
    
    println!("{}", res)
}
