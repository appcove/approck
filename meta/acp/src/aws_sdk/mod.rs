mod build;
mod clean;
mod clone;

use granite::ResultExt;
use std::path::PathBuf;

pub(crate) use self::build::build;
pub(crate) use self::clean::clean;
pub(crate) use self::clone::clone;

#[derive(serde::Deserialize)]
struct SmithyRsToml {
    repo: String,
    commit: String,
}

struct SmithyRs {
    path: PathBuf,
    repo: String,
    commit: String,
}

trait WorkspaceExt {
    fn aws_sdk_path(&self) -> PathBuf;
    fn smithy_rs(&self) -> granite::Result<SmithyRs>;
}

impl WorkspaceExt for crate::Workspace {
    fn aws_sdk_path(&self) -> PathBuf {
        self.path.join("aws-sdk")
    }

    fn smithy_rs(&self) -> granite::Result<SmithyRs> {
        let toml_path = self.aws_sdk_path().join("smithy-rs.toml");
        let settings = std::fs::read_to_string(&toml_path)?;
        let smithy_rs_toml: SmithyRsToml = toml::from_str(&settings)
            .amend(|e| e.add_context(format!("While loading `{}`", toml_path.display())))?;

        Ok(SmithyRs {
            path: self.aws_sdk_path().join("smithy-rs"),
            repo: smithy_rs_toml.repo,
            commit: smithy_rs_toml.commit,
        })
    }
}
