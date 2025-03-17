├── .changes
    ├── config.json
    └── readme.md
├── .github
    └── workflows
    │   ├── audit.yml
    │   ├── clippy-fmt.yml
    │   ├── covector-status.yml
    │   ├── covector-version-or-publish.yml
    │   └── test.yml
├── .gitignore
├── CHANGELOG.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── LICENSE.spdx
├── README.md
├── examples
    ├── egui.rs
    ├── icon.png
    ├── tao.rs
    └── winit.rs
├── renovate.json
└── src
    ├── counter.rs
    ├── error.rs
    ├── icon.rs
    ├── lib.rs
    ├── platform_impl
        ├── gtk
        │   ├── icon.rs
        │   └── mod.rs
        ├── macos
        │   ├── icon.rs
        │   └── mod.rs
        ├── mod.rs
        └── windows
        │   ├── icon.rs
        │   ├── mod.rs
        │   └── util.rs
    └── tray_icon_id.rs


/.changes/config.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "gitSiteUrl": "https://www.github.com/tauri-apps/tray-icon",
 3 |   "timeout": 3600000,
 4 |   "pkgManagers": {
 5 |     "rust": {
 6 |       "version": true,
 7 |       "getPublishedVersion": "cargo search ${ pkg.pkg } --limit 1 | sed -nE 's/^[^\"]*\"//; s/\".*//1p' -",
 8 |       "prepublish": [
 9 |         "sudo apt-get update",
10 |         "sudo apt-get install -y libgtk-3-dev libxdo-dev libayatana-appindicator3-dev"
11 |       ],
12 |       "publish": [
13 |         {
14 |           "command": "cargo package --no-verify",
15 |           "dryRunCommand": true
16 |         },
17 |         {
18 |           "command": "echo '<details>\n<summary><em><h4>Cargo Publish</h4></em></summary>\n\n```'",
19 |           "dryRunCommand": true,
20 |           "pipe": true
21 |         },
22 |         {
23 |           "command": "cargo publish",
24 |           "dryRunCommand": "cargo publish --dry-run",
25 |           "pipe": true
26 |         },
27 |         {
28 |           "command": "echo '```\n\n</details>\n'",
29 |           "dryRunCommand": true,
30 |           "pipe": true
31 |         }
32 |       ],
33 |       "postpublish": [
34 |         "git tag ${ pkg.pkg }-v${ pkgFile.versionMajor } -f",
35 |         "git tag ${ pkg.pkg }-v${ pkgFile.versionMajor }.${ pkgFile.versionMinor } -f",
36 |         "git push --tags -f"
37 |       ]
38 |     }
39 |   },
40 |   "packages": {
41 |     "tray-icon": {
42 |       "path": ".",
43 |       "manager": "rust",
44 |       "assets": [
45 |         {
46 |           "path": "${ pkg.path }/target/package/tray-icon-${ pkgFile.version }.crate",
47 |           "name": "${ pkg.pkg }-${ pkgFile.version }.crate"
48 |         }
49 |       ]
50 |     }
51 |   }
52 | }


--------------------------------------------------------------------------------
/.changes/readme.md:
--------------------------------------------------------------------------------
 1 | # Changes
 2 | 
 3 | ##### via https://github.com/jbolda/covector
 4 | 
 5 | As you create PRs and make changes that require a version bump, please add a new markdown file in this folder. You do not note the version _number_, but rather the type of bump that you expect: major, minor, or patch. The filename is not important, as long as it is a `.md`, but we recommend it represents the overall change for our sanity.
 6 | 
 7 | When you select the version bump required, you do _not_ need to consider dependencies. Only note the package with the actual change, and any packages that depend on that package will be bumped automatically in the process.
 8 | 
 9 | Use the following format:
