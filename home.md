# Home

## Main

Hi


### Rust code

```rust
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use ui::App;


fn main() {

    // Urls are relative to your Cargo.toml file
    // const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

```


### Nix code
---
formatters:
    nix: nixpkgs-fmt
---
```nix
{
  lib,
  stdenv,
  fetchFromGitHub,
  cairo,
  libxkbcommon,
  meson,
  ninja,
  pkg-config,
  scdoc,
  wayland,
  wayland-protocols,
  wayland-scanner,
  buildDocs ? true,
}:

stdenv.mkDerivation (finalAttrs: {
  pname = "slurp";
  version = "1.5.0";

  src = fetchFromGitHub {
    owner = "emersion";
    repo = "slurp";
    rev = "v${finalAttrs.version}";
    hash = "sha256-2M8f3kN6tihwKlUCp2Qowv5xD6Ufb71AURXqwQShlXI=";
  };

  depsBuildBuild = [ pkg-config ];

  nativeBuildInputs = [
    meson
    ninja
    pkg-config
    wayland-scanner
  ] ++ lib.optional buildDocs scdoc;

  buildInputs = [
    cairo
    libxkbcommon
    wayland
    wayland-protocols
  ];

  strictDeps = true;

  mesonFlags = [ (lib.mesonEnable "man-pages" buildDocs) ];

  meta = {
    changelog = "https://github.com/emersion/slurp/releases/tag/v${finalAttrs.version}";
    description = "Select a region in a Wayland compositor";
    platforms = lib.platforms.linux;
    homepage = "https://github.com/emersion/slurp";
    license = lib.licenses.mit;
    mainProgram = "slurp";
    maintainers = with lib.maintainers; [ nickcao ];
  };
})
```
