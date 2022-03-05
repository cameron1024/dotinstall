use std::{ffi::OsString, error::Error, path::Path, env};

use crate::{Installable, Context};


/// A set of directories to be created
#[derive(Debug, Clone)]
pub struct EnsureDirs {
    pub absolute_paths: Vec<OsString>,
    pub home_paths: Vec<OsString>,
}

impl Installable for EnsureDirs {
    fn install(&self, _ctx: &Context) -> Result<(), Box<dyn Error>> {
        for path in &self.absolute_paths {
            info!("creating directory: {}", path.to_string_lossy());
            std::fs::create_dir_all(path)?;
        }

        for path in &self.home_paths {
            info!("creating directory: ~/{}", path.to_string_lossy());
            let path = Path::new(&env::var("HOME")?).join(path);
            std::fs::create_dir_all(path)?;
        }

        Ok(())
    }
}