10 | 
11 | ```md
12 | ---
13 | "tray-icon": patch
14 | ---
15 | 
16 | Change summary goes here
17 | ```
18 | 
19 | Summaries do not have a specific character limit, but are text only. These summaries are used within the (future implementation of) changelogs. They will give context to the change and also point back to the original PR if more details and context are needed.
20 | 
21 | Changes will be designated as a `major`, `minor` or `patch` as further described in [semver](https://semver.org/).
22 | 
23 | Given a version number MAJOR.MINOR.PATCH, increment the:
24 | 
25 | - MAJOR version when you make incompatible API changes,
26 | - MINOR version when you add functionality in a backwards compatible manner, and
27 | - PATCH version when you make backwards compatible bug fixes.
28 | 
29 | Additional labels for pre-release and build metadata are available as extensions to the MAJOR.MINOR.PATCH format, but will be discussed prior to usage (as extra steps will be necessary in consideration of merging and publishing).
30 | 


--------------------------------------------------------------------------------
/.github/workflows/audit.yml:
--------------------------------------------------------------------------------
 1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | # SPDX-License-Identifier: Apache-2.0
 3 | # SPDX-License-Identifier: MIT
 4 | 
 5 | name: audit
 6 | 
 7 | on:
 8 |   workflow_dispatch:
 9 |   schedule:
10 |     - cron: '0 0 * * *'
11 |   push:
12 |     branches:
13 |       - dev
14 |     paths:
15 |       - 'Cargo.lock'
16 |       - 'Cargo.toml'
17 |   pull_request:
18 |     paths:
19 |       - 'Cargo.lock'
20 |       - 'Cargo.toml'
21 | 
22 | concurrency:
23 |   group: ${{ github.workflow }}-${{ github.ref }}
24 |   cancel-in-progress: true
25 | 
26 | jobs:
27 |   audit:
28 |     runs-on: ubuntu-latest
29 |     steps:
30 |       - uses: actions/checkout@v4
31 |       - uses: rustsec/audit-check@v1
32 |         with:
33 |           token: ${{ secrets.GITHUB_TOKEN }}


--------------------------------------------------------------------------------
/.github/workflows/clippy-fmt.yml:
--------------------------------------------------------------------------------
 1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | # SPDX-License-Identifier: Apache-2.0
 3 | # SPDX-License-Identifier: MIT
 4 | 
 5 | name: clippy & fmt
 6 | 
 7 | on:
 8 |   push:
 9 |     branches:
10 |       - dev
11 |   pull_request:
12 | 
13 | concurrency:
14 |   group: ${{ github.workflow }}-${{ github.ref }}
15 |   cancel-in-progress: true
16 | 
17 | jobs:
18 |   clippy:
19 |     strategy:
20 |       fail-fast: false
21 |       matrix:
22 |         platform: [ubuntu-latest, macos-latest, windows-latest]
23 | 
24 |     runs-on: ${{ matrix.platform }}
25 | 
26 |     steps:
27 |       - uses: actions/checkout@v4
28 |       - name: install system deps
29 |         if: matrix.platform == 'ubuntu-latest'
30 |         run: |
31 |           sudo apt-get update
32 |           sudo apt-get install -y libgtk-3-dev libxdo-dev libayatana-appindicator3-dev
33 | 
34 |       - uses: dtolnay/rust-toolchain@stable
35 |         with:
36 |           components: clippy
37 | 
38 |       - run: cargo clippy --all-targets --all-features -- -D warnings
39 | 
40 |   fmt:
41 |     runs-on: ubuntu-latest
42 |     steps:
43 |       - uses: actions/checkout@v4
44 |       - uses: dtolnay/rust-toolchain@stable
45 |         with:
46 |           components: rustfmt
47 | 
48 |       - run: cargo fmt --all -- --check
49 | 


--------------------------------------------------------------------------------
/.github/workflows/covector-status.yml:
--------------------------------------------------------------------------------
 1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | # SPDX-License-Identifier: Apache-2.0
 3 | # SPDX-License-Identifier: MIT
 4 | 
 5 | name: covector status
 6 | on: [pull_request]
 7 | 
 8 | jobs:
 9 |   covector:
10 |     runs-on: ubuntu-latest
11 | 
12 |     steps:
13 |       - uses: actions/checkout@v4
14 |       - name: covector status
15 |         uses: jbolda/covector/packages/action@covector-v0
16 |         id: covector
17 |         with:
18 |           command: "status"
19 |           token: ${{ secrets.GITHUB_TOKEN }}
20 |           comment: true
21 | 


--------------------------------------------------------------------------------
/.github/workflows/covector-version-or-publish.yml:
--------------------------------------------------------------------------------
 1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | # SPDX-License-Identifier: Apache-2.0
 3 | # SPDX-License-Identifier: MIT
 4 | 
 5 | name: covector version or publish
 6 | 
 7 | on:
 8 |   push:
 9 |     branches:
10 |       - dev
11 | 
12 | jobs:
13 |   version-or-publish:
14 |     runs-on: ubuntu-latest
15 |     timeout-minutes: 65
16 |     outputs:
17 |       change: ${{ steps.covector.outputs.change }}
18 |       commandRan: ${{ steps.covector.outputs.commandRan }}
19 |       successfulPublish: ${{ steps.covector.outputs.successfulPublish }}
20 | 
21 |     steps:
22 |       - uses: actions/checkout@v4
23 |         with:
24 |           fetch-depth: 0
25 |       - name: cargo login
26 |         run: cargo login ${{ secrets.ORG_CRATES_IO_TOKEN  }}
27 |         
28 |       - name: git config
29 |         run: |
30 |           git config --global user.name "${{ github.event.pusher.name }}"
31 |           git config --global user.email "${{ github.event.pusher.email }}"
32 | 
33 |       - name: covector version or publish (publish when no change files present)
34 |         uses: jbolda/covector/packages/action@covector-v0
35 |         id: covector
36 |         env:
37 |           NODE_AUTH_TOKEN: ${{ secrets.ORG_NPM_TOKEN }}
38 |         with:
39 |           token: ${{ secrets.GITHUB_TOKEN }}
40 |           command: 'version-or-publish'
41 |           createRelease: true
42 |           recognizeContributors: true
43 | 
44 |       - name: Create Pull Request With Versions Bumped
45 |         if: steps.covector.outputs.commandRan == 'version'
46 |         uses: tauri-apps/create-pull-request@v3
47 |         with:
48 |           token: ${{ secrets.GITHUB_TOKEN }}
49 |           title: Apply Version Updates From Current Changes
50 |           commit-message: 'apply version updates'
51 |           labels: 'version updates'
52 |           branch: 'release'
53 |           body: ${{ steps.covector.outputs.change }}
54 | 


--------------------------------------------------------------------------------
/.github/workflows/test.yml:
--------------------------------------------------------------------------------
 1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | # SPDX-License-Identifier: Apache-2.0
 3 | # SPDX-License-Identifier: MIT
 4 | 
 5 | name: test
 6 | 
 7 | on:
 8 |   push:
 9 |     branches:
10 |       - dev
11 |   pull_request:
12 | 
13 | env:
14 |   RUST_BACKTRACE: 1
15 | 
16 | concurrency:
17 |   group: ${{ github.workflow }}-${{ github.ref }}
18 |   cancel-in-progress: true
19 | 
20 | jobs:
21 |   test:
22 |     strategy:
23 |       fail-fast: false
24 |       matrix:
25 |         platform: ["windows-latest", "macos-latest", "ubuntu-latest"]
26 | 
27 |     runs-on: ${{ matrix.platform }}
28 | 
29 |     steps:
30 |       - uses: actions/checkout@v4
31 | 
32 |       - name: install system deps
33 |         if: matrix.platform == 'ubuntu-latest'
34 |         run: |
35 |           sudo apt-get update
36 |           sudo apt-get install -y libgtk-3-dev libxdo-dev libayatana-appindicator3-dev
37 | 
38 |       - uses: dtolnay/rust-toolchain@1.71
39 |       - run: cargo build
40 | 
41 |       - uses: dtolnay/rust-toolchain@stable
42 |       - run: cargo test
43 | 


--------------------------------------------------------------------------------
/.gitignore:
--------------------------------------------------------------------------------
1 | # Copyright 2022-2022 Tauri Programme within The Commons Conservancy
2 | # SPDX-License-Identifier: Apache-2.0
3 | # SPDX-License-Identifier: MIT
4 | 
5 | /target
6 | .vscode/


--------------------------------------------------------------------------------
/CHANGELOG.md:
--------------------------------------------------------------------------------
  1 | # Changelog
  2 | 
  3 | ## \[0.20.0]
  4 | 
  5 | - [`e94976b`](https://www.github.com/tauri-apps/tray-icon/commit/e94976bb48bbe97ac5ab215c6da7c7ca746a5c8a) ([#237](https://www.github.com/tauri-apps/tray-icon/pull/237) by [@renovate](https://www.github.com/tauri-apps/tray-icon/../../renovate)) Updated `muda` to 0.16.0
  6 | 
  7 | ## \[0.19.3]
  8 | 
  9 | - [`d6fee6f`](https://www.github.com/tauri-apps/tray-icon/commit/d6fee6ff627e6ff08bf8bf9a2880197d0f07271e) ([#226](https://www.github.com/tauri-apps/tray-icon/pull/226) by [@madsmtm](https://www.github.com/tauri-apps/tray-icon/../../madsmtm)) Update `objc2` to v0.6.
 10 | 
 11 | ## \[0.19.2]
 12 | 
 13 | - [`1f0e1f8`](https://www.github.com/tauri-apps/tray-icon/commit/1f0e1f8f0d0ad65cd0ab549655fac26c0f524de6) ([#204](https://www.github.com/tauri-apps/tray-icon/pull/204) by [@mrexox](https://www.github.com/tauri-apps/tray-icon/../../mrexox)) Add `set_icon_with_as_template` method to update icon and `is_template` property, preventing glitchy effects during icon animation on macOS.
 14 | 
 15 | ## \[0.19.1]
 16 | 
 17 | - [`19e67de`](https://www.github.com/tauri-apps/tray-icon/commit/19e67de6ff0b66241fddde507eb82e96781b6c36) ([#199](https://www.github.com/tauri-apps/tray-icon/pull/199) by [@Klemen2](https://www.github.com/tauri-apps/tray-icon/../../Klemen2)) Implemented `TrayIcon::set_show_menu_on_left_click` on windows
 18 | 
 19 | ## \[0.19.0]
 20 | 
 21 | - [`bf5cec4`](https://www.github.com/tauri-apps/tray-icon/commit/bf5cec4c3242534cb068978bb27e37551bcb63f9) ([#196](https://www.github.com/tauri-apps/tray-icon/pull/196) by [@amrbashir](https://www.github.com/tauri-apps/tray-icon/../../amrbashir)) **Breaking change** Changed `serde` derive implementation for `TrayIconEvent` to use `serde(tag = "type")` and `rename_all = "camelCase"` on variants so the expected JSON serialization would look like this
 22 | 
 23 |   ```json
 24 |   {
 25 |     "type": "Click",
 26 |     "button": "Left",
 27 |     "buttonState": "Down",
 28 |     "id": "some id",
 29 |     "position": {
 30 |       "x": 0,
 31 |       "y": 0
 32 |     },
 33 |     "rect": {
 34 |       "size": {
 35 |         "width": 0,
 36 |         "height": 0
 37 |       },
 38 |       "position": {
 39 |         "x": 0,
 40 |         "y": 0
 41 |       }
 42 |     }
 43 |   }
 44 |   ```
 45 | 
 46 | ## \[0.18.0]
 47 | 
 48 | - [`c63733c`](https://www.github.com/tauri-apps/tray-icon/commit/c63733c45f5fd34bc16c9310cb4f1a063e5e21c7) ([#193](https://www.github.com/tauri-apps/tray-icon/pull/193) by [@amrbashir](https://www.github.com/tauri-apps/tray-icon/../../amrbashir)) Update `muda` crate to `0.15`
 49 | 
 50 | ## \[0.17.0]
 51 | 
 52 | - [`e711c1f`](https://www.github.com/tauri-apps/tray-icon/commit/e711c1f5e6aef3052694ee8da33b2de624093ec8) ([#189](https://www.github.com/tauri-apps/tray-icon/pull/189) by [@htngr](https://www.github.com/tauri-apps/tray-icon/../../htngr)) Implemented `TrayIcon::with_menu_on_left_click` on windows
 53 | 
 54 | ## \[0.16.0]
 55 | 
 56 | - [`20819e4`](https://www.github.com/tauri-apps/tray-icon/commit/20819e445e23f1d3749d03534eba9641404a8db6) ([#187](https://www.github.com/tauri-apps/tray-icon/pull/187) by [@amrbashir](https://www.github.com/tauri-apps/tray-icon/../../amrbashir)) Removed `button_state` field in `TrayIconEvent::DoubleClick` variant.
 57 | 
 58 | ## \[0.15.2]
 59 | 
 60 | - [`a1303c3`](https://www.github.com/tauri-apps/tray-icon/commit/a1303c39020befc977d818d7cce1a039d416ccd2) ([#185](https://www.github.com/tauri-apps/tray-icon/pull/185) by [@amrbashir](https://www.github.com/tauri-apps/tray-icon/../../amrbashir)) On Windows, Add and emit `DoubleClick` variant for `TrayIconEvent`.
 61 | 
 62 | ## \[0.15.1]
 63 | 
 64 | - [`5a381ff`](https://www.github.com/tauri-apps/tray-icon/commit/5a381ffd3d0f8ab8b1a88a95e557c5837c17a1b7) Update `core-foundation` crate to `0.10` and `core-graphics` to `0.24`
 65 | 
 66 | ## \[0.15.0]
 67 | 
 68 | - [`d407869`](https://www.github.com/tauri-apps/tray-icon/commit/d4078696edba67b0ab42cef67e6a421a0332c96f) ([#172](https://www.github.com/tauri-apps/tray-icon/pull/172)) Added a new variant `NotMainThread` to the `Error` enum, which is emitted on macOS when trying to create tray icons from a thread that is not the main thread.
 69 | - [`8857b7d`](https://www.github.com/tauri-apps/tray-icon/commit/8857b7dd12ba523532ac7a58bf08302316af13c8) Updated `muda` crate to `0.14`
 70 | - [`d407869`](https://www.github.com/tauri-apps/tray-icon/commit/d4078696edba67b0ab42cef67e6a421a0332c96f) ([#172](https://www.github.com/tauri-apps/tray-icon/pull/172)) Rewrite the internals of the crate to use `objc2` instead of `objc`.
 71 | 
 72 |   This should have no user-facing changes, other than improved memory safety, and less leaking.
 73 | 
 74 | ## \[0.14.3]
 75 | 
 76 | - [`e257d6b`](https://www.github.com/tauri-apps/tray-icon/commit/e257d6bf510b34707d48964a2914ee5c91b13570) ([#169](https://www.github.com/tauri-apps/tray-icon/pull/169)) On Windows, fix `Enter` event emitted only once and never emitted again.
 77 | 
 78 | ## \[0.14.2]
 79 | 
 80 | - [`f1f3adb`](https://www.github.com/tauri-apps/tray-icon/commit/f1f3adb5ec726335226ab8ec1d8c6c41012cb9c5)([#166](https://www.github.com/tauri-apps/tray-icon/pull/166)) Switch from `dirs_next` to `dirs` as `dirs_next` is now unmaintained while `dirs` is
 81 | 
 82 | ## \[0.14.1]
 83 | 
 84 | - [`b491c98`](https://www.github.com/tauri-apps/tray-icon/commit/b491c9886619d3a26876476b078d99a0ae788918)([#164](https://www.github.com/tauri-apps/tray-icon/pull/164)) Fix tray icon rect scaled by dpi on Windows
 85 | 
 86 | ## \[0.14.0]
 87 | 
 88 | - [`587292b`](https://www.github.com/tauri-apps/tray-icon/commit/587292b2e7bfbebdd2677c51b34c6362730d5111)([#161](https://www.github.com/tauri-apps/tray-icon/pull/161)) This release contains **breaking change** to the event structs in order to be able to add new `Enter`, `Move` and `Leave` events:
 89 | 
 90 |   - Changed `TrayIconEvent` to be an enum instead of a struct.
 91 |   - Added new events for when the mouse enters, moves or leaves the tray icon region.
 92 |   - Removed `ClickType` enum and replaced it with `MouseButton` enum.
 93 |   - Added `MouseButtonState` enum.
 94 | 
 95 | ## \[0.13.5]
 96 | 
 97 | - [`a1cd50e`](https://www.github.com/tauri-apps/tray-icon/commit/a1cd50e53021474ad87cdf2e269acfb56d36cc14)([#145](https://www.github.com/tauri-apps/tray-icon/pull/145)) Fix tray icon gets blurry after changing dpi on Windows
 98 | - [`ad317c7`](https://www.github.com/tauri-apps/tray-icon/commit/ad317c7dab271145c641f0c4c22e283bb2aa0c91)([#150](https://www.github.com/tauri-apps/tray-icon/pull/150)) On macOS, fix tray event position not scaled properly.
 99 | - [`6d099ee`](https://www.github.com/tauri-apps/tray-icon/commit/6d099ee2a4c455561f4c6f86ea995df267469eca)([#149](https://www.github.com/tauri-apps/tray-icon/pull/149)) On macOS, fix the `y` position of the tray icon to be top-left not bottom-left of the icon.
100 | - [`599bb8f`](https://www.github.com/tauri-apps/tray-icon/commit/599bb8f55546d674892a80051766d36656975e86)([#147](https://www.github.com/tauri-apps/tray-icon/pull/147)) Add `TrayIcon::rect` method to retrieve the tray icon rectangle on Windows and macOS.
101 | 
102 | ## \[0.13.4]
103 | 
104 | - [`6b09b8e`](https://www.github.com/tauri-apps/tray-icon/commit/6b09b8e920e79d7768c3a55324431cbd0acadb27)([#136](https://www.github.com/tauri-apps/tray-icon/pull/136)) Add `Icon::from_resource_name` to support icon resource without a ordinal id on Windows
105 | 
106 | ## \[0.13.3]
107 | 
108 | - [`646f56c`](https://www.github.com/tauri-apps/tray-icon/commit/646f56cb6786377b8dbae1e742bb94e7b6f1bb09)([#138](https://www.github.com/tauri-apps/tray-icon/pull/138)) Fix unexpected crashes on I/O or Png encoding errors on macOS and Linux.
109 | 
110 | ## \[0.13.2]
111 | 
112 | - [`c368bbc`](https://www.github.com/tauri-apps/tray-icon/commit/c368bbc6a24b24767c902508651d856413039108)([#134](https://www.github.com/tauri-apps/tray-icon/pull/134)) Fix incorrect icon size reported in events on macOS
113 | 
114 | ## \[0.13.1]
115 | 
116 | - [`784e01e`](https://www.github.com/tauri-apps/tray-icon/commit/784e01e5b4392a39fbec47f17cdcbee7f27af2bc)([#130](https://www.github.com/tauri-apps/tray-icon/pull/130)) On macOS, reset the tray icon when using `setIconAsTemplate` to avoid artifacts.
117 | 
118 | ## \[0.13.0]
119 | 
120 | - [`63abc69`](https://www.github.com/tauri-apps/tray-icon/commit/63abc69affffdd2849d3d42178d76b9bf1ea994a)([#127](https://www.github.com/tauri-apps/tray-icon/pull/127)) Update `muda` dependency to `0.13`
121 | - [`63abc69`](https://www.github.com/tauri-apps/tray-icon/commit/63abc69affffdd2849d3d42178d76b9bf1ea994a)([#127](https://www.github.com/tauri-apps/tray-icon/pull/127)) Added `dpi` module and changed position and sizes in `TrayIconEvent` to use the new `dpi` module:
122 | 
123 |   - Removed `TrayIconEvent.x` and `TrayIconEvent.y` and replaced with `TrayIconEvent.position`
124 |   - Replaced `Rectangle` type with `Rect` which has just two fields `position` and `size`.
125 | 
126 | ## \[0.12.0]
127 | 
128 | - [`91a5bf6`](https://www.github.com/tauri-apps/tray-icon/commit/91a5bf65d7e3895e9f2eedf4e7ffaf7cc9d082ad)([#119](https://www.github.com/tauri-apps/tray-icon/pull/119)) Updated `muda` dependency to `0.12`
129 | 
130 | ## \[0.11.3]
131 | 
132 | - [`5407f14`](https://www.github.com/tauri-apps/tray-icon/commit/5407f140e12aa83984f6a5402ab99e70a4d4f82c)([#114](https://www.github.com/tauri-apps/tray-icon/pull/114)) On Linux, fix `TrayIcon::set_visible` incorrect inverted behavior.
133 | 
134 | ## \[0.11.2]
135 | 
136 | - [`ca3bed5`](https://www.github.com/tauri-apps/tray-icon/commit/ca3bed51b5d6e8b7e04429f8f90a2d514393b034)([#109](https://www.github.com/tauri-apps/tray-icon/pull/109)) On Windows, add `Icon::from_handle`
137 | 
138 | ## \[0.11.1]
139 | 
140 | - [`6382ea5`](https://www.github.com/tauri-apps/tray-icon/commit/6382ea5b47813ce1546dff6e8a69ca053dc6f145)([#103](https://www.github.com/tauri-apps/tray-icon/pull/103)) On Linux, fix tray menu failing to show.
141 | 
142 | ## \[0.11.0]
143 | 
144 | - [`6e8374a`](https://www.github.com/tauri-apps/tray-icon/commit/6e8374a81a2e84bf38c8678085986e569e517e76) Update `muda` crate to `0.11`
145 | 
146 | ## \[0.10.0]
147 | 
148 | - [`8463328`](https://www.github.com/tauri-apps/tray-icon/commit/84633285a0b465fe4c261ff0c7be035ce7615715)([#92](https://www.github.com/tauri-apps/tray-icon/pull/92)) Upgraded `gtk` to 0.18 and bumped MSRV to 1.70.0.
149 | 
150 | ## \[0.9.0]
151 | 
152 | - [`32b3523`](https://www.github.com/tauri-apps/tray-icon/commit/32b352371b6da730abbb024730015492f87205c0) Update `muda` crate to `0.9`
153 | 
154 | ## \[0.8.3]
155 | 
156 | - [`75fed4a`](https://www.github.com/tauri-apps/tray-icon/commit/75fed4aeca82c5614777865a9f6fa2d4457f47a1) Derive `serde` for more types.
157 | 
158 | ## \[0.8.2]
159 | 
160 | - [`cd6fb13`](https://www.github.com/tauri-apps/tray-icon/commit/cd6fb1300e2b2bf78781777de45302c98cfcabd4)([#80](https://www.github.com/tauri-apps/tray-icon/pull/80)) Add `PartialEq<&str> for &TrayIconId` and `PartialEq<String> for &TrayIconId` implementations. Also add a blanket `From<T> for TrayIconId` where `T: ToString` implementation.
161 | 
162 | ## \[0.8.1]
163 | 
164 | - [`0cf36ad`](https://www.github.com/tauri-apps/tray-icon/commit/0cf36ad6afd1ddd93b7087e8eb4475410fb9be8a)([#77](https://www.github.com/tauri-apps/tray-icon/pull/77)) Add `TrayIconId::new` convenience method.
165 | 
166 | ## \[0.8.0]
167 | 
168 | - [`95c1be8`](https://www.github.com/tauri-apps/tray-icon/commit/95c1be8a459f2ef146ccaccfe858c427678613af)([#75](https://www.github.com/tauri-apps/tray-icon/pull/75)) Th `icon` module has been removed and instead its types are exported from crate root.
169 | - [`95c1be8`](https://www.github.com/tauri-apps/tray-icon/commit/95c1be8a459f2ef146ccaccfe858c427678613af)([#75](https://www.github.com/tauri-apps/tray-icon/pull/75)) Update to `muda@0.8`
170 | - [`f93b57d`](https://www.github.com/tauri-apps/tray-icon/commit/f93b57d08a84a8c7ff7f9035f8cc73a3e48e90b9) Add `TrayIconId` struct an changed all `.id()` methods to return `TrayIconId` instead of a u32.
171 | - [`95c1be8`](https://www.github.com/tauri-apps/tray-icon/commit/95c1be8a459f2ef146ccaccfe858c427678613af)([#75](https://www.github.com/tauri-apps/tray-icon/pull/75)) Changed the order of arguments for `TrayIcon::with_id` function to take the `id` as the first argument instead of the second.
172 | 
173 | ## \[0.7.7]
174 | 
175 | - [`197f431`](https://www.github.com/tauri-apps/tray-icon/commit/197f43161cd1806fcae15b19b4f8335d9b3492b6)([#73](https://www.github.com/tauri-apps/tray-icon/pull/73)) Always highlight tray icon on click on macOS.
176 | 
177 | ## \[0.7.6]
178 | 
179 | - [`a458317`](https://www.github.com/tauri-apps/tray-icon/commit/a458317ad1d85ac9477a019f86580a14d4082c7f)([#71](https://www.github.com/tauri-apps/tray-icon/pull/71)) Fixes a crash on mouse events on macOS.
180 | 
181 | ## \[0.7.5]
182 | 
183 | - [`54fc7de`](https://www.github.com/tauri-apps/tray-icon/commit/54fc7de37c3568312b27c30bdd22e830b1f15a3b)([#69](https://www.github.com/tauri-apps/tray-icon/pull/69)) Refactor macOS implementation to fix missing click issues.
184 | 
185 | ## \[0.7.4]
186 | 
187 | - [`71d25a1`](https://www.github.com/tauri-apps/tray-icon/commit/71d25a14ecd2bf0996223127b2fa01ec7f915fce)([#66](https://www.github.com/tauri-apps/tray-icon/pull/66)) On Linux, fix the issue that gtk caches the icon if you use `TrayIcon::set_icon` repeatedly.
188 | 
189 | ## \[0.7.3]
190 | 
191 | - [`c0d16c5`](https://www.github.com/tauri-apps/tray-icon/commit/c0d16c5f90c3e3b4acadee9c5c83bd5e9a3671f6)([#63](https://www.github.com/tauri-apps/tray-icon/pull/63)) Fixes multiple `set_menu` calls not updating the tray menu on macOS.
192 | 
193 | ## \[0.7.2]
194 | 
195 | - [`d0a25b0`](https://www.github.com/tauri-apps/tray-icon/commit/d0a25b0e980d01306344dd4903c1e2e8ef4519ac)([#61](https://www.github.com/tauri-apps/tray-icon/pull/61)) On Windows, fix dropping tray icon caused the whole process to close.
196 | - [`d0a25b0`](https://www.github.com/tauri-apps/tray-icon/commit/d0a25b0e980d01306344dd4903c1e2e8ef4519ac)([#61](https://www.github.com/tauri-apps/tray-icon/pull/61)) On Windows, fix `TrayIcon::set_menu` not firing events for the new menu.
197 | 
198 | ## \[0.7.1]
199 | 
200 | - [`04ed58f`](https://www.github.com/tauri-apps/tray-icon/commit/04ed58f954b113e1f4d52c161231d52c9f5c3546) Remove accidental impl of `Sync` and `Send` for `TrayIcon` where it is not.
201 | 
202 | ## \[0.7.0]
203 | 
204 | - [`d8d6082`](https://www.github.com/tauri-apps/tray-icon/commit/d8d6082c73b1fa6047ead13d228cf7de1ad0d71c)([#57](https://www.github.com/tauri-apps/tray-icon/pull/57)) Add `TrayIconBuilder::id` to access the unique id that will be assigend to the tray icon upon creation.
205 | - [`dd63ef3`](https://www.github.com/tauri-apps/tray-icon/commit/dd63ef3b68c35fc8b8fbc1d59975d8826420ae51) Add `TrayIconEvent::id` method.
206 | - [`3901519`](https://www.github.com/tauri-apps/tray-icon/commit/3901519a48f76b57174b36ce36c7f803dbfb5536) Update to `muda@0.7`
207 | - [`13d448a`](https://www.github.com/tauri-apps/tray-icon/commit/13d448a9ee7c013f0cc13391ea498da93e806551)([#55](https://www.github.com/tauri-apps/tray-icon/pull/55)) Implement `Clone` for `TrayIcon`.
208 | - [`13d448a`](https://www.github.com/tauri-apps/tray-icon/commit/13d448a9ee7c013f0cc13391ea498da93e806551)([#55](https://www.github.com/tauri-apps/tray-icon/pull/55)) - **Breaking change**: `TrayEvent` has been renamed to `TrayIconEvent` for consistency with other struct names.
209 |   - **Breaking change**: `ClickEvent` enum has been renamed to `ClickType` and `TrayEvent`'s `event` field has been renamed to `click_type`
210 | - [`d8d6082`](https://www.github.com/tauri-apps/tray-icon/commit/d8d6082c73b1fa6047ead13d228cf7de1ad0d71c)([#57](https://www.github.com/tauri-apps/tray-icon/pull/57)) Add `TrayIcon::with_id` and `TrayIconBuilder::with_id` to create the tray icon with specified id.
211 | 
212 | ## \[0.6.0]
213 | 
214 | - [`934b927`](https://www.github.com/tauri-apps/tray-icon/commit/934b927e552641c3d319981cdeae84ca901ae399)([#49](https://www.github.com/tauri-apps/tray-icon/pull/49)) Expose `muda` crate feature flags.
215 | 
216 | ## \[0.5.2]
217 | 
218 | - [`9409f36`](https://www.github.com/tauri-apps/tray-icon/commit/9409f36c5293e7fb0c8dd7d0fd74a59472aedfcb)([#46](https://www.github.com/tauri-apps/tray-icon/pull/46)) Fix compiling on `i686-pc-windows-msvc` target
219 | 
220 | ## \[0.5.1]
221 | 
222 | - [`ff7f7bc`](https://www.github.com/tauri-apps/tray-icon/commit/ff7f7bc4400a6f7aa0b5c025c85ab6c4f89e9109)([#40](https://www.github.com/tauri-apps/tray-icon/pull/40)) Fix building for `i686-pc-windows-msvc` target.
223 | 
224 | ## \[0.5.0]
225 | 
226 | - On macOS, fix `set_visible(false)` still occupying space on the system menu bar.
227 |   - [71f9d29](https://www.github.com/tauri-apps/tray-icon/commit/71f9d292dd69b498e57fcebeb76ad6a1365144cd) fix(macos): remove tray icon when `set_visible(false)` ([#37](https://www.github.com/tauri-apps/tray-icon/pull/37)) on 2023-04-18
228 | 
229 | ## \[0.4.4]
230 | 
231 | - Make Rectangle's point fields public.
232 |   - [12a0daf](https://www.github.com/tauri-apps/tray-icon/commit/12a0daf92352fbecddd7b0afdfc0c633232fb15c) Make Rectangle's point fields public. ([#33](https://www.github.com/tauri-apps/tray-icon/pull/33)) on 2023-03-23
233 | 
234 | ## \[0.4.3]
235 | 
236 | - Update documentation.
237 |   - [258b49a](https://www.github.com/tauri-apps/tray-icon/commit/258b49aaebd81b6e4327cca1a1a0a2d9bb64188a) docs: update docs on 2023-02-08
238 |   - [3293885](https://www.github.com/tauri-apps/tray-icon/commit/3293885ae5ef19e14f2fe1baaf4d35719f3b3344) Apply Version Updates From Current Changes ([#22](https://www.github.com/tauri-apps/tray-icon/pull/22)) on 2023-02-08
239 |   - [e58a6ce](https://www.github.com/tauri-apps/tray-icon/commit/e58a6cecfffa63096d459429c5d31ec5b3475a9b) docs: document menu and icon relation on Linux on 2023-02-12
240 | 
241 | ## \[0.4.2]
242 | 
243 | - Update docs.
244 |   - [258b49a](https://www.github.com/tauri-apps/tray-icon/commit/258b49aaebd81b6e4327cca1a1a0a2d9bb64188a) docs: update docs on 2023-02-08
245 | 
246 | ## \[0.4.1]
247 | 
248 | - Bump `muda` to `0.4` and `libappindicator` to `0.8`
249 |   - [d92dd6d](https://www.github.com/tauri-apps/tray-icon/commit/d92dd6dc25d268befe9c14cfe193e1de10bc5717) chore(deps): update deps ([#17](https://www.github.com/tauri-apps/tray-icon/pull/17)) on 2023-01-26
250 | 
251 | ## \[0.4.0]
252 | 
253 | - On macOS and Linux, add `TrayIconBuilder::with_title` and `TrayIcon::set_title` to optionally add a text next to the icon.
254 |   - [6df6fc7](https://www.github.com/tauri-apps/tray-icon/commit/6df6fc78885204be5189b41527a39324851c9671) feat: add `with_title` and `set_title` ([#11](https://www.github.com/tauri-apps/tray-icon/pull/11)) on 2023-01-10
255 |   - [b83f14e](https://www.github.com/tauri-apps/tray-icon/commit/b83f14ee66f9d3801535697c30f54bccc433cce1) chore: adjust change bumps on 2023-01-12
256 | - Add `TrayIcon::set_visible`.
257 |   - [ba4580e](https://www.github.com/tauri-apps/tray-icon/commit/ba4580ec8bd061a76575859b5ead8ec16e3b7817) feat: add `set_visible` ([#14](https://www.github.com/tauri-apps/tray-icon/pull/14)) on 2023-01-12
258 |   - [b83f14e](https://www.github.com/tauri-apps/tray-icon/commit/b83f14ee66f9d3801535697c30f54bccc433cce1) chore: adjust change bumps on 2023-01-12
259 | 
260 | ## \[0.3.0]
261 | 
262 | - Add `TrayEvent::set_event_handler` to set a handler for new tray events.
263 |   - [9247abb](https://www.github.com/tauri-apps/tray-icon/commit/9247abb69ce297096b2c388d67b250509fe44efa) refactor: allow changing the menu event sender ([#8](https://www.github.com/tauri-apps/tray-icon/pull/8)) on 2023-01-03
264 | - Update `muda` to `0.3`.
265 |   - [9247abb](https://www.github.com/tauri-apps/tray-icon/commit/9247abb69ce297096b2c388d67b250509fe44efa) refactor: allow changing the menu event sender ([#8](https://www.github.com/tauri-apps/tray-icon/pull/8)) on 2023-01-03
266 |   - [b64b57e](https://www.github.com/tauri-apps/tray-icon/commit/b64b57ec565dada4bc06201f5b4529725bb0009f) chore: update changefile on 2023-01-03
267 | - **Breaking change** Remove `tray_event_receiver` function, use `TrayEvent::receiver` instead.
268 |   - [9247abb](https://www.github.com/tauri-apps/tray-icon/commit/9247abb69ce297096b2c388d67b250509fe44efa) refactor: allow changing the menu event sender ([#8](https://www.github.com/tauri-apps/tray-icon/pull/8)) on 2023-01-03
269 | 
270 | ## \[0.2.0]
271 | 
272 | - Update `muda` dependency to `0.2`.
273 |   - [aa3aa1e](https://www.github.com/tauri-apps/tray-icon/commit/aa3aa1ec0bdcb48ecf9d17204809802c4e6559fc) chore: add change file on 2022-12-30
274 | 
275 | ## \[0.1.1]
276 | 
277 | - Initial Release.
278 |   - [0651773](https://www.github.com/tauri-apps/tray-icon/commit/0651773ad248d34141fbefc1c65a8889a90a8c9b) chore: prepare for initial release on 2022-12-05
279 | 


--------------------------------------------------------------------------------
/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "tray-icon"
 3 | version = "0.20.0"
 4 | edition = "2021"
 5 | description = "Create tray icons for desktop applications"
 6 | homepage = "https://github.com/tauri-apps/tray-icon"
 7 | repository = "https://github.com/tauri-apps/tray-icon"
 8 | license = "MIT OR Apache-2.0"
 9 | categories = ["gui"]
10 | rust-version = "1.71"
11 | 
12 | [features]
13 | default = ["libxdo"]
14 | libxdo = ["muda/libxdo"]
15 | serde = ["muda/serde", "dep:serde"]
16 | common-controls-v6 = ["muda/common-controls-v6"]
17 | 
18 | [dependencies]
19 | muda = { version = "0.16", default-features = false }
20 | crossbeam-channel = "0.5"
21 | once_cell = "1"
22 | thiserror = "2.0"
23 | serde = { version = "1", optional = true }
24 | 
25 | [target."cfg(target_os = \"windows\")".dependencies.windows-sys]
26 | version = "0.59"
27 | features = [
28 |   "Win32_UI_WindowsAndMessaging",
29 |   "Win32_Foundation",
30 |   "Win32_System_SystemServices",
31 |   "Win32_Graphics_Gdi",
32 |   "Win32_UI_Shell",
33 | ]
34 | 
35 | [target."cfg(target_os = \"linux\")".dependencies]
36 | libappindicator = "0.9"
37 | dirs = "6"
38 | 
39 | [target."cfg(target_os = \"linux\")".dev-dependencies]
40 | gtk = "0.18"
41 | 
42 | [target."cfg(target_os = \"macos\")".dependencies]
43 | objc2 = "0.6.0"
44 | objc2-core-graphics = { version = "0.3.0", default-features = false, features = [
45 |   "std",
46 |   "CGDirectDisplay",
47 | ] }
48 | objc2-core-foundation = { version = "0.3.0", default-features = false, features = [
49 |   "std",
50 |   "CFCGTypes",
51 |   "CFRunLoop",
52 | ] }
53 | objc2-foundation = { version = "0.3.0", default-features = false, features = [
54 |   "std",
55 |   "block2",
56 |   "objc2-core-foundation",
57 |   "NSArray",
58 |   "NSData",
59 |   "NSEnumerator",
60 |   "NSGeometry",
61 |   "NSString",
62 |   "NSThread",
63 | ] }
64 | objc2-app-kit = { version = "0.3.0", default-features = false, features = [
65 |   "std",
66 |   "objc2-core-foundation",
67 |   "NSButton",
68 |   "NSCell",
69 |   "NSControl",
70 |   "NSEvent",
71 |   "NSImage",
72 |   "NSMenu",
73 |   "NSResponder",
74 |   "NSStatusBar",
75 |   "NSStatusBarButton",
76 |   "NSStatusItem",
77 |   "NSTrackingArea",
78 |   "NSView",
79 |   "NSWindow",
80 | ] }
81 | 
82 | [target."cfg(any(target_os = \"linux\", target_os = \"macos\"))".dependencies]
83 | png = "0.17"
84 | 
85 | [dev-dependencies]
86 | winit = "0.30"
87 | tao = "0.31"
88 | image = "0.25"
89 | eframe = "0.30"
90 | serde_json = "1"
91 | 


--------------------------------------------------------------------------------
/LICENSE-APACHE:
--------------------------------------------------------------------------------
  1 |                               Apache License
  2 |                         Version 2.0, January 2004
  3 |                      http://www.apache.org/licenses/
  4 | 
  5 | TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
  6 | 
  7 | 1. Definitions.
  8 | 
  9 |    "License" shall mean the terms and conditions for use, reproduction,
 10 |    and distribution as defined by Sections 1 through 9 of this document.
 11 | 
 12 |    "Licensor" shall mean the copyright owner or entity authorized by
 13 |    the copyright owner that is granting the License.
 14 | 
 15 |    "Legal Entity" shall mean the union of the acting entity and all
 16 |    other entities that control, are controlled by, or are under common
 17 |    control with that entity. For the purposes of this definition,
 18 |    "control" means (i) the power, direct or indirect, to cause the
 19 |    direction or management of such entity, whether by contract or
 20 |    otherwise, or (ii) ownership of fifty percent (50%) or more of the
 21 |    outstanding shares, or (iii) beneficial ownership of such entity.
 22 | 
 23 |    "You" (or "Your") shall mean an individual or Legal Entity
 24 |    exercising permissions granted by this License.
 25 | 
 26 |    "Source" form shall mean the preferred form for making modifications,
 27 |    including but not limited to software source code, documentation
 28 |    source, and configuration files.
 29 | 
 30 |    "Object" form shall mean any form resulting from mechanical
 31 |    transformation or translation of a Source form, including but
 32 |    not limited to compiled object code, generated documentation,
 33 |    and conversions to other media types.
 34 | 
 35 |    "Work" shall mean the work of authorship, whether in Source or
 36 |    Object form, made available under the License, as indicated by a
 37 |    copyright notice that is included in or attached to the work
 38 |    (an example is provided in the Appendix below).
 39 | 
 40 |    "Derivative Works" shall mean any work, whether in Source or Object
 41 |    form, that is based on (or derived from) the Work and for which the
 42 |    editorial revisions, annotations, elaborations, or other modifications
 43 |    represent, as a whole, an original work of authorship. For the purposes
 44 |    of this License, Derivative Works shall not include works that remain
 45 |    separable from, or merely link (or bind by name) to the interfaces of,
 46 |    the Work and Derivative Works thereof.
 47 | 
 48 |    "Contribution" shall mean any work of authorship, including
 49 |    the original version of the Work and any modifications or additions
 50 |    to that Work or Derivative Works thereof, that is intentionally
 51 |    submitted to Licensor for inclusion in the Work by the copyright owner
 52 |    or by an individual or Legal Entity authorized to submit on behalf of
 53 |    the copyright owner. For the purposes of this definition, "submitted"
 54 |    means any form of electronic, verbal, or written communication sent
 55 |    to the Licensor or its representatives, including but not limited to
 56 |    communication on electronic mailing lists, source code control systems,
 57 |    and issue tracking systems that are managed by, or on behalf of, the
 58 |    Licensor for the purpose of discussing and improving the Work, but
 59 |    excluding communication that is conspicuously marked or otherwise
 60 |    designated in writing by the copyright owner as "Not a Contribution."
 61 | 
 62 |    "Contributor" shall mean Licensor and any individual or Legal Entity
 63 |    on behalf of whom a Contribution has been received by Licensor and
 64 |    subsequently incorporated within the Work.
 65 | 
 66 | 2. Grant of Copyright License. Subject to the terms and conditions of
 67 |    this License, each Contributor hereby grants to You a perpetual,
 68 |    worldwide, non-exclusive, no-charge, royalty-free, irrevocable
 69 |    copyright license to reproduce, prepare Derivative Works of,
 70 |    publicly display, publicly perform, sublicense, and distribute the
 71 |    Work and such Derivative Works in Source or Object form.
 72 | 
 73 | 3. Grant of Patent License. Subject to the terms and conditions of
 74 |    this License, each Contributor hereby grants to You a perpetual,
 75 |    worldwide, non-exclusive, no-charge, royalty-free, irrevocable
 76 |    (except as stated in this section) patent license to make, have made,
 77 |    use, offer to sell, sell, import, and otherwise transfer the Work,
 78 |    where such license applies only to those patent claims licensable
 79 |    by such Contributor that are necessarily infringed by their
 80 |    Contribution(s) alone or by combination of their Contribution(s)
 81 |    with the Work to which such Contribution(s) was submitted. If You
 82 |    institute patent litigation against any entity (including a
 83 |    cross-claim or counterclaim in a lawsuit) alleging that the Work
 84 |    or a Contribution incorporated within the Work constitutes direct
 85 |    or contributory patent infringement, then any patent licenses
 86 |    granted to You under this License for that Work shall terminate
 87 |    as of the date such litigation is filed.
 88 | 
 89 | 4. Redistribution. You may reproduce and distribute copies of the
 90 |    Work or Derivative Works thereof in any medium, with or without
 91 |    modifications, and in Source or Object form, provided that You
 92 |    meet the following conditions:
 93 | 
 94 |    (a) You must give any other recipients of the Work or
 95 |        Derivative Works a copy of this License; and
 96 | 
 97 |    (b) You must cause any modified files to carry prominent notices
 98 |        stating that You changed the files; and
 99 | 
100 |    (c) You must retain, in the Source form of any Derivative Works
101 |        that You distribute, all copyright, patent, trademark, and
102 |        attribution notices from the Source form of the Work,
103 |        excluding those notices that do not pertain to any part of
104 |        the Derivative Works; and
105 | 
106 |    (d) If the Work includes a "NOTICE" text file as part of its
107 |        distribution, then any Derivative Works that You distribute must
108 |        include a readable copy of the attribution notices contained
109 |        within such NOTICE file, excluding those notices that do not
110 |        pertain to any part of the Derivative Works, in at least one
111 |        of the following places: within a NOTICE text file distributed
112 |        as part of the Derivative Works; within the Source form or
113 |        documentation, if provided along with the Derivative Works; or,
114 |        within a display generated by the Derivative Works, if and
115 |        wherever such third-party notices normally appear. The contents
116 |        of the NOTICE file are for informational purposes only and
117 |        do not modify the License. You may add Your own attribution
118 |        notices within Derivative Works that You distribute, alongside
119 |        or as an addendum to the NOTICE text from the Work, provided
120 |        that such additional attribution notices cannot be construed
121 |        as modifying the License.
122 | 
123 |    You may add Your own copyright statement to Your modifications and
124 |    may provide additional or different license terms and conditions
125 |    for use, reproduction, or distribution of Your modifications, or
126 |    for any such Derivative Works as a whole, provided Your use,
127 |    reproduction, and distribution of the Work otherwise complies with
128 |    the conditions stated in this License.
129 | 
130 | 5. Submission of Contributions. Unless You explicitly state otherwise,
131 |    any Contribution intentionally submitted for inclusion in the Work
132 |    by You to the Licensor shall be under the terms and conditions of
133 |    this License, without any additional terms or conditions.
134 |    Notwithstanding the above, nothing herein shall supersede or modify
135 |    the terms of any separate license agreement you may have executed
136 |    with Licensor regarding such Contributions.
137 | 
138 | 6. Trademarks. This License does not grant permission to use the trade
139 |    names, trademarks, service marks, or product names of the Licensor,
140 |    except as required for reasonable and customary use in describing the
141 |    origin of the Work and reproducing the content of the NOTICE file.
142 | 
143 | 7. Disclaimer of Warranty. Unless required by applicable law or
144 |    agreed to in writing, Licensor provides the Work (and each
145 |    Contributor provides its Contributions) on an "AS IS" BASIS,
146 |    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
147 |    implied, including, without limitation, any warranties or conditions
148 |    of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
149 |    PARTICULAR PURPOSE. You are solely responsible for determining the
150 |    appropriateness of using or redistributing the Work and assume any
151 |    risks associated with Your exercise of permissions under this License.
152 | 
153 | 8. Limitation of Liability. In no event and under no legal theory,
154 |    whether in tort (including negligence), contract, or otherwise,
155 |    unless required by applicable law (such as deliberate and grossly
156 |    negligent acts) or agreed to in writing, shall any Contributor be
157 |    liable to You for damages, including any direct, indirect, special,
158 |    incidental, or consequential damages of any character arising as a
159 |    result of this License or out of the use or inability to use the
160 |    Work (including but not limited to damages for loss of goodwill,
161 |    work stoppage, computer failure or malfunction, or any and all
162 |    other commercial damages or losses), even if such Contributor
163 |    has been advised of the possibility of such damages.
164 | 
165 | 9. Accepting Warranty or Additional Liability. While redistributing
166 |    the Work or Derivative Works thereof, You may choose to offer,
167 |    and charge a fee for, acceptance of support, warranty, indemnity,
168 |    or other liability obligations and/or rights consistent with this
169 |    License. However, in accepting such obligations, You may act only
170 |    on Your own behalf and on Your sole responsibility, not on behalf
171 |    of any other Contributor, and only if You agree to indemnify,
172 |    defend, and hold each Contributor harmless for any liability
173 |    incurred by, or claims asserted against, such Contributor by reason
174 |    of your accepting any such warranty or additional liability.
175 | 
176 | END OF TERMS AND CONDITIONS
177 | 
178 | APPENDIX: How to apply the Apache License to your work.
179 | 
180 |    To apply the Apache License to your work, attach the following
181 |    boilerplate notice, with the fields enclosed by brackets "[]"
182 |    replaced with your own identifying information. (Don't include
183 |    the brackets!)  The text should be enclosed in the appropriate
184 |    comment syntax for the file format. We also recommend that a
185 |    file or class name and description of purpose be included on the
186 |    same "printed page" as the copyright notice for easier
187 |    identification within third-party archives.
188 | 
189 | Copyright [yyyy] [name of copyright owner]
190 | 
191 | Licensed under the Apache License, Version 2.0 (the "License");
192 | you may not use this file except in compliance with the License.
193 | You may obtain a copy of the License at
194 | 
195 | 	http://www.apache.org/licenses/LICENSE-2.0
196 | 
197 | Unless required by applicable law or agreed to in writing, software
198 | distributed under the License is distributed on an "AS IS" BASIS,
199 | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
200 | See the License for the specific language governing permissions and
201 | limitations under the License.
202 | 


--------------------------------------------------------------------------------
/LICENSE-MIT:
--------------------------------------------------------------------------------
 1 | MIT License
 2 | 
 3 | Copyright (c) 2022-2022 Tauri Programme within The Commons Conservancy
 4 | 
 5 | Permission is hereby granted, free of charge, to any person obtaining a copy
 6 | of this software and associated documentation files (the "Software"), to deal
 7 | in the Software without restriction, including without limitation the rights
 8 | to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 9 | copies of the Software, and to permit persons to whom the Software is
10 | furnished to do so, subject to the following conditions:
11 | 
12 | The above copyright notice and this permission notice shall be included in all
13 | copies or substantial portions of the Software.
14 | 
15 | THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
16 | IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
17 | FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
18 | AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
19 | LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
20 | OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
21 | SOFTWARE.
22 | 


--------------------------------------------------------------------------------
/LICENSE.spdx:
--------------------------------------------------------------------------------
 1 | SPDXVersion: SPDX-2.1
 2 | DataLicense: CC0-1.0
 3 | PackageName: tray-icon
 4 | DataFormat: SPDXRef-1
 5 | PackageSupplier: Organization: The Tauri Programme in the Commons Conservancy
 6 | PackageHomePage: https://tauri.app
 7 | PackageLicenseDeclared: Apache-2.0
 8 | PackageLicenseDeclared: MIT
 9 | PackageCopyrightText: 2020-2022, The Tauri Programme in the Commons Conservancy
10 | PackageSummary: <text>Create tray icons for desktop applications.
11 |                 </text>
12 | PackageComment: <text>The package includes the following libraries; see
13 | Relationship information.
14 |                 </text>
15 | Created: 2022-12-05T09:00:00Z
16 | PackageDownloadLocation: git://github.com/tauri-apps/tray-icon
17 | PackageDownloadLocation: git+https://github.com/tauri-apps/tray-icon.git
18 | PackageDownloadLocation: git+ssh://github.com/tauri-apps/tray-icon.git
19 | Creator: Person: Daniel Thompson-Yvetot


--------------------------------------------------------------------------------
/README.md:
--------------------------------------------------------------------------------
  1 | tray-icon lets you create tray icons for desktop applications.
  2 | 
  3 | ## Platforms supported:
  4 | 
  5 | - Windows
  6 | - macOS
  7 | - Linux (gtk Only)
  8 | 
  9 | ## Platform-specific notes:
 10 | 
 11 | - On Windows and Linux, an event loop must be running on the thread, on Windows, a win32 event loop and on Linux, a gtk event loop. It doesn't need to be the main thread but you have to create the tray icon on the same thread as the event loop.
 12 | - On macOS, an event loop must be running on the main thread so you also need to create the tray icon on the main thread.
 13 | 
 14 | ### Cargo Features
 15 | 
 16 | - `common-controls-v6`: Use `TaskDialogIndirect` API from `ComCtl32.dll` v6 on Windows for showing the predefined `About` menu item dialog.
 17 | - `libxdo`: Enables linking to `libxdo` which is used for the predfined `Copy`, `Cut`, `Paste` and `SelectAll` menu item, see https://github.com/tauri-apps/muda#cargo-features
 18 | - `serde`: Enables de/serializing derives.
 19 | 
 20 | ## Dependencies (Linux Only)
 21 | 
 22 | On Linux, `gtk`, `libxdo` is used to make the predfined `Copy`, `Cut`, `Paste` and `SelectAll` menu items work and `libappindicator` or `libayatnat-appindicator` are used to create the tray icon, so make sure to install them on your system.
 23 | 
 24 | #### Arch Linux / Manjaro:
 25 | 
 26 | ```sh
 27 | pacman -S gtk3 xdotool libappindicator-gtk3 #or libayatana-appindicator
 28 | ```
 29 | 
 30 | #### Debian / Ubuntu:
 31 | 
 32 | ```sh
 33 | sudo apt install libgtk-3-dev libxdo-dev libappindicator3-dev #or libayatana-appindicator3-dev
 34 | ```
 35 | 
 36 | ## Examples
 37 | 
 38 | #### Create a tray icon without a menu.
 39 | 
 40 | ```rs
 41 | use tray_icon::TrayIconBuilder;
 42 | 
 43 | let tray_icon = TrayIconBuilder::new()
 44 |     .with_tooltip("system-tray - tray icon library!")
 45 |     .with_icon(icon)
 46 |     .build()
 47 |     .unwrap();
 48 | ```
 49 | 
 50 | #### Create a tray icon with a menu.
 51 | 
 52 | ```rs
 53 | use tray_icon::{TrayIconBuilder, menu::Menu};
 54 | 
 55 | let tray_menu = Menu::new();
 56 | let tray_icon = TrayIconBuilder::new()
 57 |     .with_menu(Box::new(tray_menu))
 58 |     .with_tooltip("system-tray - tray icon library!")
 59 |     .with_icon(icon)
 60 |     .build()
 61 |     .unwrap();
 62 | ```
 63 | 
 64 | ## Processing tray events
 65 | 
 66 | You can use `TrayIconEvent::receiver` to get a reference to the `TrayIconEventReceiver`
 67 | which you can use to listen to events when a click happens on the tray icon
 68 | 
 69 | ```rs
 70 | use tray_icon::TrayIconEvent;
 71 | 
 72 | if let Ok(event) = TrayIconEvent::receiver().try_recv() {
 73 |     println!("{:?}", event);
 74 | }
 75 | ```
 76 | 
 77 | You can also listen for the menu events using `MenuEvent::receiver` to get events for the tray context menu.
 78 | 
 79 | ```rs
 80 | use tray_icon::{TrayIconEvent, menu::{MenuEvent}};
 81 | 
 82 | if let Ok(event) = TrayIconEvent::receiver().try_recv() {
 83 |     println!("tray event: {:?}", event);
 84 | }
 85 | 
 86 | if let Ok(event) = MenuEvent::receiver().try_recv() {
 87 |     println!("menu event: {:?}", event);
 88 | }
 89 | ```
 90 | 
 91 | ### Note for [winit] or [tao] users:
 92 | 
 93 | You should use [`TrayIconEvent::set_event_handler`] and forward
 94 | the tray icon events to the event loop by using [`EventLoopProxy`]
 95 | so that the event loop is awakened on each tray icon event.
 96 | Same can be done for menu events using [`MenuEvent::set_event_handler`].
 97 | 
 98 | ```rust
 99 | enum UserEvent {
100 |   TrayIconEvent(tray_icon::TrayIconEvent)
101 |   MenuEvent(tray_icon::menu::MenuEvent)
102 | }
103 | 
104 | let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
105 | 
106 | let proxy = event_loop.create_proxy();
107 | tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
108 |     proxy.send_event(UserEvent::TrayIconEvent(event));
109 | }));
110 | 
111 | let proxy = event_loop.create_proxy();
112 | tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
113 |     proxy.send_event(UserEvent::MenuEvent(event));
114 | }));
115 | ```
116 | 
117 | [`EventLoopProxy`]: https://docs.rs/winit/latest/winit/event_loop/struct.EventLoopProxy.html
118 | [winit]: https://docs.rs/winit
119 | [tao]: https://docs.rs/tao
120 | 
121 | ## License
122 | 
123 | Apache-2.0/MIT
124 | 


--------------------------------------------------------------------------------
/examples/egui.rs:
--------------------------------------------------------------------------------
 1 | #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
 2 | 
 3 | #[cfg(not(target_os = "linux"))]
 4 | use std::{cell::RefCell, rc::Rc};
 5 | 
 6 | use eframe::egui;
 7 | use tray_icon::TrayIconBuilder;
 8 | 
 9 | fn main() -> Result<(), eframe::Error> {
10 |     let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
11 |     let icon = load_icon(std::path::Path::new(path));
12 | 
13 |     // Since egui uses winit under the hood and doesn't use gtk on Linux, and we need gtk for
14 |     // the tray icon to show up, we need to spawn a thread
15 |     // where we initialize gtk and create the tray_icon
16 |     #[cfg(target_os = "linux")]
17 |     std::thread::spawn(|| {
18 |         use tray_icon::menu::Menu;
19 | 
20 |         gtk::init().unwrap();
21 |         let _tray_icon = TrayIconBuilder::new()
22 |             .with_menu(Box::new(Menu::new()))
23 |             .with_icon(icon)
24 |             .build()
25 |             .unwrap();
26 | 
27 |         gtk::main();
28 |     });
29 | 
30 |     #[cfg(not(target_os = "linux"))]
31 |     let mut _tray_icon = Rc::new(RefCell::new(None));
32 |     #[cfg(not(target_os = "linux"))]
33 |     let tray_c = _tray_icon.clone();
34 | 
35 |     eframe::run_native(
36 |         "My egui App",
37 |         eframe::NativeOptions::default(),
38 |         Box::new(move |_cc| {
39 |             #[cfg(not(target_os = "linux"))]
40 |             {
41 |                 tray_c
42 |                     .borrow_mut()
43 |                     .replace(TrayIconBuilder::new().with_icon(icon).build().unwrap());
44 |             }
45 |             Ok(Box::<MyApp>::default())
46 |         }),
47 |     )
48 | }
49 | 
50 | struct MyApp {
51 |     name: String,
52 |     age: u32,
53 | }
54 | 
55 | impl Default for MyApp {
56 |     fn default() -> Self {
57 |         Self {
58 |             name: "Arthur".to_owned(),
59 |             age: 42,
60 |         }
61 |     }
62 | }
63 | 
64 | impl eframe::App for MyApp {
65 |     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
66 |         use tray_icon::TrayIconEvent;
67 | 
68 |         if let Ok(event) = TrayIconEvent::receiver().try_recv() {
69 |             println!("tray event: {event:?}");
70 |         }
71 | 
72 |         egui::CentralPanel::default().show(ctx, |ui| {
73 |             ui.heading("My egui Application");
74 |             ui.horizontal(|ui| {
75 |                 let name_label = ui.label("Your name: ");
76 |                 ui.text_edit_singleline(&mut self.name)
77 |                     .labelled_by(name_label.id);
78 |             });
79 |             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
80 |             if ui.button("Click each year").clicked() {
81 |                 self.age += 1;
82 |             }
83 |             ui.label(format!("Hello '{}', age {}", self.name, self.age));
84 |         });
85 |     }
86 | }
87 | 
88 | fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
89 |     let (icon_rgba, icon_width, icon_height) = {
90 |         let image = image::open(path)
91 |             .expect("Failed to open icon path")
92 |             .into_rgba8();
93 |         let (width, height) = image.dimensions();
94 |         let rgba = image.into_raw();
95 |         (rgba, width, height)
96 |     };
97 |     tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
98 | }
99 | 


--------------------------------------------------------------------------------
/examples/icon.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/tauri-apps/tray-icon/7c612040cee0d6764280055ba348c59223c28581/examples/icon.png


--------------------------------------------------------------------------------
/examples/tao.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | #![allow(unused)]
  6 | 
  7 | use tao::{
  8 |     event::Event,
  9 |     event_loop::{ControlFlow, EventLoopBuilder},
 10 | };
 11 | use tray_icon::{
 12 |     menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
 13 |     TrayIconBuilder, TrayIconEvent,
 14 | };
 15 | 
 16 | enum UserEvent {
 17 |     TrayIconEvent(tray_icon::TrayIconEvent),
 18 |     MenuEvent(tray_icon::menu::MenuEvent),
 19 | }
 20 | 
 21 | fn main() {
 22 |     let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
 23 | 
 24 |     let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
 25 | 
 26 |     // set a tray event handler that forwards the event and wakes up the event loop
 27 |     let proxy = event_loop.create_proxy();
 28 |     TrayIconEvent::set_event_handler(Some(move |event| {
 29 |         proxy.send_event(UserEvent::TrayIconEvent(event));
 30 |     }));
 31 | 
 32 |     // set a menu event handler that forwards the event and wakes up the event loop
 33 |     let proxy = event_loop.create_proxy();
 34 |     MenuEvent::set_event_handler(Some(move |event| {
 35 |         proxy.send_event(UserEvent::MenuEvent(event));
 36 |     }));
 37 | 
 38 |     let tray_menu = Menu::new();
 39 | 
 40 |     let quit_i = MenuItem::new("Quit", true, None);
 41 |     tray_menu.append_items(&[
 42 |         &PredefinedMenuItem::about(
 43 |             None,
 44 |             Some(AboutMetadata {
 45 |                 name: Some("tao".to_string()),
 46 |                 copyright: Some("Copyright tao".to_string()),
 47 |                 ..Default::default()
 48 |             }),
 49 |         ),
 50 |         &PredefinedMenuItem::separator(),
 51 |         &quit_i,
 52 |     ]);
 53 | 
 54 |     let mut tray_icon = None;
 55 | 
 56 |     let menu_channel = MenuEvent::receiver();
 57 |     let tray_channel = TrayIconEvent::receiver();
 58 | 
 59 |     event_loop.run(move |event, _, control_flow| {
 60 |         *control_flow = ControlFlow::Wait;
 61 | 
 62 |         match event {
 63 |             Event::NewEvents(tao::event::StartCause::Init) => {
 64 |                 let icon = load_icon(std::path::Path::new(path));
 65 | 
 66 |                 // We create the icon once the event loop is actually running
 67 |                 // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
 68 |                 tray_icon = Some(
 69 |                     TrayIconBuilder::new()
 70 |                         .with_menu(Box::new(tray_menu.clone()))
 71 |                         .with_tooltip("tao - awesome windowing lib")
 72 |                         .with_icon(icon)
 73 |                         .build()
 74 |                         .unwrap(),
 75 |                 );
 76 | 
 77 |                 // We have to request a redraw here to have the icon actually show up.
 78 |                 // Tao only exposes a redraw method on the Window so we use core-foundation directly.
 79 |                 #[cfg(target_os = "macos")]
 80 |                 unsafe {
 81 |                     use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};
 82 | 
 83 |                     let rl = CFRunLoopGetMain().unwrap();
 84 |                     CFRunLoopWakeUp(&rl);
 85 |                 }
 86 |             }
 87 | 
 88 |             Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
 89 |                 println!("{event:?}");
 90 |             }
 91 | 
 92 |             Event::UserEvent(UserEvent::MenuEvent(event)) => {
 93 |                 println!("{event:?}");
 94 | 
 95 |                 if event.id == quit_i.id() {
 96 |                     tray_icon.take();
 97 |                     *control_flow = ControlFlow::Exit;
 98 |                 }
 99 |             }
100 | 
101 |             _ => {}
102 |         }
103 |     })
104 | }
105 | 
106 | fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
107 |     let (icon_rgba, icon_width, icon_height) = {
108 |         let image = image::open(path)
109 |             .expect("Failed to open icon path")
110 |             .into_rgba8();
111 |         let (width, height) = image.dimensions();
112 |         let rgba = image.into_raw();
113 |         (rgba, width, height)
114 |     };
115 |     tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
116 | }
117 | 


--------------------------------------------------------------------------------
/examples/winit.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | #![allow(unused)]
  6 | 
  7 | use tray_icon::{
  8 |     menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
  9 |     TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
 10 | };
 11 | use winit::{
 12 |     application::ApplicationHandler,
 13 |     event::Event,
 14 |     event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
 15 | };
 16 | 
 17 | #[derive(Debug)]
 18 | enum UserEvent {
 19 |     TrayIconEvent(tray_icon::TrayIconEvent),
 20 |     MenuEvent(tray_icon::menu::MenuEvent),
 21 | }
 22 | 
 23 | struct Application {
 24 |     tray_icon: Option<TrayIcon>,
 25 | }
 26 | 
 27 | impl Application {
 28 |     fn new() -> Application {
 29 |         Application { tray_icon: None }
 30 |     }
 31 | 
 32 |     fn new_tray_icon() -> TrayIcon {
 33 |         let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
 34 |         let icon = load_icon(std::path::Path::new(path));
 35 | 
 36 |         TrayIconBuilder::new()
 37 |             .with_menu(Box::new(Self::new_tray_menu()))
 38 |             .with_tooltip("winit - awesome windowing lib")
 39 |             .with_icon(icon)
 40 |             .with_title("x")
 41 |             .build()
 42 |             .unwrap()
 43 |     }
 44 | 
 45 |     fn new_tray_menu() -> Menu {
 46 |         let menu = Menu::new();
 47 |         let item1 = MenuItem::new("item1", true, None);
 48 |         if let Err(err) = menu.append(&item1) {
 49 |             println!("{err:?}");
 50 |         }
 51 |         menu
 52 |     }
 53 | }
 54 | 
 55 | impl ApplicationHandler<UserEvent> for Application {
 56 |     fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
 57 | 
 58 |     fn window_event(
 59 |         &mut self,
 60 |         _event_loop: &winit::event_loop::ActiveEventLoop,
 61 |         _window_id: winit::window::WindowId,
 62 |         _event: winit::event::WindowEvent,
 63 |     ) {
 64 |     }
 65 | 
 66 |     fn new_events(
 67 |         &mut self,
 68 |         _event_loop: &winit::event_loop::ActiveEventLoop,
 69 |         cause: winit::event::StartCause,
 70 |     ) {
 71 |         // We create the icon once the event loop is actually running
 72 |         // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
 73 |         if winit::event::StartCause::Init == cause {
 74 |             #[cfg(not(target_os = "linux"))]
 75 |             {
 76 |                 self.tray_icon = Some(Self::new_tray_icon());
 77 |             }
 78 | 
 79 |             // We have to request a redraw here to have the icon actually show up.
 80 |             // Winit only exposes a redraw method on the Window so we use core-foundation directly.
 81 |             #[cfg(target_os = "macos")]
 82 |             unsafe {
 83 |                 use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};
 84 | 
 85 |                 let rl = CFRunLoopGetMain().unwrap();
 86 |                 CFRunLoopWakeUp(&rl);
 87 |             }
 88 |         }
 89 |     }
 90 | 
 91 |     fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
 92 |         println!("{event:?}");
 93 |     }
 94 | }
 95 | 
 96 | fn main() {
 97 |     let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
 98 | 
 99 |     // set a tray event handler that forwards the event and wakes up the event loop
100 |     let proxy = event_loop.create_proxy();
101 |     TrayIconEvent::set_event_handler(Some(move |event| {
102 |         proxy.send_event(UserEvent::TrayIconEvent(event));
103 |     }));
104 |     let proxy = event_loop.create_proxy();
105 |     MenuEvent::set_event_handler(Some(move |event| {
106 |         proxy.send_event(UserEvent::MenuEvent(event));
107 |     }));
108 | 
109 |     let mut app = Application::new();
110 | 
111 |     let menu_channel = MenuEvent::receiver();
112 |     let tray_channel = TrayIconEvent::receiver();
113 | 
114 |     // Since winit doesn't use gtk on Linux, and we need gtk for
115 |     // the tray icon to show up, we need to spawn a thread
116 |     // where we initialize gtk and create the tray_icon
117 |     #[cfg(target_os = "linux")]
118 |     std::thread::spawn(|| {
119 |         gtk::init().unwrap();
120 | 
121 |         let _tray_icon = Application::new_tray_icon();
122 | 
123 |         gtk::main();
124 |     });
125 | 
126 |     if let Err(err) = event_loop.run_app(&mut app) {
127 |         println!("Error: {:?}", err);
128 |     }
129 | }
130 | 
131 | fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
132 |     let (icon_rgba, icon_width, icon_height) = {
133 |         let image = image::open(path)
134 |             .expect("Failed to open icon path")
135 |             .into_rgba8();
136 |         let (width, height) = image.dimensions();
137 |         let rgba = image.into_raw();
138 |         (rgba, width, height)
139 |     };
140 |     tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
141 | }
142 | 


--------------------------------------------------------------------------------
/renovate.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "extends": ["config:recommended"],
 3 |   "rangeStrategy": "replace",
 4 |   "packageRules": [
 5 |     {
 6 |       "semanticCommitType": "chore",
 7 |       "matchPackageNames": ["*"]
 8 |     }
 9 |   ]
10 | }
11 | 


--------------------------------------------------------------------------------
/src/counter.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | use std::sync::atomic::{AtomicU32, Ordering};
 6 | 
 7 | pub struct Counter(AtomicU32);
 8 | 
 9 | impl Counter {
10 |     pub const fn new() -> Self {
11 |         Self(AtomicU32::new(1))
12 |     }
13 | 
14 |     pub fn next(&self) -> u32 {
15 |         self.0.fetch_add(1, Ordering::Relaxed)
16 |     }
17 | }
18 | 


--------------------------------------------------------------------------------
/src/error.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | use thiserror::Error;
 6 | 
 7 | /// Errors returned by tray-icon.
 8 | #[non_exhaustive]
 9 | #[derive(Error, Debug)]
10 | pub enum Error {
11 |     #[error(transparent)]
12 |     OsError(#[from] std::io::Error),
13 |     #[cfg(any(target_os = "linux", target_os = "macos"))]
14 |     #[error(transparent)]
15 |     PngEncodingError(#[from] png::EncodingError),
16 |     #[error("not on the main thread")]
17 |     NotMainThread,
18 | }
19 | 
20 | /// Convenient type alias of Result type for tray-icon.
21 | pub type Result<T> = std::result::Result<T, Error>;
22 | 


--------------------------------------------------------------------------------
/src/icon.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | // taken from https://github.com/rust-windowing/winit/blob/92fdf5ba85f920262a61cee4590f4a11ad5738d1/src/icon.rs
  6 | 
  7 | use crate::platform_impl::PlatformIcon;
  8 | use std::{error::Error, fmt, io, mem};
  9 | 
 10 | #[repr(C)]
 11 | #[derive(Debug)]
 12 | pub(crate) struct Pixel {
 13 |     pub(crate) r: u8,
 14 |     pub(crate) g: u8,
 15 |     pub(crate) b: u8,
 16 |     pub(crate) a: u8,
 17 | }
 18 | 
 19 | pub(crate) const PIXEL_SIZE: usize = mem::size_of::<Pixel>();
 20 | 
 21 | #[derive(Debug)]
 22 | /// An error produced when using [`Icon::from_rgba`] with invalid arguments.
 23 | pub enum BadIcon {
 24 |     /// Produced when the length of the `rgba` argument isn't divisible by 4, thus `rgba` can't be
 25 |     /// safely interpreted as 32bpp RGBA pixels.
 26 |     ByteCountNotDivisibleBy4 { byte_count: usize },
 27 |     /// Produced when the number of pixels (`rgba.len() / 4`) isn't equal to `width * height`.
 28 |     /// At least one of your arguments is incorrect.
 29 |     DimensionsVsPixelCount {
 30 |         width: u32,
 31 |         height: u32,
 32 |         width_x_height: usize,
 33 |         pixel_count: usize,
 34 |     },
 35 |     /// Produced when underlying OS functionality failed to create the icon
 36 |     OsError(io::Error),
 37 | }
 38 | 
 39 | impl fmt::Display for BadIcon {
 40 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
 41 |         match self {
 42 |             BadIcon::ByteCountNotDivisibleBy4 { byte_count } => write!(f,
 43 |                 "The length of the `rgba` argument ({:?}) isn't divisible by 4, making it impossible to interpret as 32bpp RGBA pixels.",
 44 |                 byte_count,
 45 |             ),
 46 |             BadIcon::DimensionsVsPixelCount {
 47 |                 width,
 48 |                 height,
 49 |                 width_x_height,
 50 |                 pixel_count,
 51 |             } => write!(f,
 52 |                 "The specified dimensions ({:?}x{:?}) don't match the number of pixels supplied by the `rgba` argument ({:?}). For those dimensions, the expected pixel count is {:?}.",
 53 |                 width, height, pixel_count, width_x_height,
 54 |             ),
 55 |             BadIcon::OsError(e) => write!(f, "OS error when instantiating the icon: {:?}", e),
 56 |         }
 57 |     }
 58 | }
 59 | 
 60 | impl Error for BadIcon {
 61 |     fn source(&self) -> Option<&(dyn Error + 'static)> {
 62 |         Some(self)
 63 |     }
 64 | }
 65 | 
 66 | #[derive(Debug, Clone, PartialEq, Eq)]
 67 | pub(crate) struct RgbaIcon {
 68 |     pub(crate) rgba: Vec<u8>,
 69 |     pub(crate) width: u32,
 70 |     pub(crate) height: u32,
 71 | }
 72 | 
 73 | /// For platforms which don't have window icons (e.g. web)
 74 | #[derive(Debug, Clone, PartialEq, Eq)]
 75 | pub(crate) struct NoIcon;
 76 | 
 77 | #[allow(dead_code)] // These are not used on every platform
 78 | mod constructors {
 79 |     use super::*;
 80 | 
 81 |     impl RgbaIcon {
 82 |         pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
 83 |             if rgba.len() % PIXEL_SIZE != 0 {
 84 |                 return Err(BadIcon::ByteCountNotDivisibleBy4 {
 85 |                     byte_count: rgba.len(),
 86 |                 });
 87 |             }
 88 |             let pixel_count = rgba.len() / PIXEL_SIZE;
 89 |             if pixel_count != (width * height) as usize {
 90 |                 Err(BadIcon::DimensionsVsPixelCount {
 91 |                     width,
 92 |                     height,
 93 |                     width_x_height: (width * height) as usize,
 94 |                     pixel_count,
 95 |                 })
 96 |             } else {
 97 |                 Ok(RgbaIcon {
 98 |                     rgba,
 99 |                     width,
100 |                     height,
101 |                 })
102 |             }
103 |         }
104 |     }
105 | 
106 |     impl NoIcon {
107 |         pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
108 |             // Create the rgba icon anyway to validate the input
109 |             let _ = RgbaIcon::from_rgba(rgba, width, height)?;
110 |             Ok(NoIcon)
111 |         }
112 |     }
113 | }
114 | 
115 | /// An icon used for the window titlebar, taskbar, etc.
116 | #[derive(Clone)]
117 | pub struct Icon {
118 |     pub(crate) inner: PlatformIcon,
119 | }
120 | 
121 | impl fmt::Debug for Icon {
122 |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
123 |         fmt::Debug::fmt(&self.inner, formatter)
124 |     }
125 | }
126 | 
127 | impl Icon {
128 |     /// Creates an icon from 32bpp RGBA data.
129 |     ///
130 |     /// The length of `rgba` must be divisible by 4, and `width * height` must equal
131 |     /// `rgba.len() / 4`. Otherwise, this will return a `BadIcon` error.
132 |     pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
133 |         Ok(Icon {
134 |             inner: PlatformIcon::from_rgba(rgba, width, height)?,
135 |         })
136 |     }
137 | 
138 |     /// Create an icon from a file path.
139 |     ///
140 |     /// Specify `size` to load a specific icon size from the file, or `None` to load the default
141 |     /// icon size from the file.
142 |     ///
143 |     /// In cases where the specified size does not exist in the file, Windows may perform scaling
144 |     /// to get an icon of the desired size.
145 |     #[cfg(windows)]
146 |     pub fn from_path<P: AsRef<std::path::Path>>(
147 |         path: P,
148 |         size: Option<(u32, u32)>,
149 |     ) -> Result<Self, BadIcon> {
150 |         let win_icon = PlatformIcon::from_path(path, size)?;
151 |         Ok(Icon { inner: win_icon })
152 |     }
153 | 
154 |     /// Create an icon from a resource embedded in this executable or library.
155 |     ///
156 |     /// Specify `size` to load a specific icon size from the file, or `None` to load the default
157 |     /// icon size from the file.
158 |     ///
159 |     /// In cases where the specified size does not exist in the file, Windows may perform scaling
160 |     /// to get an icon of the desired size.
161 |     #[cfg(windows)]
162 |     pub fn from_resource(ordinal: u16, size: Option<(u32, u32)>) -> Result<Self, BadIcon> {
163 |         let win_icon = PlatformIcon::from_resource(ordinal, size)?;
164 |         Ok(Icon { inner: win_icon })
165 |     }
166 | 
167 |     /// This is basically the same as from_resource, but takes a resource name
168 |     /// rather than oridinal id.
169 |     #[cfg(windows)]
170 |     pub fn from_resource_name(
171 |         resource_name: &str,
172 |         size: Option<(u32, u32)>,
173 |     ) -> Result<Self, BadIcon> {
174 |         let win_icon = PlatformIcon::from_resource_name(resource_name, size)?;
175 |         Ok(Icon { inner: win_icon })
176 |     }
177 | 
178 |     /// Create an icon from an HICON
179 |     #[cfg(windows)]
180 |     pub fn from_handle(handle: isize) -> Self {
181 |         let win_icon = PlatformIcon::from_handle(handle as _);
182 |         Icon { inner: win_icon }
183 |     }
184 | }
185 | 


--------------------------------------------------------------------------------
/src/lib.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | #![allow(clippy::uninlined_format_args)]
  6 | 
  7 | //! tray-icon lets you create tray icons for desktop applications.
  8 | //!
  9 | //! # Platforms supported:
 10 | //!
 11 | //! - Windows
 12 | //! - macOS
 13 | //! - Linux (gtk Only)
 14 | //!
 15 | //! # Platform-specific notes:
 16 | //!
 17 | //! - On Windows and Linux, an event loop must be running on the thread, on Windows, a win32 event loop and on Linux, a gtk event loop. It doesn't need to be the main thread but you have to create the tray icon on the same thread as the event loop.
 18 | //! - On macOS, an event loop must be running on the main thread so you also need to create the tray icon on the main thread. You must make sure that the event loop is already running and not just created before creating a TrayIcon to prevent issues with fullscreen apps. In Winit for example the earliest you can create icons is on [`StartCause::Init`](https://docs.rs/winit/latest/winit/event/enum.StartCause.html#variant.Init).
 19 | //!
 20 | //! # Dependencies (Linux Only)
 21 | //!
 22 | //! On Linux, `gtk`, `libxdo` is used to make the predfined `Copy`, `Cut`, `Paste` and `SelectAll` menu items work and `libappindicator` or `libayatnat-appindicator` are used to create the tray icon, so make sure to install them on your system.
 23 | //!
 24 | //! #### Arch Linux / Manjaro:
 25 | //!
 26 | //! ```sh
 27 | //! pacman -S gtk3 xdotool libappindicator-gtk3 #or libayatana-appindicator
 28 | //! ```
 29 | //!
 30 | //! #### Debian / Ubuntu:
 31 | //!
 32 | //! ```sh
 33 | //! sudo apt install libgtk-3-dev libxdo-dev libappindicator3-dev #or libayatana-appindicator3-dev
 34 | //! ```
 35 | //!
 36 | //! # Examples
 37 | //!
 38 | //! #### Create a tray icon without a menu.
 39 | //!
 40 | //! ```no_run
 41 | //! use tray_icon::{TrayIconBuilder, Icon};
 42 | //!
 43 | //! # let icon = Icon::from_rgba(Vec::new(), 0, 0).unwrap();
 44 | //! let tray_icon = TrayIconBuilder::new()
 45 | //!     .with_tooltip("system-tray - tray icon library!")
 46 | //!     .with_icon(icon)
 47 | //!     .build()
 48 | //!     .unwrap();
 49 | //! ```
 50 | //!
 51 | //! #### Create a tray icon with a menu.
 52 | //!
 53 | //! ```no_run
 54 | //! use tray_icon::{TrayIconBuilder, menu::Menu,Icon};
 55 | //!
 56 | //! # let icon = Icon::from_rgba(Vec::new(), 0, 0).unwrap();
 57 | //! let tray_menu = Menu::new();
 58 | //! let tray_icon = TrayIconBuilder::new()
 59 | //!     .with_menu(Box::new(tray_menu))
 60 | //!     .with_tooltip("system-tray - tray icon library!")
 61 | //!     .with_icon(icon)
 62 | //!     .build()
 63 | //!     .unwrap();
 64 | //! ```
 65 | //!
 66 | //! # Processing tray events
 67 | //!
 68 | //! You can use [`TrayIconEvent::receiver`] to get a reference to the [`TrayIconEventReceiver`]
 69 | //! which you can use to listen to events when a click happens on the tray icon
 70 | //! ```no_run
 71 | //! use tray_icon::TrayIconEvent;
 72 | //!
 73 | //! if let Ok(event) = TrayIconEvent::receiver().try_recv() {
 74 | //!     println!("{:?}", event);
 75 | //! }
 76 | //! ```
 77 | //!
 78 | //! You can also listen for the menu events using [`MenuEvent::receiver`](crate::menu::MenuEvent::receiver) to get events for the tray context menu.
 79 | //!
 80 | //! ```no_run
 81 | //! use tray_icon::{TrayIconEvent, menu::MenuEvent};
 82 | //!
 83 | //! if let Ok(event) = TrayIconEvent::receiver().try_recv() {
 84 | //!     println!("tray event: {:?}", event);
 85 | //! }
 86 | //!
 87 | //! if let Ok(event) = MenuEvent::receiver().try_recv() {
 88 | //!     println!("menu event: {:?}", event);
 89 | //! }
 90 | //! ```
 91 | //!
 92 | //! ### Note for [winit] or [tao] users:
 93 | //!
 94 | //! You should use [`TrayIconEvent::set_event_handler`] and forward
 95 | //! the tray icon events to the event loop by using [`EventLoopProxy`]
 96 | //! so that the event loop is awakened on each tray icon event.
 97 | //! Same can be done for menu events using [`MenuEvent::set_event_handler`].
 98 | //!
 99 | //! ```no_run
100 | //! # use winit::event_loop::EventLoop;
101 | //! enum UserEvent {
102 | //!   TrayIconEvent(tray_icon::TrayIconEvent),
103 | //!   MenuEvent(tray_icon::menu::MenuEvent)
104 | //! }
105 | //!
106 | //! let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
107 | //!
108 | //! let proxy = event_loop.create_proxy();
109 | //! tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
110 | //!     proxy.send_event(UserEvent::TrayIconEvent(event));
111 | //! }));
112 | //!
113 | //! let proxy = event_loop.create_proxy();
114 | //! tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
115 | //!     proxy.send_event(UserEvent::MenuEvent(event));
116 | //! }));
117 | //! ```
118 | //!
119 | //! [`EventLoopProxy`]: https://docs.rs/winit/latest/winit/event_loop/struct.EventLoopProxy.html
120 | //! [winit]: https://docs.rs/winit
121 | //! [tao]: https://docs.rs/tao
122 | 
123 | use std::{
124 |     cell::RefCell,
125 |     path::{Path, PathBuf},
126 |     rc::Rc,
127 | };
128 | 
129 | use counter::Counter;
130 | use crossbeam_channel::{unbounded, Receiver, Sender};
131 | use once_cell::sync::{Lazy, OnceCell};
132 | 
133 | mod counter;
134 | mod error;
135 | mod icon;
136 | mod platform_impl;
137 | mod tray_icon_id;
138 | 
139 | pub use self::error::*;
140 | pub use self::icon::{BadIcon, Icon};
141 | pub use self::tray_icon_id::TrayIconId;
142 | 
143 | /// Re-export of [muda](::muda) crate and used for tray context menu.
144 | pub mod menu {
145 |     pub use muda::*;
146 | }
147 | pub use muda::dpi;
148 | 
149 | static COUNTER: Counter = Counter::new();
150 | 
151 | /// Attributes to use when creating a tray icon.
152 | pub struct TrayIconAttributes {
153 |     /// Tray icon tooltip
154 |     ///
155 |     /// ## Platform-specific:
156 |     ///
157 |     /// - **Linux:** Unsupported.
158 |     pub tooltip: Option<String>,
159 | 
160 |     /// Tray menu
161 |     ///
162 |     /// ## Platform-specific:
163 |     ///
164 |     /// - **Linux**: once a menu is set, it cannot be removed.
165 |     pub menu: Option<Box<dyn menu::ContextMenu>>,
166 | 
167 |     /// Tray icon
168 |     ///
169 |     /// ## Platform-specific:
170 |     ///
171 |     /// - **Linux:** Sometimes the icon won't be visible unless a menu is set.
172 |     ///     Setting an empty [`Menu`](crate::menu::Menu) is enough.
173 |     pub icon: Option<Icon>,
174 | 
175 |     /// Tray icon temp dir path. **Linux only**.
176 |     pub temp_dir_path: Option<PathBuf>,
177 | 
178 |     /// Use the icon as a [template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc). **macOS only**.
179 |     pub icon_is_template: bool,
180 | 
181 |     /// Whether to show the tray menu on left click or not, default is `true`. **macOS & Windows only**.
182 |     pub menu_on_left_click: bool,
183 | 
184 |     /// Tray icon title.
185 |     ///
186 |     /// ## Platform-specific
187 |     ///
188 |     /// - **Linux:** The title will not be shown unless there is an icon
189 |     ///   as well.  The title is useful for numerical and other frequently
190 |     ///   updated information.  In general, it shouldn't be shown unless a
191 |     ///   user requests it as it can take up a significant amount of space
192 |     ///   on the user's panel.  This may not be shown in all visualizations.
193 |     /// - **Windows:** Unsupported.
194 |     pub title: Option<String>,
195 | }
196 | 
197 | impl Default for TrayIconAttributes {
198 |     fn default() -> Self {
199 |         Self {
200 |             tooltip: None,
201 |             menu: None,
202 |             icon: None,
203 |             temp_dir_path: None,
204 |             icon_is_template: false,
205 |             menu_on_left_click: true,
206 |             title: None,
207 |         }
208 |     }
209 | }
210 | 
211 | /// [`TrayIcon`] builder struct and associated methods.
212 | #[derive(Default)]
213 | pub struct TrayIconBuilder {
214 |     id: TrayIconId,
215 |     attrs: TrayIconAttributes,
216 | }
217 | 
218 | impl TrayIconBuilder {
219 |     /// Creates a new [`TrayIconBuilder`] with default [`TrayIconAttributes`].
220 |     ///
221 |     /// See [`TrayIcon::new`] for more info.
222 |     pub fn new() -> Self {
223 |         Self {
224 |             id: TrayIconId(COUNTER.next().to_string()),
225 |             attrs: TrayIconAttributes::default(),
226 |         }
227 |     }
228 | 
229 |     /// Sets the unique id to build the tray icon with.
230 |     pub fn with_id<I: Into<TrayIconId>>(mut self, id: I) -> Self {
231 |         self.id = id.into();
232 |         self
233 |     }
234 | 
235 |     /// Set the a menu for this tray icon.
236 |     ///
237 |     /// ## Platform-specific:
238 |     ///
239 |     /// - **Linux**: once a menu is set, it cannot be removed or replaced but you can change its content.
240 |     pub fn with_menu(mut self, menu: Box<dyn menu::ContextMenu>) -> Self {
241 |         self.attrs.menu = Some(menu);
242 |         self
243 |     }
244 | 
245 |     /// Set an icon for this tray icon.
246 |     ///
247 |     /// ## Platform-specific:
248 |     ///
249 |     /// - **Linux:** Sometimes the icon won't be visible unless a menu is set.
250 |     ///   Setting an empty [`Menu`](crate::menu::Menu) is enough.
251 |     pub fn with_icon(mut self, icon: Icon) -> Self {
252 |         self.attrs.icon = Some(icon);
253 |         self
254 |     }
255 | 
256 |     /// Set a tooltip for this tray icon.
257 |     ///
258 |     /// ## Platform-specific:
259 |     ///
260 |     /// - **Linux:** Unsupported.
261 |     pub fn with_tooltip<S: AsRef<str>>(mut self, s: S) -> Self {
262 |         self.attrs.tooltip = Some(s.as_ref().to_string());
263 |         self
264 |     }
265 | 
266 |     /// Set the tray icon title.
267 |     ///
268 |     /// ## Platform-specific
269 |     ///
270 |     /// - **Linux:** The title will not be shown unless there is an icon
271 |     ///   as well.  The title is useful for numerical and other frequently
272 |     ///   updated information.  In general, it shouldn't be shown unless a
273 |     ///   user requests it as it can take up a significant amount of space
274 |     ///   on the user's panel.  This may not be shown in all visualizations.
275 |     /// - **Windows:** Unsupported.
276 |     pub fn with_title<S: AsRef<str>>(mut self, title: S) -> Self {
277 |         self.attrs.title.replace(title.as_ref().to_string());
278 |         self
279 |     }
280 | 
281 |     /// Set tray icon temp dir path. **Linux only**.
282 |     ///
283 |     /// On Linux, we need to write the icon to the disk and usually it will
284 |     /// be `$XDG_RUNTIME_DIR/tray-icon` or `$TEMP/tray-icon`.
285 |     pub fn with_temp_dir_path<P: AsRef<Path>>(mut self, s: P) -> Self {
286 |         self.attrs.temp_dir_path = Some(s.as_ref().to_path_buf());
287 |         self
288 |     }
289 | 
290 |     /// Use the icon as a [template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc). **macOS only**.
291 |     pub fn with_icon_as_template(mut self, is_template: bool) -> Self {
292 |         self.attrs.icon_is_template = is_template;
293 |         self
294 |     }
295 | 
296 |     /// Whether to show the tray menu on left click or not, default is `true`. **macOS only**.
297 |     pub fn with_menu_on_left_click(mut self, enable: bool) -> Self {
298 |         self.attrs.menu_on_left_click = enable;
299 |         self
300 |     }
301 | 
302 |     /// Access the unique id that will be assigned to the tray icon
303 |     /// this builder will create.
304 |     pub fn id(&self) -> &TrayIconId {
305 |         &self.id
306 |     }
307 | 
308 |     /// Builds and adds a new [`TrayIcon`] to the system tray.
309 |     pub fn build(self) -> Result<TrayIcon> {
310 |         TrayIcon::with_id(self.id, self.attrs)
311 |     }
312 | }
313 | 
314 | /// Tray icon struct and associated methods.
315 | ///
316 | /// This type is reference-counted and the icon is removed when the last instance is dropped.
317 | #[derive(Clone)]
318 | pub struct TrayIcon {
319 |     id: TrayIconId,
320 |     tray: Rc<RefCell<platform_impl::TrayIcon>>,
321 | }
322 | 
323 | impl TrayIcon {
324 |     /// Builds and adds a new tray icon to the system tray.
325 |     ///
326 |     /// ## Platform-specific:
327 |     ///
328 |     /// - **Linux:** Sometimes the icon won't be visible unless a menu is set.
329 |     ///   Setting an empty [`Menu`](crate::menu::Menu) is enough.
330 |     pub fn new(attrs: TrayIconAttributes) -> Result<Self> {
331 |         let id = TrayIconId(COUNTER.next().to_string());
332 |         Ok(Self {
333 |             tray: Rc::new(RefCell::new(platform_impl::TrayIcon::new(
334 |                 id.clone(),
335 |                 attrs,
336 |             )?)),
337 |             id,
338 |         })
339 |     }
340 | 
341 |     /// Builds and adds a new tray icon to the system tray with the specified Id.
342 |     ///
343 |     /// See [`TrayIcon::new`] for more info.
344 |     pub fn with_id<I: Into<TrayIconId>>(id: I, attrs: TrayIconAttributes) -> Result<Self> {
345 |         let id = id.into();
346 |         Ok(Self {
347 |             tray: Rc::new(RefCell::new(platform_impl::TrayIcon::new(
348 |                 id.clone(),
349 |                 attrs,
350 |             )?)),
351 |             id,
352 |         })
353 |     }
354 | 
355 |     /// Returns the id associated with this tray icon.
356 |     pub fn id(&self) -> &TrayIconId {
357 |         &self.id
358 |     }
359 | 
360 |     /// Set new tray icon. If `None` is provided, it will remove the icon.
361 |     pub fn set_icon(&self, icon: Option<Icon>) -> Result<()> {
362 |         self.tray.borrow_mut().set_icon(icon)
363 |     }
364 | 
365 |     /// Set new tray menu.
366 |     ///
367 |     /// ## Platform-specific:
368 |     ///
369 |     /// - **Linux**: once a menu is set it cannot be removed so `None` has no effect
370 |     pub fn set_menu(&self, menu: Option<Box<dyn menu::ContextMenu>>) {
371 |         self.tray.borrow_mut().set_menu(menu)
372 |     }
373 | 
374 |     /// Sets the tooltip for this tray icon.
375 |     ///
376 |     /// ## Platform-specific:
377 |     ///
378 |     /// - **Linux:** Unsupported
379 |     pub fn set_tooltip<S: AsRef<str>>(&self, tooltip: Option<S>) -> Result<()> {
380 |         self.tray.borrow_mut().set_tooltip(tooltip)
381 |     }
382 | 
383 |     /// Sets the tooltip for this tray icon.
384 |     ///
385 |     /// ## Platform-specific:
386 |     ///
387 |     /// - **Linux:** The title will not be shown unless there is an icon
388 |     ///   as well.  The title is useful for numerical and other frequently
389 |     ///   updated information.  In general, it shouldn't be shown unless a
390 |     ///   user requests it as it can take up a significant amount of space
391 |     ///   on the user's panel.  This may not be shown in all visualizations.
392 |     /// - **Windows:** Unsupported
393 |     pub fn set_title<S: AsRef<str>>(&self, title: Option<S>) {
394 |         self.tray.borrow_mut().set_title(title)
395 |     }
396 | 
397 |     /// Show or hide this tray icon
398 |     pub fn set_visible(&self, visible: bool) -> Result<()> {
399 |         self.tray.borrow_mut().set_visible(visible)
400 |     }
401 | 
402 |     /// Sets the tray icon temp dir path. **Linux only**.
403 |     ///
404 |     /// On Linux, we need to write the icon to the disk and usually it will
405 |     /// be `$XDG_RUNTIME_DIR/tray-icon` or `$TEMP/tray-icon`.
406 |     pub fn set_temp_dir_path<P: AsRef<Path>>(&self, path: Option<P>) {
407 |         #[cfg(target_os = "linux")]
408 |         self.tray.borrow_mut().set_temp_dir_path(path);
409 |         #[cfg(not(target_os = "linux"))]
410 |         let _ = path;
411 |     }
412 | 
413 |     /// Set the current icon as a [template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc). **macOS only**.
414 |     pub fn set_icon_as_template(&self, is_template: bool) {
415 |         #[cfg(target_os = "macos")]
416 |         self.tray.borrow_mut().set_icon_as_template(is_template);
417 |         #[cfg(not(target_os = "macos"))]
418 |         let _ = is_template;
419 |     }
420 | 
421 |     pub fn set_icon_with_as_template(&self, icon: Option<Icon>, is_template: bool) -> Result<()> {
422 |         #[cfg(target_os = "macos")]
423 |         return self
424 |             .tray
425 |             .borrow_mut()
426 |             .set_icon_with_as_template(icon, is_template);
427 |         #[cfg(not(target_os = "macos"))]
428 |         {
429 |             let _ = icon;
430 |             let _ = is_template;
431 |             Ok(())
432 |         }
433 |     }
434 | 
435 |     /// Disable or enable showing the tray menu on left click.
436 |     ///
437 |     /// ## Platform-specific:
438 |     ///
439 |     /// - **Linux:** Unsupported.
440 |     pub fn set_show_menu_on_left_click(&self, enable: bool) {
441 |         #[cfg(any(target_os = "macos", target_os = "windows"))]
442 |         self.tray.borrow_mut().set_show_menu_on_left_click(enable);
443 |         #[cfg(not(any(target_os = "macos", target_os = "windows")))]
444 |         let _ = enable;
445 |     }
446 | 
447 |     /// Get tray icon rect.
448 |     ///
449 |     /// ## Platform-specific:
450 |     ///
451 |     /// - **Linux**: Unsupported.
452 |     pub fn rect(&self) -> Option<Rect> {
453 |         self.tray.borrow().rect()
454 |     }
455 | }
456 | 
457 | /// Describes a tray icon event.
458 | ///
459 | /// ## Platform-specific:
460 | ///
461 | /// - **Linux**: Unsupported. The event is not emmited even though the icon is shown
462 | ///   and will still show a context menu on right click.
463 | #[derive(Debug, Clone)]
464 | #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
465 | #[cfg_attr(feature = "serde", serde(tag = "type"))]
466 | #[non_exhaustive]
467 | pub enum TrayIconEvent {
468 |     /// A click happened on the tray icon.
469 |     #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
470 |     Click {
471 |         /// Id of the tray icon which triggered this event.
472 |         id: TrayIconId,
473 |         /// Physical Position of this event.
474 |         position: dpi::PhysicalPosition<f64>,
475 |         /// Position and size of the tray icon.
476 |         rect: Rect,
477 |         /// Mouse button that triggered this event.
478 |         button: MouseButton,
479 |         /// Mouse button state when this event was triggered.
480 |         button_state: MouseButtonState,
481 |     },
482 |     /// A double click happened on the tray icon. **Windows Only**
483 |     DoubleClick {
484 |         /// Id of the tray icon which triggered this event.
485 |         id: TrayIconId,
486 |         /// Physical Position of this event.
487 |         position: dpi::PhysicalPosition<f64>,
488 |         /// Position and size of the tray icon.
489 |         rect: Rect,
490 |         /// Mouse button that triggered this event.
491 |         button: MouseButton,
492 |     },
493 |     /// The mouse entered the tray icon region.
494 |     Enter {
495 |         /// Id of the tray icon which triggered this event.
496 |         id: TrayIconId,
497 |         /// Physical Position of this event.
498 |         position: dpi::PhysicalPosition<f64>,
499 |         /// Position and size of the tray icon.
500 |         rect: Rect,
501 |     },
502 |     /// The mouse moved over the tray icon region.
503 |     Move {
504 |         /// Id of the tray icon which triggered this event.
505 |         id: TrayIconId,
506 |         /// Physical Position of this event.
507 |         position: dpi::PhysicalPosition<f64>,
508 |         /// Position and size of the tray icon.
509 |         rect: Rect,
510 |     },
511 |     /// The mouse left the tray icon region.
512 |     Leave {
513 |         /// Id of the tray icon which triggered this event.
514 |         id: TrayIconId,
515 |         /// Physical Position of this event.
516 |         position: dpi::PhysicalPosition<f64>,
517 |         /// Position and size of the tray icon.
518 |         rect: Rect,
519 |     },
520 | }
521 | 
522 | /// Describes the mouse button state.
523 | #[derive(Clone, Copy, PartialEq, Eq, Debug)]
524 | #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
525 | pub enum MouseButtonState {
526 |     Up,
527 |     Down,
528 | }
529 | 
530 | impl Default for MouseButtonState {
531 |     fn default() -> Self {
532 |         Self::Up
533 |     }
534 | }
535 | 
536 | /// Describes which mouse button triggered the event..
537 | #[derive(Clone, Copy, PartialEq, Eq, Debug)]
538 | #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
539 | pub enum MouseButton {
540 |     Left,
541 |     Right,
542 |     Middle,
543 | }
544 | 
545 | impl Default for MouseButton {
546 |     fn default() -> Self {
547 |         Self::Left
548 |     }
549 | }
550 | 
551 | /// Describes a rectangle including position (x - y axis) and size.
552 | #[derive(Debug, PartialEq, Clone, Copy)]
553 | #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
554 | pub struct Rect {
555 |     pub size: dpi::PhysicalSize<u32>,
556 |     pub position: dpi::PhysicalPosition<f64>,
557 | }
558 | 
559 | impl Default for Rect {
560 |     fn default() -> Self {
561 |         Self {
562 |             size: dpi::PhysicalSize::new(0, 0),
563 |             position: dpi::PhysicalPosition::new(0., 0.),
564 |         }
565 |     }
566 | }
567 | 
568 | /// A reciever that could be used to listen to tray events.
569 | pub type TrayIconEventReceiver = Receiver<TrayIconEvent>;
570 | type TrayIconEventHandler = Box<dyn Fn(TrayIconEvent) + Send + Sync + 'static>;
571 | 
572 | static TRAY_CHANNEL: Lazy<(Sender<TrayIconEvent>, TrayIconEventReceiver)> = Lazy::new(unbounded);
573 | static TRAY_EVENT_HANDLER: OnceCell<Option<TrayIconEventHandler>> = OnceCell::new();
574 | 
575 | impl TrayIconEvent {
576 |     /// Returns the id of the tray icon which triggered this event.
577 |     pub fn id(&self) -> &TrayIconId {
578 |         match self {
579 |             TrayIconEvent::Click { id, .. } => id,
580 |             TrayIconEvent::DoubleClick { id, .. } => id,
581 |             TrayIconEvent::Enter { id, .. } => id,
582 |             TrayIconEvent::Move { id, .. } => id,
583 |             TrayIconEvent::Leave { id, .. } => id,
584 |         }
585 |     }
586 | 
587 |     /// Gets a reference to the event channel's [`TrayIconEventReceiver`]
588 |     /// which can be used to listen for tray events.
589 |     ///
590 |     /// ## Note
591 |     ///
592 |     /// This will not receive any events if [`TrayIconEvent::set_event_handler`] has been called with a `Some` value.
593 |     pub fn receiver<'a>() -> &'a TrayIconEventReceiver {
594 |         &TRAY_CHANNEL.1
595 |     }
596 | 
597 |     /// Set a handler to be called for new events. Useful for implementing custom event sender.
598 |     ///
599 |     /// ## Note
600 |     ///
601 |     /// Calling this function with a `Some` value,
602 |     /// will not send new events to the channel associated with [`TrayIconEvent::receiver`]
603 |     pub fn set_event_handler<F: Fn(TrayIconEvent) + Send + Sync + 'static>(f: Option<F>) {
604 |         if let Some(f) = f {
605 |             let _ = TRAY_EVENT_HANDLER.set(Some(Box::new(f)));
606 |         } else {
607 |             let _ = TRAY_EVENT_HANDLER.set(None);
608 |         }
609 |     }
610 | 
611 |     #[allow(unused)]
612 |     pub(crate) fn send(event: TrayIconEvent) {
613 |         if let Some(handler) = TRAY_EVENT_HANDLER.get_or_init(|| None) {
614 |             handler(event);
615 |         } else {
616 |             let _ = TRAY_CHANNEL.0.send(event);
617 |         }
618 |     }
619 | }
620 | 
621 | #[cfg(test)]
622 | mod tests {
623 | 
624 |     #[cfg(feature = "serde")]
625 |     #[test]
626 |     fn it_serializes() {
627 |         use super::*;
628 |         let event = TrayIconEvent::Click {
629 |             button: MouseButton::Left,
630 |             button_state: MouseButtonState::Down,
631 |             id: TrayIconId::new("id"),
632 |             position: dpi::PhysicalPosition::default(),
633 |             rect: Rect::default(),
634 |         };
635 | 
636 |         let value = serde_json::to_value(&event).unwrap();
637 |         assert_eq!(
638 |             value,
639 |             serde_json::json!({
640 |                 "type": "Click",
641 |                 "button": "Left",
642 |                 "buttonState": "Down",
643 |                 "id": "id",
644 |                 "position": {
645 |                     "x": 0.0,
646 |                     "y": 0.0,
647 |                 },
648 |                 "rect": {
649 |                     "size": {
650 |                         "width": 0,
651 |                         "height": 0,
652 |                     },
653 |                     "position": {
654 |                         "x": 0.0,
655 |                         "y": 0.0,
656 |                     },
657 |                 }
658 |             })
659 |         )
660 |     }
661 | }
662 | 


--------------------------------------------------------------------------------
/src/platform_impl/gtk/icon.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | use std::{fs::File, io::BufWriter, path::Path};
 6 | 
 7 | use crate::icon::BadIcon;
 8 | 
 9 | #[derive(Debug, Clone)]
10 | pub struct PlatformIcon {
11 |     rgba: Vec<u8>,
12 |     width: i32,
13 |     height: i32,
14 | }
15 | 
16 | impl PlatformIcon {
17 |     pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
18 |         Ok(Self {
19 |             rgba,
20 |             width: width as i32,
21 |             height: height as i32,
22 |         })
23 |     }
24 | 
25 |     pub fn write_to_png(&self, path: impl AsRef<Path>) -> crate::Result<()> {
26 |         let png = File::create(path)?;
27 |         let w = &mut BufWriter::new(png);
28 | 
29 |         let mut encoder = png::Encoder::new(w, self.width as _, self.height as _);
30 |         encoder.set_color(png::ColorType::Rgba);
31 |         encoder.set_depth(png::BitDepth::Eight);
32 | 
33 |         let mut writer = encoder.write_header()?;
34 |         writer.write_image_data(&self.rgba)?;
35 | 
36 |         Ok(())
37 |     }
38 | }
39 | 


--------------------------------------------------------------------------------
/src/platform_impl/gtk/mod.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | mod icon;
  6 | use std::path::{Path, PathBuf};
  7 | 
  8 | use crate::icon::Icon;
  9 | pub(crate) use icon::PlatformIcon;
 10 | 
 11 | use crate::{TrayIconAttributes, TrayIconId, COUNTER};
 12 | use libappindicator::{AppIndicator, AppIndicatorStatus};
 13 | 
 14 | pub struct TrayIcon {
 15 |     id: u32,
 16 |     indicator: AppIndicator,
 17 |     temp_dir_path: Option<PathBuf>,
 18 |     path: PathBuf,
 19 |     counter: u32,
 20 |     menu: Option<Box<dyn muda::ContextMenu>>,
 21 | }
 22 | 
 23 | impl TrayIcon {
 24 |     pub fn new(_id: TrayIconId, attrs: TrayIconAttributes) -> crate::Result<Self> {
 25 |         let id = COUNTER.next();
 26 |         let mut indicator = AppIndicator::new("tray-icon tray app", "");
 27 |         indicator.set_status(AppIndicatorStatus::Active);
 28 | 
 29 |         let (parent_path, icon_path) = temp_icon_path(attrs.temp_dir_path.as_ref(), id, 0)?;
 30 | 
 31 |         if let Some(icon) = attrs.icon {
 32 |             icon.inner.write_to_png(&icon_path)?;
 33 |         }
 34 | 
 35 |         indicator.set_icon_theme_path(&parent_path.to_string_lossy());
 36 |         indicator.set_icon_full(&icon_path.to_string_lossy(), "icon");
 37 | 
 38 |         if let Some(menu) = &attrs.menu {
 39 |             indicator.set_menu(&mut menu.gtk_context_menu());
 40 |         }
 41 | 
 42 |         if let Some(title) = attrs.title {
 43 |             indicator.set_label(title.as_str(), "");
 44 |         }
 45 | 
 46 |         Ok(Self {
 47 |             id,
 48 |             indicator,
 49 |             path: icon_path,
 50 |             temp_dir_path: attrs.temp_dir_path,
 51 |             counter: 0,
 52 |             menu: attrs.menu,
 53 |         })
 54 |     }
 55 |     pub fn set_icon(&mut self, icon: Option<Icon>) -> crate::Result<()> {
 56 |         let _ = std::fs::remove_file(&self.path);
 57 | 
 58 |         self.counter += 1;
 59 | 
 60 |         let (parent_path, icon_path) =
 61 |             temp_icon_path(self.temp_dir_path.as_ref(), self.id, self.counter)?;
 62 | 
 63 |         if let Some(icon) = icon {
 64 |             icon.inner.write_to_png(&icon_path)?;
 65 |         }
 66 | 
 67 |         self.indicator
 68 |             .set_icon_theme_path(&parent_path.to_string_lossy());
 69 |         self.indicator
 70 |             .set_icon_full(&icon_path.to_string_lossy(), "tray icon");
 71 |         self.path = icon_path;
 72 | 
 73 |         Ok(())
 74 |     }
 75 | 
 76 |     pub fn set_menu(&mut self, menu: Option<Box<dyn crate::menu::ContextMenu>>) {
 77 |         if let Some(menu) = &menu {
 78 |             self.indicator.set_menu(&mut menu.gtk_context_menu());
 79 |         }
 80 |         self.menu = menu;
 81 |     }
 82 | 
 83 |     pub fn set_tooltip<S: AsRef<str>>(&mut self, _tooltip: Option<S>) -> crate::Result<()> {
 84 |         Ok(())
 85 |     }
 86 | 
 87 |     pub fn set_title<S: AsRef<str>>(&mut self, title: Option<S>) {
 88 |         self.indicator
 89 |             .set_label(title.as_ref().map(|t| t.as_ref()).unwrap_or(""), "");
 90 |     }
 91 | 
 92 |     pub fn set_visible(&mut self, visible: bool) -> crate::Result<()> {
 93 |         if visible {
 94 |             self.indicator.set_status(AppIndicatorStatus::Active);
 95 |         } else {
 96 |             self.indicator.set_status(AppIndicatorStatus::Passive);
 97 |         }
 98 | 
 99 |         Ok(())
100 |     }
101 | 
102 |     pub fn set_temp_dir_path<P: AsRef<Path>>(&mut self, path: Option<P>) {
103 |         self.temp_dir_path = path.map(|p| p.as_ref().to_path_buf());
104 |     }
105 | 
106 |     pub fn rect(&self) -> Option<crate::Rect> {
107 |         None
108 |     }
109 | }
110 | 
111 | impl Drop for TrayIcon {
112 |     fn drop(&mut self) {
113 |         self.indicator.set_status(AppIndicatorStatus::Passive);
114 |         let _ = std::fs::remove_file(&self.path);
115 |     }
116 | }
117 | 
118 | /// Generates an icon path in one of the following dirs:
119 | /// 1. If `temp_icon_dir` is `Some` use that.
120 | /// 2. `$XDG_RUNTIME_DIR/tray-icon`
121 | /// 3. `/tmp/tray-icon`
122 | fn temp_icon_path(
123 |     temp_icon_dir: Option<&PathBuf>,
124 |     id: u32,
125 |     counter: u32,
126 | ) -> std::io::Result<(PathBuf, PathBuf)> {
127 |     let parent_path = match temp_icon_dir.as_ref() {
128 |         Some(path) => path.to_path_buf(),
129 |         None => dirs::runtime_dir()
130 |             .unwrap_or_else(std::env::temp_dir)
131 |             .join("tray-icon"),
132 |     };
133 | 
134 |     std::fs::create_dir_all(&parent_path)?;
135 |     let icon_path = parent_path.join(format!("tray-icon-{}-{}.png", id, counter));
136 |     Ok((parent_path, icon_path))
137 | }
138 | 
139 | #[test]
140 | fn temp_icon_path_preference_order() {
141 |     let runtime_dir = option_env!("XDG_RUNTIME_DIR");
142 |     let override_dir = PathBuf::from("/tmp/tao-tests");
143 | 
144 |     let (dir1, _file1) = temp_icon_path(Some(&override_dir), 00, 00).unwrap();
145 |     let (dir2, _file1) = temp_icon_path(None, 00, 00).unwrap();
146 |     std::env::remove_var("XDG_RUNTIME_DIR");
147 |     let (dir3, _file2) = temp_icon_path(None, 00, 00).unwrap();
148 | 
149 |     assert_eq!(dir1, override_dir);
150 |     if let Some(runtime_dir) = runtime_dir {
151 |         std::env::set_var("XDG_RUNTIME_DIR", runtime_dir);
152 |         assert_eq!(dir2, PathBuf::from(format!("{}/tray-icon", runtime_dir)));
153 |     }
154 | 
155 |     assert_eq!(dir3, PathBuf::from("/tmp/tray-icon"));
156 | }
157 | 


--------------------------------------------------------------------------------
/src/platform_impl/macos/icon.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | use crate::icon::{BadIcon, RgbaIcon};
 6 | use std::io::Cursor;
 7 | 
 8 | #[derive(Debug, Clone)]
 9 | pub struct PlatformIcon(RgbaIcon);
10 | 
11 | impl PlatformIcon {
12 |     pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
13 |         Ok(PlatformIcon(RgbaIcon::from_rgba(rgba, width, height)?))
14 |     }
15 | 
16 |     pub fn get_size(&self) -> (u32, u32) {
17 |         (self.0.width, self.0.height)
18 |     }
19 | 
20 |     pub fn to_png(&self) -> crate::Result<Vec<u8>> {
21 |         let mut png = Vec::new();
22 | 
23 |         {
24 |             let mut encoder =
25 |                 png::Encoder::new(Cursor::new(&mut png), self.0.width as _, self.0.height as _);
26 |             encoder.set_color(png::ColorType::Rgba);
27 |             encoder.set_depth(png::BitDepth::Eight);
28 | 
29 |             let mut writer = encoder.write_header()?;
30 |             writer.write_image_data(&self.0.rgba)?;
31 |         }
32 | 
33 |         Ok(png)
34 |     }
35 | }
36 | 


--------------------------------------------------------------------------------
/src/platform_impl/macos/mod.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | mod icon;
  6 | use std::cell::{Cell, RefCell};
  7 | 
  8 | use objc2::rc::Retained;
  9 | use objc2::{define_class, msg_send, AllocAnyThread, DeclaredClass, Message};
 10 | use objc2_app_kit::{
 11 |     NSCellImagePosition, NSEvent, NSImage, NSMenu, NSStatusBar, NSStatusItem, NSTrackingArea,
 12 |     NSTrackingAreaOptions, NSVariableStatusItemLength, NSView, NSWindow,
 13 | };
 14 | use objc2_core_foundation::{CGPoint, CGRect, CGSize};
 15 | use objc2_core_graphics::{CGDisplayPixelsHigh, CGMainDisplayID};
 16 | use objc2_foundation::{MainThreadMarker, NSData, NSSize, NSString};
 17 | 
 18 | pub(crate) use self::icon::PlatformIcon;
 19 | use crate::Error;
 20 | use crate::{
 21 |     icon::Icon, menu, MouseButton, MouseButtonState, Rect, TrayIconAttributes, TrayIconEvent,
 22 |     TrayIconId,
 23 | };
 24 | 
 25 | pub struct TrayIcon {
 26 |     ns_status_item: Option<Retained<NSStatusItem>>,
 27 |     tray_target: Option<Retained<TrayTarget>>,
 28 |     id: TrayIconId,
 29 |     attrs: TrayIconAttributes,
 30 |     mtm: MainThreadMarker,
 31 | }
 32 | 
 33 | impl TrayIcon {
 34 |     pub fn new(id: TrayIconId, attrs: TrayIconAttributes) -> crate::Result<Self> {
 35 |         let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread)?;
 36 |         let (ns_status_item, tray_target) = Self::create(&id, &attrs, mtm)?;
 37 | 
 38 |         let tray_icon = Self {
 39 |             ns_status_item: Some(ns_status_item),
 40 |             tray_target: Some(tray_target),
 41 |             id,
 42 |             attrs,
 43 |             mtm,
 44 |         };
 45 | 
 46 |         Ok(tray_icon)
 47 |     }
 48 | 
 49 |     fn create(
 50 |         id: &TrayIconId,
 51 |         attrs: &TrayIconAttributes,
 52 |         mtm: MainThreadMarker,
 53 |     ) -> crate::Result<(Retained<NSStatusItem>, Retained<TrayTarget>)> {
 54 |         let ns_status_item = unsafe {
 55 |             NSStatusBar::systemStatusBar().statusItemWithLength(NSVariableStatusItemLength)
 56 |         };
 57 | 
 58 |         set_icon_for_ns_status_item_button(
 59 |             &ns_status_item,
 60 |             attrs.icon.clone(),
 61 |             attrs.icon_is_template,
 62 |             mtm,
 63 |         )?;
 64 | 
 65 |         if let Some(menu) = &attrs.menu {
 66 |             unsafe {
 67 |                 ns_status_item.setMenu((menu.ns_menu() as *const NSMenu).as_ref());
 68 |             }
 69 |         }
 70 | 
 71 |         Self::set_tooltip_inner(&ns_status_item, attrs.tooltip.clone(), mtm)?;
 72 |         Self::set_title_inner(&ns_status_item, attrs.title.clone(), mtm);
 73 | 
 74 |         let tray_target = unsafe {
 75 |             let button = ns_status_item.button(mtm).unwrap();
 76 | 
 77 |             let frame = button.frame();
 78 | 
 79 |             let target = mtm.alloc().set_ivars(TrayTargetIvars {
 80 |                 id: NSString::from_str(&id.0),
 81 |                 menu: RefCell::new(
 82 |                     attrs
 83 |                         .menu
 84 |                         .as_deref()
 85 |                         .and_then(|menu| Retained::retain(menu.ns_menu().cast::<NSMenu>())),
 86 |                 ),
 87 |                 status_item: ns_status_item.retain(),
 88 |                 menu_on_left_click: Cell::new(attrs.menu_on_left_click),
 89 |             });
 90 |             let tray_target: Retained<TrayTarget> = msg_send![super(target), initWithFrame: frame];
 91 |             tray_target.setWantsLayer(true);
 92 | 
 93 |             button.addSubview(&tray_target);
 94 | 
 95 |             tray_target
 96 |         };
 97 | 
 98 |         Ok((ns_status_item, tray_target))
 99 |     }
100 | 
101 |     fn remove(&mut self) {
102 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
103 |         {
104 |             unsafe {
105 |                 NSStatusBar::systemStatusBar().removeStatusItem(ns_status_item);
106 |                 tray_target.removeFromSuperview();
107 |             }
108 |         }
109 | 
110 |         self.ns_status_item = None;
111 |         self.tray_target = None;
112 |     }
113 | 
114 |     pub fn set_icon(&mut self, icon: Option<Icon>) -> crate::Result<()> {
115 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
116 |         {
117 |             set_icon_for_ns_status_item_button(ns_status_item, icon.clone(), false, self.mtm)?;
118 |             tray_target.update_dimensions();
119 |         }
120 |         self.attrs.icon = icon;
121 |         Ok(())
122 |     }
123 | 
124 |     pub fn set_menu(&mut self, menu: Option<Box<dyn menu::ContextMenu>>) {
125 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
126 |         {
127 |             unsafe {
128 |                 let menu = menu
129 |                     .as_ref()
130 |                     .and_then(|m| m.ns_menu().cast::<NSMenu>().as_ref())
131 |                     .map(|menu| menu.retain());
132 |                 ns_status_item.setMenu(menu.as_deref());
133 |                 if let Some(menu) = &menu {
134 |                     let () = msg_send![menu, setDelegate: &**ns_status_item];
135 |                 }
136 | 
137 |                 *tray_target.ivars().menu.borrow_mut() = menu;
138 |             }
139 |         }
140 |         self.attrs.menu = menu;
141 |     }
142 | 
143 |     pub fn set_tooltip<S: AsRef<str>>(&mut self, tooltip: Option<S>) -> crate::Result<()> {
144 |         let tooltip = tooltip.map(|s| s.as_ref().to_string());
145 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
146 |         {
147 |             Self::set_tooltip_inner(ns_status_item, tooltip.clone(), self.mtm)?;
148 |             tray_target.update_dimensions();
149 |         }
150 |         self.attrs.tooltip = tooltip;
151 |         Ok(())
152 |     }
153 | 
154 |     fn set_tooltip_inner<S: AsRef<str>>(
155 |         ns_status_item: &NSStatusItem,
156 |         tooltip: Option<S>,
157 |         mtm: MainThreadMarker,
158 |     ) -> crate::Result<()> {
159 |         unsafe {
160 |             let tooltip = tooltip.map(|tooltip| NSString::from_str(tooltip.as_ref()));
161 |             if let Some(button) = ns_status_item.button(mtm) {
162 |                 button.setToolTip(tooltip.as_deref());
163 |             }
164 |         }
165 |         Ok(())
166 |     }
167 | 
168 |     pub fn set_title<S: AsRef<str>>(&mut self, title: Option<S>) {
169 |         let title = title.map(|s| s.as_ref().to_string());
170 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
171 |         {
172 |             Self::set_title_inner(ns_status_item, title.clone(), self.mtm);
173 |             tray_target.update_dimensions();
174 |         }
175 |         self.attrs.title = title;
176 |     }
177 | 
178 |     fn set_title_inner<S: AsRef<str>>(
179 |         ns_status_item: &NSStatusItem,
180 |         title: Option<S>,
181 |         mtm: MainThreadMarker,
182 |     ) {
183 |         if let Some(title) = title {
184 |             unsafe {
185 |                 if let Some(button) = ns_status_item.button(mtm) {
186 |                     button.setTitle(&NSString::from_str(title.as_ref()));
187 |                 }
188 |             }
189 |         }
190 |     }
191 | 
192 |     pub fn set_visible(&mut self, visible: bool) -> crate::Result<()> {
193 |         if visible {
194 |             if self.ns_status_item.is_none() {
195 |                 let (ns_status_item, tray_target) = Self::create(&self.id, &self.attrs, self.mtm)?;
196 |                 self.ns_status_item = Some(ns_status_item);
197 |                 self.tray_target = Some(tray_target);
198 |             }
199 |         } else {
200 |             self.remove();
201 |         }
202 | 
203 |         Ok(())
204 |     }
205 | 
206 |     pub fn set_icon_as_template(&mut self, is_template: bool) {
207 |         if let Some(ns_status_item) = &self.ns_status_item {
208 |             unsafe {
209 |                 let button = ns_status_item.button(self.mtm).unwrap();
210 |                 if let Some(nsimage) = button.image() {
211 |                     nsimage.setTemplate(is_template);
212 |                     button.setImage(Some(&nsimage));
213 |                 }
214 |             }
215 |         }
216 |         self.attrs.icon_is_template = is_template;
217 |     }
218 | 
219 |     pub fn set_icon_with_as_template(
220 |         &mut self,
221 |         icon: Option<Icon>,
222 |         is_template: bool,
223 |     ) -> crate::Result<()> {
224 |         if let (Some(ns_status_item), Some(tray_target)) = (&self.ns_status_item, &self.tray_target)
225 |         {
226 |             set_icon_for_ns_status_item_button(
227 |                 ns_status_item,
228 |                 icon.clone(),
229 |                 is_template,
230 |                 self.mtm,
231 |             )?;
232 |             tray_target.update_dimensions();
233 |         }
234 |         self.attrs.icon = icon;
235 |         self.attrs.icon_is_template = is_template;
236 |         Ok(())
237 |     }
238 | 
239 |     pub fn set_show_menu_on_left_click(&mut self, enable: bool) {
240 |         if let Some(tray_target) = &self.tray_target {
241 |             tray_target.ivars().menu_on_left_click.set(enable);
242 |         }
243 |         self.attrs.menu_on_left_click = enable;
244 |     }
245 | 
246 |     pub fn rect(&self) -> Option<Rect> {
247 |         let ns_status_item = self.ns_status_item.as_deref()?;
248 |         unsafe {
249 |             let button = ns_status_item.button(self.mtm).unwrap();
250 |             let window = button.window();
251 |             window.map(|window| get_tray_rect(&window))
252 |         }
253 |     }
254 | }
255 | 
256 | impl Drop for TrayIcon {
257 |     fn drop(&mut self) {
258 |         self.remove()
259 |     }
260 | }
261 | 
262 | fn set_icon_for_ns_status_item_button(
263 |     ns_status_item: &NSStatusItem,
264 |     icon: Option<Icon>,
265 |     icon_is_template: bool,
266 |     mtm: MainThreadMarker,
267 | ) -> crate::Result<()> {
268 |     let button = unsafe { ns_status_item.button(mtm).unwrap() };
269 | 
270 |     if let Some(icon) = icon {
271 |         let png_icon = icon.inner.to_png()?;
272 | 
273 |         let (width, height) = icon.inner.get_size();
274 | 
275 |         let icon_height: f64 = 18.0;
276 |         let icon_width: f64 = (width as f64) / (height as f64 / icon_height);
277 | 
278 |         unsafe {
279 |             // build our icon
280 |             let nsdata = NSData::from_vec(png_icon);
281 | 
282 |             let nsimage = NSImage::initWithData(NSImage::alloc(), &nsdata).unwrap();
283 |             let new_size = NSSize::new(icon_width, icon_height);
284 | 
285 |             button.setImage(Some(&nsimage));
286 |             nsimage.setSize(new_size);
287 |             // The image is to the right of the title
288 |             button.setImagePosition(NSCellImagePosition::ImageLeft);
289 |             nsimage.setTemplate(icon_is_template);
290 |         }
291 |     } else {
292 |         unsafe { button.setImage(None) };
293 |     }
294 | 
295 |     Ok(())
296 | }
297 | 
298 | #[derive(Debug)]
299 | struct TrayTargetIvars {
300 |     id: Retained<NSString>,
301 |     menu: RefCell<Option<Retained<NSMenu>>>,
302 |     status_item: Retained<NSStatusItem>,
303 |     menu_on_left_click: Cell<bool>,
304 | }
305 | 
306 | define_class!(
307 |     #[unsafe(super(NSView))]
308 |     #[name = "TaoTrayTarget"]
309 |     #[ivars = TrayTargetIvars]
310 |     struct TrayTarget;
311 | 
312 |     /// Mouse events on NSResponder
313 |     impl TrayTarget {
314 |         #[unsafe(method(mouseDown:))]
315 |         fn on_mouse_down(&self, event: &NSEvent) {
316 |             send_mouse_event(
317 |                 self,
318 |                 event,
319 |                 MouseEventType::Click,
320 |                 Some(MouseClickEvent {
321 |                     button: MouseButton::Left,
322 |                     state: MouseButtonState::Down,
323 |                 }),
324 |             );
325 |             on_tray_click(self, MouseButton::Left);
326 |         }
327 | 
328 |         #[unsafe(method(mouseUp:))]
329 |         fn on_mouse_up(&self, event: &NSEvent) {
330 |             let mtm = MainThreadMarker::from(self);
331 |             unsafe {
332 |                 let button = self.ivars().status_item.button(mtm).unwrap();
333 |                 button.highlight(false);
334 |             }
335 |             send_mouse_event(
336 |                 self,
337 |                 event,
338 |                 MouseEventType::Click,
339 |                 Some(MouseClickEvent {
340 |                     button: MouseButton::Left,
341 |                     state: MouseButtonState::Up,
342 |                 }),
343 |             );
344 |         }
345 | 
346 |         #[unsafe(method(rightMouseDown:))]
347 |         fn on_right_mouse_down(&self, event: &NSEvent) {
348 |             send_mouse_event(
349 |                 self,
350 |                 event,
351 |                 MouseEventType::Click,
352 |                 Some(MouseClickEvent {
353 |                     button: MouseButton::Right,
354 |                     state: MouseButtonState::Down,
355 |                 }),
356 |             );
357 |             on_tray_click(self, MouseButton::Right);
358 |         }
359 | 
360 |         #[unsafe(method(rightMouseUp:))]
361 |         fn on_right_mouse_up(&self, event: &NSEvent) {
362 |             send_mouse_event(
363 |                 self,
364 |                 event,
365 |                 MouseEventType::Click,
366 |                 Some(MouseClickEvent {
367 |                     button: MouseButton::Right,
368 |                     state: MouseButtonState::Up,
369 |                 }),
370 |             );
371 |         }
372 | 
373 |         #[unsafe(method(otherMouseDown:))]
374 |         fn on_other_mouse_down(&self, event: &NSEvent) {
375 |             let button_number = unsafe { event.buttonNumber() };
376 |             if button_number == 2 {
377 |                 send_mouse_event(
378 |                     self,
379 |                     event,
380 |                     MouseEventType::Click,
381 |                     Some(MouseClickEvent {
382 |                         button: MouseButton::Middle,
383 |                         state: MouseButtonState::Down,
384 |                     }),
385 |                 );
386 |             }
387 |         }
388 | 
389 |         #[unsafe(method(otherMouseUp:))]
390 |         fn on_other_mouse_up(&self, event: &NSEvent) {
391 |             let button_number = unsafe { event.buttonNumber() };
392 |             if button_number == 2 {
393 |                 send_mouse_event(
394 |                     self,
395 |                     event,
396 |                     MouseEventType::Click,
397 |                     Some(MouseClickEvent {
398 |                         button: MouseButton::Middle,
399 |                         state: MouseButtonState::Up,
400 |                     }),
401 |                 );
402 |             }
403 |         }
404 | 
405 |         #[unsafe(method(mouseEntered:))]
406 |         fn on_mouse_entered(&self, event: &NSEvent) {
407 |             send_mouse_event(self, event, MouseEventType::Enter, None);
408 |         }
409 | 
410 |         #[unsafe(method(mouseExited:))]
411 |         fn on_mouse_exited(&self, event: &NSEvent) {
412 |             send_mouse_event(self, event, MouseEventType::Leave, None);
413 |         }
414 | 
415 |         #[unsafe(method(mouseMoved:))]
416 |         fn on_mouse_moved(&self, event: &NSEvent) {
417 |             send_mouse_event(self, event, MouseEventType::Move, None);
418 |         }
419 |     }
420 | 
421 |     /// Tracking mouse enter/exit/move events
422 |     impl TrayTarget {
423 |         #[unsafe(method(updateTrackingAreas))]
424 |         fn update_tracking_areas(&self) {
425 |             unsafe {
426 |                 let areas = self.trackingAreas();
427 |                 for area in areas {
428 |                     self.removeTrackingArea(&area);
429 |                 }
430 | 
431 |                 let _: () = msg_send![super(self), updateTrackingAreas];
432 | 
433 |                 let options = NSTrackingAreaOptions::MouseEnteredAndExited
434 |                     | NSTrackingAreaOptions::MouseMoved
435 |                     | NSTrackingAreaOptions::ActiveAlways
436 |                     | NSTrackingAreaOptions::InVisibleRect;
437 |                 let rect = CGRect {
438 |                     origin: CGPoint { x: 0.0, y: 0.0 },
439 |                     size: CGSize {
440 |                         width: 0.0,
441 |                         height: 0.0,
442 |                     },
443 |                 };
444 |                 let area = NSTrackingArea::initWithRect_options_owner_userInfo(
445 |                     NSTrackingArea::alloc(),
446 |                     rect,
447 |                     options,
448 |                     Some(self),
449 |                     None,
450 |                 );
451 |                 self.addTrackingArea(&area);
452 |             }
453 |         }
454 |     }
455 | );
456 | 
457 | impl TrayTarget {
458 |     fn update_dimensions(&self) {
459 |         let mtm = MainThreadMarker::from(self);
460 |         unsafe {
461 |             let button = self.ivars().status_item.button(mtm).unwrap();
462 |             self.setFrame(button.frame());
463 |         }
464 |     }
465 | }
466 | 
467 | fn on_tray_click(this: &TrayTarget, button: MouseButton) {
468 |     let mtm = MainThreadMarker::from(this);
469 |     unsafe {
470 |         let ns_button = this.ivars().status_item.button(mtm).unwrap();
471 | 
472 |         let menu_on_left_click = this.ivars().menu_on_left_click.get();
473 |         if button == MouseButton::Right || (menu_on_left_click && button == MouseButton::Left) {
474 |             let has_items = if let Some(menu) = &*this.ivars().menu.borrow() {
475 |                 menu.numberOfItems() > 0
476 |             } else {
477 |                 false
478 |             };
479 |             if has_items {
480 |                 ns_button.performClick(None);
481 |             } else {
482 |                 ns_button.highlight(true);
483 |             }
484 |         } else {
485 |             ns_button.highlight(true);
486 |         }
487 |     }
488 | }
489 | 
490 | fn get_tray_rect(window: &NSWindow) -> Rect {
491 |     let frame = window.frame();
492 |     let scale_factor = window.backingScaleFactor();
493 | 
494 |     Rect {
495 |         size: crate::dpi::LogicalSize::new(frame.size.width, frame.size.height)
496 |             .to_physical(scale_factor),
497 |         position: crate::dpi::LogicalPosition::new(
498 |             frame.origin.x,
499 |             flip_window_screen_coordinates(frame.origin.y) - frame.size.height,
500 |         )
501 |         .to_physical(scale_factor),
502 |     }
503 | }
504 | 
505 | fn send_mouse_event(
506 |     this: &TrayTarget,
507 |     event: &NSEvent,
508 |     mouse_event_type: MouseEventType,
509 |     click_event: Option<MouseClickEvent>,
510 | ) {
511 |     let mtm = MainThreadMarker::from(this);
512 |     unsafe {
513 |         let tray_id = TrayIconId(this.ivars().id.to_string());
514 | 
515 |         // icon position & size
516 |         let window = event.window(mtm).unwrap();
517 |         let icon_rect = get_tray_rect(&window);
518 | 
519 |         // cursor position
520 |         let mouse_location = NSEvent::mouseLocation();
521 |         let scale_factor = window.backingScaleFactor();
522 |         let cursor_position = crate::dpi::LogicalPosition::new(
523 |             mouse_location.x,
524 |             flip_window_screen_coordinates(mouse_location.y),
525 |         )
526 |         .to_physical(scale_factor);
527 | 
528 |         let event = match mouse_event_type {
529 |             MouseEventType::Click => {
530 |                 let click_event = click_event.unwrap();
531 |                 TrayIconEvent::Click {
532 |                     id: tray_id,
533 |                     position: cursor_position,
534 |                     rect: icon_rect,
535 |                     button: click_event.button,
536 |                     button_state: click_event.state,
537 |                 }
538 |             }
539 |             MouseEventType::Enter => TrayIconEvent::Enter {
540 |                 id: tray_id,
541 |                 position: cursor_position,
542 |                 rect: icon_rect,
543 |             },
544 |             MouseEventType::Leave => TrayIconEvent::Leave {
545 |                 id: tray_id,
546 |                 position: cursor_position,
547 |                 rect: icon_rect,
548 |             },
549 |             MouseEventType::Move => TrayIconEvent::Move {
550 |                 id: tray_id,
551 |                 position: cursor_position,
552 |                 rect: icon_rect,
553 |             },
554 |         };
555 | 
556 |         TrayIconEvent::send(event);
557 |     }
558 | }
559 | 
560 | #[derive(Debug)]
561 | enum MouseEventType {
562 |     Click,
563 |     Enter,
564 |     Leave,
565 |     Move,
566 | }
567 | 
568 | #[derive(Debug)]
569 | struct MouseClickEvent {
570 |     button: MouseButton,
571 |     state: MouseButtonState,
572 | }
573 | 
574 | /// Core graphics screen coordinates are relative to the top-left corner of
575 | /// the so-called "main" display, with y increasing downwards - which is
576 | /// exactly what we want in Winit.
577 | ///
578 | /// However, `NSWindow` and `NSScreen` changes these coordinates to:
579 | /// 1. Be relative to the bottom-left corner of the "main" screen.
580 | /// 2. Be relative to the bottom-left corner of the window/screen itself.
581 | /// 3. Have y increasing upwards.
582 | ///
583 | /// This conversion happens to be symmetric, so we only need this one function
584 | /// to convert between the two coordinate systems.
585 | fn flip_window_screen_coordinates(y: f64) -> f64 {
586 |     unsafe { CGDisplayPixelsHigh(CGMainDisplayID()) as f64 - y }
587 | }
588 | 


--------------------------------------------------------------------------------
/src/platform_impl/mod.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | #[cfg(target_os = "windows")]
 6 | #[path = "windows/mod.rs"]
 7 | mod platform;
 8 | #[cfg(target_os = "linux")]
 9 | #[path = "gtk/mod.rs"]
10 | mod platform;
11 | #[cfg(target_os = "macos")]
12 | #[path = "macos/mod.rs"]
13 | mod platform;
14 | 
15 | pub(crate) use self::platform::*;
16 | 


--------------------------------------------------------------------------------
/src/platform_impl/windows/icon.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | // taken from https://github.com/rust-windowing/winit/blob/92fdf5ba85f920262a61cee4590f4a11ad5738d1/src/platform_impl/windows/icon.rs
  6 | 
  7 | use std::{fmt, io, mem, path::Path, sync::Arc};
  8 | 
  9 | use windows_sys::{
 10 |     core::PCWSTR,
 11 |     Win32::UI::WindowsAndMessaging::{
 12 |         CreateIcon, DestroyIcon, LoadImageW, HICON, IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE,
 13 |     },
 14 | };
 15 | 
 16 | use crate::icon::*;
 17 | 
 18 | use super::util;
 19 | 
 20 | impl Pixel {
 21 |     fn convert_to_bgra(&mut self) {
 22 |         mem::swap(&mut self.r, &mut self.b);
 23 |     }
 24 | }
 25 | 
 26 | impl RgbaIcon {
 27 |     fn into_windows_icon(self) -> Result<WinIcon, BadIcon> {
 28 |         let rgba = self.rgba;
 29 |         let pixel_count = rgba.len() / PIXEL_SIZE;
 30 |         let mut and_mask = Vec::with_capacity(pixel_count);
 31 |         let pixels =
 32 |             unsafe { std::slice::from_raw_parts_mut(rgba.as_ptr() as *mut Pixel, pixel_count) };
 33 |         for pixel in pixels {
 34 |             and_mask.push(pixel.a.wrapping_sub(u8::MAX)); // invert alpha channel
 35 |             pixel.convert_to_bgra();
 36 |         }
 37 |         assert_eq!(and_mask.len(), pixel_count);
 38 |         let handle = unsafe {
 39 |             CreateIcon(
 40 |                 std::ptr::null_mut(),
 41 |                 self.width as i32,
 42 |                 self.height as i32,
 43 |                 1,
 44 |                 (PIXEL_SIZE * 8) as u8,
 45 |                 and_mask.as_ptr(),
 46 |                 rgba.as_ptr(),
 47 |             )
 48 |         };
 49 |         if !handle.is_null() {
 50 |             Ok(WinIcon::from_handle(handle))
 51 |         } else {
 52 |             Err(BadIcon::OsError(io::Error::last_os_error()))
 53 |         }
 54 |     }
 55 | }
 56 | 
 57 | #[derive(Debug)]
 58 | struct RaiiIcon {
 59 |     handle: HICON,
 60 | }
 61 | 
 62 | #[derive(Clone)]
 63 | pub(crate) struct WinIcon {
 64 |     inner: Arc<RaiiIcon>,
 65 | }
 66 | 
 67 | unsafe impl Send for WinIcon {}
 68 | 
 69 | impl WinIcon {
 70 |     pub fn as_raw_handle(&self) -> HICON {
 71 |         self.inner.handle
 72 |     }
 73 | 
 74 |     pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
 75 |         let rgba_icon = RgbaIcon::from_rgba(rgba, width, height)?;
 76 |         rgba_icon.into_windows_icon()
 77 |     }
 78 | 
 79 |     pub(crate) fn from_handle(handle: HICON) -> Self {
 80 |         Self {
 81 |             #[allow(clippy::arc_with_non_send_sync)]
 82 |             inner: Arc::new(RaiiIcon { handle }),
 83 |         }
 84 |     }
 85 | 
 86 |     pub(crate) fn from_path<P: AsRef<Path>>(
 87 |         path: P,
 88 |         size: Option<(u32, u32)>,
 89 |     ) -> Result<Self, BadIcon> {
 90 |         // width / height of 0 along with LR_DEFAULTSIZE tells windows to load the default icon size
 91 |         let (width, height) = size.unwrap_or((0, 0));
 92 | 
 93 |         let wide_path = util::encode_wide(path.as_ref());
 94 | 
 95 |         let handle = unsafe {
 96 |             LoadImageW(
 97 |                 std::ptr::null_mut(),
 98 |                 wide_path.as_ptr(),
 99 |                 IMAGE_ICON,
100 |                 width as i32,
101 |                 height as i32,
102 |                 LR_DEFAULTSIZE | LR_LOADFROMFILE,
103 |             )
104 |         };
105 |         if !handle.is_null() {
106 |             Ok(WinIcon::from_handle(handle as HICON))
107 |         } else {
108 |             Err(BadIcon::OsError(io::Error::last_os_error()))
109 |         }
110 |     }
111 | 
112 |     fn from_resource_inner_name(name: PCWSTR, size: Option<(u32, u32)>) -> Result<Self, BadIcon> {
113 |         // width / height of 0 along with LR_DEFAULTSIZE tells windows to load the default icon size
114 |         let (width, height) = size.unwrap_or((0, 0));
115 |         let handle = unsafe {
116 |             LoadImageW(
117 |                 util::get_instance_handle(),
118 |                 name,
119 |                 IMAGE_ICON,
120 |                 width as i32,
121 |                 height as i32,
122 |                 LR_DEFAULTSIZE,
123 |             )
124 |         };
125 |         if !handle.is_null() {
126 |             Ok(WinIcon::from_handle(handle as HICON))
127 |         } else {
128 |             Err(BadIcon::OsError(io::Error::last_os_error()))
129 |         }
130 |     }
131 | 
132 |     pub(crate) fn from_resource(
133 |         resource_id: u16,
134 |         size: Option<(u32, u32)>,
135 |     ) -> Result<Self, BadIcon> {
136 |         Self::from_resource_inner_name(resource_id as PCWSTR, size)
137 |     }
138 | 
139 |     pub(crate) fn from_resource_name(
140 |         resource_name: &str,
141 |         size: Option<(u32, u32)>,
142 |     ) -> Result<Self, BadIcon> {
143 |         let wide_name = util::encode_wide(resource_name);
144 |         Self::from_resource_inner_name(wide_name.as_ptr(), size)
145 |     }
146 | }
147 | 
148 | impl Drop for RaiiIcon {
149 |     fn drop(&mut self) {
150 |         unsafe { DestroyIcon(self.handle) };
151 |     }
152 | }
153 | 
154 | impl fmt::Debug for WinIcon {
155 |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
156 |         (*self.inner).fmt(formatter)
157 |     }
158 | }
159 | 


--------------------------------------------------------------------------------
/src/platform_impl/windows/mod.rs:
--------------------------------------------------------------------------------
  1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
  2 | // SPDX-License-Identifier: Apache-2.0
  3 | // SPDX-License-Identifier: MIT
  4 | 
  5 | mod icon;
  6 | mod util;
  7 | use std::ptr;
  8 | 
  9 | use once_cell::sync::Lazy;
 10 | use windows_sys::{
 11 |     s,
 12 |     Win32::{
 13 |         Foundation::{FALSE, HWND, LPARAM, LRESULT, POINT, RECT, S_OK, TRUE, WPARAM},
 14 |         UI::{
 15 |             Shell::{
 16 |                 Shell_NotifyIconGetRect, Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP,
 17 |                 NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER,
 18 |             },
 19 |             WindowsAndMessaging::{
 20 |                 CreateWindowExW, DefWindowProcW, DestroyWindow, GetCursorPos, KillTimer,
 21 |                 RegisterClassW, RegisterWindowMessageA, SendMessageW, SetForegroundWindow,
 22 |                 SetTimer, TrackPopupMenu, CREATESTRUCTW, CW_USEDEFAULT, GWL_USERDATA, HICON, HMENU,
 23 |                 TPM_BOTTOMALIGN, TPM_LEFTALIGN, WM_CREATE, WM_DESTROY, WM_LBUTTONDBLCLK,
 24 |                 WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP,
 25 |                 WM_MOUSEMOVE, WM_NCCREATE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP,
 26 |                 WM_TIMER, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
 27 |                 WS_EX_TRANSPARENT, WS_OVERLAPPED,
 28 |             },
 29 |         },
 30 |     },
 31 | };
 32 | 
 33 | use crate::{
 34 |     dpi::PhysicalPosition, icon::Icon, menu, MouseButton, MouseButtonState, Rect,
 35 |     TrayIconAttributes, TrayIconEvent, TrayIconId, COUNTER,
 36 | };
 37 | 
 38 | pub(crate) use self::icon::WinIcon as PlatformIcon;
 39 | 
 40 | const WM_USER_TRAYICON: u32 = 6002;
 41 | const WM_USER_UPDATE_TRAYMENU: u32 = 6003;
 42 | const WM_USER_UPDATE_TRAYICON: u32 = 6004;
 43 | const WM_USER_SHOW_TRAYICON: u32 = 6005;
 44 | const WM_USER_HIDE_TRAYICON: u32 = 6006;
 45 | const WM_USER_UPDATE_TRAYTOOLTIP: u32 = 6007;
 46 | const WM_USER_LEAVE_TIMER_ID: u32 = 6008;
 47 | const WM_USER_SHOW_MENU_ON_LEFT_CLICK: u32 = 6009;
 48 | /// When the taskbar is created, it registers a message with the "TaskbarCreated" string and then broadcasts this message to all top-level windows
 49 | /// When the application receives this message, it should assume that any taskbar icons it added have been removed and add them again.
 50 | static S_U_TASKBAR_RESTART: Lazy<u32> =
 51 |     Lazy::new(|| unsafe { RegisterWindowMessageA(s!("TaskbarCreated")) });
 52 | 
 53 | struct TrayUserData {
 54 |     internal_id: u32,
 55 |     id: TrayIconId,
 56 |     hwnd: HWND,
 57 |     hpopupmenu: Option<HMENU>,
 58 |     icon: Option<Icon>,
 59 |     tooltip: Option<String>,
 60 |     entered: bool,
 61 |     last_position: Option<PhysicalPosition<f64>>,
 62 |     menu_on_left_click: bool,
 63 | }
 64 | 
 65 | pub struct TrayIcon {
 66 |     hwnd: HWND,
 67 |     menu: Option<Box<dyn menu::ContextMenu>>,
 68 |     internal_id: u32,
 69 | }
 70 | 
 71 | impl TrayIcon {
 72 |     pub fn new(id: TrayIconId, attrs: TrayIconAttributes) -> crate::Result<Self> {
 73 |         let internal_id = COUNTER.next();
 74 | 
 75 |         let class_name = util::encode_wide("tray_icon_app");
 76 |         unsafe {
 77 |             let hinstance = util::get_instance_handle();
 78 | 
 79 |             let wnd_class = WNDCLASSW {
 80 |                 lpfnWndProc: Some(tray_proc),
 81 |                 lpszClassName: class_name.as_ptr(),
 82 |                 hInstance: hinstance,
 83 |                 ..std::mem::zeroed()
 84 |             };
 85 | 
 86 |             RegisterClassW(&wnd_class);
 87 | 
 88 |             let traydata = TrayUserData {
 89 |                 id,
 90 |                 internal_id,
 91 |                 hwnd: std::ptr::null_mut(),
 92 |                 hpopupmenu: attrs.menu.as_ref().map(|m| m.hpopupmenu() as _),
 93 |                 icon: attrs.icon.clone(),
 94 |                 tooltip: attrs.tooltip.clone(),
 95 |                 entered: false,
 96 |                 last_position: None,
 97 |                 menu_on_left_click: attrs.menu_on_left_click,
 98 |             };
 99 | 
100 |             let hwnd = CreateWindowExW(
101 |                 WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED |
102 |             // WS_EX_TOOLWINDOW prevents this window from ever showing up in the taskbar, which
103 |             // we want to avoid. If you remove this style, this window won't show up in the
104 |             // taskbar *initially*, but it can show up at some later point. This can sometimes
105 |             // happen on its own after several hours have passed, although this has proven
106 |             // difficult to reproduce. Alternatively, it can be manually triggered by killing
107 |             // `explorer.exe` and then starting the process back up.
108 |             // It is unclear why the bug is triggered by waiting for several hours.
109 |             WS_EX_TOOLWINDOW,
110 |                 class_name.as_ptr(),
111 |                 ptr::null(),
112 |                 WS_OVERLAPPED,
113 |                 CW_USEDEFAULT,
114 |                 0,
115 |                 CW_USEDEFAULT,
116 |                 0,
117 |                 std::ptr::null_mut(),
118 |                 std::ptr::null_mut(),
119 |                 hinstance,
120 |                 Box::into_raw(Box::new(traydata)) as _,
121 |             );
122 |             if hwnd.is_null() {
123 |                 return Err(crate::Error::OsError(std::io::Error::last_os_error()));
124 |             }
125 | 
126 |             let hicon = attrs.icon.as_ref().map(|i| i.inner.as_raw_handle());
127 | 
128 |             if !register_tray_icon(hwnd, internal_id, &hicon, &attrs.tooltip) {
129 |                 return Err(crate::Error::OsError(std::io::Error::last_os_error()));
130 |             }
131 | 
132 |             if let Some(menu) = &attrs.menu {
133 |                 menu.attach_menu_subclass_for_hwnd(hwnd as _);
134 |             }
135 | 
136 |             Ok(Self {
137 |                 hwnd,
138 |                 internal_id,
139 |                 menu: attrs.menu,
140 |             })
141 |         }
142 |     }
143 | 
144 |     pub fn set_icon(&mut self, icon: Option<Icon>) -> crate::Result<()> {
145 |         unsafe {
146 |             let mut nid = NOTIFYICONDATAW {
147 |                 uFlags: NIF_ICON,
148 |                 hWnd: self.hwnd,
149 |                 uID: self.internal_id,
150 |                 ..std::mem::zeroed()
151 |             };
152 | 
153 |             if let Some(hicon) = icon.as_ref().map(|i| i.inner.as_raw_handle()) {
154 |                 nid.hIcon = hicon;
155 |             }
156 | 
157 |             if Shell_NotifyIconW(NIM_MODIFY, &mut nid as _) == 0 {
158 |                 return Err(crate::Error::OsError(std::io::Error::last_os_error()));
159 |             }
160 | 
161 |             // send the new icon to the subclass proc to store it in the tray data
162 |             SendMessageW(
163 |                 self.hwnd,
164 |                 WM_USER_UPDATE_TRAYICON,
165 |                 Box::into_raw(Box::new(icon)) as _,
166 |                 0,
167 |             );
168 |         }
169 | 
170 |         Ok(())
171 |     }
172 | 
173 |     pub fn set_menu(&mut self, menu: Option<Box<dyn menu::ContextMenu>>) {
174 |         // Safety: self.hwnd is valid as long as as the TrayIcon is
175 |         if let Some(menu) = &self.menu {
176 |             unsafe { menu.detach_menu_subclass_from_hwnd(self.hwnd as _) };
177 |         }
178 |         if let Some(menu) = &menu {
179 |             unsafe { menu.attach_menu_subclass_for_hwnd(self.hwnd as _) };
180 |         }
181 | 
182 |         unsafe {
183 |             // send the new menu to the subclass proc where we will update there
184 |             SendMessageW(
185 |                 self.hwnd,
186 |                 WM_USER_UPDATE_TRAYMENU,
187 |                 Box::into_raw(Box::new(menu.as_ref().map(|m| m.hpopupmenu()))) as _,
188 |                 0,
189 |             );
190 |         }
191 | 
192 |         self.menu = menu;
193 |     }
194 | 
195 |     pub fn set_tooltip<S: AsRef<str>>(&mut self, tooltip: Option<S>) -> crate::Result<()> {
196 |         unsafe {
197 |             let mut nid = NOTIFYICONDATAW {
198 |                 uFlags: NIF_TIP,
199 |                 hWnd: self.hwnd,
200 |                 uID: self.internal_id,
201 |                 ..std::mem::zeroed()
202 |             };
203 |             if let Some(tooltip) = &tooltip {
204 |                 let tip = util::encode_wide(tooltip.as_ref());
205 |                 #[allow(clippy::manual_memcpy)]
206 |                 for i in 0..tip.len().min(128) {
207 |                     nid.szTip[i] = tip[i];
208 |                 }
209 |             }
210 | 
211 |             if Shell_NotifyIconW(NIM_MODIFY, &mut nid as _) == 0 {
212 |                 return Err(crate::Error::OsError(std::io::Error::last_os_error()));
213 |             }
214 | 
215 |             // send the new tooltip to the subclass proc to store it in the tray data
216 |             SendMessageW(
217 |                 self.hwnd,
218 |                 WM_USER_UPDATE_TRAYTOOLTIP,
219 |                 Box::into_raw(Box::new(tooltip.map(|t| t.as_ref().to_string()))) as _,
220 |                 0,
221 |             );
222 |         }
223 | 
224 |         Ok(())
225 |     }
226 | 
227 |     pub fn set_show_menu_on_left_click(&mut self, enable: bool) {
228 |         unsafe {
229 |             SendMessageW(
230 |                 self.hwnd,
231 |                 WM_USER_SHOW_MENU_ON_LEFT_CLICK,
232 |                 enable as usize,
233 |                 0,
234 |             );
235 |         }
236 |     }
237 | 
238 |     pub fn set_title<S: AsRef<str>>(&mut self, _title: Option<S>) {}
239 | 
240 |     pub fn set_visible(&mut self, visible: bool) -> crate::Result<()> {
241 |         unsafe {
242 |             SendMessageW(
243 |                 self.hwnd,
244 |                 if visible {
245 |                     WM_USER_SHOW_TRAYICON
246 |                 } else {
247 |                     WM_USER_HIDE_TRAYICON
248 |                 },
249 |                 0,
250 |                 0,
251 |             );
252 |         }
253 | 
254 |         Ok(())
255 |     }
256 | 
257 |     pub fn rect(&self) -> Option<Rect> {
258 |         get_tray_rect(self.internal_id, self.hwnd).map(Into::into)
259 |     }
260 | }
261 | 
262 | impl Drop for TrayIcon {
263 |     fn drop(&mut self) {
264 |         unsafe {
265 |             remove_tray_icon(self.hwnd, self.internal_id);
266 | 
267 |             if let Some(menu) = &self.menu {
268 |                 menu.detach_menu_subclass_from_hwnd(self.hwnd as _);
269 |             }
270 | 
271 |             // destroy the hidden window used by the tray
272 |             DestroyWindow(self.hwnd);
273 |         }
274 |     }
275 | }
276 | 
277 | unsafe extern "system" fn tray_proc(
278 |     hwnd: HWND,
279 |     msg: u32,
280 |     wparam: WPARAM,
281 |     lparam: LPARAM,
282 | ) -> LRESULT {
283 |     let userdata_ptr = unsafe { util::get_window_long(hwnd, GWL_USERDATA) };
284 |     let userdata_ptr = match (userdata_ptr, msg) {
285 |         (0, WM_NCCREATE) => {
286 |             let createstruct = unsafe { &mut *(lparam as *mut CREATESTRUCTW) };
287 |             let userdata = unsafe { &mut *(createstruct.lpCreateParams as *mut TrayUserData) };
288 |             userdata.hwnd = hwnd;
289 |             util::set_window_long(hwnd, GWL_USERDATA, createstruct.lpCreateParams as _);
290 |             return DefWindowProcW(hwnd, msg, wparam, lparam);
291 |         }
292 |         // Getting here should quite frankly be impossible,
293 |         // but we'll make window creation fail here just in case.
294 |         (0, WM_CREATE) => return -1,
295 |         (_, WM_CREATE) => return DefWindowProcW(hwnd, msg, wparam, lparam),
296 |         (0, _) => return DefWindowProcW(hwnd, msg, wparam, lparam),
297 |         _ => userdata_ptr as *mut TrayUserData,
298 |     };
299 | 
300 |     let userdata = &mut *(userdata_ptr);
301 | 
302 |     match msg {
303 |         WM_DESTROY => {
304 |             drop(Box::from_raw(userdata_ptr));
305 |             return 0;
306 |         }
307 |         WM_USER_UPDATE_TRAYMENU => {
308 |             let hpopupmenu = Box::from_raw(wparam as *mut Option<isize>);
309 |             userdata.hpopupmenu = (*hpopupmenu).map(|h| h as *mut _);
310 |         }
311 |         WM_USER_UPDATE_TRAYICON => {
312 |             let icon = Box::from_raw(wparam as *mut Option<Icon>);
313 |             userdata.icon = *icon;
314 |         }
315 |         WM_USER_SHOW_TRAYICON => {
316 |             register_tray_icon(
317 |                 userdata.hwnd,
318 |                 userdata.internal_id,
319 |                 &userdata.icon.as_ref().map(|i| i.inner.as_raw_handle()),
320 |                 &userdata.tooltip,
321 |             );
322 |         }
323 |         WM_USER_HIDE_TRAYICON => {
324 |             remove_tray_icon(userdata.hwnd, userdata.internal_id);
325 |         }
326 |         WM_USER_UPDATE_TRAYTOOLTIP => {
327 |             let tooltip = Box::from_raw(wparam as *mut Option<String>);
328 |             userdata.tooltip = *tooltip;
329 |         }
330 |         _ if msg == *S_U_TASKBAR_RESTART => {
331 |             remove_tray_icon(userdata.hwnd, userdata.internal_id);
332 |             register_tray_icon(
333 |                 userdata.hwnd,
334 |                 userdata.internal_id,
335 |                 &userdata.icon.as_ref().map(|i| i.inner.as_raw_handle()),
336 |                 &userdata.tooltip,
337 |             );
338 |         }
339 |         WM_USER_SHOW_MENU_ON_LEFT_CLICK => {
340 |             userdata.menu_on_left_click = wparam != 0;
341 |         }
342 | 
343 |         WM_USER_TRAYICON
344 |             if matches!(
345 |                 lparam as u32,
346 |                 WM_LBUTTONDOWN
347 |                     | WM_RBUTTONDOWN
348 |                     | WM_MBUTTONDOWN
349 |                     | WM_LBUTTONUP
350 |                     | WM_RBUTTONUP
351 |                     | WM_MBUTTONUP
352 |                     | WM_LBUTTONDBLCLK
353 |                     | WM_RBUTTONDBLCLK
354 |                     | WM_MBUTTONDBLCLK
355 |                     | WM_MOUSEMOVE
356 |             ) =>
357 |         {
358 |             let mut cursor = POINT { x: 0, y: 0 };
359 |             if GetCursorPos(&mut cursor as _) == 0 {
360 |                 return 0;
361 |             }
362 | 
363 |             let id = userdata.id.clone();
364 |             let position = PhysicalPosition::new(cursor.x as f64, cursor.y as f64);
365 | 
366 |             let rect = match get_tray_rect(userdata.internal_id, hwnd) {
367 |                 Some(rect) => Rect::from(rect),
368 |                 None => return 0,
369 |             };
370 | 
371 |             let event = match lparam as u32 {
372 |                 WM_LBUTTONDOWN => TrayIconEvent::Click {
373 |                     id,
374 |                     rect,
375 |                     position,
376 |                     button: MouseButton::Left,
377 |                     button_state: MouseButtonState::Down,
378 |                 },
379 |                 WM_RBUTTONDOWN => TrayIconEvent::Click {
380 |                     id,
381 |                     rect,
382 |                     position,
383 |                     button: MouseButton::Right,
384 |                     button_state: MouseButtonState::Down,
385 |                 },
386 |                 WM_MBUTTONDOWN => TrayIconEvent::Click {
387 |                     id,
388 |                     rect,
389 |                     position,
390 |                     button: MouseButton::Middle,
391 |                     button_state: MouseButtonState::Down,
392 |                 },
393 |                 WM_LBUTTONUP => TrayIconEvent::Click {
394 |                     id,
395 |                     rect,
396 |                     position,
397 |                     button: MouseButton::Left,
398 |                     button_state: MouseButtonState::Up,
399 |                 },
400 |                 WM_RBUTTONUP => TrayIconEvent::Click {
401 |                     id,
402 |                     rect,
403 |                     position,
404 |                     button: MouseButton::Right,
405 |                     button_state: MouseButtonState::Up,
406 |                 },
407 |                 WM_MBUTTONUP => TrayIconEvent::Click {
408 |                     id,
409 |                     rect,
410 |                     position,
411 |                     button: MouseButton::Middle,
412 |                     button_state: MouseButtonState::Up,
413 |                 },
414 |                 WM_LBUTTONDBLCLK => TrayIconEvent::DoubleClick {
415 |                     id,
416 |                     rect,
417 |                     position,
418 |                     button: MouseButton::Left,
419 |                 },
420 |                 WM_RBUTTONDBLCLK => TrayIconEvent::DoubleClick {
421 |                     id,
422 |                     rect,
423 |                     position,
424 |                     button: MouseButton::Right,
425 |                 },
426 |                 WM_MBUTTONDBLCLK => TrayIconEvent::DoubleClick {
427 |                     id,
428 |                     rect,
429 |                     position,
430 |                     button: MouseButton::Middle,
431 |                 },
432 |                 WM_MOUSEMOVE if !userdata.entered => {
433 |                     userdata.entered = true;
434 |                     TrayIconEvent::Enter { id, rect, position }
435 |                 }
436 |                 WM_MOUSEMOVE if userdata.entered => {
437 |                     // handle extra WM_MOUSEMOVE events, ignore if position hasn't changed
438 |                     let cursor_moved = userdata.last_position != Some(position);
439 |                     userdata.last_position = Some(position);
440 |                     if cursor_moved {
441 |                         // Set or update existing timer, where we check if cursor left
442 |                         SetTimer(hwnd, WM_USER_LEAVE_TIMER_ID as _, 15, Some(tray_timer_proc));
443 | 
444 |                         TrayIconEvent::Move { id, rect, position }
445 |                     } else {
446 |                         return 0;
447 |                     }
448 |                 }
449 | 
450 |                 _ => unreachable!(),
451 |             };
452 | 
453 |             TrayIconEvent::send(event);
454 | 
455 |             if lparam as u32 == WM_RBUTTONDOWN
456 |                 || (userdata.menu_on_left_click && lparam as u32 == WM_LBUTTONDOWN)
457 |             {
458 |                 if let Some(menu) = userdata.hpopupmenu {
459 |                     show_tray_menu(hwnd, menu, cursor.x, cursor.y);
460 |                 }
461 |             }
462 |         }
463 | 
464 |         WM_TIMER if wparam as u32 == WM_USER_LEAVE_TIMER_ID => {
465 |             if let Some(position) = userdata.last_position.take() {
466 |                 let mut cursor = POINT { x: 0, y: 0 };
467 |                 if GetCursorPos(&mut cursor as _) == 0 {
468 |                     return 0;
469 |                 }
470 | 
471 |                 let rect = match get_tray_rect(userdata.internal_id, hwnd) {
472 |                     Some(r) => r,
473 |                     None => return 0,
474 |                 };
475 | 
476 |                 let in_x = (rect.left..rect.right).contains(&cursor.x);
477 |                 let in_y = (rect.top..rect.bottom).contains(&cursor.y);
478 | 
479 |                 if !in_x || !in_y {
480 |                     KillTimer(hwnd, WM_USER_LEAVE_TIMER_ID as _);
481 |                     userdata.entered = false;
482 | 
483 |                     TrayIconEvent::send(TrayIconEvent::Leave {
484 |                         id: userdata.id.clone(),
485 |                         rect: rect.into(),
486 |                         position,
487 |                     });
488 |                 }
489 |             }
490 | 
491 |             return 0;
492 |         }
493 | 
494 |         _ => {}
495 |     }
496 | 
497 |     DefWindowProcW(hwnd, msg, wparam, lparam)
498 | }
499 | 
500 | unsafe extern "system" fn tray_timer_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: u32) {
501 |     tray_proc(hwnd, msg, wparam, lparam as _);
502 | }
503 | 
504 | #[inline]
505 | unsafe fn show_tray_menu(hwnd: HWND, menu: HMENU, x: i32, y: i32) {
506 |     // bring the hidden window to the foreground so the pop up menu
507 |     // would automatically hide on click outside
508 |     SetForegroundWindow(hwnd);
509 |     TrackPopupMenu(
510 |         menu,
511 |         // align bottom / right, maybe we could expose this later..
512 |         TPM_BOTTOMALIGN | TPM_LEFTALIGN,
513 |         x,
514 |         y,
515 |         0,
516 |         hwnd,
517 |         std::ptr::null_mut(),
518 |     );
519 | }
520 | 
521 | #[inline]
522 | unsafe fn register_tray_icon(
523 |     hwnd: HWND,
524 |     tray_id: u32,
525 |     hicon: &Option<HICON>,
526 |     tooltip: &Option<String>,
527 | ) -> bool {
528 |     let mut h_icon = std::ptr::null_mut();
529 |     let mut flags = NIF_MESSAGE;
530 |     let mut sz_tip: [u16; 128] = [0; 128];
531 | 
532 |     if let Some(hicon) = hicon {
533 |         flags |= NIF_ICON;
534 |         h_icon = *hicon;
535 |     }
536 | 
537 |     if let Some(tooltip) = tooltip {
538 |         flags |= NIF_TIP;
539 |         let tip = util::encode_wide(tooltip);
540 |         #[allow(clippy::manual_memcpy)]
541 |         for i in 0..tip.len().min(128) {
542 |             sz_tip[i] = tip[i];
543 |         }
544 |     }
545 | 
546 |     let mut nid = NOTIFYICONDATAW {
547 |         uFlags: flags,
548 |         hWnd: hwnd,
549 |         uID: tray_id,
550 |         uCallbackMessage: WM_USER_TRAYICON,
551 |         hIcon: h_icon,
552 |         szTip: sz_tip,
553 |         ..std::mem::zeroed()
554 |     };
555 | 
556 |     Shell_NotifyIconW(NIM_ADD, &mut nid as _) == TRUE
557 | }
558 | 
559 | #[inline]
560 | unsafe fn remove_tray_icon(hwnd: HWND, id: u32) {
561 |     let mut nid = NOTIFYICONDATAW {
562 |         uFlags: NIF_ICON,
563 |         hWnd: hwnd,
564 |         uID: id,
565 |         ..std::mem::zeroed()
566 |     };
567 | 
568 |     if Shell_NotifyIconW(NIM_DELETE, &mut nid as _) == FALSE {
569 |         eprintln!("Error removing system tray icon");
570 |     }
571 | }
572 | 
573 | #[inline]
574 | fn get_tray_rect(id: u32, hwnd: HWND) -> Option<RECT> {
575 |     let nid = NOTIFYICONIDENTIFIER {
576 |         hWnd: hwnd,
577 |         cbSize: std::mem::size_of::<NOTIFYICONIDENTIFIER>() as _,
578 |         uID: id,
579 |         ..unsafe { std::mem::zeroed() }
580 |     };
581 | 
582 |     let mut rect = RECT {
583 |         left: 0,
584 |         bottom: 0,
585 |         right: 0,
586 |         top: 0,
587 |     };
588 |     if unsafe { Shell_NotifyIconGetRect(&nid, &mut rect) } == S_OK {
589 |         Some(rect)
590 |     } else {
591 |         None
592 |     }
593 | }
594 | 
595 | impl From<RECT> for Rect {
596 |     fn from(rect: RECT) -> Self {
597 |         Self {
598 |             position: crate::dpi::PhysicalPosition::new(rect.left.into(), rect.top.into()),
599 |             size: crate::dpi::PhysicalSize::new(
600 |                 rect.right.saturating_sub(rect.left) as u32,
601 |                 rect.bottom.saturating_sub(rect.top) as u32,
602 |             ),
603 |         }
604 |     }
605 | }
606 | 


--------------------------------------------------------------------------------
/src/platform_impl/windows/util.rs:
--------------------------------------------------------------------------------
 1 | // Copyright 2022-2022 Tauri Programme within The Commons Conservancy
 2 | // SPDX-License-Identifier: Apache-2.0
 3 | // SPDX-License-Identifier: MIT
 4 | 
 5 | use std::ops::{Deref, DerefMut};
 6 | 
 7 | use windows_sys::Win32::{
 8 |     Foundation::HWND,
 9 |     UI::WindowsAndMessaging::{ACCEL, WINDOW_LONG_PTR_INDEX},
10 | };
11 | 
12 | pub fn encode_wide<S: AsRef<std::ffi::OsStr>>(string: S) -> Vec<u16> {
13 |     std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
14 |         .chain(std::iter::once(0))
15 |         .collect()
16 | }
17 | 
18 | /// ACCEL wrapper to implement Debug
19 | #[derive(Clone)]
20 | #[repr(transparent)]
21 | pub struct Accel(pub ACCEL);
22 | 
23 | impl std::fmt::Debug for Accel {
24 |     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
25 |         f.debug_struct("ACCEL")
26 |             .field("key", &self.0.key)
27 |             .field("cmd", &self.0.cmd)
28 |             .field("fVirt", &self.0.fVirt)
29 |             .finish()
30 |     }
31 | }
32 | 
33 | impl Deref for Accel {
34 |     type Target = ACCEL;
35 | 
36 |     fn deref(&self) -> &Self::Target {
37 |         &self.0
38 |     }
39 | }
40 | 
41 | impl DerefMut for Accel {
42 |     fn deref_mut(&mut self) -> &mut Self::Target {
43 |         &mut self.0
44 |     }
45 | }
46 | 
47 | // taken from winit's code base
48 | // https://github.com/rust-windowing/winit/blob/ee88e38f13fbc86a7aafae1d17ad3cd4a1e761df/src/platform_impl/windows/util.rs#L138
49 | pub fn get_instance_handle() -> windows_sys::Win32::Foundation::HMODULE {
50 |     // Gets the instance handle by taking the address of the
51 |     // pseudo-variable created by the microsoft linker:
52 |     // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483
53 | 
54 |     // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
55 |     // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance
56 | 
57 |     extern "C" {
58 |         static __ImageBase: windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;
59 |     }
60 | 
61 |     unsafe { &__ImageBase as *const _ as _ }
62 | }
63 | 
64 | #[inline(always)]
65 | pub unsafe fn get_window_long(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize {
66 |     #[cfg(target_pointer_width = "64")]
67 |     return unsafe { windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd, nindex) };
68 |     #[cfg(target_pointer_width = "32")]
69 |     return unsafe {
70 |         windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongW(hwnd, nindex) as isize
71 |     };
72 | }
73 | 
74 | #[inline(always)]
75 | pub unsafe fn set_window_long(
76 |     hwnd: HWND,
77 |     nindex: WINDOW_LONG_PTR_INDEX,
78 |     dwnewlong: isize,
79 | ) -> isize {
80 |     #[cfg(target_pointer_width = "64")]
81 |     return unsafe {
82 |         windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW(hwnd, nindex, dwnewlong)
83 |     };
84 |     #[cfg(target_pointer_width = "32")]
85 |     return unsafe {
86 |         windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongW(hwnd, nindex, dwnewlong as i32)
87 |             as isize
88 |     };
89 | }
90 | 


--------------------------------------------------------------------------------
/src/tray_icon_id.rs:
--------------------------------------------------------------------------------
 1 | use std::{convert::Infallible, str::FromStr};
 2 | 
 3 | /// An unique id that is associated with a tray icon.
 4 | #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
 5 | #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 6 | pub struct TrayIconId(pub String);
 7 | 
 8 | impl TrayIconId {
 9 |     /// Create a new tray icon id.
10 |     pub fn new<S: AsRef<str>>(id: S) -> Self {
11 |         Self(id.as_ref().to_string())
12 |     }
13 | }
14 | 
15 | impl AsRef<str> for TrayIconId {
16 |     fn as_ref(&self) -> &str {
17 |         self.0.as_ref()
18 |     }
19 | }
20 | 
21 | impl<T: ToString> From<T> for TrayIconId {
22 |     fn from(value: T) -> Self {
23 |         Self::new(value.to_string())
24 |     }
25 | }
26 | 
27 | impl FromStr for TrayIconId {
28 |     type Err = Infallible;
29 | 
30 |     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
31 |         Ok(Self::new(s))
32 |     }
33 | }
34 | 
35 | impl PartialEq<&str> for TrayIconId {
36 |     fn eq(&self, other: &&str) -> bool {
37 |         self.0 == *other
38 |     }
39 | }
40 | 
41 | impl PartialEq<&str> for &TrayIconId {
42 |     fn eq(&self, other: &&str) -> bool {
43 |         self.0 == *other
44 |     }
45 | }
46 | 
47 | impl PartialEq<String> for TrayIconId {
48 |     fn eq(&self, other: &String) -> bool {
49 |         self.0 == *other
50 |     }
51 | }
52 | 
53 | impl PartialEq<String> for &TrayIconId {
54 |     fn eq(&self, other: &String) -> bool {
55 |         self.0 == *other
56 |     }
57 | }
58 | 
59 | impl PartialEq<&String> for TrayIconId {
60 |     fn eq(&self, other: &&String) -> bool {
61 |         self.0 == **other
62 |     }
63 | }
64 | 
65 | impl PartialEq<&TrayIconId> for TrayIconId {
66 |     fn eq(&self, other: &&TrayIconId) -> bool {
67 |         other.0 == self.0
68 |     }
69 | }
70 | 
71 | #[cfg(test)]
72 | mod test {
73 |     use crate::TrayIconId;
74 | 
75 |     #[test]
76 |     fn is_eq() {
77 |         assert_eq!(TrayIconId::new("t"), "t",);
78 |         assert_eq!(TrayIconId::new("t"), String::from("t"));
79 |         assert_eq!(TrayIconId::new("t"), &String::from("t"));
80 |         assert_eq!(TrayIconId::new("t"), TrayIconId::new("t"));
81 |         assert_eq!(TrayIconId::new("t"), &TrayIconId::new("t"));
82 |         assert_eq!(&TrayIconId::new("t"), &TrayIconId::new("t"));
83 |         assert_eq!(TrayIconId::new("t").as_ref(), "t");
84 |     }
85 | }
86 | 


--------------------------------------------------------------------------------