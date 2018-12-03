extern crate compiletest_rs;

use std::env::set_var;
use std::path::PathBuf;
use compiletest_rs::{common, run_tests, Config};

static LOCAL_DRIVER_PATH: &'static str = "target/debug/mir-dump-driver";

fn get_driver_path() -> PathBuf {
    if PathBuf::from(LOCAL_DRIVER_PATH).exists() {
        return PathBuf::from(LOCAL_DRIVER_PATH);
    }
    unreachable!();
}

fn run_verification(group_name: &str) {
    set_var("MIR_DUMP_FULL_COMPILATION", "true");

    // This flag informs the driver that we are running the test suite, so that some additional
    // checks are enabled. For example, comparison of the computed definitely initialized
    // information with the expected one.
    set_var("MIR_DUMP_TEST", "true");

    let mut config = Config::default();
    config.rustc_path = get_driver_path();
    config.link_deps();

    let path = PathBuf::from(format!("tests/{}/ui", group_name));
    if path.exists() {
        config.mode = common::Mode::Ui;
        config.src_base = path;
        run_tests(&config);
    }

    let path = PathBuf::from(format!("tests/{}/pass", group_name));
    if path.exists() {
        config.mode = common::Mode::RunPass;
        config.src_base = path;
        run_tests(&config);
    }

    let path = PathBuf::from(format!("tests/{}/fail", group_name));
    if path.exists() {
        config.mode = common::Mode::CompileFail;
        config.src_base = path;
        run_tests(&config);
    }
}

#[test]
fn typecheck_test() {
    run_verification("verify");
}
