mod aws_sdk;
mod build;
pub mod cli;
mod parser;
mod process;
mod schema;
mod util;

use indexmap::IndexMap;
use serde::Deserialize;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CrateName(String);

#[derive(Debug)]
pub struct Workspace {
    pub path: PathBuf,
    pub workspace_cargo_toml_path: PathBuf,
    pub local_toml_path: PathBuf,
    pub crate_map: IndexMap<CrateName, Crate>,
    pub app_map: IndexMap<CrateName, Application>,
}

// create a struct to represent a member project
#[derive(Debug)]
pub struct Crate {
    pub ident: String,
    pub crate_name: CrateName,
    pub rel_path: String,
    pub abs_path: PathBuf,
    pub version: String,
    pub edition: String,
    pub dependencies: IndexMap<String, CrateDependency>,

    /// This is the actual items from the Cargo.toml, not a sorted MRO
    pub extends: Vec<CrateName>,

    pub app_config: Option<CrateApp>,

    pub mod_config: Option<CrateMod>,

    /// this crate is included in the current build
    pub in_build: bool,

    /// this crate is referenced in pnpm-workspace.yaml
    pub in_pnpm: bool,

    /// Crate config will be added here
    pub config: toml::Table,
}

#[derive(Debug)]
pub struct CrateDependency {
    pub workspace: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug)]
pub struct CrateApp {
    pub port: u16,
}

#[derive(Debug)]
pub struct CrateMod {}

/// This is the struct that represents a crate which is a top level application, referenced in LOCAL.yaml
/// It contains a vec of topologically sorted crate names, where the first crate is same as the Application itself, and the last is the furthest dependancy
#[derive(Debug)]
pub struct Application {
    pub ident: String,
    pub crate_name: CrateName,
    pub rel_path: String,
    pub abs_path: PathBuf,

    /// this app is included in the current build
    pub in_build: bool,

    /// App config will be added here
    pub config: toml::Table,

    /// This is a vec of topologically sorted crate names,
    /// where the first crate is same as AppCrate itself, and the last is the furthest dependancy
    pub extends_expanded: Vec<CrateName>,
}

