├── .circleci
    └── config.yml
├── .github
    └── workflows
    │   └── rust.yml
├── .gitignore
├── .vscode
    └── settings.json
├── Cargo.toml
├── LICENSE
├── README.md
├── examples
    ├── linux-edit-menu-items
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   └── src
    │   │   └── main.rs
    ├── linux-embeded-icon
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   └── src
    │   │   └── main.rs
    ├── linux-gresources
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── build.rs
    │   └── src
    │   │   └── main.rs
    ├── linux-named-resource
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   └── src
    │   │   └── main.rs
    ├── macos.rs
    ├── resources
    │   ├── tray-icon.xml
    │   ├── tray_icon-green.png
    │   └── tray_icon-red.png
    ├── windows-edit-menu-items
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── build.rs
    │   ├── icons
    │   │   ├── app-icon.ico
    │   │   ├── icon-green.ico
    │   │   └── icon-red.ico
    │   ├── src
    │   │   └── main.rs
    │   └── tray-example.rc
    └── windows
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── build.rs
    │   ├── icons
    │       ├── app-icon.ico
    │       ├── icon-green.ico
    │       └── icon-red.ico
    │   ├── src
    │       └── main.rs
    │   └── tray-example.rc
├── src
    ├── api
    │   ├── linux_ksni
    │   │   └── mod.rs
    │   ├── linux_libappindicator
    │   │   └── mod.rs
    │   ├── macos
    │   │   ├── callback.rs
    │   │   └── mod.rs
    │   ├── mod.rs
    │   └── windows
    │   │   ├── funcs.rs
    │   │   ├── mod.rs
    │   │   └── structs.rs
    ├── error.rs
    └── lib.rs
└── tools
    ├── build-linux.sh
    ├── build-macos.sh
    ├── build-windows.sh
    ├── linux-build-container.sh
    └── windows-build-container.sh


/.circleci/config.yml:
--------------------------------------------------------------------------------
 1 | version: 2
 2 | 
 3 | jobs:
 4 |   linux:
 5 |     docker:
 6 |       - image: olback/rust-linux-gtk
 7 | 
 8 |     steps:
 9 |       - checkout
10 | 
11 |       - run:
12 |           name: Change Permissions for Cargo Cache
13 |           command: |
14 |                     if [ -d "/usr/local/cargo" ]; then
15 |                       sudo chown -R circleci:circleci /usr/local/cargo
16 |                     fi
17 |       - restore_cache:
18 |           key: stable-cache-v1-{{ checksum "Cargo.toml" }}
19 | 
20 |       - run:
21 |           name: Show Version
22 |           command: |
23 |                     rustc --version --verbose
24 |                     rustup --version
25 |                     cargo --version --verbose
26 | 
27 |       - run:
28 |           name: Run Check (ksni)
29 |           command: cargo check --verbose --features ksni
30 | 
31 |       - run:
32 |           name: Run Check (libappindicator)
33 |           command: cargo check --verbose --features libappindicator
34 | 
35 |       - run:
36 |           name: Run Check on examples/linux-edit-menu-items
37 |           command: cd examples/linux-edit-menu-items && cargo check --verbose
38 | 
39 |       - run:
40 |           name: Run Check on examples/linux-embeded-icon
41 |           command: cd examples/linux-embeded-icon && cargo check --verbose
42 | 
43 |       - run:
44 |           name: Run Check examples/linux-gresources
45 |           command: cd examples/linux-gresources && cargo check --verbose
46 | 
47 |       - run:
48 |           name: Run Check on examples/linux-named-resource
49 |           command: cd examples/linux-named-resource && cargo check --verbose
50 | 
51 |       - save_cache:
52 |           key: stable-cache-v1-{{ checksum "Cargo.toml" }}
53 |           paths:
54 |             - "~/.cargo/"
55 |             - "~/.rustup/"
56 |             - "./target"
57 |             - "/usr/local/cargo"
58 | 
59 | workflows:
60 |   version: 2
61 |   check:
62 |     jobs:
63 |       - linux
64 | 
65 | 


--------------------------------------------------------------------------------
/.github/workflows/rust.yml:
--------------------------------------------------------------------------------
 1 | name: Cargo Check
 2 | 
 3 | on:
 4 |   push:
 5 |     branches: [master]
 6 |   pull_request:
 7 |     branches: [master]
 8 | 
 9 | env:
10 |   CARGO_TERM_COLOR: always
11 | 
12 | jobs:
13 |   check:
14 |     runs-on: ${{ matrix.os }}
15 |     strategy:
16 |       matrix:
17 |         os: [macos-latest, windows-latest]
18 | 
19 |     steps:
20 |       - uses: actions/checkout@v2
21 |       - name: Build
22 |         run: cargo check --verbose
23 | 


--------------------------------------------------------------------------------
/.gitignore:
--------------------------------------------------------------------------------
1 | /target
2 | Cargo.lock
3 | 


--------------------------------------------------------------------------------
/.vscode/settings.json:
--------------------------------------------------------------------------------
1 | {
2 |     "rust-analyzer.cargo.features": [
3 |         "libappindicator"
4 |     ]
5 | }


--------------------------------------------------------------------------------
/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "tray-item"
 3 | version = "0.10.0"
 4 | authors = ["Edwin Svensson <trayitemrs@olback.net>"]
 5 | edition = "2021"
 6 | description = "Super simple API to make tray icons/menus on Windows, Mac & Linux"
 7 | homepage = "https://github.com/olback/tray-item-rs"
 8 | repository = "https://github.com/olback/tray-item-rs"
 9 | readme = "README.md"
10 | keywords = ["gui", "tray", "desktop", "tray-item"]
11 | license = "MIT"
12 | 
13 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
14 | [features]
15 | ksni = ["dep:ksni"]
16 | libappindicator = ["dep:libappindicator", "dep:gtk"]
17 | 
18 | [dependencies]
19 | ksni = { version = "0.2.0", optional = true }
20 | libappindicator = { version = "0.9", optional = true } # Tray icon
21 | gtk = { version = "0.18", optional = true }
22 | 
23 | [target.'cfg(target_os="windows")'.dependencies]
24 | padlock = "0.2"
25 | 
26 | [target.'cfg(target_os="windows")'.dependencies.windows-sys]
27 | version = "0.52.0"
28 | features = [
29 |     "Win32_Foundation",
30 |     "Win32_Graphics_Gdi",
31 |     "Win32_System_LibraryLoader",
32 |     "Win32_UI_Shell",
33 |     "Win32_UI_WindowsAndMessaging",
34 | ]
35 | 
36 | [target.'cfg(target_os="macos")'.dependencies]
37 | cocoa = "0.25"
38 | objc = "0.2"
39 | core-graphics = "0.23"
40 | objc-foundation = "0.1"
41 | objc_id = "0.1"
42 | libc = "0.2"
43 | 
44 | [profile.release]
45 | strip = true
46 | opt-level = "z"
47 | lto = true
48 | incremental = false
49 | codegen-units = 1
50 | panic = "abort"
51 | 


