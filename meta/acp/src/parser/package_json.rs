use indexmap::IndexMap;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct PackageJson {
    pub name: String,
    pub version: String,
    pub dependencies: IndexMap<String, String>,
}

impl PackageJson {
    pub fn load(package_json_path: &Path) -> Self {
        // open and read package.json
        let package_json = std::fs::read_to_string(package_json_path).unwrap_or_else(|e| {
            panic!("Error reading {} due to {}", package_json_path.display(), e)
        });

        // parse package.json
        serde_json::from_str(&package_json).unwrap_or_else(|e| {
            panic!("Error parsing {} due to {}", package_json_path.display(), e)
        })
    }

    pub fn save(&self, package_json_path: &Path) {
        // serialize and write
        let package_json = serde_json::to_string_pretty(&self).unwrap_or_else(|e| {
            panic!(
                "Could not serialize {} due to {}",
                package_json_path.display(),
                e
            )
        });
        std::fs::write(package_json_path, package_json).unwrap_or_else(|e| {
            panic!("Error writing {} due to {}", package_json_path.display(), e)
        });
    }
}
