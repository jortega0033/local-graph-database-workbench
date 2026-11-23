//! Entry point and CLI: subcommands demo, init, load, query and serve.

use std::io::Write;

const USAGE: &str = "waymark: offline graph workbench

Usage: waymark <command>

Commands:
  demo    run the bundled seed
  init    create a database
  load    load a batch into a database
  query   run a query
  serve   serve a database in the browser

Options:
  -h, --help     print this help
  -V, --version  print the version
";

/// Runs the command line and returns the process exit code.
pub fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => {
            let _ = out.write_all(USAGE.as_bytes());
            0
        }
        Some("--version" | "-V") => {
            let _ = writeln!(out, "waymark {}", env!("CARGO_PKG_VERSION"));
            0
        }
        Some("demo") => not_implemented(err, 2),
        Some("init" | "load" | "serve") => not_implemented(err, 4),
        Some("query") => not_implemented(err, 5),
        Some(other) => {
            let _ = writeln!(err, "unknown command: {other}");
            2
        }
        None => {
            let _ = err.write_all(USAGE.as_bytes());
            2
        }
    }
}

fn not_implemented(err: &mut dyn Write, ticket: u32) -> i32 {
    let _ = writeln!(err, "not implemented yet: see #{ticket}");
    1
}