--------------------------------------------------------------------------------
/LICENSE:
--------------------------------------------------------------------------------
 1 | MIT License
 2 | 
 3 | Copyright (c) 2020 Edwin
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
/README.md:
--------------------------------------------------------------------------------
 1 | # Multi-platform Tray Indicator
 2 | 
 3 | [![Cargo Check](https://github.com/olback/tray-item-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/olback/tray-item-rs/actions/workflows/rust.yml) [![CircleCI](https://circleci.com/gh/olback/tray-item-rs/tree/master.svg?style=svg)](https://circleci.com/gh/olback/tray-item-rs/tree/master)
 4 | 
 5 | Please see the [examples](https://github.com/olback/tray-item-rs/tree/master/examples) as documentation is currently lacking.
 6 | 
 7 | Tray Indicator uses icons from gresources on Linux and `.rc`-files on Windows.  
 8 | These resourses have to be packed into the final binary.
 9 | 
10 | * [x] Linux
11 | * [x] Windows
12 | * [x] MacOS*
13 | 
14 | \* MacOS does not allow running applications in threads other than main, meaning that
15 | it is not possible to listen for events in a new thread. See the `macos.rs` example for a how-to.
16 | 
17 | ### Todo:
18 | * [ ] Docs
19 | 


--------------------------------------------------------------------------------
/examples/linux-edit-menu-items/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/linux-edit-menu-items/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "linux-edit-menu-items"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | tray-item = { path = "../../", features = ["ksni"] }
10 | png = "0.16"
11 | 


--------------------------------------------------------------------------------
/examples/linux-edit-menu-items/src/main.rs:
--------------------------------------------------------------------------------
 1 | use {std::io::Cursor, std::sync::mpsc, tray_item::IconSource, tray_item::TrayItem};
 2 | 
 3 | enum Message {
 4 |     Quit,
 5 |     Update,
 6 | }
 7 | 
 8 | fn main() {
 9 |     let cursor_red = Cursor::new(include_bytes!("../../resources/tray_icon-red.png"));
10 |     let decoder_red = png::Decoder::new(cursor_red);
11 |     let (info_red, mut reader_red) = decoder_red.read_info().unwrap();
12 |     let mut buf_red = vec![0; info_red.buffer_size()];
13 |     reader_red.next_frame(&mut buf_red).unwrap();
14 | 
15 |     let icon_red = IconSource::Data {
16 |         data: buf_red,
17 |         height: 32,
18 |         width: 32,
19 |     };
20 | 
21 |     let mut tray = TrayItem::new("Tray Example", icon_red).unwrap();
22 | 
23 |     tray.add_label("Tray Label").unwrap();
24 | 
25 |     let (tx, rx) = mpsc::sync_channel::<Message>(2);
26 |     let update_tx = tx.clone();
27 |     let id_menu = tray
28 |         .inner_mut()
29 |         .add_menu_item_with_id("Update Menu Item", move || {
30 |             update_tx.send(Message::Update).unwrap();
31 |         })
32 |         .unwrap();
33 | 
34 |     let quit_tx = tx.clone();
35 |     tray.add_menu_item("Quit", move || {
36 |         quit_tx.send(Message::Quit).unwrap();
37 |     })
38 |     .unwrap();
39 | 
40 |     loop {
41 |         match rx.recv() {
42 |             Ok(Message::Quit) => {
43 |                 println!("Quit");
44 |                 break;
45 |             }
46 |             Ok(Message::Update) => {
47 |                 println!("Update Menu Item!");
48 |                 tray.inner_mut()
49 |                     .set_menu_item_label("Menu Updated", id_menu)
50 |                     .unwrap();
51 |             }
52 |             _ => {}
53 |         }
54 |     }
55 | }
56 | 


--------------------------------------------------------------------------------
/examples/linux-embeded-icon/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/linux-embeded-icon/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "linux-embeded-icon"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | tray-item = { path = "../../", features = ["ksni"] }
10 | png = "0.16"
11 | 


--------------------------------------------------------------------------------
/examples/linux-embeded-icon/src/main.rs:
--------------------------------------------------------------------------------
 1 | use {
 2 |     std::io::Cursor,
 3 |     std::sync::mpsc,
 4 |     tray_item::TrayItem,
 5 |     tray_item::IconSource
 6 | };
 7 | 
 8 | enum Message {
 9 |     Quit,
10 |     Red,
11 |     Green
12 | }
13 | 
14 | fn main() {
15 |     let cursor_red = Cursor::new(include_bytes!("../../resources/tray_icon-red.png"));
16 |     let decoder_red = png::Decoder::new(cursor_red);
17 |     let (info_red, mut reader_red) = decoder_red.read_info().unwrap();
18 |     let mut buf_red = vec![0;info_red.buffer_size()];
19 |     reader_red.next_frame(&mut buf_red).unwrap();
20 | 
21 |     let icon_red = IconSource::Data{data: buf_red, height: 32, width: 32};
22 | 
23 |     let mut tray = TrayItem::new("Tray Example", icon_red).unwrap();
24 | 
25 |     tray.add_label("Tray Label").unwrap();
26 | 
27 |     let (tx, rx) = mpsc::sync_channel::<Message>(2);
28 |     let green_tx = tx.clone();
29 |     tray.add_menu_item("Set icon green", move || {
30 |         green_tx.send(Message::Green).unwrap();
31 |     })
32 |     .unwrap();
33 | 
34 |     let red_tx = tx.clone();
35 |     tray.add_menu_item("Set icon red", move || {
36 |         red_tx.send(Message::Red).unwrap();
37 |     })
38 |     .unwrap();
39 | 
40 |     let quit_tx = tx.clone();
41 |     tray.add_menu_item("Quit", move || {
42 |         quit_tx.send(Message::Quit).unwrap();
43 |     })
44 |     .unwrap();
45 | 
46 |     loop {
47 |         match rx.recv() {
48 |             Ok(Message::Quit) => {
49 |                 println!("Quit");
50 |                 break
51 |             },
52 |             Ok(Message::Green) =>{
53 |                 println!("Green!");
54 |                 let cursor_green = Cursor::new(include_bytes!("../../resources/tray_icon-green.png"));
55 |                 let decoder_green = png::Decoder::new(cursor_green);
56 |                 let (info_green, mut reader_green) = decoder_green.read_info().unwrap();
57 |                 let mut buf_green = vec![0;info_green.buffer_size()];
58 |                 reader_green.next_frame(&mut buf_green).unwrap();
59 |                 let icon_green = IconSource::Data{data: buf_green, height: 32, width: 32};
60 |                 tray.set_icon(icon_green).unwrap();
61 |             },
62 |             Ok(Message::Red) => {
63 |                 println!("Red!");
64 |                 let cursor_red = Cursor::new(include_bytes!("../../resources/tray_icon-red.png"));
65 |                 let decoder_red = png::Decoder::new(cursor_red);
66 |                 let (info_red, mut reader_red) = decoder_red.read_info().unwrap();
67 |                 let mut buf_red = vec![0;info_red.buffer_size()];
68 |                 reader_red.next_frame(&mut buf_red).unwrap();
69 |                 let icon_red = IconSource::Data{data: buf_red, height: 32, width: 32};
70 |                 tray.set_icon(icon_red).unwrap();
71 |             },
72 |             _ => {}
73 |         }
74 |     }
75 | 
76 | }
77 | 


--------------------------------------------------------------------------------
/examples/linux-gresources/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/linux-gresources/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "linux-gresource-example"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | build = "build.rs"
 6 | 
 7 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 8 | 
 9 | [dependencies]
10 | tray-item = { path = "../../", features = ["libappindicator"] }
11 | gtk = "0.18"
12 | gio = "0.18"
13 | glib = "0.18"
14 | 
15 | [build-dependencies]
16 | glib-build-tools = "0.18"
17 | 


--------------------------------------------------------------------------------
/examples/linux-gresources/build.rs:
--------------------------------------------------------------------------------
1 | fn main() {
2 |     glib_build_tools::compile_resources(
3 |         &["../resources"],
4 |         "../resources/tray-icon.xml",
5 |         "compiled.gresource",
6 |     );
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/linux-gresources/src/main.rs:
--------------------------------------------------------------------------------
  1 | use {
  2 |     gio::ResourceLookupFlags, std::sync::mpsc, std::thread, tray_item::IconSource,
  3 |     tray_item::TrayItem,
  4 | };
  5 | 
  6 | enum Message {
  7 |     Quit,
  8 |     NOP,
  9 |     Red,
 10 |     Green,
 11 | }
 12 | 
 13 | fn main() {
 14 |     gtk::init().unwrap();
 15 | 
 16 |     // gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");
 17 |     let res_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/compiled.gresource"));
 18 |     let data = gtk::glib::Bytes::from(&res_bytes[..]);
 19 |     let resource = gio::Resource::from_data(&data).unwrap();
 20 |     gio::resources_register(&resource);
 21 |     let children = resource.enumerate_children("/", ResourceLookupFlags::all());
 22 |     print!("{:#?}", children);
 23 | 
 24 |     let png = gio::resources_lookup_data("/name-of-icon-in-rc-file", ResourceLookupFlags::all())
 25 |         .expect("Failed to load png");
 26 |     println!("png size: {}", png.len());
 27 | 
 28 |     let mut tray = TrayItem::new(
 29 |         "Tray Example",
 30 |         IconSource::Resource("/name-of-icon-in-rc-file"),
 31 |     )
 32 |     .unwrap();
 33 | 
 34 |     tray.add_label("Tray Label").unwrap();
 35 | 
 36 |     let (tx, rx) = mpsc::sync_channel::<Message>(2);
 37 |     let green_tx = tx.clone();
 38 |     tray.add_menu_item("Set icon green", move || {
 39 |         green_tx.send(Message::Green).unwrap();
 40 |     })
 41 |     .unwrap();
 42 | 
 43 |     let red_tx = tx.clone();
 44 |     tray.add_menu_item("Set icon red", move || {
 45 |         red_tx.send(Message::Red).unwrap();
 46 |     })
 47 |     .unwrap();
 48 | 
 49 |     let quit_tx = tx.clone();
 50 |     tray.add_menu_item("Quit", move || {
 51 |         quit_tx.send(Message::Quit).unwrap();
 52 |     })
 53 |     .unwrap();
 54 | 
 55 |     glib::idle_add_local(move || match rx.recv() {
 56 |         Ok(Message::Quit) => {
 57 |             gtk::main_quit();
 58 |             println!("Quit!");
 59 |             glib::ControlFlow::Break
 60 |         }
 61 |         Ok(Message::Green) => {
 62 |             println!("Green!");
 63 |             tray.set_icon(IconSource::Resource("/another-name-from-rc-file"))
 64 |                 .unwrap();
 65 |             glib::ControlFlow::Continue
 66 |         }
 67 |         Ok(Message::Red) => {
 68 |             println!("Red!");
 69 |             tray.set_icon(IconSource::Resource("/name-of-icon-in-rc-file"))
 70 |                 .unwrap();
 71 |             glib::ControlFlow::Continue
 72 |         }
 73 |         _ => {
 74 |             println!("Default!");
 75 |             glib::ControlFlow::Continue
 76 |         }
 77 |     });
 78 | 
 79 |     thread::spawn(move || {
 80 |         let mut count = 0;
 81 |         loop {
 82 |             // Menu doesn't show up until after hitting enter a few times?
 83 |             //let mut s = String::new();
 84 |             //std::io::stdin().read_line(&mut s).unwrap();
 85 |             //if s.as_bytes()[0] == b'q' {
 86 |             //    println!("stopping thread loop!");
 87 |             //    break
 88 |             //}
 89 | 
 90 |             // glib::idle_add_local doesn't loop without this?
 91 |             count += 1;
 92 |             thread::sleep(std::time::Duration::from_millis(10));
 93 |             if count % 100 == 0 {
 94 |                 tx.send(Message::NOP).unwrap();
 95 |                 println!("Idle loop, {}!", count);
 96 |             }
 97 |         }
 98 |     });
 99 |     gtk::main();
100 | }
101 | 


--------------------------------------------------------------------------------
/examples/linux-named-resource/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/linux-named-resource/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "linux-named-resource-example"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | tray-item = { path = "../../", features = ["libappindicator"] }
10 | gtk = "0.18"
11 | 


--------------------------------------------------------------------------------
/examples/linux-named-resource/src/main.rs:
--------------------------------------------------------------------------------
 1 | use tray_item::{TrayItem, IconSource};
 2 | 
 3 | fn main() {
 4 |     gtk::init().unwrap();
 5 | 
 6 |     let mut tray = TrayItem::new("Tray Example", IconSource::Resource("accessories-calculator")).unwrap();
 7 | 
 8 |     tray.add_label("Tray Label").unwrap();
 9 | 
10 |     tray.add_menu_item("Hello", || {
11 |         println!("Hello!");
12 |     }).unwrap();
13 | 
14 |     tray.add_menu_item("Quit", || {
15 |         gtk::main_quit();
16 |     }).unwrap();
17 | 
18 |     gtk::main();
19 | }
20 | 


--------------------------------------------------------------------------------
/examples/macos.rs:
--------------------------------------------------------------------------------
 1 | use tray_item::{TrayItem, IconSource};
 2 | 
 3 | fn main() {
 4 | 
 5 |     let mut tray = TrayItem::new("Tray Example", IconSource::Resource("")).unwrap();
 6 | 
 7 |     tray.add_label("Tray Label").unwrap();
 8 | 
 9 |     tray.add_menu_item("Hello", || {
10 |         println!("Hello!");
11 |     }).unwrap();
12 | 
13 |     let mut inner = tray.inner_mut();
14 |     inner.add_quit_item("Quit");
15 |     inner.display();
16 | 
17 | }
18 | 


--------------------------------------------------------------------------------
/examples/resources/tray-icon.xml:
--------------------------------------------------------------------------------
1 | <?xml version="1.0" encoding="UTF-8"?>
2 | <gresources>
3 |   <gresource prefix="">
4 |     <file alias="name-of-icon-in-rc-file">tray_icon-red.png</file>
5 |     <file alias="another-name-from-rc-file">tray_icon-green.png</file>
6 |   </gresource>
7 | </gresources>


--------------------------------------------------------------------------------
/examples/resources/tray_icon-green.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/resources/tray_icon-green.png


--------------------------------------------------------------------------------
/examples/resources/tray_icon-red.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/resources/tray_icon-red.png


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "windows-edit-menu-items"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | tray-item = { path = "../../" }
10 | 
11 | [build-dependencies]
12 | windres = "*"
13 | 


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/build.rs:
--------------------------------------------------------------------------------
1 | use windres::Build;
2 | 
3 | fn main() {
4 |     Build::new().compile("tray-example.rc").unwrap();
5 | }
6 | 


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/icons/app-icon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows-edit-menu-items/icons/app-icon.ico


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/icons/icon-green.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows-edit-menu-items/icons/icon-green.ico


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/icons/icon-red.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows-edit-menu-items/icons/icon-red.ico


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/src/main.rs:
--------------------------------------------------------------------------------
 1 | use std::sync::mpsc;
 2 | use tray_item::{IconSource, TrayItem};
 3 | 
 4 | enum Message {
 5 |     Quit,
 6 |     ChangeIcon,
 7 |     Hello,
 8 | }
 9 | 
10 | enum Icon {
11 |     Red,
12 |     Green,
13 | }
14 | 
15 | impl Icon {
16 |     fn resource(&self) -> IconSource {
17 |         match self {
18 |             Self::Red => IconSource::Resource("another-name-from-rc-file"),
19 |             Self::Green => IconSource::Resource("name-of-icon-in-rc-file"),
20 |         }
21 |     }
22 | }
23 | 
24 | fn main() {
25 |     let mut tray = TrayItem::new(
26 |         "Tray Example",
27 |         Icon::Green.resource(),
28 |     )
29 |     .unwrap();
30 | 
31 |     let label_id = tray.inner_mut().add_label_with_id("Tray Label").unwrap();
32 | 
33 |     tray.inner_mut().add_separator().unwrap();
34 | 
35 |     let (tx, rx) = mpsc::sync_channel(1);
36 | 
37 |     let hello_tx = tx.clone();
38 |     tray.add_menu_item("Hello!", move || {
39 |         hello_tx.send(Message::Hello).unwrap();
40 |     })
41 |     .unwrap();
42 | 
43 |     let color_tx = tx.clone();
44 |     let color_id = tray.inner_mut().add_menu_item_with_id("Change to Red", move || {
45 |         color_tx.send(Message::ChangeIcon).unwrap();
46 |     })
47 |     .unwrap();
48 |     let mut current_icon = Icon::Green;
49 | 
50 |     tray.inner_mut().add_separator().unwrap();
51 | 
52 |     let quit_tx = tx.clone();
53 |     tray.add_menu_item("Quit", move || {
54 |         quit_tx.send(Message::Quit).unwrap();
55 |     })
56 |     .unwrap();
57 | 
58 |     loop {
59 |         match rx.recv() {
60 |             Ok(Message::Quit) => {
61 |                 println!("Quit");
62 |                 break;
63 |             }
64 |             Ok(Message::ChangeIcon) => {
65 |                 let (next_icon, next_message) = match current_icon {
66 |                     Icon::Red => (Icon::Green, "Change to Red"),
67 |                     Icon::Green => (Icon::Red, "Change to Green"),
68 |                 };
69 |                 current_icon = next_icon;
70 | 
71 |                 tray.inner_mut().set_menu_item_label(next_message, color_id).unwrap();
72 |                 tray.set_icon(current_icon.resource())
73 |                     .unwrap();
74 |             },
75 |             Ok(Message::Hello) => {
76 |                 tray.inner_mut().set_label("Hi there!", label_id).unwrap();
77 |             },
78 |             _ => {}
79 |         }
80 |     }
81 | }
82 | 


--------------------------------------------------------------------------------
/examples/windows-edit-menu-items/tray-example.rc:
--------------------------------------------------------------------------------
1 | aa-exe-icon ICON "icons/app-icon.ico"
2 | name-of-icon-in-rc-file ICON "icons/icon-green.ico"
3 | another-name-from-rc-file ICON "icons/icon-red.ico"
4 | 


--------------------------------------------------------------------------------
/examples/windows/.gitignore:
--------------------------------------------------------------------------------
1 | target/
2 | 
3 | 


--------------------------------------------------------------------------------
/examples/windows/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "windows"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | tray-item = { path = "../../" }
10 | 
11 | [build-dependencies]
12 | embed-resource = "2.3"
13 | 


--------------------------------------------------------------------------------
/examples/windows/build.rs:
--------------------------------------------------------------------------------
1 | extern crate embed_resource;
2 | 
3 | fn main() {
4 |     embed_resource::compile("tray-example.rc", embed_resource::NONE);
5 | }
6 | 


--------------------------------------------------------------------------------
/examples/windows/icons/app-icon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows/icons/app-icon.ico


--------------------------------------------------------------------------------
/examples/windows/icons/icon-green.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows/icons/icon-green.ico


--------------------------------------------------------------------------------
/examples/windows/icons/icon-red.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/olback/tray-item-rs/07b6e4802e0536e830be8f021d76989105849174/examples/windows/icons/icon-red.ico


--------------------------------------------------------------------------------
/examples/windows/src/main.rs:
--------------------------------------------------------------------------------
 1 | use std::sync::mpsc;
 2 | use tray_item::{IconSource, TrayItem};
 3 | 
 4 | enum Message {
 5 |     Quit,
 6 |     Green,
 7 |     Red,
 8 | }
 9 | 
10 | fn main() {
11 |     let mut tray = TrayItem::new(
12 |         "Tray Example",
13 |         IconSource::Resource("name-of-icon-in-rc-file"),
14 |     )
15 |     .unwrap();
16 | 
17 |     tray.add_label("Tray Label").unwrap();
18 | 
19 |     tray.add_menu_item("Hello", || {
20 |         println!("Hello!");
21 |     })
22 |     .unwrap();
23 | 
24 |     tray.inner_mut().add_separator().unwrap();
25 | 
26 |     let (tx, rx) = mpsc::sync_channel(1);
27 | 
28 |     let red_tx = tx.clone();
29 |     tray.add_menu_item("Red", move || {
30 |         red_tx.send(Message::Red).unwrap();
31 |     })
32 |     .unwrap();
33 | 
34 |     let green_tx = tx.clone();
35 |     tray.add_menu_item("Green", move || {
36 |         green_tx.send(Message::Green).unwrap();
37 |     })
38 |     .unwrap();
39 | 
40 |     tray.inner_mut().add_separator().unwrap();
41 | 
42 |     let quit_tx = tx.clone();
43 |     tray.add_menu_item("Quit", move || {
44 |         quit_tx.send(Message::Quit).unwrap();
45 |     })
46 |     .unwrap();
47 | 
48 |     loop {
49 |         match rx.recv() {
50 |             Ok(Message::Quit) => {
51 |                 println!("Quit");
52 |                 break;
53 |             }
54 |             Ok(Message::Red) => {
55 |                 println!("Red");
56 |                 tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
57 |                     .unwrap();
58 |             }
59 |             Ok(Message::Green) => {
60 |                 println!("Green");
61 |                 tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
62 |                     .unwrap()
63 |             }
64 |             _ => {}
65 |         }
66 |     }
67 | }
68 | 


--------------------------------------------------------------------------------
/examples/windows/tray-example.rc:
--------------------------------------------------------------------------------
1 | aa-exe-icon ICON "icons/app-icon.ico"
2 | name-of-icon-in-rc-file ICON "icons/icon-green.ico"
3 | another-name-from-rc-file ICON "icons/icon-red.ico"
4 | tray-default ICON "icons/app-icon.ico"
5 | 


--------------------------------------------------------------------------------
/src/api/linux_ksni/mod.rs:
--------------------------------------------------------------------------------
  1 | use crate::{IconSource, TIError};
  2 | use ksni::{menu::StandardItem, Handle, Icon};
  3 | use std::sync::{Arc, Mutex};
  4 | 
  5 | enum TrayItem {
  6 |     Label(String),
  7 |     MenuItem {
  8 |         id: u32,
  9 |         label: String,
 10 |         action: Arc<dyn Fn() + Send + Sync + 'static>,
 11 |     },
 12 |     Separator,
 13 | }
 14 | 
 15 | struct Tray {
 16 |     title: String,
 17 |     icon: IconSource,
 18 |     actions: Vec<TrayItem>,
 19 |     next_id: u32,
 20 | }
 21 | 
 22 | pub struct TrayItemLinux {
 23 |     tray: Handle<Tray>,
 24 | }
 25 | 
 26 | impl ksni::Tray for Tray {
 27 |     fn id(&self) -> String {
 28 |         self.title.clone()
 29 |     }
 30 | 
 31 |     fn title(&self) -> String {
 32 |         self.title.clone()
 33 |     }
 34 | 
 35 |     fn icon_name(&self) -> String {
 36 |         match &self.icon {
 37 |             IconSource::Resource(name) => name.to_string(),
 38 |             IconSource::Data { .. } => String::new(),
 39 |         }
 40 |     }
 41 | 
 42 |     fn icon_pixmap(&self) -> Vec<Icon> {
 43 |         match &self.icon {
 44 |             IconSource::Resource(_) => vec![],
 45 |             IconSource::Data {
 46 |                 data,
 47 |                 height,
 48 |                 width,
 49 |             } => {
 50 |                 vec![Icon {
 51 |                     width: *height,
 52 |                     height: *width,
 53 |                     data: data.clone(),
 54 |                 }]
 55 |             }
 56 |         }
 57 |     }
 58 | 
 59 |     fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
 60 |         self.actions
 61 |             .iter()
 62 |             .map(|item| match item {
 63 |                 TrayItem::Label(label) => StandardItem {
 64 |                     label: label.clone(),
 65 |                     enabled: false,
 66 |                     ..Default::default()
 67 |                 }
 68 |                 .into(),
 69 |                 TrayItem::MenuItem { label, action, .. } => {
 70 |                     let action = action.clone();
 71 |                     StandardItem {
 72 |                         label: label.clone(),
 73 |                         activate: Box::new(move |_| {
 74 |                             action();
 75 |                         }),
 76 |                         ..Default::default()
 77 |                     }
 78 |                     .into()
 79 |                 }
 80 |                 TrayItem::Separator => ksni::MenuItem::Separator,
 81 |             })
 82 |             .collect()
 83 |     }
 84 | }
 85 | 
 86 | impl TrayItemLinux {
 87 |     pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
 88 |         let svc = ksni::TrayService::new(Tray {
 89 |             title: title.to_string(),
 90 |             icon,
 91 |             actions: vec![],
 92 |             next_id: 0,
 93 |         });
 94 | 
 95 |         let handle = svc.handle();
 96 |         svc.spawn();
 97 | 
 98 |         Ok(Self { tray: handle })
 99 |     }
100 | 
101 |     pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
102 |         self.tray.update(|tray| tray.icon = icon.clone());
103 | 
104 |         Ok(())
105 |     }
106 | 
107 |     pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
108 |         self.tray.update(move |tray| {
109 |             tray.actions.push(TrayItem::Label(label.to_string()));
110 |         });
111 | 
112 |         Ok(())
113 |     }
114 | 
115 |     pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
116 |     where
117 |         F: Fn() -> () + Send + Sync + 'static,
118 |     {
119 |         self.add_menu_item_with_id(label, cb)?;
120 |         Ok(())
121 |     }
122 | 
123 |     pub fn add_menu_item_with_id<F>(&mut self, label: &str, cb: F) -> Result<u32, TIError>
124 |     where
125 |         F: Fn() + Send + Sync + 'static,
126 |     {
127 |         let action = Arc::new(cb);
128 |         let item_id = Arc::new(Mutex::new(0));
129 |         let item_id_clone = Arc::clone(&item_id);
130 | 
131 |         self.tray.update(move |tray| {
132 |             let mut id = item_id_clone.lock().unwrap();
133 |             *id = tray.next_id;
134 |             tray.next_id += 1;
135 | 
136 |             tray.actions.push(TrayItem::MenuItem {
137 |                 id: *id,
138 |                 label: label.to_string(),
139 |                 action: action.clone(),
140 |             });
141 |         });
142 | 
143 |         let final_id = *item_id.lock().unwrap();
144 |         Ok(final_id)
145 |     }
146 | 
147 |     pub fn set_menu_item_label(&mut self, label: &str, id: u32) -> Result<(), TIError> {
148 |         self.tray.update(move |tray| {
149 |             if let Some(item) = tray.actions.iter_mut().find_map(|item| match item {
150 |                 TrayItem::MenuItem {
151 |                     id: item_id, label, ..
152 |                 } if *item_id == id => Some(label),
153 |                 _ => None,
154 |             }) {
155 |                 *item = label.to_string();
156 |             }
157 |         });
158 | 
159 |         Ok(())
160 |     }
161 | 
162 |     pub fn add_separator(&mut self) -> Result<(), TIError> {
163 |         self.tray.update(move |tray| {
164 |             tray.actions.push(TrayItem::Separator);
165 |         });
166 | 
167 |         Ok(())
168 |     }
169 | }
170 | 


