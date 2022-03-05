fn main() {
    let installer = Installer::new();
    let context = dotinstall::Context::init();
    
    // don't actually run it, it needs root
    let _run = || {
        installer.install(&context).unwrap();
    };
}

dotinstall::installer! {
    ensure {
        "~/.local/bin"
    };

    packages {
        "unzip",
        "build-essential" => {
            pacman = "base-devel"
        }
    };

    exec "some_script.sh";

    cargo {
        "ripgrep",
        "exa",
    };

    symlinks {
        "foo" => "bar",
    };

}


