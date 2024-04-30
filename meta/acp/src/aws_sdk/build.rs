use super::WorkspaceExt;

/// This is the translation function between the CLI invocation and the back-end.
pub(crate) fn build(workspace: &'static crate::Workspace) -> granite::Result<()> {
    let smithy_rs = workspace.smithy_rs()?;

    if !smithy_rs.path.exists() {
        granite::return_invalid_operation!(
            "Repo not cloned at `{}`. Please run `ace aws-sdk clone` first.",
            smithy_rs.path.display()
        );
    }

    // clean and reset the repo
    let mut cmd = std::process::Command::new("git");
    cmd.current_dir(&smithy_rs.path);
    cmd.arg("clean").arg("-fdx");

    let status = cmd.status()?;
    if !status.success() {
        granite::return_process_error!("Failed to clean `{}`", smithy_rs.path.display());
    }

    let mut cmd = std::process::Command::new("git");
    cmd.current_dir(&smithy_rs.path);
    cmd.arg("checkout");
    cmd.arg(&smithy_rs.commit);

    let status = cmd.status()?;
    if !status.success() {
        granite::return_process_error!(
            "Failed to reset `{}` to commit `{}`",
            smithy_rs.path.display(),
            smithy_rs.commit
        );
    }

    Ok(())
}
