use crate::utils::{record::Record, test_runner::run_test};

#[test]
fn test_run() {
    let input_file_name = "test2";
    let expected_results = vec![Record::new(1, 5.0, 0.0, 5.0, false)];

    run_test(input_file_name, expected_results);
}
