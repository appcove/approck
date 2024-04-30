use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub static DPRINT_BINARY: &str = "meta/dprint-0.45.0";
pub static ESBUILD_BINARY: &str = "meta/esbuild-0.19.11";
pub static PNPM_BINARY: &str = "meta/pnpm-8.15.3";

pub fn call_cargo_limited(workspace: &'static crate::Workspace, subcmd: &str, args: Vec<String>) {
    let build_crates = workspace.get_build_crates();

    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir(&workspace.path);
    cmd.arg(subcmd);
    for crate_ref in build_crates {
        cmd.arg("-p");
        cmd.arg(&crate_ref.crate_name.0);
    }
    cmd.args(args);

    let status = cmd.status().expect("failed to execute process");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

pub fn call_cargo_all(workspace: &'static crate::Workspace, subcmd: &str, args: Vec<String>) {
    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir(&workspace.path);
    cmd.arg(subcmd);
    cmd.args(args);

    let status = cmd.status().expect("failed to execute process");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

pub fn call_dprint(workspace: &'static crate::Workspace, subcmd: &str, args: Vec<String>) {
    let crate_list = workspace.get_build_crates();

    let mut cmd = std::process::Command::new(workspace.path.join(DPRINT_BINARY));
    cmd.current_dir(&workspace.path);
    cmd.arg(subcmd);
    cmd.args(args);
    for crate_ref in crate_list {
        cmd.arg(format!("{}/**", &crate_ref.rel_path));
    }

    let status = cmd.status().expect("failed to execute process");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

pub fn call_cargo_run_multi(workspace: &'static crate::Workspace) {
    let apps = workspace.get_build_apps();

    let mut max_length = 0;

    let executables = apps
        .iter()
        .map(|app| {
            if app.crate_name.0.len() > max_length {
                max_length = app.crate_name.0.len();
            }
            let p = workspace.path.join("target/debug").join(&app.crate_name.0);
            (app.crate_name.clone(), p)
        })
        .filter(|(_app_name, app_binary)| app_binary.exists())
        .collect::<Vec<_>>();

    let handles: Vec<_> = executables
        .into_iter()
        .map(|(app_name, executable_path)| {
            thread::spawn(move || {
                let mut child = Command::new(executable_path)
                    .current_dir(&workspace.path)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to start process");

                // prefix should be padded to a consistent length based on max_length
                let prefix = format!("[{:width$}] ", app_name, width = max_length);

                let stdout = child.stdout.take().expect("Failed to take stdout of child");
                let reader = BufReader::new(stdout);

                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}{}", prefix, line),
                        Err(e) => eprintln!("{} Error reading line: {}", prefix, e),
                    }
                }

                // Optional: Handle termination of child process if Ctrl+C is pressed
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Optional: Cleanup or signal handling for child processes
}
