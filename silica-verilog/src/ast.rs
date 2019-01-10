use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
    pub top: Mod,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.top)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mod {
    pub name: String,
    pub ports: Vec<Port>,
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ports = String::new();
        let ports_len = self.ports.len();
        for (idx, port) in self.ports.iter().enumerate() {
            if idx == (ports_len - 1) {
                ports.push_str(format!("\t{} \n", port).as_str());
            } else {
                ports.push_str(format!("\t{}, \n", port).as_str());
            }
        }

        let mut stmts = String::new();
        for stmt in self.stmts.iter() {
            stmts.push_str(format!("\t{} \n", stmt).as_str());
        }

        write!(
            f,
            "module {} ( \n{}); \n{}endmodule \n",
            self.name, ports, stmts
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Port {
    pub dir: Dir,
    pub name: String,
    pub ty: Type,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.dir, self.ty, self.name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dir {
    Input,
    Output,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Input => "input",
                Dir::Output => "output",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Wire,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Wire => "wire",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign { id: String, ex: Expr },
    Declare { id: String, ty: Type },
    DeclareAssign { id: String, ty: Type, ex: Expr },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Assign { id, ex } => write!(f, "assign {} = {};", id, ex),
            Stmt::Declare { id, ty } => write!(f, "{} {};", ty, id),
            Stmt::DeclareAssign { id, ty, ex } => write!(f, "{} {} = {};", ty, id, ex),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary {
        lex: Box<Expr>,
        op: BinaryOp,
        rex: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        ex: Box<Expr>,
    },
    Paren {
        ex: Box<Expr>,
    },
    Ident {
        id: String,
    },
    Litrl {
        val: String,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Binary { lex, op, rex } => write!(f, "{} {} {}", lex, op, rex),
            Expr::Unary { op, ex } => write!(f, "{}{}", op, ex),
            Expr::Paren { ex } => write!(f, "( {} )", ex),
            Expr::Ident { id } => write!(f, "{}", id),
            Expr::Litrl { val } => write!(f, "{}", val),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    ShiftLeft,
    ShiftRight,
    BitAnd,
    BitXor,
    BitOr,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinaryOp::ShiftLeft => "<<",
                BinaryOp::ShiftRight => ">>",
                BinaryOp::BitAnd => "&",
                BinaryOp::BitXor => "^",
                BinaryOp::BitOr => "|",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Negate,
    ReductAnd,
    ReductXor,
    ReductOr,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnaryOp::Negate => "~",
                UnaryOp::ReductAnd => "&",
                UnaryOp::ReductXor => "^",
                UnaryOp::ReductOr => "|",
            }
        )
    }
}
