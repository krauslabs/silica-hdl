extern crate syntax;

#[test]
// Tests that trailing commas (wherever they are allowed) do not affect the
// AST that is generated.
//
// Currently port lists are the only places where trailing commas are allowed.
fn trailing_comma() {

    let no_comma = 
        "top mod a (
            in a: bool,
            out b: bool
        ) {
            b = a;
        }";

    let comma = 
        "top mod a (
            in a: bool,
            out b: bool,
        ) {
            b = a;
        }";

    let ast_no_comma = syntax::Ast::new(no_comma);
    let ast_comma = syntax::Ast::new(comma);

    assert_eq!(ast_comma, ast_no_comma);
}