--------------------------------------------------------------------------------
/src/api/linux_libappindicator/mod.rs:
--------------------------------------------------------------------------------
 1 | use {
 2 |     crate::{IconSource, TIError},
 3 |     gtk::prelude::*,
 4 |     libappindicator::{AppIndicator, AppIndicatorStatus},
 5 | };
 6 | 
 7 | pub struct TrayItemLinux {
 8 |     tray: AppIndicator,
 9 |     menu: gtk::Menu,
10 | }
11 | 
12 | impl TrayItemLinux {
13 |     pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
14 |         let mut t = Self {
15 |             tray: AppIndicator::new(title, icon.as_str()),
16 |             menu: gtk::Menu::new(),
17 |         };
18 | 
19 |         t.set_icon(icon)?;
20 | 
21 |         Ok(t)
22 |     }
23 | 
24 |     pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
25 |         self.tray.set_icon(icon.as_str());
26 |         self.tray.set_status(AppIndicatorStatus::Active);
27 | 
28 |         Ok(())
29 |     }
30 | 
31 |     pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
32 |         let item = gtk::MenuItem::with_label(label);
33 |         item.set_sensitive(false);
34 |         self.menu.append(&item);
35 |         self.menu.show_all();
36 |         self.tray.set_menu(&mut self.menu);
37 | 
38 |         Ok(())
39 |     }
40 | 
41 |     pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
42 |     where
43 |         F: Fn() + Send + 'static,
44 |     {
45 |         let item = gtk::MenuItem::with_label(label);
46 |         item.connect_activate(move |_| {
47 |             cb();
48 |         });
49 |         self.menu.append(&item);
50 |         self.menu.show_all();
51 |         self.tray.set_menu(&mut self.menu);
52 | 
53 |         Ok(())
54 |     }
55 | 
56 |     pub fn add_separator(&mut self) -> Result<(), TIError> {
57 |         let item = gtk::SeparatorMenuItem::new();
58 |         self.menu.append(&item);
59 |         self.menu.show_all();
60 |         self.tray.set_menu(&mut self.menu);
61 | 
62 |         Ok(())
63 |     }
64 | }
65 | 


