use super::*;


fn buld_args(arg : &[&str]) -> Vec<String> {
    let mut res = Vec::new();
    res.push("test_app_name".to_string());
    for s in arg {
        res.push(s.to_string());
    }
    return res;
}

#[test]
fn basic_arguments_test() {
    let args = Arguments::parse(buld_args(&[]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*");
    assert_eq!(args.flags(), OptEnum::A|OptEnum::C|OptEnum::M);
}

#[test]
fn flags_test() {
    let args = Arguments::parse(buld_args(&["-c"]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*");
    assert_eq!(args.flags(), OptEnum::C);
}

#[test]
fn flags_test_2() {
    let args = Arguments::parse(buld_args(&["-l"]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*");
    assert_eq!(args.flags(), EnumSet::new());
}

#[test]
fn glob_test() {
    let args = Arguments::parse(buld_args(&["*.jpg"]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*.jpg");
    assert_eq!(args.flags(), OptEnum::A|OptEnum::C|OptEnum::M);
}

#[test]
fn glob_test_2() {
    let args = Arguments::parse(buld_args(&["*.jpg", "-", "*.png", "*.gif"]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*.jpg,!*.png,!*.gif");
    assert_eq!(args.flags(), OptEnum::A|OptEnum::C|OptEnum::M);
}

#[test]
fn all_test() {
    let args = Arguments::parse(buld_args(&["-a", "*.jpg", "-", "*.png", "*.gif"]));
    assert!(args.is_ok());
    let args = args.unwrap();
    assert_eq!(args.glob(), "*.jpg,!*.png,!*.gif");
    assert_eq!(args.flags(), OptEnum::A);
}