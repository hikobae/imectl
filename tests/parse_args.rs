use imectl::parse_args;

#[test]
fn parse_args_defaults_to_false_when_no_argument() {
    let args = vec![String::from("imectl")];
    assert_eq!(parse_args(&args), Ok(false));
}

#[test]
fn parse_args_returns_true_for_nonzero_argument() {
    let args = vec![String::from("imectl"), String::from("1")];
    assert_eq!(parse_args(&args), Ok(true));
}

#[test]
fn parse_args_returns_false_for_zero_argument() {
    let args = vec![String::from("imectl"), String::from("0")];
    assert_eq!(parse_args(&args), Ok(false));
}

#[test]
fn parse_args_returns_error_for_invalid_argument() {
    let args = vec![String::from("imectl"), String::from("invalid")];
    assert!(parse_args(&args).is_err());
}

#[test]
fn parse_args_too_many_arguments() {
    let args = vec![
        String::from("imectl"),
        String::from("1"),
        String::from("extra"),
    ];
    assert!(parse_args(&args).is_err());
}
