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
