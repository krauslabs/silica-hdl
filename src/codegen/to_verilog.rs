use codegen::CodeGen;
use syntax::ast;

pub trait ToVerilog {
    fn to_verilog(&self, _codegen: &CodeGen) -> String;
}

impl ToVerilog for ast::Ast {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        self.top.to_verilog(&_codegen)
    }
}

impl ToVerilog for ast::Mod {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {

        let mut output = format!("module {} ( \n", self.id.to_verilog(_codegen));

        let port_len = self.ports.len();
        for (idx, port) in self.ports.iter().enumerate() {
            if idx == (port_len - 1) {
                output.push_str(&format!("\t{} \n", port.to_verilog(_codegen)));
            } else {
                output.push_str(&format!("\t{}, \n", port.to_verilog(_codegen)));
            }
        }

        output.push_str("); \n");

        for stmt in self.stmts.iter() {
            output.push_str(&format!("\t{} \n", stmt.to_verilog(_codegen)));
        }

        output.push_str("endmodule \n");

        output
    }
}

impl ToVerilog for ast::Port {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {

        let dir_str = match self.dir {
            ast::PortDir::Input => "input",
            ast::PortDir::Output => "output",
        };

        let ty_str = match self.ty {
            ast::Type::Bool => "wire",
        };

        format!("{} {} {}", dir_str, ty_str, self.id.to_verilog(_codegen))
    }
}

impl ToVerilog for ast::Stmt {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        match self.kind {
            ast::StmtKind::Assign(ref assign) => assign.to_verilog(_codegen),
        }
    }
}

impl ToVerilog for ast::AssignStmt {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        format!("assign {} = {};", self.id.to_verilog(_codegen), self.expr.to_verilog(_codegen))
    }
}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        match self.kind {
            ast::ExprKind::Binary(ref binary) => binary.to_verilog(_codegen),
            ast::ExprKind::Ident(ref ident) => ident.to_verilog(_codegen),
            ast::ExprKind::Litrl(ref litrl) => litrl.to_verilog(_codegen), 
        }
    }
}

impl ToVerilog for ast::BinaryExpr {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        let op_str = match self.op {
            ast::BinaryOp::BitAnd => "&".to_string(),
            ast::BinaryOp::BitOr => "|".to_string(),
        };

        format!("{} {} {}", self.ex1.to_verilog(_codegen), op_str, self.ex2.to_verilog(_codegen))
    }
}

impl ToVerilog for ast::Ident {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        self.val.clone()
    }
}

impl ToVerilog for ast::Litrl {
    fn to_verilog(&self, _codegen: &CodeGen) -> String {
        self.val.clone()
    }
}
