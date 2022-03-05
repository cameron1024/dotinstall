# dotinstall

A library to build dotfile installers.

Disclaimer: I created this pretty much entirely for personal use, so features are things that I personally find useful. If you have a use case that isn't covered, file an issue or open a PR and I'll be happy to add it

## Example

There are some things that I want to have whenever I move to a new system. For example, some packages from the system's package manager, some tools via `cargo install`, and I want certain directory structures to exist.

`dotinstall` provides a DSL for automating some of these tasks in a (slightly) platform-independent way:
```rust
use dotinstall::{installer, Context};

fn main() {
  // create an `Installer`, defined below
  let installer = Installer::new();  

  // `Context` contains various metadata about the local system
  let ctx = Context::init();

  installer.install(&ctx).unwrap();
}

installer! {
  packages {
    "alacritty",
    "neovim",
    "build-essential" => {
      pacman = "base-devel"  // on arch systems, use the name `base-devel` instead
    }
  };

  cargo {
    "ripgrep",
    "exa",
  };

  ensure {
    "~/.local/bin"  // ensure this directory exists
  };

  exec "post-install.sh";  // arbitrary script
}
```

This will do largely what you'd expect, in the order it is written:
 - installs `alacritty`, `neovim` and `build-essential` (or `base-devel` on Arch) via `apt`/`pacman`
 - `cargo install ripgrep` and `cargo install exa`
 - creates the directory `~/.local/bin`
 - runs `post-install.sh`

I plan to make this more feature rich and extensible as/when I have time

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
