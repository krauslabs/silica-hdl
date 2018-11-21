//! The Verilog Code Generator
//!
//! The verilog code generator creates the verilog source that is the output
//! of the Silica compiler. It uses data created from previous parts of the
//! compilation process (such as the AST and name resolution tables) to 
//! generate suitable output code.

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