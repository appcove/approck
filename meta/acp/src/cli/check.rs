use std::io::Write;
use termcolor::WriteColor;

pub(super) fn check(workspace: &'static crate::Workspace, all: bool) {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
    let mut write_warning = |msg: &str| {
        stdout
            .set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)))
            .expect("Failed to set color");
        writeln!(stdout, "{}", msg).expect("Failed to write to stdout");
        stdout.reset().expect("Failed to reset color");
    };

    let mut audit_crate = |crate_ref: &crate::Crate| {
        for (dep_name, dep_struct) in crate_ref.dependencies.iter() {
            if dep_struct.version.is_some() {
                write_warning(&format!("Warning in `{}/Cargo.toml`: dependency `{}` has a version specified.  It should be in the workspace, or have path specified.", crate_ref.rel_path, dep_name));
            }
        }
    };
    if all {
        crate::process::call_cargo_all(workspace, "check", vec![]);
        for crate_ref in workspace.crate_map.values() {
            audit_crate(crate_ref);
        }
    } else {
        crate::process::call_cargo_limited(workspace, "check", vec![]);
        for crate_ref in workspace.get_build_crates() {
            audit_crate(crate_ref);
        }
    }
}
