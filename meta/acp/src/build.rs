use crate::parser::package_json::PackageJson;
use crate::Workspace;
use indexmap::IndexMap;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn pnpm_install(workspace: &'static crate::Workspace) {
    let mut cmd = std::process::Command::new(crate::process::PNPM_BINARY);
    cmd.current_dir(&workspace.path);
    cmd.arg("install");
    let status = cmd.status().expect("failed to execute process");
    if !status.success() {
        println!("Error: `pnpm install` failed");
        std::process::exit(1);
    }
}

pub fn write_config(workspace: &'static crate::Workspace) {
    for app_ref in workspace.get_build_apps() {
        // create a config file in target/debug/<cratename>.json
        let config_dir = workspace.path.join("target/debug");
        let config_path = config_dir.join(format!("{}.toml", app_ref.crate_name));

        // create the directory if it doesn't exist
        std::fs::create_dir_all(config_dir).expect("could not create config directory");

        // encode app_ref.config to toml
        let toml = toml::to_string(&app_ref.config).expect("could not serialize toml");

        // write to disk
        std::fs::write(&config_path, toml).expect("could not write config file");
    }
}

pub fn codegen(workspace: &'static Workspace) {
    let build_apps: Vec<&crate::Application> = workspace.app_map.values().collect();
    let build_modules: Vec<&crate::Crate> = workspace
        .crate_map
        .values()
        .filter(|m| m.mod_config.is_some())
        .collect();

    // Print a nice little display
    println!("--------------------------------------------------------------------------------");
    for app in build_apps.iter() {
        println!("App {}:", app.crate_name);
        println!(
            " - port: {}",
            app.config
                .get("webserver")
                .unwrap()
                .as_table()
                .unwrap()
                .get("port")
                .unwrap()
        );
        println!(" - rel_path: {}", app.rel_path);
        println!(
            " - extends: {}",
            app.extends_expanded
                .iter()
                .map(|v| { v.0.clone() })
                .collect::<Vec<_>>()
                .join(" -> ")
        );
        println!();
    }

    // Print a nice little display of all crates involved
    println!("--------------------------------------------------------------------------------");
    for crate_ref in build_modules.iter() {
        println!("Crate {}:", crate_ref.crate_name);
        println!(" - rel_path: {}", crate_ref.rel_path);
        println!(
            " - extends: [{}]",
            crate_ref
                .extends
                .iter()
                .map(|v| { v.0.clone() })
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }

    // iterate over references in pnpm crate names and update package.json and tsconfig.json
    // Weird, we are doign this for everything, not just the referenced crates... is that okay?
    println!("--------------------------------------------------------------------------------");
    for crate_ref in &workspace.get_pnpm_crates() {
        // deal with <crate>/package.json
        {
            let package_json_path = crate_ref.abs_path.join("package.json");

            // Load that struct and update it or create an new struct if it doesn't exist
            let package_json: PackageJson = if package_json_path.exists() {
                let mut package_json = PackageJson::load(&package_json_path);
                package_json.name = crate_ref.crate_name.to_string();
                package_json.version = crate_ref.version.clone();
                package_json
            } else {
                PackageJson {
                    name: crate_ref.crate_name.to_string(),
                    version: crate_ref.version.clone(),
                    dependencies: IndexMap::new(),
                }
            };

            // save the package.json back to disk
            package_json.save(&package_json_path);
        }

        // Handle tsconfig.json
        {
            let ts_config_path = crate_ref.abs_path.join("tsconfig.json");

            let extended_crate_refs = workspace.get_extended_crate_refs(&crate_ref.extends);

            let mut paths = IndexMap::new();
            paths.insert("@crate/*".to_string(), vec!["./src/*".to_string()]);
            for extended_crate_ref in &extended_crate_refs {
                paths.insert(
                    format!("@{}/*", extended_crate_ref.crate_name),
                    vec![format!(
                        "{}/src/*",
                        extended_crate_ref.abs_path.to_string_lossy()
                    )],
                );
            }

            let references = extended_crate_refs
                .iter()
                .map(|extended_crate_ref| json!({"path": extended_crate_ref.abs_path}))
                .collect::<Vec<_>>();

            let ts_config_json = json!(
                {
                    "compilerOptions": {
                        "composite": true,
                        "rootDir": "./src",
                        "esModuleInterop": true,
                        "forceConsistentCasingInFileNames": true,
                        "isolatedModules": true,
                        "lib": [
                            "dom",
                            "esnext"
                        ],
                        "module": "esnext",
                        "moduleResolution": "Bundler",
                        "skipLibCheck": true,
                        "sourceMap": true,
                        "strict": true,
                        "target": "es6",
                        "paths": paths,
                    },
                    "references": references
                }
            );

            let ts_config_json = serde_json::to_string_pretty(&ts_config_json)
                .expect("Could not serialize tsconfig.json");
            std::fs::write(ts_config_path, ts_config_json).expect("Error writing tsconfig.json");
        }
    }

    // the final step is to invoke the proper build commands for esbuild
    for app_ref in build_apps.iter() {
        let esbuild_target = workspace
            .path
            .join("target/esbuild")
            .join(&app_ref.crate_name.to_string());

        // delete it if it exists
        if esbuild_target.exists() {
            std::fs::remove_dir_all(&esbuild_target)
                .expect("Error removing esbuild target directory");
        }
        // create it
        std::fs::create_dir_all(&esbuild_target).expect("Error creating esbuild target directory");

        // also pre-create src/web dir
        std::fs::create_dir_all(&esbuild_target.join("src/web"))
            .expect("Error creating esbuild target directory");

        for crate_ref in app_ref.get_extended_crates(workspace) {
            println!(
                "--------------------------------------------------------------------------------"
            );
            println!("esbuild on crate: {}", crate_ref.crate_name);

            let entrypoints = crate_ref.get_entrypoints();

            println!("ts_entrypoints: {:?}", entrypoints);

            if !entrypoints.is_empty() {
                esbuild(
                    workspace,
                    &crate_ref.abs_path,
                    &esbuild_target.join("src/web"),
                    &entrypoints,
                );
            }
        }

        println!(
            "--------------------------------------------------------------------------------"
        );

        // The contents of the output directory are exactly what we'll serve, so we need to lis them
        let (allpaths, csspaths) = {
            let mut allpaths = Vec::new();
            let mut csspaths = Vec::new();
            let walker = ignore::Walk::new(&esbuild_target.join("src/web"));

            for entry in walker {
                match entry {
                    Ok(entry) => {
                        let p = entry.path();
                        if p.is_file() {
                            if p.extension().unwrap_or_default() == "css" {
                                csspaths.push(p.to_owned());
                            }

                            allpaths.push((
                                p.to_string_lossy().to_string(),
                                format!(
                                    "/{}",
                                    p.strip_prefix(&esbuild_target.join("src/web"))
                                        .unwrap()
                                        .to_string_lossy()
                                ),
                            ));
                        }
                    }
                    Err(err) => {
                        panic!("32i4808294893893893 error: {}", err);
                    }
                }
            }
            (allpaths, csspaths)
        };

        bundle_css(&csspaths);

        for (abs_path, web_path) in allpaths.iter() {
            println!("esbuild output: {} -> {}", web_path, abs_path);
        }

        // build the router function
        println!("Building router for app: {}", app_ref.crate_name);
        let extended_crates: Vec<approck_compiler::http_routing::CrateInfo> = app_ref
            .get_extended_crates(workspace)
            .iter()
            .map(|extended_crate| approck_compiler::http_routing::CrateInfo {
                name: extended_crate.crate_name.0.clone(),
                ident: extended_crate.ident.clone(),
                rel_path: extended_crate.rel_path.clone(),
                abs_path: extended_crate.abs_path.clone(),
            })
            .collect();

        let router_code = approck_compiler::http_routing::build_and_quote_router_function(
            &workspace.path,
            &app_ref.crate_name.0,
            &extended_crates,
            &allpaths,
        );

        write_and_format_rust_file(
            &app_ref.abs_path.join("src/approck_generated.rs"),
            &router_code,
        );
    }
}

fn write_and_format_rust_file(dest_path: &Path, rust_code: &proc_macro2::TokenStream) {
    let rust_tokens = rust_code.to_string();

    {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dest_path)
            .expect("Could not open or create file");

        f.write_all(rust_tokens.as_bytes())
            .expect("Could not write to file");
    }

    // run rustfmt on the code
    std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(dest_path)
        .spawn()
        .expect("Failed to run rustfmt");
}

