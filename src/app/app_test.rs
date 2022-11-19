use super::*;

fn buld_args(arg: &[&str]) -> Vec<String> {
    let mut res = Vec::new();
    res.push("test_app_name".to_string());
    for s in arg {
        res.push(s.to_string());
    }
    return res;
}

fn glob(a: &Args) -> String {
    let mut local_globs = a.files.clone();
    let mut excl_globs = a.exclude_files.clone();
    let mut excl_globs: Vec<String> = excl_globs.iter_mut().map(|s| "!".to_owned() + s).collect();

    local_globs.append(&mut excl_globs);
    return local_globs[..].join(",");
}

#[test]
fn basic_arguments_test() {
    let args = Args::parse_vec(buld_args(&[]));
    assert!(args.is_ok());
    assert_eq!(glob(&args), "*");
    assert_eq!(args.flags(), EnumSet::all());
}

#[test]
#[cfg(windows)]
fn flags_test() {
    let args = Args::parse_vec(buld_args(&["-c"]));
    assert!(args.is_ok());

    assert_eq!(glob(&args), "*");
    assert_eq!(args.flags(), OptEnum::C);
}

#[test]
fn flags_test_2() {
    let args = Args::parse_vec(buld_args(&["-l"]));
    assert!(args.is_ok());

    assert_eq!(glob(&args), "*");
    assert_eq!(args.flags(), EnumSet::new());
}

#[test]
fn glob_test() {
    let args = Args::parse_vec(buld_args(&["*.jpg"]));
    assert!(args.is_ok());

    assert_eq!(glob(&args), "*.jpg");
    
    assert_eq!(args.flags(), EnumSet::all());
}

#[test]
fn glob_test_2() {
    let args = Args::parse_vec(buld_args(&["*.jpg", "--", "*.png", "*.gif"]));
    assert!(args.is_ok());
    assert_eq!(glob(&args), "*.jpg,!*.png,!*.gif");
    assert_eq!(args.flags(), EnumSet::all());
}

#[test]
fn all_test() {
    let args = Args::parse_vec(buld_args(&["-a", "*.jpg", "--", "*.png", "*.gif"]));
    assert!(args.is_ok());
    assert_eq!(glob(&args), "*.jpg,!*.png,!*.gif");
    assert_eq!(args.flags(), OptEnum::A);
}

#[test]
fn base_app_test() {
    let app = App::create(buld_args(&["-l", r"tests/assets/*.jpg"]));
    assert!(app.is_ok());
    let app = app.unwrap();
    assert!(app.files.len() > 0);

    let app = App::create(buld_args(&["-l", r"./tests/assets/*.jpg"]));
    assert!(app.is_ok());
    let app = app.unwrap();
    assert!(app.files.len() > 0);

    let app = App::create(buld_args(&[
        "-l",
        r"./tests/assets/*.jpg",
        "--",
        "./tests/assets/202104*.jpg",
    ]));
    assert!(app.is_ok());
    let app = app.unwrap();
    assert_ne!(app.files.len(), 0);
}