--------------------------------------------------------------------------------
/src/api/macos/callback.rs:
--------------------------------------------------------------------------------
 1 | use {
 2 |     libc::c_void,
 3 |     objc::{
 4 |         declare::ClassDecl,
 5 |         runtime::{Class, Object, Sel},
 6 |         sel, sel_impl, Message,
 7 |     },
 8 |     objc_foundation::{INSObject, NSObject},
 9 |     objc_id::Id,
10 |     std::mem,
11 | };
12 | 
13 | pub(crate) enum Callback {}
14 | unsafe impl Message for Callback {}
15 | 
16 | // SO.. some explanation is in order here.  We want to allow closure callbacks that
17 | // can modify their environment.  But we can't keep them on the $name object because
18 | // that is really just a stateless proxy for the objc object.  So we store them
19 | // as numeric pointers values in "ivar" fields on that object.  But, if we store a pointer to the
20 | // closure object, we'll run into issues with thin/fat pointer conversions (because
21 | // closure objects are trait objects and thus fat pointers).  So we wrap the closure in
22 | // another boxed object ($cbs_name), which, since it doesn't use traits, is actually a
23 | // regular "thin" pointer, and store THAT pointer in the ivar.  But...so...oy.
24 | pub(crate) struct CallbackState {
25 |     cb: Box<dyn Fn() -> ()>,
26 | }
27 | 
28 | impl Callback {
29 |     pub(crate) fn from(cb: Box<dyn Fn() -> ()>) -> Id<Self> {
30 |         let cbs = CallbackState { cb };
31 |         let bcbs = Box::new(cbs);
32 | 
33 |         let ptr = Box::into_raw(bcbs);
34 |         let ptr = ptr as *mut c_void as usize;
35 |         let mut oid = <Callback as INSObject>::new();
36 |         (*oid).setptr(ptr);
37 |         oid
38 |     }
39 | 
40 |     pub(crate) fn setptr(&mut self, uptr: usize) {
41 |         unsafe {
42 |             let obj = &mut *(self as *mut _ as *mut ::objc::runtime::Object);
43 |             obj.set_ivar("_cbptr", uptr);
44 |         }
45 |     }
46 | }
47 | 
48 | // TODO: Drop for $name doesn't get called, probably because objc manages the memory and
49 | // releases it for us.  so we leak the boxed callback right now.
50 | 
51 | impl INSObject for Callback {
52 |     fn class() -> &'static Class {
53 |         let cname = "Callback";
54 | 
55 |         let mut klass = Class::get(cname);
56 |         if klass.is_none() {
57 |             let superclass = NSObject::class();
58 |             let mut decl = ClassDecl::new(&cname, superclass).unwrap();
59 |             decl.add_ivar::<usize>("_cbptr");
60 | 
61 |             extern "C" fn sysbar_callback_call(this: &Object, _cmd: Sel) {
62 |                 unsafe {
63 |                     let pval: usize = *this.get_ivar("_cbptr");
64 |                     let ptr = pval as *mut c_void;
65 |                     let ptr = ptr as *mut CallbackState;
66 |                     let bcbs: Box<CallbackState> = Box::from_raw(ptr);
67 |                     {
68 |                         (*bcbs.cb)();
69 |                     }
70 |                     mem::forget(bcbs);
71 |                 }
72 |             }
73 | 
74 |             unsafe {
75 |                 decl.add_method(
76 |                     sel!(call),
77 |                     sysbar_callback_call as extern "C" fn(&Object, Sel),
78 |                 );
79 |             }
80 | 
81 |             decl.register();
82 |             klass = Class::get(cname);
83 |         }
84 |         klass.unwrap()
85 |     }
86 | }
87 | 


