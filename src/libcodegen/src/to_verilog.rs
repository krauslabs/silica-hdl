use syntax::ast;

pub trait ToVerilog {
    fn to_verilog(&self) -> String;
}

impl ToVerilog for ast::Ast {
    fn to_verilog(&self) -> String {
        let ast::Ast(module) = self;
        module.to_verilog()
    }
}

impl ToVerilog for ast::Mod {
    fn to_verilog(&self) -> String {

        let ast::Mod(id, ports, stmts) = self;

        let mut output = format!("module {} ( \n", id.to_verilog());

        let port_len = ports.len();
        for (idx, port) in ports.iter().enumerate() {
            if idx == (port_len - 1) {
                output.push_str(&format!("\t{} \n", port.to_verilog()));
            } else {
                output.push_str(&format!("\t{}, \n", port.to_verilog()));
            }
        }

        output.push_str("); \n");

        for stmt in stmts.iter() {
            output.push_str(&format!("\t{} \n", stmt.to_verilog()));
        }

        output.push_str("endmodule \n");

        output
    }
}

impl ToVerilog for ast::Port {
    fn to_verilog(&self) -> String {
        let ast::Port(dir, id, ty) = self;
        format!("{} {} {}",
            dir.to_verilog(),
            ty.to_verilog(),
            id.to_verilog())
    }
}

impl ToVerilog for ast::Dir {
    fn to_verilog(&self) -> String {
        match self {
            ast::Dir::Input => "input".to_string(),
            ast::Dir::Output => "output".to_string(),
        }
    }
}

impl ToVerilog for ast::Type {
    fn to_verilog(&self) -> String {
        match self {
            ast::Type::Bool => "wire".to_string(),
        }
    }
}

impl ToVerilog for ast::Stmt {
    fn to_verilog(&self) -> String {
        match self {
            ast::Stmt::Assign(ref id, ref ex) => {
                format!("assign {} = {};",
                    id.to_verilog(),
                    ex.to_verilog())
            }
        }
    }
}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self) -> String {
        match self {
            ast::Expr::Binary(ref ex1, ref op, ref ex2) => {
                format!("{} {} {}",
                    ex1.to_verilog(),
                    op.to_verilog(),
                    ex2.to_verilog())
            }
            ast::Expr::Ident(ref ident) => {
                ident.to_verilog()
            }
            ast::Expr::Litrl(ref litrl) => {
                litrl.to_verilog()
            }
        }
    }
}

impl ToVerilog for ast::BinaryOp {
    fn to_verilog(&self) -> String {
        match self {
            ast::BinaryOp::BitAnd => "&".to_string(),
            ast::BinaryOp::BitOr => "|".to_string(),
        }
    }
}

impl ToVerilog for ast::Ident {
    fn to_verilog(&self) -> String {
        let ast::Ident(s) = self;
        s.to_string()
    }
}

impl ToVerilog for ast::Litrl {
    fn to_verilog(&self) -> String {
        let ast::Litrl(s) = self;
        s.to_string()
    }
}
