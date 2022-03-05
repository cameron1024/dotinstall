use std::error::Error;

use crate::{Installable, util::run, Context};


/// A list of crates to be installed via `cargo install`
#[derive(Debug, Clone)]
pub struct CargoInstall {
    pub crates: Vec<String>,
}

impl Installable for CargoInstall {
    fn install(&self, _ctx: &Context) -> Result<(), Box<dyn Error>> {
        for krate in &self.crates {
            info!("`cargo install`ing {}", krate);
            run(cmd!("cargo", "install", krate))?;
        }

        Ok(())
    }
}