--------------------------------------------------------------------------------
/src/api/macos/mod.rs:
--------------------------------------------------------------------------------
  1 | use cocoa::{
  2 |     base::id,
  3 |     foundation::{NSData, NSSize},
  4 | };
  5 | 
  6 | use {
  7 |     crate::IconSource,
  8 |     crate::TIError,
  9 |     callback::*,
 10 |     cocoa::{
 11 |         appkit::{
 12 |             NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps, NSImage, NSMenu,
 13 |             NSMenuItem, NSRunningApplication, NSStatusBar, NSStatusItem, NSWindow,
 14 |         },
 15 |         base::{nil, YES},
 16 |         foundation::{NSAutoreleasePool, NSString},
 17 |     },
 18 |     objc::{msg_send, sel, sel_impl},
 19 |     std::thread::JoinHandle,
 20 | };
 21 | 
 22 | mod callback;
 23 | 
 24 | fn get_icon_image(icon: IconSource) -> Option<id> {
 25 |     unsafe {
 26 |         match icon {
 27 |             IconSource::Resource(_) => {
 28 |                 let icon = icon.as_str();
 29 |                 let icon = Some(icon).filter(|icon| !icon.is_empty());
 30 |                 icon.map(|icon_name| {
 31 |                     let icon_name = NSString::alloc(nil).init_str(icon_name);
 32 |                     NSImage::imageNamed_(NSImage::alloc(nil), icon_name)
 33 |                 })
 34 |             }
 35 |             IconSource::Data {
 36 |                 height,
 37 |                 width,
 38 |                 data,
 39 |             } => {
 40 |                 let data = NSData::dataWithBytes_length_(
 41 |                     nil,
 42 |                     data.as_ptr() as *const std::os::raw::c_void,
 43 |                     data.len() as u64,
 44 |                 );
 45 |                 let image = NSImage::initWithData_(NSImage::alloc(nil), data);
 46 |                 let new_size = if width != 0 && height != 0 {
 47 |                     let icon_height: f64 = 18.0;
 48 |                     let icon_width: f64 = (width as f64) / (height as f64 / icon_height);
 49 |                     NSSize::new(icon_width, icon_height)
 50 |                 } else {
 51 |                     NSSize::new(18.0, 18.0)
 52 |                 };
 53 |                 let _: () = msg_send![image, setSize: new_size];
 54 |                 Some(image)
 55 |             }
 56 |         }
 57 |     }
 58 | }
 59 | 
 60 | pub struct TrayItemMacOS {
 61 |     name: String,
 62 |     menu: *mut objc::runtime::Object,
 63 |     _pool: *mut objc::runtime::Object,
 64 |     icon: Option<*mut objc::runtime::Object>,
 65 |     main_thread: Option<JoinHandle<()>>,
 66 | }
 67 | 
 68 | impl TrayItemMacOS {
 69 |     pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
 70 |         let t = unsafe {
 71 |             let pool = NSAutoreleasePool::new(nil);
 72 | 
 73 |             TrayItemMacOS {
 74 |                 name: title.to_string(),
 75 |                 _pool: pool,
 76 |                 icon: get_icon_image(icon),
 77 |                 menu: NSMenu::new(nil).autorelease(),
 78 |                 main_thread: None,
 79 |             }
 80 |         };
 81 |         Ok(t)
 82 |     }
 83 | 
 84 |     pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
 85 |         self.icon = get_icon_image(icon);
 86 |         Ok(())
 87 |     }
 88 | 
 89 |     pub fn set_icon_template(&mut self, icon: &str) -> Result<(), TIError> {
 90 |         unsafe {
 91 |             let icon_name = NSString::alloc(nil).init_str(icon);
 92 |             let image = NSImage::imageNamed_(NSImage::alloc(nil), icon_name);
 93 |             let _: () = msg_send![image, setTemplate: YES];
 94 |             self.icon = Some(image);
 95 |         }
 96 |         Ok(())
 97 |     }
 98 | 
 99 |     pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
100 |         unsafe {
101 |             let no_key = NSString::alloc(nil).init_str(""); // TODO want this eventually
102 |             let itemtitle = NSString::alloc(nil).init_str(label);
103 |             let action = sel!(call);
104 |             let item = NSMenuItem::alloc(nil)
105 |                 .initWithTitle_action_keyEquivalent_(itemtitle, action, no_key);
106 |             let _: () = msg_send![item, setTitle: itemtitle];
107 | 
108 |             NSMenu::addItem_(self.menu, item);
109 |         }
110 | 
111 |         Ok(())
112 |     }
113 | 
114 |     pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
115 |     where
116 |         F: Fn() -> () + Send + 'static,
117 |     {
118 |         let cb_obj = Callback::from(Box::new(cb));
119 | 
120 |         unsafe {
121 |             let no_key = NSString::alloc(nil).init_str(""); // TODO want this eventually
122 |             let itemtitle = NSString::alloc(nil).init_str(label);
123 |             let action = sel!(call);
124 |             let item = NSMenuItem::alloc(nil)
125 |                 .initWithTitle_action_keyEquivalent_(itemtitle, action, no_key);
126 |             let _: () = msg_send![item, setTarget: cb_obj];
127 | 
128 |             NSMenu::addItem_(self.menu, item);
129 |         }
130 | 
131 |         Ok(())
132 |     }
133 | 
134 |     // private
135 | 
136 |     pub fn add_quit_item(&mut self, label: &str) {
137 |         unsafe {
138 |             let no_key = NSString::alloc(nil).init_str("");
139 |             let pref_item = NSString::alloc(nil).init_str(label);
140 |             let pref_action = sel!(terminate:);
141 |             let menuitem = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
142 |                 pref_item,
143 |                 pref_action,
144 |                 no_key,
145 |             );
146 | 
147 |             self.menu.addItem_(menuitem);
148 |         }
149 |     }
150 | 
151 |     pub fn display(&mut self) {
152 |         unsafe {
153 |             let app = NSApp();
154 |             app.activateIgnoringOtherApps_(YES);
155 | 
156 |             let item = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(-1.0);
157 |             let title = NSString::alloc(nil).init_str(&self.name);
158 |             if let Some(icon) = self.icon {
159 |                 let _: () = msg_send![item, setImage: icon];
160 |             } else {
161 |                 item.setTitle_(title);
162 |             }
163 |             item.setMenu_(self.menu);
164 | 
165 |             let current_app = NSRunningApplication::currentApplication(nil);
166 |             current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
167 | 
168 |             app.run();
169 |         }
170 |     }
171 | }
172 | 
173 | impl Drop for TrayItemMacOS {
174 |     fn drop(&mut self) {
175 |         match self.main_thread.take() {
176 |             Some(t) => t.join(),
177 |             None => Ok(()),
178 |         }
179 |         .unwrap()
180 |     }
181 | }
182 | 


--------------------------------------------------------------------------------
/src/api/mod.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(all(target_os = "linux", feature = "ksni"))]
 2 | mod linux_ksni;
 3 | 
 4 | #[cfg(all(target_os = "linux", feature = "libappindicator"))]
 5 | mod linux_libappindicator;
 6 | 
 7 | #[cfg(target_os = "windows")]
 8 | mod windows;
 9 | 
10 | #[cfg(target_os = "macos")]
11 | mod macos;
12 | 
13 | // Set type depending on OS and feature
14 | #[cfg(all(target_os = "linux", feature = "ksni"))]
15 | pub type TrayItemImpl = linux_ksni::TrayItemLinux;
16 | 
17 | #[cfg(all(target_os = "linux", feature = "libappindicator"))]
18 | pub type TrayItemImpl = linux_libappindicator::TrayItemLinux;
19 | 
20 | #[cfg(target_os = "windows")]
21 | pub type TrayItemImpl = windows::TrayItemWindows;
22 | 
23 | #[cfg(target_os = "macos")]
24 | pub type TrayItemImpl = macos::TrayItemMacOS;
25 | 


