use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub engine: Option<String>,
    pub iwad: Option<String>,
    pub mods: Vec<String>,
    pub args: Vec<String>,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            engine: None,
            iwad: None,
            mods: vec![],
            args: vec![],
        }
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let text = toml::to_string_pretty(self)?;
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, text)?;
        std::fs::rename(tmp, path)?;
        Ok(())
    }
}

pub mod pathbuf {
    use std::path::PathBuf;
    use crate::interface::ctx::Context;

    pub fn profile_path(ctx: &Context, name: &str) -> PathBuf {
        ctx.profiles_dir.join(format!("{name}.toml"))
    }
}
