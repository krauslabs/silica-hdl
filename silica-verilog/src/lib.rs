mod ast;
mod transform;

pub struct Verilog {
    ast: ast::Ast,
}

impl Verilog {
    pub fn new(ast: &syntax::ast::Ast) -> Verilog {
        Verilog {
            ast: transform::transform_ast(ast),
        }
    }

    pub fn build(&mut self) -> String {
        format!("{}", self.ast)
    }
}
