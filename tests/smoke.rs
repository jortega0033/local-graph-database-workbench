use waymark::cli;

fn run(args: &[&str]) -> (i32, String, String) {
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    let (mut out, mut err) = (Vec::<u8>::new(), Vec::<u8>::new());
    let code = cli::run(&args, &mut out, &mut err);
    let out = String::from_utf8(out).unwrap();
    let err = String::from_utf8(err).unwrap();
    (code, out, err)
}

#[test]
fn help_prints_usage() {
    let (code, out, _) = run(&["--help"]);
    assert_eq!(code, 0);
    assert!(out.contains("Usage: waymark"));
}

#[test]
fn version_prints_package_version() {
    let (code, out, _) = run(&["--version"]);
    assert_eq!(code, 0);
    assert_eq!(out, "waymark 0.0.0\n");
}

#[test]
fn stub_command_exits_non_zero() {
    let (code, _, err) = run(&["demo"]);
    assert_ne!(code, 0);
    assert_eq!(err, "not implemented yet: see #2\n");
}

#[test]
fn batch_module_exists() {
    #[allow(unused_imports)]
    use waymark::batch;
}

#[test]
fn cli_module_exists() {
    #[allow(unused_imports)]
    use waymark::cli;
}

#[test]
fn exec_module_exists() {
    #[allow(unused_imports)]
    use waymark::exec;
}

#[test]
fn graph_module_exists() {
    #[allow(unused_imports)]
    use waymark::graph;
}

#[test]
fn http_module_exists() {
    #[allow(unused_imports)]
    use waymark::http;
}

#[test]
fn index_module_exists() {
    #[allow(unused_imports)]
    use waymark::index;
}

#[test]
fn journal_module_exists() {
    #[allow(unused_imports)]
    use waymark::journal;
}

#[test]
fn query_module_exists() {
    #[allow(unused_imports)]
    use waymark::query;
}

#[test]
fn response_module_exists() {
    #[allow(unused_imports)]
    use waymark::response;
}
