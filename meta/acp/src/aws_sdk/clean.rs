use super::WorkspaceExt;

/// This is the translation function between the CLI invocation and the back-end.
pub(crate) fn clean(workspace: &'static crate::Workspace) -> granite::Result<()> {
    let smithy_rs = workspace.smithy_rs()?;

    if smithy_rs.path.exists() {
        std::fs::remove_dir_all(&smithy_rs.path)?;
    }

    Ok(())
}
