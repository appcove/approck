use serde::Deserialize;

// create a struct for workspace Cargo.toml

#[derive(Debug, Deserialize)]
pub(crate) struct WorkspaceCargoToml {
    pub workspace: WorkspaceCargoTomlWorkspace,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WorkspaceCargoTomlWorkspace {
    pub members: Vec<String>,
}
