use std::path::PathBuf;

use crate::{Installable, util::run};

pub struct Script {
    pub path: PathBuf,
}

impl Installable for Script {
    fn install(&self, _: &crate::Context) -> Result<(), Box<dyn std::error::Error>> {
        run(cmd!(&self.path))
    }
}
