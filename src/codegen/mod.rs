mod to_verilog;

use super::syntax;
use self::to_verilog::ToVerilog;

#[derive(Clone, Debug)]
pub struct CodeGen {
    ast: Box<syntax::ast::Mod>,
}

impl CodeGen {
    pub fn new(ast: Box<syntax::ast::Mod>) -> CodeGen {
        CodeGen {ast: ast }
    }

    pub fn generate(&self) -> String {
        self.ast.to_verilog(&self)
    }
}