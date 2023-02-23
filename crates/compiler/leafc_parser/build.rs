use cargo_emit::rerun_if_changed;

fn main() {
    // lalrpop();
}

/// The **LALRPOP** grammar file.
const LALRPOP_GRAMMAR: &str = "src/grammar/lalrpop/leaf.lalrpop";

fn lalrpop() {
    // Only regenerate the correctness parser if the grammar has changed
    rerun_if_changed!(LALRPOP_GRAMMAR);
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_report(true)
        .process_file(LALRPOP_GRAMMAR)
        .unwrap(); // TODO: see if we can handle this error in build script
                   // better
}