impl Workspace {
    /// This is how you create a Workspace instance, and should only be called once typically, notwithstanding tests.
    pub fn init(orig_cwd: &Path) -> Self {
        // assert that orig_cwd is an absolute path
        assert!(orig_cwd.is_absolute());

        let mut port_to_name_map = HashMap::new();

        // find the root of the git repo
        let path = {
            let mut cwd = orig_cwd.to_owned();
            loop {
                if cwd.join(".git").exists() {
                    break cwd;
                }
                if !cwd.pop() {
                    panic!("Not in a git repo at {}", orig_cwd.display());
                };
            }
        };

        let workspace_cargo_toml_path = path.join("Cargo.toml");
        let local_toml_path = path.join("LOCAL.toml");

        // open and read LOCAL.toml
        let local_toml: crate::parser::local_toml::LocalToml = toml::from_str(
            &std::fs::read_to_string(&local_toml_path).expect("Error reading LOCAL.toml"),
        )
        .expect("Error parsing LOCAL.toml");

        // open and read Cargo.toml
        let workspace_cargo_toml =
            std::fs::read_to_string(&workspace_cargo_toml_path).expect("Error reading Cargo.toml");

        // parse top level Cargo.toml
        let workspace_cargo_toml: crate::parser::workspace_cargo_toml::WorkspaceCargoToml =
            toml::from_str(&workspace_cargo_toml).expect("Error parsing Cargo.toml");

        let pnpm_workspace = crate::parser::pnpm_workspace_yaml::PnpmWorkspaceYaml::load(
            &path.join("pnpm-workspace.yaml"),
        );

        // create a map of workspace crates - every crate referenced by the top level Cargo.toml
        let mut crate_map: IndexMap<CrateName, Crate> =
            IndexMap::with_capacity(workspace_cargo_toml.workspace.members.len());

        for member in &workspace_cargo_toml.workspace.members {
            let crate_ref = Crate::load(&path, member);

            // validate for unique port numbers
            if let Some(crate_app) = &crate_ref.app_config {
                if port_to_name_map.contains_key(&crate_app.port) {
                    panic!(
                        "Error: port {} is in use by both `{}` and `{}`.  Look in the respective Cargo.toml's to sort it out.",
                        crate_app.port,
                        crate_ref.crate_name.0,
                        port_to_name_map[&crate_app.port]
                    );
                }
                port_to_name_map.insert(crate_app.port, crate_ref.crate_name.0.to_owned());
            }

            crate_map.insert(crate_ref.crate_name.clone(), crate_ref);
        }

        // create a map of app crates... this is any that have `is_app = true` in their `Cargo.toml`
        let mut app_map: IndexMap<CrateName, Application> = crate_map
            .iter()
            .filter_map(|(crate_name, crate_ref)| {
                crate_ref.app_config.as_ref().map(|crate_app| {
                    // create the default config (presently, only port is included)
                    let mut webserver = toml::Table::new();
                    webserver.insert(
                        "port".to_string(),
                        toml::Value::Integer(crate_app.port as i64),
                    );

                    let mut config = toml::Table::new();
                    config.insert("webserver".to_string(), toml::Value::Table(webserver));

                    (
                        crate_name.clone(),
                        Application {
                            ident: crate_ref.ident.clone(),
                            crate_name: crate_ref.crate_name.clone(),
                            rel_path: crate_ref.rel_path.clone(),
                            abs_path: crate_ref.abs_path.clone(),
                            extends_expanded: Application::extends_expand(crate_ref, &crate_map),

                            // the following may be updated later if this is referenced by LOCAL.toml
                            in_build: false,
                            config,
                        },
                    )
                })
            })
            .collect();

        // update crate_map with any crates refrenced in pnmp-workspace.yaml
        for crate_name in pnpm_workspace.get_crate_names() {
            let crate_ref = match crate_map.get_mut(&crate_name) {
                Some(crate_ref) => crate_ref,
                None => panic!("`pnpm-workspace.yaml` references `{}` which is not defined as a crate in the workspace.", crate_name),
            };
            crate_ref.in_pnpm = true;
        }

        // update crate_map with any crates referenced in LOCAL.toml
        for (crate_name, crate_toml) in &local_toml.crate_map {
            let crate_name = CrateName(crate_name.clone());
            let crate_ref = match crate_map.get_mut(&crate_name) {
                Some(crate_ref) => crate_ref,
                None => panic!("`LOCAL.toml` references `[crate.{0}]` which is not defined as a crate in the workspace.", crate_name),
            };
            crate_ref.in_build = true;
            crate::util::deep_merge_table(
                &mut crate_ref.config,
                crate_toml,
                format!("LOCAL.toml: {}", crate_name),
            );
        }

        // update app_map with any apps referenced in LOCAL.toml
        for (app_name, app_toml) in &local_toml.app_map {
            let app_name = CrateName(app_name.clone());
            let app = match app_map.get_mut(&app_name) {
                Some(app) => app,
                None => match crate_map.get(&app_name) {
                    Some(crate_ref) => panic!("`LOCAL.toml` references `[app.{0}]` which is defined as a crate, but not an app, in the workspace.  \n - If the intention is to define it as an application, then add `package.metadata.acp.is_app = true` to `{1}/Cargo.toml`.  \n - If the intention is to just add it to the build list, then add `[crate.{0}]` to `LOCAL.toml`", app_name, crate_ref.rel_path),
                    None => panic!("`LOCAL.toml` references `[app.{0}]` which is not defined as a crate in the workspace.", app_name),
                },
            };
            // extend the

            app.in_build = true;
            crate::util::deep_merge_table(
                &mut app.config,
                app_toml,
                format!("LOCAL.toml: {}", app_name),
            );

            // Iterate over the extends and set those crates to also build
            for crate_name in app.extends_expanded.iter() {
                let crate_ref = match crate_map.get_mut(crate_name) {
                    Some(crate_ref) => crate_ref,
                    None => panic!(
                        "`{0}` extends `{1}` which is not defined as a crate in the workspace.",
                        app_name, crate_name
                    ),
                };
                crate_ref.in_build = true;
            }
        }

        Workspace {
            path,
            local_toml_path,
            workspace_cargo_toml_path,
            crate_map,
            app_map,
        }
    }

