use std::{collections::HashMap, ffi::OsString, os::unix::fs::symlink, path::PathBuf};

use crate::{Context, Installable};

/// A set of symlinks to create
///
/// If the link path already contains a symlink, it will be deleted
#[derive(Debug, Clone)]
pub struct Symlinks {
    links: HashMap<OsString, OsString>,
}

impl Installable for Symlinks {
    fn install(&self, _ctx: &Context) -> Result<(), Box<dyn std::error::Error>> {
        for (original, link) in &self.links {
            if PathBuf::from(link).is_symlink() {
                std::fs::remove_file(link)?;
            }
            symlink(original, link)?;
        }

        Ok(())
    }
}
