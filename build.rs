
fn main() {
    lalrpop::Configuration::new()
    	.set_in_dir("src/syntax")
        .process()
        .unwrap();

    println!("cargo:rerun-if-changed=src/syntax/grammar.lalrpop");
}