use std::env;
use std::process::{Command, ExitCode};

fn run_step(name: &str, args: &[&str]) -> Result<(), i32> {
    println!("==> {name}");

    let status = Command::new("cargo")
        .args(args)
        .status()
        .expect("failed to spawn cargo");

    if status.success() {
        Ok(())
    } else {
        Err(status.code().unwrap_or(1))
    }
}

fn has_subcommand(name: &str) -> bool {
    Command::new("cargo")
        .arg("--list")
        .output()
        .map(|out| {
            let text = String::from_utf8_lossy(&out.stdout);
            text.lines().any(|line| line.trim_start().starts_with(name))
        })
        .unwrap_or(false)
}

fn strict_mode() -> bool {
    match env::var("RUN_CHECKS_STRICT") {
        Ok(v) => v == "1" || v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("yes"),
        Err(_) => false,
    }
}

fn main() -> ExitCode {
    let strict = strict_mode();

    let required_steps: [(&str, &[&str]); 4] = [
        ("fmt", &["fmt", "--all", "--", "--check"]),
        (
            "clippy",
            &[
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("check", &["check", "--all-targets", "--all-features"]),
        ("test", &["test", "--all-targets", "--all-features"]),
    ];

    for (name, args) in required_steps {
        if let Err(code) = run_step(name, args) {
            eprintln!("step failed: {name}");
            return ExitCode::from(code as u8);
        }
    }

    let optional_steps: [(&str, &[&str], &str); 2] = [
        ("audit", &["audit"], "cargo-audit"),
        ("deny", &["deny", "check"], "cargo-deny"),
    ];

    for (name, args, install_hint) in optional_steps {
        if has_subcommand(name) {
            if let Err(code) = run_step(name, args) {
                eprintln!("step failed: {name}");
                return ExitCode::from(code as u8);
            }
        } else if strict {
            eprintln!("missing required cargo subcommand: {name}");
            eprintln!("install with: cargo install {install_hint} --locked");
            return ExitCode::from(1);
        } else {
            eprintln!("warning: skipping {name} (subcommand not installed)");
            eprintln!("hint: cargo install {install_hint} --locked");
        }
    }

    println!("all checks passed");
    ExitCode::SUCCESS
}
