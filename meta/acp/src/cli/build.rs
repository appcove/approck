pub(super) fn build(workspace: &'static crate::Workspace) {
    crate::build::pnpm_install(workspace);
    crate::build::write_config(workspace);
    crate::build::codegen(workspace);
    crate::process::call_cargo_limited(workspace, "build", vec![]);
}
