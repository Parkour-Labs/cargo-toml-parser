use macros::Builder;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder,
)]
pub struct CargoToml {
    pub package: Option<Package>,
    pub workspace: Option<Workspace>,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder,
)]
pub struct Package {
    pub name: Option<String>,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder,
)]
pub struct Workspace {
    pub members: Vec<String>,
}

impl TryFrom<&str> for CargoToml {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

impl TryFrom<&str> for Package {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

impl TryFrom<&str> for Workspace {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
