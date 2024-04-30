use crate::CrateName;
use std::path::Path;

#[derive(serde::Deserialize)]
pub(crate) struct PnpmWorkspaceYaml {
    pub packages: Vec<String>,
}

impl PnpmWorkspaceYaml {
    pub(crate) fn load(pnpm_workspace_path: &Path) -> Self {
        // open and read pnpm-workspace.json
        let pnpm_workspace = std::fs::read_to_string(pnpm_workspace_path).unwrap_or_else(|e| {
            panic!(
                "Error reading {} due to {}",
                pnpm_workspace_path.display(),
                e
            )
        });

        // parse pnpm-workspace.json
        serde_yaml::from_str(&pnpm_workspace).unwrap_or_else(|e| {
            panic!(
                "Error parsing {} due to {}",
                pnpm_workspace_path.display(),
                e
            )
        })
    }

    pub(crate) fn get_crate_names(&self) -> Vec<CrateName> {
        self.packages
            .iter()
            .map(|v| {
                // convert v to a pathbuf and take the basename
                CrateName(
                    v.rsplit_once('/')
                        .unwrap_or_else(|| panic!("Error splitting {}", v,))
                        .1
                        .to_owned(),
                )
            })
            .collect()
    }
}
