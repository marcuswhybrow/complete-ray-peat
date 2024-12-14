use crate::asset::{Asset, Cached};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub assets: BTreeMap<String, Asset<Cached>>,
}

impl Catalog {
    pub fn assets(&'static self) -> Vec<&'static Asset<Cached>> {
        self.assets.values().collect()
    }
}

impl Catalog {
    #[cfg(feature = "ssr")]
    pub async fn new(path: &Path) -> Result<Self, &'static str> {
        use indicatif::ProgressBar;
        use tokio::{fs, task::JoinSet};

        let mut entries = match fs::read_dir(path).await {
            Ok(entries) => entries,
            Err(_) => return Err("Failed to read assets directory"),
        };

        let mut paths = vec![];
        while let Some(entry) = entries.next_entry().await.unwrap() {
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "md" {
                paths.push(path.to_path_buf());
            }
        }

        let progress = ProgressBar::new(paths.len() as u64);

        let mut set = JoinSet::new();
        for path in paths {
            set.spawn(async move { Asset::new_cached(path).await.unwrap() });
        }

        let mut assets = BTreeMap::new();
        while let Some(result) = set.join_next().await {
            let asset = result.expect("Thread did not return a value");
            assets.insert(asset.slug.clone(), asset);
            progress.inc(1);
        }

        progress.finish_and_clear();

        Ok(Catalog { assets })
    }
}
