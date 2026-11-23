use imectl::parse_args;

macro_rules! args {
    ($($s:expr),* $(,)?) => {
        &[$(String::from($s)),*][..]
    };
}

#[test]
fn defaults_to_false_when_no_argument() {
    assert_eq!(parse_args(args!["imectl"]), Ok(false));
}

#[test]
fn empty_args_is_false() {
    assert_eq!(parse_args(&[]), Ok(false));
}

#[test]
fn one_is_true() {
    assert_eq!(parse_args(args!["imectl", "1"]), Ok(true));
}

#[test]
fn zero_is_false() {
    assert_eq!(parse_args(args!["imectl", "0"]), Ok(false));
}

#[test]
fn invalid_is_error() {
    assert!(parse_args(args!["imectl", "invalid"]).is_err());
    assert!(parse_args(args!["imectl", ""]).is_err());
    assert!(parse_args(args!["imectl", "99999999999999999999"]).is_err());
}

#[test]
fn extra_args_is_error() {
    assert!(parse_args(args!["imectl", "1", "0"]).is_err());
}