--------------------------------------------------------------------------------
/src/api/windows/funcs.rs:
--------------------------------------------------------------------------------
  1 | use std::{ffi::OsStr, mem, os::windows::ffi::OsStrExt, ptr};
  2 | 
  3 | use windows_sys::Win32::{
  4 |     Foundation::{GetLastError, HWND, LRESULT, POINT},
  5 |     System::LibraryLoader::GetModuleHandleW,
  6 |     UI::{
  7 |         Shell::{NIF_MESSAGE, NIM_ADD, NIF_ICON},
  8 |         WindowsAndMessaging::{
  9 |             CreatePopupMenu, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetCursorPos,
 10 |             GetMenuItemID, GetMessageW, PostQuitMessage, RegisterClassW, SetForegroundWindow,
 11 |             SetMenuInfo, TrackPopupMenu, TranslateMessage, CW_USEDEFAULT, MENUINFO,
 12 |             MIM_APPLYTOSUBMENUS, MIM_STYLE, MNS_NOTIFYBYPOS, MSG, TPM_BOTTOMALIGN, TPM_LEFTALIGN,
 13 |             TPM_LEFTBUTTON, WM_LBUTTONUP, WM_MENUCOMMAND, WM_QUIT, WM_RBUTTONUP, WM_USER,
 14 |             WNDCLASSW, WS_OVERLAPPEDWINDOW, WM_CREATE, HICON, IDI_APPLICATION, LoadIconW, 
 15 |             RegisterWindowMessageW,
 16 |         },
 17 |     },
 18 | };
 19 | 
 20 | use {super::*, crate::TIError};
 21 | 
 22 | pub(crate) fn to_wstring(str: &str) -> Vec<u16> {
 23 |     OsStr::new(str)
 24 |         .encode_wide()
 25 |         .chain(Some(0).into_iter())
 26 |         .collect::<Vec<_>>()
 27 | }
 28 | 
 29 | pub(crate) unsafe fn get_win_os_error(msg: &str) -> TIError {
 30 |     TIError::new_with_location(
 31 |         format!("{}: {}", &msg, GetLastError()),
 32 |         std::file!(),
 33 |         std::line!(),
 34 |     )
 35 | }
 36 | 
 37 | pub(crate) unsafe extern "system" fn window_proc(
 38 |     h_wnd: HWND,
 39 |     msg: u32,
 40 |     w_param: WPARAM,
 41 |     l_param: LPARAM,
 42 | ) -> LRESULT {
 43 |     static mut U_TASKBAR_RESTART: u32 = 0;
 44 | 
 45 |     if msg == WM_MENUCOMMAND {
 46 |         WININFO_STASH.with(|stash| {
 47 |             let stash = stash.borrow();
 48 |             let stash = stash.as_ref();
 49 |             if let Some(stash) = stash {
 50 |                 let menu_id = GetMenuItemID(stash.info.hmenu, w_param as i32) as i32;
 51 |                 if menu_id != -1 {
 52 |                     stash.tx.send(WindowsTrayEvent(menu_id as u32)).ok();
 53 |                 }
 54 |             }
 55 |         });
 56 |     }
 57 | 
 58 |     if msg == WM_USER + 1 && (l_param as u32 == WM_LBUTTONUP || l_param as u32 == WM_RBUTTONUP) {
 59 |         let mut point = POINT { x: 0, y: 0 };
 60 |         if GetCursorPos(&mut point) == 0 {
 61 |             return 1;
 62 |         }
 63 | 
 64 |         SetForegroundWindow(h_wnd);
 65 | 
 66 |         WININFO_STASH.with(|stash| {
 67 |             let stash = stash.borrow();
 68 |             let stash = stash.as_ref();
 69 |             if let Some(stash) = stash {
 70 |                 TrackPopupMenu(
 71 |                     stash.info.hmenu,
 72 |                     TPM_LEFTBUTTON | TPM_BOTTOMALIGN | TPM_LEFTALIGN,
 73 |                     point.x,
 74 |                     point.y,
 75 |                     0,
 76 |                     h_wnd,
 77 |                     ptr::null(),
 78 |                 );
 79 |             }
 80 |         });
 81 |     }
 82 | 
 83 |     if msg == WM_CREATE {
 84 |         U_TASKBAR_RESTART = RegisterWindowMessageW(to_wstring("TaskbarCreated").as_ptr());
 85 |     }
 86 | 
 87 |     // If windows explorer restarts and we need to recreate the tray icon
 88 |     if msg == U_TASKBAR_RESTART { 
 89 |         let icon: HICON = unsafe {
 90 |             let mut handle = LoadIconW(GetModuleHandleW(std::ptr::null()),
 91 |                 to_wstring("tray-default")
 92 |                 .as_ptr());
 93 |             if handle == 0 {
 94 |                 handle = LoadIconW(0, IDI_APPLICATION);
 95 |             }
 96 |             if handle == 0 {
 97 |                 println!("Error setting icon from resource");
 98 |                 PostQuitMessage(0);
 99 |             }
100 |             handle as HICON
101 |         };
102 |         let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
103 |         nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
104 |         nid.hWnd = h_wnd;
105 |         nid.uID = 1;
106 |         nid.uFlags = NIF_MESSAGE | NIF_ICON;
107 |         nid.hIcon = icon;
108 |         nid.uCallbackMessage = WM_USER + 1;
109 |         if Shell_NotifyIconW(NIM_ADD, &nid) == 0 {
110 |             println!("Error adding menu icon");
111 |             PostQuitMessage(0);
112 |         }
113 |     }
114 | 
115 |     if msg == WM_DESTROY {
116 |         PostQuitMessage(0);
117 |     }
118 | 
119 |     DefWindowProcW(h_wnd, msg, w_param, l_param)
120 | }
121 | 
122 | pub(crate) unsafe fn init_window() -> Result<WindowInfo, TIError> {
123 |     let hmodule = GetModuleHandleW(ptr::null());
124 |     if hmodule == 0 {
125 |         return Err(get_win_os_error("Error getting module handle"));
126 |     }
127 | 
128 |     let class_name = to_wstring("my_window");
129 | 
130 |     let mut wnd = unsafe { mem::zeroed::<WNDCLASSW>() };
131 |     wnd.lpfnWndProc = Some(window_proc);
132 |     wnd.lpszClassName = class_name.as_ptr();
133 |     
134 |     RegisterClassW(&wnd);
135 | 
136 |     let hwnd = CreateWindowExW(
137 |         0,
138 |         class_name.as_ptr(),
139 |         to_wstring("rust_systray_window").as_ptr(),
140 |         WS_OVERLAPPEDWINDOW,
141 |         CW_USEDEFAULT,
142 |         0,
143 |         CW_USEDEFAULT,
144 |         0,
145 |         0,
146 |         0,
147 |         0,
148 |         ptr::null(),
149 |     );
150 |     if hwnd == 0 {
151 |         return Err(get_win_os_error("Error creating window"));
152 |     }
153 |     
154 |     let icon: HICON = unsafe {
155 |         let mut handle = LoadIconW(GetModuleHandleW(std::ptr::null()), 
156 |             to_wstring("tray-default")
157 |             .as_ptr());
158 |         if handle == 0 {
159 |             handle = LoadIconW(0, IDI_APPLICATION);
160 |         }
161 |         if handle == 0 {
162 |             return Err(get_win_os_error("Error setting icon from resource"));
163 |         }
164 |         handle as HICON
165 |     };
166 | 
167 |     let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
168 |     nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
169 |     nid.hWnd = hwnd;
170 |     nid.uID = 1;
171 |     nid.uFlags = NIF_MESSAGE | NIF_ICON;
172 |     nid.hIcon = icon;
173 |     nid.uCallbackMessage = WM_USER + 1;
174 |     if Shell_NotifyIconW(NIM_ADD, &nid) == 0 {
175 |         return Err(get_win_os_error("Error adding menu icon"));
176 |     }
177 | 
178 |     // Setup menu
179 |     let mut info = unsafe { mem::zeroed::<MENUINFO>() };
180 |     info.cbSize = mem::size_of::<MENUINFO>() as u32;
181 |     info.fMask = MIM_APPLYTOSUBMENUS | MIM_STYLE;
182 |     info.dwStyle = MNS_NOTIFYBYPOS;
183 |     let hmenu = CreatePopupMenu();
184 |     if hmenu == 0 {
185 |         return Err(get_win_os_error("Error creating popup menu"));
186 |     }
187 |     if SetMenuInfo(hmenu, &info) == 0 {
188 |         return Err(get_win_os_error("Error setting up menu"));
189 |     }
190 | 
191 |     Ok(WindowInfo {
192 |         hwnd,
193 |         hmenu,
194 |         hmodule,
195 |     })
196 | }
197 | 
198 | pub(crate) unsafe fn run_loop() {
199 |     // Run message loop
200 |     let mut msg = unsafe { mem::zeroed::<MSG>() };
201 |     loop {
202 |         GetMessageW(&mut msg, 0, 0, 0);
203 |         if msg.message == WM_QUIT {
204 |             break;
205 |         }
206 |         TranslateMessage(&msg);
207 |         DispatchMessageW(&msg);
208 |     }
209 | }
210 | 


