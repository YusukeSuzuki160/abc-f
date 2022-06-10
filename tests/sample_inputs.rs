use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"3 5
3 5 2
8 1 3
1 2 2 3
1 3 1 3
1 1 1 1
2 2 2 2
3 3 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2
1
11
6
10
");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 1
9
100
1 1 1 1
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "109
");
    assert!(output.stderr_str().is_empty());
}