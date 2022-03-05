use std::error::Error;

use package::PACKAGE_MANAGERS; use util::has_binary;
pub mod cargo;
pub mod dirs;
pub mod package;
pub mod symlinks;
pub mod script;
pub mod util;

pub use cargo::CargoInstall;
pub use dirs::EnsureDirs;
pub use package::{Package, PackageManager};
pub use symlinks::Symlinks;
pub use script::Script;

pub use dotinstall_macro::installer;


#[macro_use]
extern crate tracing;

#[macro_use]
extern crate duct;

/// Context regarding the system this is being run on
pub struct Context {
    package_manager: PackageManager,
}

impl Context {
    pub fn init() -> Self {
        let package_manager = PACKAGE_MANAGERS
            .iter()
            .find(|p| has_binary(p.executable()).unwrap())
            .expect("No suitable package managers found");

        Self { package_manager: *package_manager }
    }
}

pub trait Installable {
    fn install(&self, ctx: &Context) -> Result<(), Box<dyn Error>>;
}

impl Installable for Vec<Package> {
    fn install(&self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        for package in self {
            package.install(ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("testing/ui/pass/*.rs");
    t.compile_fail("testing/ui/fail/*.rs");
}
