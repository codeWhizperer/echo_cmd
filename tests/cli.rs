use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]

fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello").assert().success();
}

// #[test]

// fn hello1() -> TestResult {
//     let expected = fs::read_to_string("tests/expected/hello1.txt")?;
//     let mut cmd = Command::cargo_bin("echo")?;
//     cmd.arg("Hello there").assert().success().stdout(expected);
//     Ok(())
// }
// #[test]
// fn hello2() -> TestResult {
//     let expected = fs::read_to_string("tests/expected/hello2.txt")?;
//     let mut cmd = Command::cargo_bin("echo")?;
//     cmd.args(vec!["Hello", "there"])
//         .assert()
//         .success()
//         .stdout(expected);
//     Ok(())
// }

fn run(args:&[&str], expected_file:&str) -> TestResult{
    let mut expected = fs::read_to_string(expected_file)?;
    println!("expected output: {:?}",expected);
    Command::cargo_bin("echo")?.args(args).assert().success().stdout(expected);
    Ok(())
}



#[test]
fn hello1() -> TestResult {
run(&["Hello there"], "tests/expected/hello1.txt")
}
// #[test]
// fn hello2() -> TestResult {
// run(&["Hello", "there"], "tests/expected/hello2.txt")
// }
// #[test]
// fn hello1_no_newline() -> TestResult {
// run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
// }
// #[test]
// fn hello2_no_newline() -> TestResult {
// run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
// }
// l