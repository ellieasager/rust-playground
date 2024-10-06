use crate::utils::{record::Record, test_runner::run_test};

#[test]
fn test_run() {
    let input_file_name = "test1";
    let expected_results = vec![
        Record::new(1, 3.0, 0.0, 3.0, true),
        Record::new(2, 2.0, 0.0, 2.0, false),
        Record::new(3, 4.5, 0.0, 4.5, false),
    ];

    run_test(input_file_name, expected_results);
}