--------------------------------------------------------------------------------
/src/api/windows/mod.rs:
--------------------------------------------------------------------------------
  1 | // Most of this code is taken from https://github.com/qdot/systray-rs/blob/master/src/api/win32/mod.rs
  2 | 
  3 | mod funcs;
  4 | mod structs;
  5 | 
  6 | use std::{
  7 |     cell::RefCell,
  8 |     mem,
  9 |     sync::{
 10 |         mpsc::{channel, Sender},
 11 |         Arc, Mutex,
 12 |     },
 13 |     thread,
 14 | };
 15 | 
 16 | use windows_sys::Win32::{
 17 |     Foundation::{LPARAM, WPARAM},
 18 |     UI::{
 19 |         Shell::{Shell_NotifyIconW, NIF_ICON, NIF_TIP, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW},
 20 |         WindowsAndMessaging::{
 21 |             InsertMenuItemW, LoadImageW, PostMessageW, SetMenuItemInfoW, HICON, IMAGE_ICON,
 22 |             LR_DEFAULTCOLOR, MENUITEMINFOW, MFS_DISABLED, MFS_UNHILITE, MFT_SEPARATOR, MFT_STRING,
 23 |             MIIM_FTYPE, MIIM_ID, MIIM_STATE, MIIM_STRING, WM_DESTROY,
 24 |         },
 25 |     },
 26 | };
 27 | 
 28 | use crate::{IconSource, TIError};
 29 | 
 30 | use funcs::*;
 31 | use structs::*;
 32 | 
 33 | thread_local!(static WININFO_STASH: RefCell<Option<WindowsLoopData>> = RefCell::new(None));
 34 | 
 35 | type CallBackEntry = Option<Box<dyn Fn() + Send + 'static>>;
 36 | 
 37 | pub struct TrayItemWindows {
 38 |     entries: Arc<Mutex<Vec<CallBackEntry>>>,
 39 |     info: WindowInfo,
 40 |     windows_loop: Option<thread::JoinHandle<()>>,
 41 |     event_loop: Option<thread::JoinHandle<()>>,
 42 |     event_tx: Sender<WindowsTrayEvent>,
 43 | }
 44 | 
 45 | impl TrayItemWindows {
 46 |     pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
 47 |         let entries = Arc::new(Mutex::new(Vec::new()));
 48 |         let (event_tx, event_rx) = channel::<WindowsTrayEvent>();
 49 | 
 50 |         let entries_clone = Arc::clone(&entries);
 51 |         let event_loop = thread::spawn(move || loop {
 52 |             if let Ok(v) = event_rx.recv() {
 53 |                 if v.0 == u32::MAX {
 54 |                     break;
 55 |                 }
 56 | 
 57 |                 padlock::mutex_lock(&entries_clone, |ents: &mut Vec<CallBackEntry>| match &ents
 58 |                     [v.0 as usize]
 59 |                 {
 60 |                     Some(f) => f(),
 61 |                     None => (),
 62 |                 })
 63 |             }
 64 |         });
 65 | 
 66 |         let (tx, rx) = channel();
 67 | 
 68 |         let event_tx_clone = event_tx.clone();
 69 |         let windows_loop = thread::spawn(move || unsafe {
 70 |             let info = match init_window() {
 71 |                 Ok(info) => {
 72 |                     tx.send(Ok(info.clone())).ok();
 73 |                     info
 74 |                 }
 75 | 
 76 |                 Err(e) => {
 77 |                     tx.send(Err(e)).ok();
 78 |                     return;
 79 |                 }
 80 |             };
 81 | 
 82 |             WININFO_STASH.with(|stash| {
 83 |                 let data = WindowsLoopData {
 84 |                     info,
 85 |                     tx: event_tx_clone,
 86 |                 };
 87 | 
 88 |                 (*stash.borrow_mut()) = Some(data);
 89 |             });
 90 | 
 91 |             run_loop();
 92 |         });
 93 | 
 94 |         let info = match rx.recv().unwrap() {
 95 |             Ok(i) => i,
 96 |             Err(e) => return Err(e),
 97 |         };
 98 | 
 99 |         let w = Self {
100 |             entries,
101 |             info,
102 |             windows_loop: Some(windows_loop),
103 |             event_loop: Some(event_loop),
104 |             event_tx,
105 |         };
106 | 
107 |         w.set_tooltip(title)?;
108 |         w.set_icon(icon)?;
109 | 
110 |         Ok(w)
111 |     }
112 | 
113 |     pub fn set_icon(&self, icon: IconSource) -> Result<(), TIError> {
114 |         match icon {
115 |             IconSource::Resource(icon_str) => return self.set_icon_from_resource(icon_str),
116 |             IconSource::RawIcon(raw_icon) => self._set_icon(raw_icon),
117 |         }
118 |     }
119 | 
120 |     pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
121 |         self.add_label_with_id(label)?;
122 |         Ok(())
123 |     }
124 | 
125 |     pub fn add_label_with_id(&mut self, label: &str) -> Result<u32, TIError> {
126 |         let item_idx = padlock::mutex_lock(&self.entries, |entries| {
127 |             let len = entries.len();
128 |             entries.push(None);
129 |             len
130 |         }) as u32;
131 | 
132 |         let mut st = to_wstring(label);
133 |         let mut item = unsafe { mem::zeroed::<MENUITEMINFOW>() };
134 |         item.cbSize = mem::size_of::<MENUITEMINFOW>() as u32;
135 |         item.fMask = MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE;
136 |         item.fType = MFT_STRING;
137 |         item.fState = MFS_DISABLED | MFS_UNHILITE;
138 |         item.wID = item_idx;
139 |         item.dwTypeData = st.as_mut_ptr();
140 |         item.cch = (label.len() * 2) as u32;
141 | 
142 |         unsafe {
143 |             if InsertMenuItemW(self.info.hmenu, item_idx, 1, &item) == 0 {
144 |                 return Err(get_win_os_error("Error inserting menu item"));
145 |             }
146 |         }
147 |         Ok(item_idx)
148 |     }
149 | 
150 |     pub fn set_label(&mut self, label: &str, id: u32) -> Result<(), TIError> {
151 |         let mut st = to_wstring(label);
152 |         let mut item = unsafe { mem::zeroed::<MENUITEMINFOW>() };
153 |         item.cbSize = mem::size_of::<MENUITEMINFOW>() as u32;
154 |         item.fMask = MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE;
155 |         item.fType = MFT_STRING;
156 |         item.fState = MFS_DISABLED | MFS_UNHILITE;
157 |         item.wID = id;
158 |         item.dwTypeData = st.as_mut_ptr();
159 |         item.cch = (label.len() * 2) as u32;
160 | 
161 |         unsafe {
162 |             if SetMenuItemInfoW(self.info.hmenu, id, 1, &item) == 0 {
163 |                 return Err(get_win_os_error("Error inserting menu item"));
164 |             }
165 |         }
166 |         Ok(())
167 | 
168 |     }
169 | 
170 |     pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
171 |     where
172 |         F: Fn() + Send + 'static,
173 |     {
174 |         self.add_menu_item_with_id(label, cb)?;
175 |         Ok(())
176 |     }
177 | 
178 |     pub fn add_menu_item_with_id<F>(&mut self, label: &str, cb: F) -> Result<u32, TIError>
179 |     where
180 |         F: Fn() + Send + 'static,
181 |     {
182 |         let item_idx = padlock::mutex_lock(&self.entries, |entries| {
183 |             let len = entries.len();
184 |             entries.push(Some(Box::new(cb)));
185 |             len
186 |         }) as u32;
187 | 
188 |         let mut st = to_wstring(label);
189 |         let mut item = unsafe { mem::zeroed::<MENUITEMINFOW>() };
190 |         item.cbSize = mem::size_of::<MENUITEMINFOW>() as u32;
191 |         item.fMask = MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE;
192 |         item.fType = MFT_STRING;
193 |         item.wID = item_idx;
194 |         item.dwTypeData = st.as_mut_ptr();
195 |         item.cch = (label.len() * 2) as u32;
196 | 
197 |         unsafe {
198 |             if InsertMenuItemW(self.info.hmenu, item_idx, 1, &item) == 0 {
199 |                 return Err(get_win_os_error("Error inserting menu item"));
200 |             }
201 |         }
202 |         Ok(item_idx)
203 |     }
204 | 
205 |     pub fn set_menu_item_label(&mut self, label: &str, id: u32) -> Result<(), TIError> {
206 |         let mut st = to_wstring(label);
207 |         let mut item = unsafe { mem::zeroed::<MENUITEMINFOW>() };
208 |         item.cbSize = mem::size_of::<MENUITEMINFOW>() as u32;
209 |         item.fMask = MIIM_FTYPE | MIIM_STRING | MIIM_ID | MIIM_STATE;
210 |         item.fType = MFT_STRING;
211 |         item.wID = id;
212 |         item.dwTypeData = st.as_mut_ptr();
213 |         item.cch = (label.len() * 2) as u32;
214 | 
215 |         unsafe {
216 |             if SetMenuItemInfoW(self.info.hmenu, id, 1, &item) == 0 {
217 |                 return Err(get_win_os_error("Error setting menu item"));
218 |             }
219 |         }
220 |         Ok(())
221 |     }
222 | 
223 |     pub fn add_separator(&mut self) -> Result<(), TIError> {
224 |         self.add_separator_with_id()?;
225 |         Ok(())
226 |     }
227 | 
228 |     pub fn add_separator_with_id(&mut self) -> Result<u32, TIError> {
229 |         let item_idx = padlock::mutex_lock(&self.entries, |entries| {
230 |             let len = entries.len();
231 |             entries.push(None);
232 |             len
233 |         }) as u32;
234 | 
235 |         let mut item = unsafe { mem::zeroed::<MENUITEMINFOW>() };
236 |         item.cbSize = mem::size_of::<MENUITEMINFOW>() as u32;
237 |         item.fMask = MIIM_FTYPE | MIIM_ID | MIIM_STATE;
238 |         item.fType = MFT_SEPARATOR;
239 |         item.wID = item_idx;
240 | 
241 |         unsafe {
242 |             if InsertMenuItemW(self.info.hmenu, item_idx, 1, &item) == 0 {
243 |                 return Err(get_win_os_error("Error inserting menu separator"));
244 |             }
245 |         }
246 |         Ok(item_idx)
247 |     }
248 | 
249 |     pub fn set_tooltip(&self, tooltip: &str) -> Result<(), TIError> {
250 |         let wide_tooltip = to_wstring(tooltip);
251 |         if wide_tooltip.len() > 128 {
252 |             return Err(TIError::new("The tooltip may not exceed 127 wide bytes"));
253 |         }
254 | 
255 |         let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
256 |         nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
257 |         nid.hWnd = self.info.hwnd;
258 |         nid.uID = 1;
259 |         nid.uFlags = NIF_TIP;
260 | 
261 |         #[cfg(target_arch = "x86")]
262 |         {
263 |             let mut tip_data = [0u16; 128];
264 |             tip_data[..wide_tooltip.len()].copy_from_slice(&wide_tooltip);
265 |             nid.szTip = tip_data;
266 |         }
267 | 
268 |         #[cfg(not(target_arch = "x86"))]
269 |         nid.szTip[..wide_tooltip.len()].copy_from_slice(&wide_tooltip);
270 | 
271 |         unsafe {
272 |             if Shell_NotifyIconW(NIM_MODIFY, &nid) == 0 {
273 |                 return Err(get_win_os_error("Error setting tooltip"));
274 |             }
275 |         }
276 |         Ok(())
277 |     }
278 | 
279 |     fn set_icon_from_resource(&self, resource_name: &str) -> Result<(), TIError> {
280 |         let icon = unsafe {
281 |             let handle = LoadImageW(
282 |                 self.info.hmodule,
283 |                 to_wstring(resource_name).as_ptr(),
284 |                 IMAGE_ICON,
285 |                 64,
286 |                 64,
287 |                 LR_DEFAULTCOLOR,
288 |             );
289 | 
290 |             if handle == 0 {
291 |                 return Err(get_win_os_error("Error setting icon from resource"));
292 |             }
293 | 
294 |             handle
295 |         };
296 | 
297 |         self._set_icon(icon)
298 |     }
299 | 
300 |     fn _set_icon(&self, icon: HICON) -> Result<(), TIError> {
301 |         let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
302 |         nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
303 |         nid.hWnd = self.info.hwnd;
304 |         nid.uID = 1;
305 |         nid.uFlags = NIF_ICON;
306 |         nid.hIcon = icon;
307 | 
308 |         unsafe {
309 |             if Shell_NotifyIconW(NIM_MODIFY, &nid) == 0 {
310 |                 return Err(get_win_os_error("Error setting icon"));
311 |             }
312 |         }
313 |         Ok(())
314 |     }
315 | 
316 |     pub fn quit(&mut self) {
317 |         unsafe {
318 |             PostMessageW(self.info.hwnd, WM_DESTROY, 0, 0);
319 |         }
320 | 
321 |         if let Some(t) = self.windows_loop.take() {
322 |             t.join().ok();
323 |         }
324 | 
325 |         if let Some(t) = self.event_loop.take() {
326 |             self.event_tx.send(WindowsTrayEvent(u32::MAX)).ok();
327 |             t.join().ok();
328 |         }
329 |     }
330 | 
331 |     pub fn shutdown(&self) -> Result<(), TIError> {
332 |         let mut nid = unsafe { mem::zeroed::<NOTIFYICONDATAW>() };
333 |         nid.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
334 |         nid.hWnd = self.info.hwnd;
335 |         nid.uID = 1;
336 |         nid.uFlags = NIF_ICON;
337 | 
338 |         unsafe {
339 |             if Shell_NotifyIconW(NIM_DELETE, &nid) == 0 {
340 |                 return Err(get_win_os_error("Error deleting icon from menu"));
341 |             }
342 |         }
343 | 
344 |         Ok(())
345 |     }
346 | }
347 | 
348 | impl Drop for TrayItemWindows {
349 |     fn drop(&mut self) {
350 |         self.shutdown().ok();
351 |         self.quit();
352 |     }
353 | }
354 | 


