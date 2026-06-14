use imectl::parse_args;

#[test]
fn parse_args_defaults_to_false_when_no_argument() {
    let args = vec![String::from("imectl")];
    assert_eq!(parse_args(&args), false);
}

#[test]
fn parse_args_returns_true_for_nonzero_argument() {
    let args = vec![String::from("imectl"), String::from("1")];
    assert_eq!(parse_args(&args), true);
}

#[test]
fn parse_args_returns_false_for_zero_argument() {
    let args = vec![String::from("imectl"), String::from("0")];
    assert_eq!(parse_args(&args), false);
}

#[test]
fn parse_args_returns_false_for_invalid_argument() {
    let args = vec![String::from("imectl"), String::from("invalid")];
    assert_eq!(parse_args(&args), false);
}
