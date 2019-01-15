use syntax::{ast::*, parse_source};

fn assert_expr(source: &str, expected: &Expr) {
    let module = format!("top mod a ( out y: bit ) {{ y = {}; }}", source);

    let ast = parse_source(&module).unwrap();
    let expr = match ast.top.stmts[0] {
        Stmt::Assign { id: _, ref ex } => ex,
        _ => panic!(),
    };

    assert_eq!(expr, expected);
}

fn assert_stmt(source: &str, expected: &Stmt) {
    let module = format!("top mod a ( out y: bit ) {{ {} }}", source);

    let ast = parse_source(&module).unwrap();
    let stmt = &ast.top.stmts[0];

    assert_eq!(stmt, expected);
}

#[test]
fn assign_stmt() {
    assert_stmt(
        "y = 5;",
        &Stmt::Assign {
            id: "y".to_string(),
            ex: Expr::Litrl {
                val: "5".to_string(),
            },
        },
    );
}

#[test]
fn declare_stmt() {
    assert_stmt(
        "let y: bit;",
        &Stmt::Declare {
            id: "y".to_string(),
            ty: Type::Bit,
        },
    );
}

#[test]
fn declare_assign_stmt() {
    assert_stmt(
        "let y: bit = 1;",
        &Stmt::DeclareAssign {
            id: "y".to_string(),
            ty: Type::Bit,
            ex: Expr::Litrl {
                val: "1".to_string(),
            },
        },
    );
}

#[test]
fn vector_declare_stmt() {
    assert_stmt(
        "let y: bit[3:0] = 1;",
        &Stmt::DeclareAssign {
            id: "y".to_string(),
            ty: Type::BitVec { high: 3, low: 0 },
            ex: Expr::Litrl {
                val: "1".to_string(),
            },
        },
    );
}

#[test]
fn precedence() {
    assert_expr(
        "1 | 2 ^ 3 & & 4 << 5",
        &Expr::Binary {
            lex: Box::new(Expr::Litrl {
                val: "1".to_string(),
            }),
            op: BinaryOp::BitOr,
            rex: Box::new(Expr::Binary {
                lex: Box::new(Expr::Litrl {
                    val: "2".to_string(),
                }),
                op: BinaryOp::BitXor,
                rex: Box::new(Expr::Binary {
                    lex: Box::new(Expr::Litrl {
                        val: "3".to_string(),
                    }),
                    op: BinaryOp::BitAnd,
                    rex: Box::new(Expr::Binary {
                        lex: Box::new(Expr::Unary {
                            op: UnaryOp::ReductAnd,
                            ex: Box::new(Expr::Litrl {
                                val: "4".to_string(),
                            }),
                        }),
                        op: BinaryOp::ShiftLeft,
                        rex: Box::new(Expr::Litrl {
                            val: "5".to_string(),
                        }),
                    }),
                }),
            }),
        },
    );
}

#[test]
fn paren_grouping() {
    assert_expr(
        "1 << ( 2 | 3 ) >> 1",
        &Expr::Binary {
            lex: Box::new(Expr::Binary {
                lex: Box::new(Expr::Litrl {
                    val: "1".to_string(),
                }),
                op: BinaryOp::ShiftLeft,
                rex: Box::new(Expr::Paren {
                    ex: Box::new(Expr::Binary {
                        lex: Box::new(Expr::Litrl {
                            val: "2".to_string(),
                        }),
                        op: BinaryOp::BitOr,
                        rex: Box::new(Expr::Litrl {
                            val: "3".to_string(),
                        }),
                    }),
                }),
            }),
            op: BinaryOp::ShiftRight,
            rex: Box::new(Expr::Litrl {
                val: "1".to_string(),
            }),
        },
    );
}

#[test]
fn trailing_comma() {
    let no_comma = "top mod a (
            in x: bit,
            out y: bit
        ) {
            y = x;
        }";

    let comma = "top mod a (
            in x: bit,
            out y: bit,
        ) {
            y = x;
        }";

    let ast_no_comma = parse_source(no_comma).unwrap();
    let ast_comma = parse_source(comma).unwrap();

    assert_eq!(ast_comma, ast_no_comma);
}

#[test]
fn module() {
    let source = "top mod a (
            out y: bit
        ) {
            y = 1;
        }";

    let ast = parse_source(source).unwrap();

    let expected = Ast {
        top: Mod {
            name: "a".to_string(),
            ports: vec![Port {
                dir: Dir::Output,
                name: "y".to_string(),
                ty: Type::Bit,
            }],
            stmts: vec![Stmt::Assign {
                id: "y".to_string(),
                ex: Expr::Litrl {
                    val: "1".to_string(),
                },
            }],
        },
    };

    assert_eq!(ast, expected);
}