fn esbuild(
    workspace: &crate::Workspace,
    input_directory: &Path,
    output_directory: &Path,
    relative_entrypoints: &[PathBuf],
) {
    // invoke esbuild
    let mut cmd = std::process::Command::new(workspace.path.join(crate::process::ESBUILD_BINARY));
    cmd.current_dir(input_directory);
    cmd.arg("--bundle");
    cmd.arg("--sourcemap");
    cmd.arg("--format=esm");
    cmd.arg("--loader:.mts=ts");
    cmd.arg("--loader:.mcss=css");
    cmd.arg(format!("--outdir={}", output_directory.to_string_lossy()));
    cmd.arg("--outbase=./src/web");
    cmd.args(relative_entrypoints);

    // run it and check for errors
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("Error running {:?} due to {}", cmd, e);
        std::process::exit(1);
    });

    if !status.success() {
        println!("Status code of cmd: {:?} was {}", cmd, status);
        std::process::exit(1);
    }
}

#[allow(unused)]
fn bundle_css(paths: &[PathBuf]) {
    let fs = lightningcss::bundler::FileProvider::new();

    for path in paths {
        let mut bundler = lightningcss::bundler::Bundler::new(
            &fs,
            None,
            lightningcss::stylesheet::ParserOptions::default(),
        );

        let stylesheet = bundler.bundle(path).unwrap();

        let printoptions = lightningcss::printer::PrinterOptions {
            ..lightningcss::printer::PrinterOptions::default()
        };

        let css = stylesheet.to_css(printoptions).unwrap();

        // write css.code to outputfile
        std::fs::write(path, css.code).unwrap();
    }
}
