<!-- date: 2023-06-26 14:30 -->
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
### Python code


```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sun Oct 19 09:20:44 2025

@author: hamid
"""

#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Jun  5 17:41:01 2023

@author: hamid
"""

import matplotlib.pyplot as plt
import numpy as np
from scipy import stats
import seaborn as sns

mu_params = [-1, 0, 1]
sd_params = [0.5, 1, 1.5 ]
x = np.linspace(-7, 7, 100)

f, ax = plt.subplots(len(mu_params), len(sd_params), sharex=True, sharey=True)
for i in range(3):
    for j in range(3):
            mu = mu_params[i]
            sd = sd_params[j]
            y = stats.norm(mu, sd).pdf(x)
            ax[i,j].plot(x, y)
            # ax[i,j].plot(0, 0,label="$\\mu$ = {:3.2f}\n$\\sigma$ = {:3.2f}".format(mu, sd), alpha=0)
            # ax[i,j].legend(fontsize=12)
# ax[2,1].set_xlabel('$x$', fontsize=16)
# ax[1,0].set_ylabel('$pdf(x)$', fontsize=16)
plt.tight_layout()

```
