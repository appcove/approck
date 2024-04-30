use super::WorkspaceExt;

pub(crate) fn clone(workspace: &'static crate::Workspace) -> granite::Result<()> {
    let smithy_rs = workspace.smithy_rs()?;

    if smithy_rs.path.exists() {
        granite::return_invalid_operation!(
            "Repo already cloned at `{}`. Please run `ace aws-sdk clean` to cleanup.",
            smithy_rs.path.display()
        );
    }

    // clone smithy_rs.repo into smithy_rs.path
    let mut cmd = std::process::Command::new("git");
    cmd.arg("-c").arg("advice.detachedHead=false");
    cmd.arg("clone");
    cmd.arg("--branch").arg(&smithy_rs.commit);
    cmd.arg(&smithy_rs.repo);
    cmd.arg(&smithy_rs.path);

    let status = cmd.status()?;

    if !status.success() {
        granite::return_process_error!(
            "Failed to clone `{}` into `{}`",
            smithy_rs.repo,
            smithy_rs.path.display()
        );
    }

    Ok(())
}
