use std::fmt::Display;

use crate::{util::run, Installable};

pub const PACKAGE_MANAGERS: [PackageManager; 3] = [
    PackageManager::Pacman,
    PackageManager::Brew,
    PackageManager::Apt,
];

#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Pacman,
    Brew,
    Apt,
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.executable())
    }
}

impl PackageManager {
    pub fn executable(&self) -> &'static str {
        match self {
            Self::Pacman => "pacman",
            Self::Brew => "brew",
            Self::Apt => "apt",
        }
    }

    /// Return a tuple containing the command string to be executed, and the name of the package
    /// that is to be installed (for logging purposes)
    fn install_cmd(
        &self,
        Package {
            name,
            pacman,
            brew,
            apt,
        }: &Package,
    ) -> (duct::Expression, String) {
        let pacman = pacman.as_ref().unwrap_or(name);
        let brew = brew.as_ref().unwrap_or(name);
        let apt = apt.as_ref().unwrap_or(name);
        match self {
            Self::Pacman => (
                cmd!("pacman", "-Syu", pacman, "--noconfirm"),
                pacman.to_string(),
            ),
            Self::Brew => (cmd!("brew", "install", brew), brew.to_string()),
            Self::Apt => (cmd!("apt", "install", apt), apt.to_string()),
        }
    }
}

/// A package to be installed by the system's package manager
pub struct Package {
    /// The name of the package
    pub name: String,

    /// An optional override for the name of the package when installed by pacman
    pub pacman: Option<String>,

    /// An optional override for the name of the package when installed by brew
    pub brew: Option<String>,

    /// An optional override for the name of the package when installed by apt
    pub apt: Option<String>,
}

impl Package {
    pub fn new(name: String) -> Self {
        Self {
            name,
            pacman: None,
            brew: None,
            apt: None,
        }
    }
}

impl Installable for Package {
    fn install(&self, ctx: &crate::Context) -> Result<(), Box<dyn std::error::Error>> {
        let (cmd, name) = ctx.package_manager.install_cmd(self);
        info!("installing `{}` via  {}", name, ctx.package_manager);
        run(cmd)
    }
}