    pub fn get_build_apps(&self) -> Vec<&Application> {
        self.app_map
            .iter()
            .filter_map(|(_k, v)| if v.in_build { Some(v) } else { None })
            .collect()
    }

    // LUKE: why does .values().filter() complain ?
    pub fn get_build_crates(&self) -> Vec<&Crate> {
        self.crate_map
            .iter()
            .filter_map(|(_k, v)| if v.in_build { Some(v) } else { None })
            .collect()
    }

    // LUKE: why does .values().filter() complain ?
    pub fn get_extended_crate_refs(&self, extends: &[CrateName]) -> Vec<&Crate> {
        extends
            .iter()
            .map(|v| {
                self.crate_map
                    .get(v)
                    .unwrap_or_else(|| panic!("Could not load extended crate `{v}`"))
            })
            .collect()
    }

    pub fn get_pnpm_crates(&self) -> Vec<&Crate> {
        self.crate_map
            .iter()
            .filter_map(|(_k, v)| if v.in_pnpm { Some(v) } else { None })
            .collect()
    }
}

impl std::fmt::Display for CrateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for CrateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement Deserialize for CrateName
impl<'de> Deserialize<'de> for CrateName {
    fn deserialize<D>(deserializer: D) -> Result<CrateName, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(CrateName(s))
    }
}

impl Application {
    pub fn get_extended_crates<'a>(&self, workspace: &'a Workspace) -> Vec<&'a Crate> {
        self.extends_expanded
            .iter()
            .map(|v| {
                workspace
                    .crate_map
                    .get(v)
                    .unwrap_or_else(|| panic!("could not load crate name: {v}"))
            })
            .collect()
    }

    fn extends_expand(top_crate: &Crate, crate_map: &IndexMap<CrateName, Crate>) -> Vec<CrateName> {
        // Recursively build a list of pairs of app names and their extends
        let mut pairs: Vec<(CrateName, CrateName)> = Vec::new();
        Self::_extends_expand_rec(
            &mut pairs,
            top_crate.crate_name.clone(),
            top_crate.extends.clone(),
            crate_map,
        );

        // Create a topological graph
        let mut graph = petgraph::graph::DiGraph::new();

        let app_name_to_node_map = {
            let nameset = {
                let mut set = std::collections::HashSet::new();
                set.insert(top_crate.crate_name.clone());
                for (app_name, extends_app_name) in &pairs {
                    set.insert(app_name.clone());
                    set.insert(extends_app_name.clone());
                }
                set
            };

            let mut map = std::collections::HashMap::new();
            for name in nameset.iter() {
                map.insert(name.clone(), graph.add_node(name.clone()));
            }
            map
        };

        for (app_name, extends_app_name) in &pairs {
            let n1 = app_name_to_node_map.get(app_name).unwrap();
            let n2 = app_name_to_node_map.get(extends_app_name).unwrap();
            graph.add_edge(*n1, *n2, ());
        }

        let sorted_nodes = match petgraph::algo::toposort(&graph, None) {
            Ok(sorted_nodes) => sorted_nodes,
            Err(cycle_node) => {
                panic!(
                    "Circular reference detected in `extends` directive: {:?}",
                    cycle_node
                );
            }
        };

        let extends: Vec<_> = sorted_nodes
            .iter()
            .map(|node| {
                let app_name = graph.node_weight(*node).unwrap();
                app_name.to_owned()
            })
            .collect();

        extends
    }

    fn _extends_expand_rec(
        pairs: &mut Vec<(CrateName, CrateName)>,
        app_name: CrateName,
        app_extends: Vec<CrateName>,
        crate_map: &IndexMap<CrateName, Crate>,
    ) {
        for extends_app_name in &app_extends {
            pairs.push((app_name.clone(), extends_app_name.clone()));
        }
        for extends_app_name in &app_extends {
            let member_crate = crate_map
                .get(extends_app_name)
                .ok_or_else(|| {
                    panic!(
                        "Error looking up app in workspace.member_map: {}",
                        extends_app_name
                    )
                })
                .unwrap();

            if member_crate.mod_config.is_none() {
                panic!("Error: crate `{}` attempting to extend crate `{}` which does not have `package.metadata.acp.module = {{}}` in `{}/Cargo.toml`", app_name.0, extends_app_name.0, member_crate.rel_path);
            }

            Self::_extends_expand_rec(
                pairs,
                member_crate.crate_name.clone(),
                member_crate.extends.clone(),
                crate_map,
            );
        }
    }
}

