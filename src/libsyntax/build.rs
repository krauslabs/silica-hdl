
fn main() {
    lalrpop::Configuration::new()
    	.set_in_dir("src")
        .process()
        .unwrap();

    println!("cargo:rerun-if-changed=src/grammar.lalrpop");
}