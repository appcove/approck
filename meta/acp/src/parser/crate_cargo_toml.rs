use indexmap::IndexMap;
use serde::de::Error as DeError;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt;

// create a struct for member Cargo.toml
#[derive(Debug, Deserialize)]
pub(crate) struct CrateCargoToml {
    pub(crate) package: CrateCargoTomlPackage,
    /// indexmap of dependencies

    #[serde(default, deserialize_with = "deserialize_dependencies")]
    pub(crate) dependencies: IndexMap<String, CrateDependency>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CrateCargoTomlPackage {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) edition: String,

    #[serde(default)]
    pub(crate) metadata: CrateCargoTomlPackageMetadata,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct CrateCargoTomlPackageMetadata {
    #[serde(default)]
    pub(crate) acp: CrateCargoTomlPackageMetadataAcp,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct CrateCargoTomlPackageMetadataAcp {
    #[serde(default)]
    pub(crate) app: Option<CrateCargoTomlPackageMetadataAcpApp>,

    #[serde(default)]
    pub(crate) module: Option<CrateCargoTomlPackageMetadataAcpMod>,

    #[serde(default)]
    pub(crate) extends: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct CrateCargoTomlPackageMetadataAcpApp {
    pub(crate) port: u16,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct CrateCargoTomlPackageMetadataAcpMod {}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct CrateDependency {
    #[serde(default)]
    pub(crate) workspace: bool,
    pub(crate) version: Option<String>,
    pub(crate) path: Option<String>,
}

// Custom deserializer for the dependencies field to handle varying data formats
fn deserialize_dependencies<'de, D>(
    deserializer: D,
) -> Result<IndexMap<String, CrateDependency>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DependenciesVisitor;

    impl<'de> Visitor<'de> for DependenciesVisitor {
        type Value = IndexMap<String, CrateDependency>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "a string \"version\", or a table with one or more of {version, workspace, path}",
            )
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut rval = IndexMap::new();

            while let Some((key, val)) = access.next_entry::<String, toml::Value>()? {
                match val {
                    toml::Value::String(s) => {
                        rval.insert(
                            key,
                            CrateDependency {
                                version: Some(s),
                                ..Default::default()
                            },
                        );
                    }
                    toml::Value::Table(t) => {
                        rval.insert(key, t.try_into().map_err(M::Error::custom)?);
                    }
                    _ => return Err(M::Error::custom("expected a string or a table")),
                }
            }

            Ok(rval)
        }
    }

    deserializer.deserialize_map(DependenciesVisitor)
}
