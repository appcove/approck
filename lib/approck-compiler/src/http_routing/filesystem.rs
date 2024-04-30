use std::collections::HashMap;
use std::path::Path;

pub(super) fn scan(
    workspace_path: &Path,
    app_crate_name: &str,
    extended_crates: &[super::CrateInfo],
) -> Vec<super::PathHit> {
    // If there are no extended crates, then there is no need to scan
    // WARNING: this is important otherwise git grep will scan the entire workspace
    if extended_crates.is_empty() {
        return Vec::new();
    }

    let path_to_crate: HashMap<String, String> = extended_crates
        .iter()
        .map(|crate_ref| {
            if crate_ref.name == app_crate_name {
                (crate_ref.rel_path.clone(), "crate".to_string())
            } else {
                (
                    crate_ref.rel_path.clone(),
                    crate_ref
                        .rel_path
                        .split('/')
                        .last()
                        .unwrap_or_else(|| {
                            panic!(
                                "0x8327328380; for some reason path splitting failed: {}",
                                crate_ref.rel_path
                            )
                        })
                        .replace('-', "_"),
                )
            }
        })
        .collect();

    // call ripgrep to get all the files that have approck::get in them
    let mut cmd = std::process::Command::new("git");
    cmd.arg("grep");
    cmd.arg("--untracked");
    cmd.arg("--files-with-matches");
    cmd.arg("--null"); // Each line is delimited by \0
    cmd.arg("--no-color");
    cmd.current_dir(workspace_path); // always run from the workspace root
    cmd.arg(r"^#\[approck::http");
    cmd.arg("--");

    // Now add all the paths from the specified crates (only care about .../src/*)
    for crate_ref in extended_crates {
        cmd.arg(format!("{}/src", crate_ref.rel_path));
    }

    // The output is '\0' delimited filenames
    let content = cmd.output().expect("Error running git grep");

    // parse the stdout into a list of PathHits
    let mut path_hits = Vec::new();
    let output_text =
        String::from_utf8(content.stdout).expect("Failed to parse output of git grep as utf-8");
    let grep_paths = output_text.split('\0');
    for grep_path in grep_paths {
        if grep_path.is_empty() {
            continue;
        }

        let abs_path = workspace_path.join(grep_path);
        let rel_path = grep_path;
        let ext = grep_path.split('.').last().map(|s| s.to_string());

        let (crate_path, mod_path) = match rel_path.find("/src/") {
            Some(index) => {
                if !rel_path.ends_with(".rs") {
                    continue;
                }

                let before = &rel_path[..index];
                let after = &rel_path[index + "/src/".len()..rel_path.len() - ".rs".len()];
                (before.to_string(), after.to_string())
            }
            None => panic!(
                "0x8274839238; When splitting `{}` by `/src/`, the separator was not found",
                rel_path
            ),
        };

        let crate_name = path_to_crate
            .get(crate_path.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "0x8274839238; Could not find crate name for path `{}`",
                    crate_path
                )
            })
            .to_string();

        let mut rust_ident = format!("{}::{}", crate_name, mod_path.replace('/', "::"));

        if rust_ident.ends_with("::mod") {
            rust_ident = rust_ident.strip_suffix("::mod").unwrap().to_string();
        } else if rust_ident == "crate::lib" {
            rust_ident = "crate".to_string();
        }

        let path_hit = super::PathHit {
            rel_path: rel_path.to_string(),
            abs_path,
            ext,
            crate_name,
            rust_ident,
        };

        path_hits.push(path_hit);
    }

    path_hits
}
