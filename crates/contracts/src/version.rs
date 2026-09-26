use serde::{Deserialize, Serialize};

macro_rules! version_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub u16);
    };
}

version_type!(SchemaVersion);
version_type!(GeneratorVersion);
version_type!(ContentVersion);
version_type!(RecipeVersion);
