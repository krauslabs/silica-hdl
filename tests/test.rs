mod utils;

use crate::utils::*;

#[test]
fn simple_mod() {
    assert_compiled_output_eq(
        "./tests/pass-output/simple_mod.si",
        "./tests/pass-output/simple_mod.v",
    );
}

#[test]
fn simple_fail() {
    assert_compiler_passed("./tests/fail/simple_fail.si");
}
