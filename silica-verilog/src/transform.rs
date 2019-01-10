use crate::ast::*;

pub fn transform_ast(a: &syntax::ast::Ast) -> Ast {
    Ast {
        top: transform_mod(&a.top),
    }
}

fn transform_mod(m: &syntax::ast::Mod) -> Mod {
    Mod {
        name: m.name.clone(),
        ports: m.ports.iter().map(|port| transform_port(port)).collect(),
        stmts: m.stmts.iter().map(|stmt| transform_stmt(stmt)).collect(),
    }
}

fn transform_port(p: &syntax::ast::Port) -> Port {
    Port {
        dir: transform_dir(&p.dir),
        name: p.name.clone(),
        ty: transform_type(&p.ty),
    }
}

fn transform_dir(d: &syntax::ast::Dir) -> Dir {
    match d {
        syntax::ast::Dir::Input => Dir::Input,
        syntax::ast::Dir::Output => Dir::Output,
    }
}

fn transform_type(t: &syntax::ast::Type) -> Type {
    match t {
        syntax::ast::Type::Bit => Type::Wire,
    }
}

fn transform_stmt(s: &syntax::ast::Stmt) -> Stmt {
    match s {
        syntax::ast::Stmt::Assign { id, ex } => Stmt::Assign {
            id: id.clone(),
            ex: transform_expr(ex),
        },
        syntax::ast::Stmt::Declare { id, ty } => Stmt::Declare {
            id: id.clone(),
            ty: transform_type(ty),
        },
        syntax::ast::Stmt::DeclareAssign { id, ty, ex } => Stmt::DeclareAssign {
            id: id.clone(),
            ty: transform_type(ty),
            ex: transform_expr(ex),
        },
    }
}

fn transform_expr(e: &syntax::ast::Expr) -> Expr {
    match e {
        syntax::ast::Expr::Binary { lex, op, rex } => Expr::Binary {
            lex: Box::new(transform_expr(lex)),
            op: transform_binary_op(op),
            rex: Box::new(transform_expr(rex)),
        },
        syntax::ast::Expr::Unary { op, ex } => Expr::Unary {
            op: transform_unary_op(op),
            ex: Box::new(transform_expr(ex)),
        },
        syntax::ast::Expr::Paren { ex } => Expr::Paren {
            ex: Box::new(transform_expr(ex)),
        },
        syntax::ast::Expr::Ident { id } => Expr::Ident { id: id.clone() },
        syntax::ast::Expr::Litrl { val } => Expr::Litrl { val: val.clone() },
    }
}

fn transform_binary_op(b: &syntax::ast::BinaryOp) -> BinaryOp {
    match b {
        syntax::ast::BinaryOp::ShiftLeft => BinaryOp::ShiftLeft,
        syntax::ast::BinaryOp::ShiftRight => BinaryOp::ShiftRight,
        syntax::ast::BinaryOp::BitAnd => BinaryOp::BitAnd,
        syntax::ast::BinaryOp::BitXor => BinaryOp::BitXor,
        syntax::ast::BinaryOp::BitOr => BinaryOp::BitOr,
    }
}

fn transform_unary_op(u: &syntax::ast::UnaryOp) -> UnaryOp {
    match u {
        syntax::ast::UnaryOp::Negate => UnaryOp::Negate,
        syntax::ast::UnaryOp::ReductAnd => UnaryOp::ReductAnd,
        syntax::ast::UnaryOp::ReductXor => UnaryOp::ReductXor,
        syntax::ast::UnaryOp::ReductOr => UnaryOp::ReductOr,
    }
}
