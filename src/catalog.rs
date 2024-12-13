use crate::asset::Asset;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub assets: BTreeMap<String, Asset>,
}

impl Catalog {
    pub fn assets(&'static self) -> Vec<&'static Asset> {
        self.assets.values().collect()
    }
}

impl Catalog {
    #[cfg(feature = "ssr")]
    pub async fn new(path: &Path) -> Result<Self, &'static str> {
        use leptos::leptos_dom::logging::console_log;
        use tokio::task::JoinSet;

        let entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(_) => return Err("Failed to read assets directory"),
        };

        let mut set = JoinSet::new();

        for entry in entries {
            set.spawn(async move {
                let path = entry.ok()?.path();
                if !path.is_file() || path.extension()? != "md" {
                    return None;
                }
                let asset = Asset::try_from(path).ok()?;
                Some(asset)
            });
        }

        let assets = BTreeMap::from_iter(
            set.join_all()
                .await
                .into_iter()
                .filter_map(|asset| asset.map(|asset| (asset.slug.clone(), asset))),
        );

        console_log(&format!("Found {} assets", assets.len()));

        Ok(Catalog { assets })
    }
}
