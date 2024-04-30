pub(super) fn run(workspace: &'static crate::Workspace) {
    //TODO: break this up into smaller pieces so that only the relevant projects are adjusted for runtime
    crate::build::codegen(workspace);
    crate::process::call_cargo_limited(workspace, "build", vec![]);
    crate::process::call_cargo_run_multi(workspace);
}
