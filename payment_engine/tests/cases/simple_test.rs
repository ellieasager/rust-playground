use crate::utils::{record::Record, test_runner::run_test};

#[test]
fn test_run() {
    let input_file_name = "simple_test";
    let expected_results = vec![
        Record::new(1, 1.5, 0.0, 1.5, false),
        Record::new(2, 2.0, 0.0, 2.0, false),
    ];

    run_test(input_file_name, expected_results);
}