impl Crate {
    pub fn load(path: &Path, member_path: &str) -> Self {
        let cargo_toml_path = path.join(member_path).join("Cargo.toml");

        // open and read Cargo.toml
        let cargo_toml =
            std::fs::read_to_string(&cargo_toml_path).expect("Error reading Cargo.toml");

        // parse Cargo.toml
        let cargo_toml: crate::parser::crate_cargo_toml::CrateCargoToml =
            match toml::from_str(&cargo_toml) {
                Ok(cargo_toml) => cargo_toml,
                Err(e) => {
                    panic!(
                        "Error parsing Cargo.toml at `{}`\n\n{:#?}",
                        cargo_toml_path.display(),
                        e
                    );
                }
            };

        let crate_app = cargo_toml
            .package
            .metadata
            .acp
            .app
            .map(|app| CrateApp { port: app.port });

        let crate_mod = cargo_toml.package.metadata.acp.module.map(|_| CrateMod {});

        Self {
            ident: cargo_toml.package.name.replace('-', "_"),
            crate_name: CrateName(cargo_toml.package.name),
            rel_path: member_path.to_owned(),
            abs_path: path.join(member_path),
            version: cargo_toml.package.version,
            edition: cargo_toml.package.edition,
            dependencies: cargo_toml
                .dependencies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        CrateDependency {
                            workspace: v.workspace,
                            version: v.version,
                            path: v.path,
                        },
                    )
                })
                .collect(),
            extends: cargo_toml
                .package
                .metadata
                .acp
                .extends
                .iter()
                .map(|v| CrateName(v.clone()))
                .collect(),
            app_config: crate_app,
            mod_config: crate_mod,

            // the following may be updated later if this is referenced by LOCAL.toml
            in_build: false,
            in_pnpm: false,
            config: toml::Table::new(),
        }
    }

    pub fn get_entrypoints(&self) -> Vec<PathBuf> {
        let mut ts_paths = Vec::new();

        let src_web_path = self.abs_path.join("src/web");

        if !src_web_path.exists() {
            return ts_paths;
        }

        let walker = ignore::Walk::new(&src_web_path);

        for entry in walker {
            match entry {
                Ok(entry) => {
                    let p = entry.path();
                    let e = p.extension().unwrap_or_default();
                    // TODO: encountering a .ts file outside of web/ is bad
                    // the problem is that esbuild doesn't care, and will treat them as entrypoints anyway, messing up bundling.
                    if e == "ts" || e == "css" {
                        ts_paths.push(p.strip_prefix(&self.abs_path).unwrap().to_owned());
                    }
                }
                Err(err) => {
                    panic!("get_entrypoints() error: {}", err);
                }
            }
        }

        ts_paths
    }
}