--------------------------------------------------------------------------------
/src/api/windows/structs.rs:
--------------------------------------------------------------------------------
 1 | use std::sync::mpsc::Sender;
 2 | 
 3 | use windows_sys::Win32::{
 4 |     Foundation::{HMODULE, HWND},
 5 |     UI::WindowsAndMessaging::HMENU,
 6 | };
 7 | 
 8 | #[derive(Clone)]
 9 | pub(crate) struct WindowInfo {
10 |     pub hwnd: HWND,
11 |     pub hmodule: HMODULE,
12 |     pub hmenu: HMENU,
13 | }
14 | 
15 | unsafe impl Send for WindowInfo {}
16 | unsafe impl Sync for WindowInfo {}
17 | 
18 | #[derive(Clone)]
19 | pub(crate) struct WindowsLoopData {
20 |     pub info: WindowInfo,
21 |     pub tx: Sender<WindowsTrayEvent>,
22 | }
23 | 
24 | pub(crate) struct WindowsTrayEvent(pub(crate) u32);
25 | 


--------------------------------------------------------------------------------
/src/error.rs:
--------------------------------------------------------------------------------
 1 | use std::error;
 2 | 
 3 | #[derive(Debug)]
 4 | struct Location {
 5 |     file: &'static str,
 6 |     line: u32,
 7 | }
 8 | 
 9 | #[derive(Debug)]
10 | pub struct TIError {
11 |     cause: String,
12 |     location: Option<Location>,
13 | }
14 | 
15 | impl error::Error for TIError {}
16 | 
17 | impl TIError {
18 |     #[allow(dead_code)]
19 |     pub(crate) fn new<C: Into<String>>(cause: C) -> Self {
20 |         Self {
21 |             cause: cause.into(),
22 |             location: None,
23 |         }
24 |     }
25 | 
26 |     #[allow(dead_code)]
27 |     pub(crate) fn new_with_location<C: Into<String>>(
28 |         cause: C,
29 |         file: &'static str,
30 |         line: u32,
31 |     ) -> Self {
32 |         Self {
33 |             cause: cause.into(),
34 |             location: Some(Location { file, line }),
35 |         }
36 |     }
37 | }
38 | 
39 | impl std::fmt::Display for TIError {
40 |     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
41 |         match self.location {
42 |             Some(ref location) => {
43 |                 write!(f, "{} at {}#{}", self.cause, location.file, location.line)
44 |             }
45 |             None => write!(f, "{}", self.cause),
46 |         }
47 |     }
48 | }
49 | 


--------------------------------------------------------------------------------
/src/lib.rs:
--------------------------------------------------------------------------------
 1 | mod api;
 2 | mod error;
 3 | pub use error::TIError;
 4 | 
 5 | pub struct TrayItem(api::TrayItemImpl);
 6 | 
 7 | #[derive(Clone)]
 8 | pub enum IconSource {
 9 |     Resource(&'static str),
10 |     #[cfg(target_os = "windows")]
11 |     RawIcon(windows_sys::Win32::UI::WindowsAndMessaging::HICON),
12 |     #[cfg(any(target_os = "macos", all(target_os = "linux", feature = "ksni")))]
13 |     Data {
14 |         height: i32,
15 |         width: i32,
16 |         data: Vec<u8>,
17 |     },
18 | }
19 | 
20 | impl IconSource {
21 |     pub fn as_str(&self) -> &str {
22 |         match self {
23 |             IconSource::Resource(res) => res,
24 |             #[allow(unreachable_patterns)]
25 |             _ => unimplemented!(),
26 |         }
27 |     }
28 | }
29 | 
30 | impl TrayItem {
31 |     pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
32 |         Ok(Self(api::TrayItemImpl::new(title, icon)?))
33 |     }
34 | 
35 |     pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
36 |         self.0.set_icon(icon)
37 |     }
38 | 
39 |     pub fn add_label(&mut self, label: &str) -> Result<(), TIError> {
40 |         self.0.add_label(label)
41 |     }
42 | 
43 |     pub fn add_menu_item<F>(&mut self, label: &str, cb: F) -> Result<(), TIError>
44 |     where
45 |         F: Fn() + Send + Sync + 'static,
46 |     {
47 |         self.0.add_menu_item(label, cb)
48 |     }
49 | 
50 |     pub fn inner_mut(&mut self) -> &mut api::TrayItemImpl {
51 |         &mut self.0
52 |     }
53 | }
54 | 


--------------------------------------------------------------------------------
/tools/build-linux.sh:
--------------------------------------------------------------------------------
1 | #!/bin/sh
2 | 
3 | export RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
4 | export RUST_BACKTRACE=full
5 | export PKG_CONFIG_ALLOW_CROSS=0
6 | cargo build --release --target=x86_64-unknown-linux-gnu
7 | 


--------------------------------------------------------------------------------
/tools/build-macos.sh:
--------------------------------------------------------------------------------
1 | #!/bin/sh
2 | 
3 | cargo build --release --target=x86_64-apple-darwin --example macos
4 | 


--------------------------------------------------------------------------------
/tools/build-windows.sh:
--------------------------------------------------------------------------------
1 | #!/bin/sh
2 | 
3 | export RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
4 | export RUST_BACKTRACE=full
5 | export PKG_CONFIG_ALLOW_CROSS=1
6 | cargo build --release --target=x86_64-pc-windows-gnu --example windows
7 | 


--------------------------------------------------------------------------------
/tools/linux-build-container.sh:
--------------------------------------------------------------------------------
1 | #!/bin/sh
2 | 
3 | docker run \
4 |     -it \
5 |     --volume=$(pwd):/home/circleci/project \
6 |     olback/rust-gtk-linux /bin/bash \
7 | 


--------------------------------------------------------------------------------
/tools/windows-build-container.sh:
--------------------------------------------------------------------------------
 1 | #!/bin/sh
 2 | 
 3 | docker run \
 4 |     -it \
 5 |     --volume=$(pwd):/home/circleci/project \
 6 |     olback/rust-gtk-windows /bin/bash \
 7 | 
 8 | 
 9 | docker run \
10 |     -it \
11 |     --volume=$(pwd):/home/circleci/project \
12 |     --volume=$(pwd)/../../../tray-indicator:/home/circleci/tray-indicator \
13 |     olback/rust-gtk-windows /bin/bash
14 | 
15 | 
16 | 


--------------------------------------------------------------------------------