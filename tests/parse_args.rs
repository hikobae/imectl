use imectl::parse_args;

macro_rules! args {
    ($($s:expr),* $(,)?) => {
        &[$(String::from($s)),*][..]
    };
}

#[test]
fn parse_args_defaults_to_false_when_no_argument() {
    assert_eq!(parse_args(args!["imectl"]), Ok(false));
}

#[test]
fn parse_args_returns_true_for_nonzero_argument() {
    assert_eq!(parse_args(args!["imectl", "1"]), Ok(true));
}

#[test]
fn parse_args_returns_false_for_zero_argument() {
    assert_eq!(parse_args(args!["imectl", "0"]), Ok(false));
}

#[test]
fn parse_args_returns_error_for_invalid_argument() {
    assert!(parse_args(args!["imectl", "invalid"]).is_err());
}

#[test]
fn parse_args_too_many_arguments() {
    assert!(parse_args(args!["imectl", "1", "extra"]).is_err());
}
