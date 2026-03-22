# zip-install

`zip-install` is a small GUI for installing portable applications with automatic update detection.

<!-- gifs here -->


## Features

- All portable apps in one place instead of scattered across folders
- One-click install from `.zip` archives and standalone `.exe` files
- Right-click context menu for `.zip` and `.exe`
- Automatically matches to already installed apps
- Desktop and Start Menu shortcuts

<!-- ## Installation

Download the latest binary for your platform from the [Releases](../../releases) page.

On first launch, `zip-install` presents a setup screen. The **Install** button will register the utility in the context menu. -->


## How It Works

When launched with a file argument (via context menu or drag-and-drop), `zip-install` opens the package and scans for executable candidates.

It then computes a Jaccard similarity index between the archive's file structure and each previously installed app. If a match exceeds the configured threshold (default 0.8), it offers to update the existing installation; otherwise it does a new install.

Installations are stored under `%LocalAppData%/ZipInstall/Packages/<uuid>/` (Windows) or `~/.local/share/ZipInstall/Packages/<uuid>/` (Linux), with a TOML index tracking installed apps and a separate config file for user preferences.


## Build

```
git clone https://github.com/danielpancake/zip-install.git
cd zip-install
cargo build --release
```

The binary will be at `target/release/` folder.


## License

[Apache License 2.0](LICENSE)
