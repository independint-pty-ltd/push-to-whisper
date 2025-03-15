├── .github
    └── workflows
    │   └── pr.yml
├── .gitignore
├── .gitmodules
├── BUILDING.md
├── CHANGELOG.md
├── Cargo.toml
├── LICENSE
├── README.md
├── build.rs
├── examples
    ├── audio_transcription.rs
    ├── basic_use.rs
    └── full_usage
    │   ├── 2830-3980-0043.wav
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
├── src
    ├── common_logging.rs
    ├── error.rs
    ├── ggml_logging_hook.rs
    ├── lib.rs
    ├── standalone.rs
    ├── utilities.rs
    ├── whisper_ctx.rs
    ├── whisper_ctx_wrapper.rs
    ├── whisper_grammar.rs
    ├── whisper_logging_hook.rs
    ├── whisper_params.rs
    └── whisper_state.rs
└── sys
    ├── Cargo.toml
    ├── build.rs
    ├── src
        ├── bindings.rs
        └── lib.rs
    └── wrapper.h


/.github/workflows/pr.yml:
--------------------------------------------------------------------------------
 1 | name: Check code
 2 | on:
 3 |   push:
 4 |   pull_request:
 5 |   workflow_dispatch:
 6 | 
 7 | jobs:
 8 |   rustfmt:
 9 |     runs-on: ubuntu-latest
10 |     steps:
11 |       - name: Check out code into the proper directory
12 |         uses: actions/checkout@v3
13 |         with:
14 |           submodules: 'recursive'
15 | 
16 |       - name: Cache rust
17 |         uses: Swatinem/rust-cache@v2
18 | 
19 |       - name: Install rust
20 |         uses: dtolnay/rust-toolchain@master
21 |         with:
22 |           toolchain: stable
23 |           components: rustfmt
24 | 
25 |       - name: Check formatting
26 |         run: cargo fmt --check
27 | 
28 | 
29 |   clippy:
30 |     strategy:
31 |       fail-fast: false
32 |       matrix:
33 |         os: [ ubuntu-latest, windows-latest, macos-latest ]
34 |         rust-version: [ stable, nightly ]
35 |     runs-on: ${{ matrix.os }}
36 |     steps:
37 |       - name: Check out code into the proper directory
38 |         uses: actions/checkout@v3
39 |         with:
40 |           submodules: 'recursive'
41 | 
42 |       - name: Cache rust
43 |         uses: Swatinem/rust-cache@v2
44 | 
45 |       - name: Install rust
46 |         uses: dtolnay/rust-toolchain@master
47 |         with:
48 |           toolchain: ${{ matrix.rust-version }}
49 |           components: clippy
50 | 
51 |       - name: Check clippy lints
52 |         run: cargo clippy
53 | 
54 |   build:
55 |     strategy:
56 |       fail-fast: false
57 |       matrix:
58 |         os: [ ubuntu-latest, windows-latest, macos-latest ]
59 |         rust-version: [ stable, nightly ]
60 |     runs-on: ${{ matrix.os }}
61 |     steps:
62 |       - name: Check out code into the proper directory
63 |         uses: actions/checkout@v3
64 |         with:
65 |           submodules: 'recursive'
66 | 
67 |       - name: Cache rust
68 |         uses: Swatinem/rust-cache@v2
69 | 
70 |       - name: Install rust
71 |         uses: dtolnay/rust-toolchain@master
72 |         with:
73 |           toolchain: ${{ matrix.rust-version }}
74 | 
75 |       - name: Check build
76 |         run: cargo build -F log_backend,tracing_backend --verbose --examples
77 | 


--------------------------------------------------------------------------------
/.gitignore:
--------------------------------------------------------------------------------
1 | **/target
2 | **/Cargo.lock
3 | /.idea
4 | /.vscode
5 | *.bin
6 | *.wav


--------------------------------------------------------------------------------
/.gitmodules:
--------------------------------------------------------------------------------
1 | [submodule "sys/whisper.cpp"]
2 | 	path = sys/whisper.cpp
3 | 	url = https://github.com/ggerganov/whisper.cpp
4 | 


--------------------------------------------------------------------------------
/BUILDING.md:
--------------------------------------------------------------------------------
 1 | # Running on Arch Linux
 2 | `sudo pacman -Syy llvm clang cmake`
 3 | `cargo build`
 4 | 
 5 | # Running on Windows using MSYS2
 6 | 
 7 | The following are instructions for building whisper-rs on Windows using the msys2 set of compilers. 
 8 | 
 9 | 1. install msys2/mingw by following [https://code.visualstudio.com/docs/cpp/config-mingw](`https://code.visualstudio.com/docs/cpp/config-mingw`)
10 |    1. Install g++ and make within msys2 ucrt64
11 |       - `pacman -S --needed base-devel mingw-w64-x86_64-toolchain`
12 |    2. Add the msys2 ucrt64 bin folder to path `C:\msys64\ucrt64\bin`
13 | 2. Install make by running `pacman -S make` in msys2 ucrt66
14 | 3. Set rust to use msys2: by running `rustup toolchain install stable-x86_64-pc-windows-gnu` in Windows Powershell/Cmd
15 | 4. Add `.cargo/config.toml` file in the project with the following contents: 
16 | ```
17 | [target.x86_64-pc-windows-gnu]
18 | linker = "C:\\msys64\\ucrt64\\bin\\gcc.exe"
19 | ar = "C:\\msys64\\ucrt64\\bin\\ar.exe"
20 | ```
21 | 5. Run `cargo run`  in Windows Powershell/Cmd
22 | 
23 | # Running on Windows using Microsoft Visual Studio C++
24 | 
25 | It has been reported that it is also possible to build whisper-rs using Visual Studio C++.
26 | 
27 | Make sure you have installed and in the path:
28 | 
29 | - Visual Studio C++
30 | - cmake
31 | - LLVM(clang)
32 | 
33 | ### Instructions (for builds with `cuda` enabled)
34 | 1. Download [CUDA](https://developer.nvidia.com/cuda-downloads?target_os=Windows)
35 | 2. Download [Visual Studio with Desktop C++ and Clang enabled](https://visualstudio.microsoft.com/de/downloads/) (see clang link below for installer walkthrough)
36 | 3. Download [CLANG](https://www.wikihow.com/Install-Clang-on-Windows)
37 | 4. Download [CMAKE](https://cmake.org/download/)
38 | 5. Run `where.exe clang`, then `setx LIBCLANG_PATH "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\bin"` or something like that
39 | 6. Restart your shell!!!
40 | 7. Cargo build
41 | 
42 | # Running on M1 OSX
43 | 
44 | To build on a M1 Mac, make sure to add the following to your project's `.cargo/config.toml`: 
45 | 
46 | ```
47 | [target.aarch64-apple-darwin]
48 | rustflags = "-lc++ -l framework=Accelerate"
49 | ```
50 | 
51 | See https://github.com/tazz4843/whisper-rs/pull/2 for more information.
52 | 
53 | You also need to have CMake installed. You can obtain this using homebrew:
54 | 
55 | ```
56 | brew install cmake
57 | ```
58 | 
59 | CMake can also be installed from https://cmake.org/download/ but `cmake` binary needs to be in your PATH.
60 | 


--------------------------------------------------------------------------------
/CHANGELOG.md:
--------------------------------------------------------------------------------
 1 | # Version 0.8.0 (-sys bindings 0.6.1) (2023-06-18)
 2 | * Fix CUDA and OpenCL build broken due to missing API headers.
 3 | * Use PIC when building whisper.cpp (fixes building a cdylib on x86 Linux)
 4 | 
 5 | # Version 0.8.0 (2023-05-14)
 6 | * Update upstream whisper.cpp to v1.4.2 (OpenCL support)
 7 | * Add CUDA and OpenCL support to bindings
 8 |   * No MacOS testers were able to test CoreML support, so it may be broken. Please open an issue if it is.
 9 |   * Enable CUDA support by enabling the `cuda` feature.
10 |   * Enable OpenCL support by enabling the `opencl` feature.
11 | * Add `FullParams::set_detect_language`
12 | 
13 | # Version 0.7.0 (2023-05-10)
14 | * Update upstream whisper.cpp to v1.4.0 (integer quantization support, see last point for CUDA support)
15 | * Expose `WhisperState` as a public type, allowing for more control over the state.
16 |   * `WhisperContext::create_state` now returns a `WhisperState` instead of `()`.
17 |   * All methods that took a key argument in v0.6.0 have been moved to `WhisperState`.
18 | * Generic key argument on `WhisperContext` has been removed.
19 | * Note: CUDA and OpenCL acceleration is supported on the `cuda-and-opencl-support` branch of the git repo,
20 |   and will probably be released in v0.8.0.
21 | 
22 | # Version 0.6.0 (2023-04-17)
23 | * Update upstream whisper.cpp to v1.3.0
24 | * Fix breaking changes in update, which cascade to users:
25 |   * `WhisperContext`s now have a generic type parameter, which is a hashable key for a state map.
26 |     This allows for a single context to be reused for multiple different states, saving memory.
27 |     * You must create a new state upon creation, even if you are using the context only once, by calling `WhisperContext::create_key`.
28 |     * Each method that now takes a state now takes a key, which internally is used to look up the state.
29 |     * This also turns `WhisperContext` into an entirely immutable object, meaning it can be shared across threads and used concurrently, safely.
30 | * Send feedback on these changes to the PR: https://github.com/tazz4843/whisper-rs/pull/33
31 | 
32 | # Version 0.2.0 (2022-10-28)
33 | * Update upstream whisper.cpp to 2c281d190b7ec351b8128ba386d110f100993973.
34 | * Fix breaking changes in update, which cascade to users:
35 |   * `DecodeStrategy` has been renamed to `SamplingStrategy`
36 |   * `WhisperContext::sample_best`'s signature has changed: `needs_timestamp` has been removed.
37 | * New features
38 |   * `WhisperContext::full_n_tokens`
39 |   * `WhisperContext::full_get_token_text`
40 |   * `WhisperContext::full_get_token_id`
41 |   * `WhisperContext::full_get_token_prob`
42 | 


--------------------------------------------------------------------------------
/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [workspace]
 2 | members = ["sys"]
 3 | exclude = ["examples/full_usage"]
 4 | 
 5 | [package]
 6 | name = "whisper-rs"
 7 | version = "0.14.2"
 8 | edition = "2021"
 9 | description = "Rust bindings for whisper.cpp"
10 | license = "Unlicense"
11 | documentation = "https://docs.rs/whisper-rs"
12 | repository = "https://github.com/tazz4843/whisper-rs"
13 | 
14 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
15 | 
16 | [dependencies]
17 | whisper-rs-sys = { path = "sys", version = "0.12" }
18 | log = { version = "0.4", optional = true }
19 | tracing = { version = "0.1", optional = true }
20 | 
21 | [dev-dependencies]
22 | hound = "3.5.0"
23 | rand = "0.8.4"
24 | 
25 | [features]
26 | default = []
27 | 
28 | raw-api = []
29 | coreml = ["whisper-rs-sys/coreml"]
30 | cuda = ["whisper-rs-sys/cuda", "_gpu"]
31 | hipblas = ["whisper-rs-sys/hipblas", "_gpu"]
32 | openblas = ["whisper-rs-sys/openblas"]
33 | metal = ["whisper-rs-sys/metal", "_gpu"]
34 | vulkan = ["whisper-rs-sys/vulkan", "_gpu"]
35 | openmp = ["whisper-rs-sys/openmp"]
36 | _gpu = []
37 | test-with-tiny-model = []
38 | 
39 | # Bring logs into Rust via the log crate. *Warning*: not mutually exclusive with tracing_backend,
40 | # will result in duplicate logs if both are enabled and one consumes logs from the other.
41 | log_backend = ["dep:log"]
42 | 
43 | # Bring logs into Rust via the tracing crate. *Warning*: not mutually exclusive with log_backend,
44 | # will result in duplicate logs if both are enabled and one consumes logs from the other.
45 | tracing_backend = ["dep:tracing"]
46 | 


--------------------------------------------------------------------------------
/LICENSE:
--------------------------------------------------------------------------------
 1 | This is free and unencumbered software released into the public domain.
 2 | 
 3 | Anyone is free to copy, modify, publish, use, compile, sell, or
 4 | distribute this software, either in source code form or as a compiled
 5 | binary, for any purpose, commercial or non-commercial, and by any
 6 | means.
 7 | 
 8 | In jurisdictions that recognize copyright laws, the author or authors
 9 | of this software dedicate any and all copyright interest in the
10 | software to the public domain. We make this dedication for the benefit
11 | of the public at large and to the detriment of our heirs and
12 | successors. We intend this dedication to be an overt act of
13 | relinquishment in perpetuity of all present and future rights to this
14 | software under copyright law.
15 | 
16 | THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
17 | EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
18 | MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
19 | IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
20 | OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
21 | ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
22 | OTHER DEALINGS IN THE SOFTWARE.
23 | 
24 | For more information, please refer to <https://unlicense.org>


--------------------------------------------------------------------------------
/README.md:
--------------------------------------------------------------------------------
  1 | # whisper-rs
  2 | 
  3 | Rust bindings to [whisper.cpp](https://github.com/ggerganov/whisper.cpp/)
  4 | 
  5 | ## Usage
  6 | 
  7 | ```bash
  8 | git clone --recursive https://github.com/tazz4843/whisper-rs.git
  9 | 
 10 | cd whisper-rs
 11 | 
 12 | cargo run --example basic_use
 13 | 
 14 | cargo run --example audio_transcription
 15 | ```
 16 | 
 17 | ```rust
 18 | use whisper_rs::{WhisperContext, WhisperContextParameters, FullParams, SamplingStrategy};
 19 | 
 20 | fn main() {
 21 | 	let path_to_model = std::env::args().nth(1).unwrap();
 22 | 
 23 | 	// load a context and model
 24 | 	let ctx = WhisperContext::new_with_params(
 25 | 		path_to_model,
 26 | 		WhisperContextParameters::default()
 27 | 	).expect("failed to load model");
 28 | 
 29 | 	// create a params object
 30 | 	let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
 31 | 
 32 | 	// assume we have a buffer of audio data
 33 | 	// here we'll make a fake one, floating point samples, 32 bit, 16KHz, mono
 34 | 	let audio_data = vec![0_f32; 16000 * 2];
 35 | 
 36 | 	// now we can run the model
 37 | 	let mut state = ctx.create_state().expect("failed to create state");
 38 | 	state
 39 | 		.full(params, &audio_data[..])
 40 | 		.expect("failed to run model");
 41 | 
 42 | 	// fetch the results
 43 | 	let num_segments = state
 44 | 		.full_n_segments()
 45 | 		.expect("failed to get number of segments");
 46 | 	for i in 0..num_segments {
 47 | 		let segment = state
 48 | 			.full_get_segment_text(i)
 49 | 			.expect("failed to get segment");
 50 | 		let start_timestamp = state
 51 | 			.full_get_segment_t0(i)
 52 | 			.expect("failed to get segment start timestamp");
 53 | 		let end_timestamp = state
 54 | 			.full_get_segment_t1(i)
 55 | 			.expect("failed to get segment end timestamp");
 56 | 		println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
 57 | 	}
 58 | }
 59 | ```
 60 | 
 61 | See [examples/basic_use.rs](examples/basic_use.rs) for more details.
 62 | 
 63 | Lower level bindings are exposed if needed, but the above should be enough for most use cases.
 64 | See the docs: https://docs.rs/whisper-rs/ for more details.
 65 | 
 66 | ## Feature flags
 67 | 
 68 | All disabled by default unless otherwise specified.
 69 | 
 70 | * `raw-api`: expose whisper-rs-sys without having to pull it in as a dependency.
 71 |   **NOTE**: enabling this no longer guarantees semver compliance,
 72 |   as whisper-rs-sys may be upgraded to a breaking version in a patch release of whisper-rs.
 73 | * `cuda`: enable CUDA support. Implicitly enables hidden GPU flag at runtime.
 74 | * `hipblas`: enable ROCm/hipBLAS support. Only available on linux. Implicitly enables hidden GPU flag at runtime.
 75 | * `openblas`: enable OpenBLAS support.
 76 | * `metal`: enable Metal support. Implicitly enables hidden GPU flag at runtime.
 77 | * `vulkan`: enable Vulkan support. Implicitly enables hidden GPU flag at runtime.
 78 | * `whisper-cpp-log`: allows hooking into whisper.cpp's log output and sending it to the `log` backend. Requires calling
 79 | * `whisper-cpp-tracing`: allows hooking into whisper.cpp's log output and sending it to the `tracing` backend.
 80 | 
 81 | ## Building
 82 | 
 83 | See [BUILDING.md](BUILDING.md) for instructions for building whisper-rs on Windows and OSX M1. Linux builds should just
 84 | work out of the box.
 85 | 
 86 | ## Troubleshooting
 87 | 
 88 | * Something other than Windows/macOS/Linux isn't working!
 89 |     * I don't have a way to test these platforms, so I can't really help you.
 90 |         * If you can get it working, please open a PR with any changes to make it work and build instructions in
 91 |           BUILDING.md!
 92 | * I get a panic during binding generation build!
 93 |     * You can attempt to fix it yourself, or you can set the `WHISPER_DONT_GENERATE_BINDINGS` environment variable.
 94 |       This skips attempting to build the bindings whatsoever and copies the existing ones. They may be out of date,
 95 |       but it's better than nothing.
 96 |         * `WHISPER_DONT_GENERATE_BINDINGS=1 cargo build`
 97 |     * If you can fix the issue, please open a PR!
 98 | 
 99 | ## License
100 | 
101 | [Unlicense](LICENSE)
102 | 
103 | tl;dr: public domain
104 | 


--------------------------------------------------------------------------------
/build.rs:
--------------------------------------------------------------------------------
 1 | use std::env;
 2 | 
 3 | fn main() {
 4 |     let whisper_cpp_version = env::var("DEP_WHISPER_WHISPER_CPP_VERSION").unwrap_or_else(|e| {
 5 |         if env::var("DOCS_RS").is_ok() {
 6 |             // not sure why but this fails on docs.rs
 7 |             // return a default string
 8 |             "0.0.0-fake".to_string()
 9 |         } else {
10 |             panic!("Failed to find upstream whisper.cpp version: your build environment is messed up. {}", e);
11 |         }
12 |     });
13 |     println!(
14 |         "cargo:rustc-env=WHISPER_CPP_VERSION={}",
15 |         whisper_cpp_version
16 |     );
17 | }
18 | 


--------------------------------------------------------------------------------
/examples/audio_transcription.rs:
--------------------------------------------------------------------------------
  1 | // This example is not going to build in this folder.
  2 | // You need to copy this code into your project and add the dependencies whisper_rs and hound in your cargo.toml
  3 | 
  4 | use hound;
  5 | use std::fs::File;
  6 | use std::io::Write;
  7 | use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
  8 | 
  9 | /// Loads a context and model, processes an audio file, and prints the resulting transcript to stdout.
 10 | fn main() -> Result<(), &'static str> {
 11 |     // Load a context and model.
 12 |     let mut context_param = WhisperContextParameters::default();
 13 | 
 14 |     // Enable DTW token level timestamp for known model by using model preset
 15 |     context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
 16 |         model_preset: whisper_rs::DtwModelPreset::BaseEn,
 17 |     };
 18 | 
 19 |     // Enable DTW token level timestamp for unknown model by providing custom aheads
 20 |     // see details https://github.com/ggerganov/whisper.cpp/pull/1485#discussion_r1519681143
 21 |     // values corresponds to ggml-base.en.bin, result will be the same as with DtwModelPreset::BaseEn
 22 |     let custom_aheads = [
 23 |         (3, 1),
 24 |         (4, 2),
 25 |         (4, 3),
 26 |         (4, 7),
 27 |         (5, 1),
 28 |         (5, 2),
 29 |         (5, 4),
 30 |         (5, 6),
 31 |     ]
 32 |     .map(|(n_text_layer, n_head)| whisper_rs::DtwAhead {
 33 |         n_text_layer,
 34 |         n_head,
 35 |     });
 36 |     context_param.dtw_parameters.mode = whisper_rs::DtwMode::Custom {
 37 |         aheads: &custom_aheads,
 38 |     };
 39 | 
 40 |     let ctx = WhisperContext::new_with_params(
 41 |         "example/path/to/model/whisper.cpp/models/ggml-base.en.bin",
 42 |         context_param,
 43 |     )
 44 |     .expect("failed to load model");
 45 |     // Create a state
 46 |     let mut state = ctx.create_state().expect("failed to create key");
 47 | 
 48 |     // Create a params object for running the model.
 49 |     // The number of past samples to consider defaults to 0.
 50 |     let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
 51 | 
 52 |     // Edit params as needed.
 53 |     // Set the number of threads to use to 1.
 54 |     params.set_n_threads(1);
 55 |     // Enable translation.
 56 |     params.set_translate(true);
 57 |     // Set the language to translate to to English.
 58 |     params.set_language(Some("en"));
 59 |     // Disable anything that prints to stdout.
 60 |     params.set_print_special(false);
 61 |     params.set_print_progress(false);
 62 |     params.set_print_realtime(false);
 63 |     params.set_print_timestamps(false);
 64 |     // Enable token level timestamps
 65 |     params.set_token_timestamps(true);
 66 | 
 67 |     // Open the audio file.
 68 |     let reader = hound::WavReader::open("audio.wav").expect("failed to open file");
 69 |     #[allow(unused_variables)]
 70 |     let hound::WavSpec {
 71 |         channels,
 72 |         sample_rate,
 73 |         bits_per_sample,
 74 |         ..
 75 |     } = reader.spec();
 76 | 
 77 |     // Convert the audio to floating point samples.
 78 |     let samples: Vec<i16> = reader
 79 |         .into_samples::<i16>()
 80 |         .map(|x| x.expect("Invalid sample"))
 81 |         .collect();
 82 |     let mut audio = vec![0.0f32; samples.len().try_into().unwrap()];
 83 |     whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).expect("Conversion error");
 84 | 
 85 |     // Convert audio to 16KHz mono f32 samples, as required by the model.
 86 |     // These utilities are provided for convenience, but can be replaced with custom conversion logic.
 87 |     // SIMD variants of these functions are also available on nightly Rust (see the docs).
 88 |     if channels == 2 {
 89 |         audio = whisper_rs::convert_stereo_to_mono_audio(&audio).expect("Conversion error");
 90 |     } else if channels != 1 {
 91 |         panic!(">2 channels unsupported");
 92 |     }
 93 | 
 94 |     if sample_rate != 16000 {
 95 |         panic!("sample rate must be 16KHz");
 96 |     }
 97 | 
 98 |     // Run the model.
 99 |     state.full(params, &audio[..]).expect("failed to run model");
100 | 
101 |     // Create a file to write the transcript to.
102 |     let mut file = File::create("transcript.txt").expect("failed to create file");
103 | 
104 |     // Iterate through the segments of the transcript.
105 |     let num_segments = state
106 |         .full_n_segments()
107 |         .expect("failed to get number of segments");
108 |     for i in 0..num_segments {
109 |         // Get the transcribed text and timestamps for the current segment.
110 |         let segment = state
111 |             .full_get_segment_text(i)
112 |             .expect("failed to get segment");
113 |         let start_timestamp = state
114 |             .full_get_segment_t0(i)
115 |             .expect("failed to get start timestamp");
116 |         let end_timestamp = state
117 |             .full_get_segment_t1(i)
118 |             .expect("failed to get end timestamp");
119 | 
120 |         let first_token_dtw_ts = if let Ok(token_count) = state.full_n_tokens(i) {
121 |             if token_count > 0 {
122 |                 if let Ok(token_data) = state.full_get_token_data(i, 0) {
123 |                     token_data.t_dtw
124 |                 } else {
125 |                     -1i64
126 |                 }
127 |             } else {
128 |                 -1i64
129 |             }
130 |         } else {
131 |             -1i64
132 |         };
133 |         // Print the segment to stdout.
134 |         println!(
135 |             "[{} - {} ({})]: {}",
136 |             start_timestamp, end_timestamp, first_token_dtw_ts, segment
137 |         );
138 | 
139 |         // Format the segment information as a string.
140 |         let line = format!("[{} - {}]: {}\n", start_timestamp, end_timestamp, segment);
141 | 
142 |         // Write the segment information to the file.
143 |         file.write_all(line.as_bytes())
144 |             .expect("failed to write to file");
145 |     }
146 |     Ok(())
147 | }
148 | 


--------------------------------------------------------------------------------
/examples/basic_use.rs:
--------------------------------------------------------------------------------
 1 | /*
 2 | wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin
 3 | wget https://github.com/ggerganov/whisper.cpp/raw/master/samples/jfk.wav
 4 | cargo run --example basic_use ggml-tiny.bin jfk.wav
 5 | */
 6 | 
 7 | use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
 8 | 
 9 | fn main() {
10 |     let model_path = std::env::args()
11 |         .nth(1)
12 |         .expect("Please specify path to model");
13 |     let wav_path = std::env::args()
14 |         .nth(2)
15 |         .expect("Please specify path to wav file");
16 |     let language = "en";
17 | 
18 |     let samples: Vec<i16> = hound::WavReader::open(wav_path)
19 |         .unwrap()
20 |         .into_samples::<i16>()
21 |         .map(|x| x.unwrap())
22 |         .collect();
23 | 
24 |     // load a context and model
25 |     let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())
26 |         .expect("failed to load model");
27 | 
28 |     let mut state = ctx.create_state().expect("failed to create state");
29 | 
30 |     let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
31 | 
32 |     // and set the language to translate to to english
33 |     params.set_language(Some(&language));
34 | 
35 |     // we also explicitly disable anything that prints to stdout
36 |     params.set_print_special(false);
37 |     params.set_print_progress(false);
38 |     params.set_print_realtime(false);
39 |     params.set_print_timestamps(false);
40 | 
41 |     // we must convert to 16KHz mono f32 samples for the model
42 |     // some utilities exist for this
43 |     // note that you don't need to use these, you can do it yourself or any other way you want
44 |     // these are just provided for convenience
45 |     // SIMD variants of these functions are also available, but only on nightly Rust: see the docs
46 |     let mut inter_samples = vec![Default::default(); samples.len()];
47 | 
48 |     whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
49 |         .expect("failed to convert audio data");
50 |     let samples = whisper_rs::convert_stereo_to_mono_audio(&inter_samples)
51 |         .expect("failed to convert audio data");
52 | 
53 |     // now we can run the model
54 |     // note the key we use here is the one we created above
55 |     state
56 |         .full(params, &samples[..])
57 |         .expect("failed to run model");
58 | 
59 |     // fetch the results
60 |     let num_segments = state
61 |         .full_n_segments()
62 |         .expect("failed to get number of segments");
63 |     for i in 0..num_segments {
64 |         let segment = state
65 |             .full_get_segment_text(i)
66 |             .expect("failed to get segment");
67 |         let start_timestamp = state
68 |             .full_get_segment_t0(i)
69 |             .expect("failed to get segment start timestamp");
70 |         let end_timestamp = state
71 |             .full_get_segment_t1(i)
72 |             .expect("failed to get segment end timestamp");
73 |         println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
74 |     }
75 | }
76 | 


--------------------------------------------------------------------------------
/examples/full_usage/2830-3980-0043.wav:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/tazz4843/whisper-rs/f5ee632f6e1e682668295696d9cf280041a997bb/examples/full_usage/2830-3980-0043.wav


--------------------------------------------------------------------------------
/examples/full_usage/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "full_usage"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | hound = "3"
10 | whisper-rs = { path = "../.." }
11 | 


--------------------------------------------------------------------------------
/examples/full_usage/src/main.rs:
--------------------------------------------------------------------------------
 1 | #![allow(clippy::uninlined_format_args)]
 2 | 
 3 | use hound::{SampleFormat, WavReader};
 4 | use std::path::Path;
 5 | use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
 6 | 
 7 | fn parse_wav_file(path: &Path) -> Vec<i16> {
 8 |     let reader = WavReader::open(path).expect("failed to read file");
 9 | 
10 |     if reader.spec().channels != 1 {
11 |         panic!("expected mono audio file");
12 |     }
13 |     if reader.spec().sample_format != SampleFormat::Int {
14 |         panic!("expected integer sample format");
15 |     }
16 |     if reader.spec().sample_rate != 16000 {
17 |         panic!("expected 16KHz sample rate");
18 |     }
19 |     if reader.spec().bits_per_sample != 16 {
20 |         panic!("expected 16 bits per sample");
21 |     }
22 | 
23 |     reader
24 |         .into_samples::<i16>()
25 |         .map(|x| x.expect("sample"))
26 |         .collect::<Vec<_>>()
27 | }
28 | 
29 | fn main() {
30 |     let arg1 = std::env::args()
31 |         .nth(1)
32 |         .expect("first argument should be path to WAV file");
33 |     let audio_path = Path::new(&arg1);
34 |     if !audio_path.exists() {
35 |         panic!("audio file doesn't exist");
36 |     }
37 |     let arg2 = std::env::args()
38 |         .nth(2)
39 |         .expect("second argument should be path to Whisper model");
40 |     let whisper_path = Path::new(&arg2);
41 |     if !whisper_path.exists() {
42 |         panic!("whisper file doesn't exist")
43 |     }
44 | 
45 |     let original_samples = parse_wav_file(audio_path);
46 |     let mut samples = vec![0.0f32; original_samples.len()];
47 |     whisper_rs::convert_integer_to_float_audio(&original_samples, &mut samples)
48 |         .expect("failed to convert samples");
49 | 
50 |     let ctx = WhisperContext::new_with_params(
51 |         &whisper_path.to_string_lossy(),
52 |         WhisperContextParameters::default(),
53 |     )
54 |     .expect("failed to open model");
55 |     let mut state = ctx.create_state().expect("failed to create key");
56 |     let mut params = FullParams::new(SamplingStrategy::default());
57 |     params.set_initial_prompt("experience");
58 |     params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));
59 | 
60 |     let st = std::time::Instant::now();
61 |     state
62 |         .full(params, &samples)
63 |         .expect("failed to convert samples");
64 |     let et = std::time::Instant::now();
65 | 
66 |     let num_segments = state
67 |         .full_n_segments()
68 |         .expect("failed to get number of segments");
69 |     for i in 0..num_segments {
70 |         let segment = state
71 |             .full_get_segment_text(i)
72 |             .expect("failed to get segment");
73 |         let start_timestamp = state
74 |             .full_get_segment_t0(i)
75 |             .expect("failed to get start timestamp");
76 |         let end_timestamp = state
77 |             .full_get_segment_t1(i)
78 |             .expect("failed to get end timestamp");
79 |         println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
80 |     }
81 |     println!("took {}ms", (et - st).as_millis());
82 | }
83 | 


--------------------------------------------------------------------------------
/src/common_logging.rs:
--------------------------------------------------------------------------------
 1 | macro_rules! generic_error {
 2 |     ($($expr:tt)*) => {
 3 |         #[cfg(feature = "log_backend")]
 4 |         log::error!($($expr)*);
 5 |         #[cfg(feature = "tracing_backend")]
 6 |         tracing::error!($($expr)*);
 7 |     };
 8 | }
 9 | 
10 | macro_rules! generic_warn {
11 |     ($($expr:tt)*) => {
12 |         #[cfg(feature = "log_backend")]
13 |         log::warn!($($expr)*);
14 |         #[cfg(feature = "tracing_backend")]
15 |         tracing::warn!($($expr)*);
16 |     }
17 | }
18 | 
19 | macro_rules! generic_info {
20 |     ($($expr:tt)*) => {
21 |         #[cfg(feature = "log_backend")]
22 |         log::info!($($expr)*);
23 |         #[cfg(feature = "tracing_backend")]
24 |         tracing::info!($($expr)*);
25 |     }
26 | }
27 | 
28 | macro_rules! generic_debug {
29 |     ($($expr:tt)*) => {
30 |         #[cfg(feature = "log_backend")]
31 |         log::debug!($($expr)*);
32 |         #[cfg(feature = "tracing_backend")]
33 |         tracing::debug!($($expr)*);
34 |     }
35 | }
36 | 
37 | macro_rules! generic_trace {
38 |     ($($expr:tt)*) => {
39 |         #[cfg(feature = "log_backend")]
40 |         log::trace!($($expr)*);
41 |         #[cfg(feature = "tracing_backend")]
42 |         tracing::trace!($($expr)*);
43 |     }
44 | }
45 | 
46 | use whisper_rs_sys::ggml_log_level;
47 | pub(crate) use {generic_debug, generic_error, generic_info, generic_trace, generic_warn};
48 | 
49 | // Unsigned integer type on most platforms is 32 bit, niche platforms that whisper.cpp
50 | // likely doesn't even support would use 16 bit and would still fit
51 | #[cfg_attr(any(not(windows), target_env = "gnu"), repr(u32))]
52 | // Of course Windows thinks it's a special little shit and
53 | // picks a signed integer for an unsigned type
54 | #[cfg_attr(all(windows, not(target_env = "gnu")), repr(i32))]
55 | pub enum GGMLLogLevel {
56 |     None = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_NONE,
57 |     Info = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_INFO,
58 |     Warn = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_WARN,
59 |     Error = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_ERROR,
60 |     Debug = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_DEBUG,
61 |     Cont = whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_CONT,
62 |     Unknown(ggml_log_level),
63 | }
64 | impl From<ggml_log_level> for GGMLLogLevel {
65 |     fn from(level: ggml_log_level) -> Self {
66 |         match level {
67 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_NONE => GGMLLogLevel::None,
68 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_INFO => GGMLLogLevel::Info,
69 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_WARN => GGMLLogLevel::Warn,
70 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_ERROR => GGMLLogLevel::Error,
71 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_DEBUG => GGMLLogLevel::Debug,
72 |             whisper_rs_sys::ggml_log_level_GGML_LOG_LEVEL_CONT => GGMLLogLevel::Cont,
73 |             other => GGMLLogLevel::Unknown(other),
74 |         }
75 |     }
76 | }
77 | 


--------------------------------------------------------------------------------
/src/error.rs:
--------------------------------------------------------------------------------
  1 | use std::ffi::{c_int, NulError};
  2 | use std::str::Utf8Error;
  3 | 
  4 | /// If you have not configured a logging trampoline with [crate::whisper_sys_log::install_whisper_log_trampoline] or
  5 | /// [crate::whisper_sys_tracing::install_whisper_tracing_trampoline],
  6 | /// then `whisper.cpp`'s errors will be output to stderr,
  7 | /// so you can check there for more information upon receiving a `WhisperError`.
  8 | #[derive(Debug, Copy, Clone)]
  9 | pub enum WhisperError {
 10 |     /// Failed to create a new context.
 11 |     InitError,
 12 |     /// User didn't initialize spectrogram
 13 |     SpectrogramNotInitialized,
 14 |     /// Encode was not called.
 15 |     EncodeNotComplete,
 16 |     /// Decode was not called.
 17 |     DecodeNotComplete,
 18 |     /// Failed to calculate the spectrogram for some reason.
 19 |     UnableToCalculateSpectrogram,
 20 |     /// Failed to evaluate model.
 21 |     UnableToCalculateEvaluation,
 22 |     /// Failed to run the encoder
 23 |     FailedToEncode,
 24 |     /// Failed to run the decoder
 25 |     FailedToDecode,
 26 |     /// Invalid number of mel bands.
 27 |     InvalidMelBands,
 28 |     /// Invalid thread count
 29 |     InvalidThreadCount,
 30 |     /// Invalid UTF-8 detected in a string from Whisper.
 31 |     InvalidUtf8 {
 32 |         error_len: Option<usize>,
 33 |         valid_up_to: usize,
 34 |     },
 35 |     /// A null byte was detected in a user-provided string.
 36 |     NullByteInString { idx: usize },
 37 |     /// Whisper returned a null pointer.
 38 |     NullPointer,
 39 |     /// Generic whisper error. Varies depending on the function.
 40 |     GenericError(c_int),
 41 |     /// Whisper failed to convert the provided text into tokens.
 42 |     InvalidText,
 43 |     /// Creating a state pointer failed. Check stderr for more information.
 44 |     FailedToCreateState,
 45 |     /// No samples were provided.
 46 |     NoSamples,
 47 |     /// Input and output slices were not the same length.
 48 |     InputOutputLengthMismatch { input_len: usize, output_len: usize },
 49 |     /// Input slice was not an even number of samples.
 50 |     HalfSampleMissing(usize),
 51 | }
 52 | 
 53 | impl From<Utf8Error> for WhisperError {
 54 |     fn from(e: Utf8Error) -> Self {
 55 |         Self::InvalidUtf8 {
 56 |             error_len: e.error_len(),
 57 |             valid_up_to: e.valid_up_to(),
 58 |         }
 59 |     }
 60 | }
 61 | 
 62 | impl From<NulError> for WhisperError {
 63 |     fn from(e: NulError) -> Self {
 64 |         Self::NullByteInString {
 65 |             idx: e.nul_position(),
 66 |         }
 67 |     }
 68 | }
 69 | 
 70 | impl std::fmt::Display for WhisperError {
 71 |     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
 72 |         use WhisperError::*;
 73 |         match self {
 74 |             InitError => write!(f, "Failed to create a new whisper context."),
 75 |             SpectrogramNotInitialized => write!(f, "User didn't initialize spectrogram."),
 76 |             EncodeNotComplete => write!(f, "Encode was not called."),
 77 |             DecodeNotComplete => write!(f, "Decode was not called."),
 78 |             UnableToCalculateSpectrogram => {
 79 |                 write!(f, "Failed to calculate the spectrogram for some reason.")
 80 |             }
 81 |             UnableToCalculateEvaluation => write!(f, "Failed to evaluate model."),
 82 |             FailedToEncode => write!(f, "Failed to run the encoder."),
 83 |             FailedToDecode => write!(f, "Failed to run the decoder."),
 84 |             InvalidMelBands => write!(f, "Invalid number of mel bands."),
 85 |             InvalidThreadCount => write!(f, "Invalid thread count."),
 86 |             InvalidUtf8 {
 87 |                 valid_up_to,
 88 |                 error_len: Some(len),
 89 |             } => write!(
 90 |                 f,
 91 |                 "Invalid UTF-8 detected in a string from Whisper. Index: {}, Length: {}.",
 92 |                 valid_up_to, len
 93 |             ),
 94 |             InvalidUtf8 {
 95 |                 valid_up_to,
 96 |                 error_len: None,
 97 |             } => write!(
 98 |                 f,
 99 |                 "Invalid UTF-8 detected in a string from Whisper. Index: {}.",
100 |                 valid_up_to
101 |             ),
102 |             NullByteInString { idx } => write!(
103 |                 f,
104 |                 "A null byte was detected in a user-provided string. Index: {}",
105 |                 idx
106 |             ),
107 |             NullPointer => write!(f, "Whisper returned a null pointer."),
108 |             InvalidText => write!(
109 |                 f,
110 |                 "Whisper failed to convert the provided text into tokens."
111 |             ),
112 |             FailedToCreateState => write!(f, "Creating a state pointer failed."),
113 |             GenericError(c_int) => write!(
114 |                 f,
115 |                 "Generic whisper error. Varies depending on the function. Error code: {}",
116 |                 c_int
117 |             ),
118 |             NoSamples => write!(f, "Input sample buffer was empty."),
119 |             InputOutputLengthMismatch {
120 |                 output_len,
121 |                 input_len,
122 |             } => {
123 |                 write!(
124 |                     f,
125 |                     "Input and output slices were not the same length. Input: {}, Output: {}",
126 |                     input_len, output_len
127 |                 )
128 |             }
129 |             HalfSampleMissing(size) => {
130 |                 write!(
131 |                     f,
132 |                     "Input slice was not an even number of samples, got {}, expected {}",
133 |                     size,
134 |                     size + 1
135 |                 )
136 |             }
137 |         }
138 |     }
139 | }
140 | 
141 | impl std::error::Error for WhisperError {}
142 | 


--------------------------------------------------------------------------------
/src/ggml_logging_hook.rs:
--------------------------------------------------------------------------------
 1 | use crate::common_logging::{
 2 |     generic_debug, generic_error, generic_info, generic_trace, generic_warn, GGMLLogLevel,
 3 | };
 4 | use core::ffi::{c_char, c_void};
 5 | use std::borrow::Cow;
 6 | use std::ffi::CStr;
 7 | use std::sync::Once;
 8 | use whisper_rs_sys::ggml_log_level;
 9 | 
10 | static GGML_LOG_TRAMPOLINE_INSTALL: Once = Once::new();
11 | pub(crate) fn install_ggml_logging_hook() {
12 |     GGML_LOG_TRAMPOLINE_INSTALL.call_once(|| unsafe {
13 |         whisper_rs_sys::ggml_log_set(Some(ggml_logging_trampoline), std::ptr::null_mut())
14 |     });
15 | }
16 | 
17 | unsafe extern "C" fn ggml_logging_trampoline(
18 |     level: ggml_log_level,
19 |     text: *const c_char,
20 |     _: *mut c_void, // user_data
21 | ) {
22 |     if text.is_null() {
23 |         generic_error!("ggml_logging_trampoline: text is nullptr");
24 |     }
25 |     let level = GGMLLogLevel::from(level);
26 | 
27 |     // SAFETY: we must trust ggml that it will not pass us a string that does not satisfy
28 |     // from_ptr's requirements.
29 |     let log_str = unsafe { CStr::from_ptr(text) }.to_string_lossy();
30 | 
31 |     ggml_logging_trampoline_safe(level, log_str)
32 | }
33 | 
34 | // this code essentially compiles down to a noop if neither feature is enabled
35 | #[cfg_attr(
36 |     not(any(feature = "log_backend", feature = "tracing_backend")),
37 |     allow(unused_variables)
38 | )]
39 | fn ggml_logging_trampoline_safe(level: GGMLLogLevel, text: Cow<str>) {
40 |     match level {
41 |         GGMLLogLevel::None => {
42 |             // no clue what to do here, trace it?
43 |             generic_trace!("{}", text.trim());
44 |         }
45 |         GGMLLogLevel::Info => {
46 |             generic_info!("{}", text.trim());
47 |         }
48 |         GGMLLogLevel::Warn => {
49 |             generic_warn!("{}", text.trim());
50 |         }
51 |         GGMLLogLevel::Error => {
52 |             generic_error!("{}", text.trim());
53 |         }
54 |         GGMLLogLevel::Debug => {
55 |             generic_debug!("{}", text.trim());
56 |         }
57 |         GGMLLogLevel::Cont => {
58 |             // this means continue previous log
59 |             // storing state to do this is a massive pain so it's just a lot easier to not
60 |             // plus as far as i can tell it's not actually *used* anywhere
61 |             // ggml splits at 128 chars and doesn't actually change the kind of log
62 |             // so technically this is unused
63 |             generic_trace!("{}", text.trim());
64 |         }
65 |         GGMLLogLevel::Unknown(level) => {
66 |             generic_warn!(
67 |                 "ggml_logging_trampoline: unknown log level {}: message: {}",
68 |                 level,
69 |                 text.trim()
70 |             );
71 |         }
72 |     }
73 | }
74 | 


--------------------------------------------------------------------------------
/src/lib.rs:
--------------------------------------------------------------------------------
 1 | #![allow(clippy::uninlined_format_args)]
 2 | #![cfg_attr(test, feature(test))]
 3 | 
 4 | mod common_logging;
 5 | mod error;
 6 | mod ggml_logging_hook;
 7 | mod standalone;
 8 | mod utilities;
 9 | mod whisper_ctx;
10 | mod whisper_ctx_wrapper;
11 | mod whisper_grammar;
12 | mod whisper_logging_hook;
13 | mod whisper_params;
14 | mod whisper_state;
15 | 
16 | pub use common_logging::GGMLLogLevel;
17 | pub use error::WhisperError;
18 | pub use standalone::*;
19 | pub use utilities::*;
20 | pub use whisper_ctx::DtwMode;
21 | pub use whisper_ctx::DtwModelPreset;
22 | pub use whisper_ctx::DtwParameters;
23 | pub use whisper_ctx::WhisperContextParameters;
24 | use whisper_ctx::WhisperInnerContext;
25 | pub use whisper_ctx_wrapper::WhisperContext;
26 | pub use whisper_grammar::{WhisperGrammarElement, WhisperGrammarElementType};
27 | pub use whisper_params::{FullParams, SamplingStrategy, SegmentCallbackData};
28 | #[cfg(feature = "raw-api")]
29 | pub use whisper_rs_sys;
30 | pub use whisper_state::WhisperState;
31 | 
32 | pub type WhisperSysContext = whisper_rs_sys::whisper_context;
33 | pub type WhisperSysState = whisper_rs_sys::whisper_state;
34 | 
35 | pub type WhisperTokenData = whisper_rs_sys::whisper_token_data;
36 | pub type WhisperToken = whisper_rs_sys::whisper_token;
37 | pub type WhisperNewSegmentCallback = whisper_rs_sys::whisper_new_segment_callback;
38 | pub type WhisperStartEncoderCallback = whisper_rs_sys::whisper_encoder_begin_callback;
39 | pub type WhisperProgressCallback = whisper_rs_sys::whisper_progress_callback;
40 | pub type WhisperLogitsFilterCallback = whisper_rs_sys::whisper_logits_filter_callback;
41 | pub type WhisperAbortCallback = whisper_rs_sys::ggml_abort_callback;
42 | pub type WhisperLogCallback = whisper_rs_sys::ggml_log_callback;
43 | pub type DtwAhead = whisper_rs_sys::whisper_ahead;
44 | 
45 | /// The version of whisper.cpp that whisper-rs was linked with.
46 | pub static WHISPER_CPP_VERSION: &str = env!("WHISPER_CPP_VERSION");
47 | 
48 | /// Redirect all whisper.cpp and GGML logs to logging hooks installed by whisper-rs.
49 | ///
50 | /// This will stop most logs from being output to stdout/stderr and will bring them into
51 | /// `log` or `tracing`, if the `log_backend` or `tracing_backend` features, respectively,
52 | /// are enabled. If neither is enabled, this will essentially disable logging, as they won't
53 | /// be output anywhere.
54 | ///
55 | /// Note whisper.cpp and GGML do not reliably follow Rust logging conventions.
56 | /// Use your logging crate's configuration to control how these logs will be output.
57 | /// whisper-rs does not currently output any logs, but this may change in the future.
58 | /// You should configure by module path and use `whisper_rs::ggml_logging_hook`,
59 | /// and/or `whisper_rs::whisper_logging_hook`, to avoid possibly ignoring useful
60 | /// `whisper-rs` logs in the future.
61 | ///
62 | /// Safe to call multiple times. Only has an effect the first time.
63 | /// (note this means installing your own logging handlers with unsafe functions after this call
64 | /// is permanent and cannot be undone)
65 | pub fn install_logging_hooks() {
66 |     crate::whisper_logging_hook::install_whisper_logging_hook();
67 |     crate::ggml_logging_hook::install_ggml_logging_hook();
68 | }
69 | 


--------------------------------------------------------------------------------
/src/standalone.rs:
--------------------------------------------------------------------------------
  1 | //! Standalone functions that have no associated type.
  2 | 
  3 | use std::ffi::{c_int, CStr, CString};
  4 | 
  5 | /// Return the id of the specified language, returns -1 if not found
  6 | ///
  7 | /// # Arguments
  8 | /// * lang: The language to get the id for.
  9 | ///
 10 | /// # Returns
 11 | /// The ID of the language, None if not found.
 12 | ///
 13 | /// # Panics
 14 | /// Panics if the language contains a null byte.
 15 | ///
 16 | /// # C++ equivalent
 17 | /// `int whisper_lang_id(const char * lang)`
 18 | pub fn get_lang_id(lang: &str) -> Option<c_int> {
 19 |     let c_lang = CString::new(lang).expect("Language contains null byte");
 20 |     let ret = unsafe { whisper_rs_sys::whisper_lang_id(c_lang.as_ptr()) };
 21 |     if ret == -1 {
 22 |         None
 23 |     } else {
 24 |         Some(ret)
 25 |     }
 26 | }
 27 | 
 28 | /// Return the ID of the maximum language (ie the number of languages - 1)
 29 | ///
 30 | /// # Returns
 31 | /// i32
 32 | ///
 33 | /// # C++ equivalent
 34 | /// `int whisper_lang_max_id()`
 35 | pub fn get_lang_max_id() -> i32 {
 36 |     unsafe { whisper_rs_sys::whisper_lang_max_id() }
 37 | }
 38 | 
 39 | /// Get the short string of the specified language id (e.g. 2 -> "de").
 40 | ///
 41 | /// # Returns
 42 | /// The short string of the language, None if not found.
 43 | ///
 44 | /// # C++ equivalent
 45 | /// `const char * whisper_lang_str(int id)`
 46 | pub fn get_lang_str(id: i32) -> Option<&'static str> {
 47 |     let c_buf = unsafe { whisper_rs_sys::whisper_lang_str(id) };
 48 |     if c_buf.is_null() {
 49 |         None
 50 |     } else {
 51 |         let c_str = unsafe { CStr::from_ptr(c_buf) };
 52 |         Some(c_str.to_str().unwrap())
 53 |     }
 54 | }
 55 | 
 56 | /// Get the full string of the specified language name (e.g. 2 -> "german").
 57 | ///
 58 | /// # Returns
 59 | /// The full string of the language, None if not found.
 60 | ///
 61 | /// # C++ equivalent
 62 | /// `const char * whisper_lang_str_full(int id)`
 63 | pub fn get_lang_str_full(id: i32) -> Option<&'static str> {
 64 |     let c_buf = unsafe { whisper_rs_sys::whisper_lang_str_full(id) };
 65 |     if c_buf.is_null() {
 66 |         None
 67 |     } else {
 68 |         let c_str = unsafe { CStr::from_ptr(c_buf) };
 69 |         Some(c_str.to_str().unwrap())
 70 |     }
 71 | }
 72 | 
 73 | /// Callback to control logging output: default behaviour is to print to stderr.
 74 | ///
 75 | /// # Safety
 76 | /// The callback must be safe to call from C (i.e. no panicking, no unwinding, etc).
 77 | ///
 78 | /// # C++ equivalent
 79 | /// `void whisper_set_log_callback(whisper_log_callback callback);`
 80 | pub unsafe fn set_log_callback(
 81 |     log_callback: crate::WhisperLogCallback,
 82 |     user_data: *mut std::ffi::c_void,
 83 | ) {
 84 |     unsafe {
 85 |         whisper_rs_sys::whisper_log_set(log_callback, user_data);
 86 |     }
 87 | }
 88 | 
 89 | /// Print system information.
 90 | ///
 91 | /// # C++ equivalent
 92 | /// `const char * whisper_print_system_info()`
 93 | pub fn print_system_info() -> &'static str {
 94 |     let c_buf = unsafe { whisper_rs_sys::whisper_print_system_info() };
 95 |     let c_str = unsafe { CStr::from_ptr(c_buf) };
 96 |     c_str.to_str().unwrap()
 97 | }
 98 | 
 99 | /// Programmatically exposes the information provided by `print_system_info`
100 | ///
101 | /// # C++ equivalent
102 | /// `int ggml_cpu_has_...`
103 | pub struct SystemInfo {
104 |     pub avx: bool,
105 |     pub avx2: bool,
106 |     pub fma: bool,
107 |     pub f16c: bool,
108 | }
109 | 
110 | impl Default for SystemInfo {
111 |     fn default() -> Self {
112 |         unsafe {
113 |             Self {
114 |                 avx: whisper_rs_sys::ggml_cpu_has_avx() != 0,
115 |                 avx2: whisper_rs_sys::ggml_cpu_has_avx2() != 0,
116 |                 fma: whisper_rs_sys::ggml_cpu_has_fma() != 0,
117 |                 f16c: whisper_rs_sys::ggml_cpu_has_f16c() != 0,
118 |             }
119 |         }
120 |     }
121 | }
122 | 
123 | #[cfg(test)]
124 | mod tests {
125 |     use super::*;
126 | 
127 |     #[test]
128 |     fn test_openblas() {
129 |         let info = SystemInfo::default();
130 |         assert_eq!(info.blas, cfg!(feature = "openblas"));
131 |     }
132 | }
133 | 


--------------------------------------------------------------------------------
/src/utilities.rs:
--------------------------------------------------------------------------------
  1 | use crate::WhisperError;
  2 | 
  3 | /// Convert an array of 16 bit mono audio samples to a vector of 32 bit floats.
  4 | ///
  5 | /// # Arguments
  6 | /// * `samples` - The array of 16 bit mono audio samples.
  7 | /// * `output` - The vector of 32 bit floats to write the converted samples to.
  8 | ///
  9 | /// # Panics
 10 | /// * if `samples.len != output.len()`
 11 | ///
 12 | /// # Examples
 13 | /// ```
 14 | /// # use whisper_rs::convert_integer_to_float_audio;
 15 | /// let samples = [0i16; 1024];
 16 | /// let mut output = vec![0.0f32; samples.len()];
 17 | /// convert_integer_to_float_audio(&samples, &mut output).expect("input and output lengths should be equal");
 18 | /// ```
 19 | pub fn convert_integer_to_float_audio(
 20 |     samples: &[i16],
 21 |     output: &mut [f32],
 22 | ) -> Result<(), WhisperError> {
 23 |     if samples.len() != output.len() {
 24 |         return Err(WhisperError::InputOutputLengthMismatch {
 25 |             input_len: samples.len(),
 26 |             output_len: output.len(),
 27 |         });
 28 |     }
 29 | 
 30 |     for (input, output) in samples.iter().zip(output.iter_mut()) {
 31 |         *output = *input as f32 / 32768.0;
 32 |     }
 33 | 
 34 |     Ok(())
 35 | }
 36 | 
 37 | /// Convert 32-bit floating point stereo PCM audio to 32-bit floating point mono PCM audio.
 38 | ///
 39 | /// # Arguments
 40 | /// * `samples` - The array of 32-bit floating point stereo PCM audio samples.
 41 | ///
 42 | /// # Errors
 43 | /// * if `samples.len()` is odd
 44 | ///
 45 | /// # Returns
 46 | /// A vector of 32-bit floating point mono PCM audio samples.
 47 | ///
 48 | /// # Examples
 49 | /// ```
 50 | /// # use whisper_rs::convert_stereo_to_mono_audio;
 51 | /// let samples = [0.0f32; 1024];
 52 | /// let mono = convert_stereo_to_mono_audio(&samples).expect("should be no half samples missing");
 53 | /// ```
 54 | pub fn convert_stereo_to_mono_audio(samples: &[f32]) -> Result<Vec<f32>, WhisperError> {
 55 |     if samples.len() & 1 != 0 {
 56 |         return Err(WhisperError::HalfSampleMissing(samples.len()));
 57 |     }
 58 | 
 59 |     Ok(samples
 60 |         .chunks_exact(2)
 61 |         .map(|x| (x[0] + x[1]) / 2.0)
 62 |         .collect())
 63 | }
 64 | 
 65 | #[cfg(test)]
 66 | mod test {
 67 |     use super::*;
 68 |     use rand::distributions::{Distribution, Standard};
 69 |     use rand::Rng;
 70 |     use std::hint::black_box;
 71 | 
 72 |     extern crate test;
 73 | 
 74 |     fn random_sample_data<T>() -> Vec<T>
 75 |     where
 76 |         Standard: Distribution<T>,
 77 |     {
 78 |         const SAMPLE_SIZE: usize = 1_048_576;
 79 | 
 80 |         let mut rng = rand::thread_rng();
 81 |         let mut samples = Vec::with_capacity(SAMPLE_SIZE);
 82 |         for _ in 0..SAMPLE_SIZE {
 83 |             samples.push(rng.gen::<T>());
 84 |         }
 85 |         samples
 86 |     }
 87 | 
 88 |     #[test]
 89 |     pub fn assert_stereo_to_mono_err() {
 90 |         let samples = random_sample_data::<f32>();
 91 |         let mono = convert_stereo_to_mono_audio(&samples);
 92 |         assert!(mono.is_err());
 93 |     }
 94 | 
 95 |     #[bench]
 96 |     pub fn bench_stereo_to_mono(b: &mut test::Bencher) {
 97 |         let samples = random_sample_data::<f32>();
 98 |         b.iter(|| black_box(convert_stereo_to_mono_audio(black_box(&samples))));
 99 |     }
100 | 
101 |     #[bench]
102 |     pub fn bench_integer_to_float(b: &mut test::Bencher) {
103 |         let samples = random_sample_data::<i16>();
104 |         let mut output = vec![0.0f32; samples.len()];
105 |         b.iter(|| {
106 |             black_box(convert_integer_to_float_audio(
107 |                 black_box(&samples),
108 |                 black_box(&mut output),
109 |             ))
110 |         });
111 |     }
112 | }
113 | 


--------------------------------------------------------------------------------
/src/whisper_ctx.rs:
--------------------------------------------------------------------------------
  1 | use crate::error::WhisperError;
  2 | use crate::WhisperToken;
  3 | use std::ffi::{c_int, CStr, CString};
  4 | 
  5 | /// Safe Rust wrapper around a Whisper context.
  6 | ///
  7 | /// You likely want to create this with [WhisperInnerContext::new_with_params],
  8 | /// create a state with [WhisperInnerContext::create_state],
  9 | /// then run a full transcription with [WhisperState::full].
 10 | #[derive(Debug)]
 11 | pub struct WhisperInnerContext {
 12 |     pub(crate) ctx: *mut whisper_rs_sys::whisper_context,
 13 | }
 14 | 
 15 | impl WhisperInnerContext {
 16 |     /// Create a new WhisperContext from a file, with parameters.
 17 |     ///
 18 |     /// # Arguments
 19 |     /// * path: The path to the model file.
 20 |     /// * parameters: A parameter struct containing the parameters to use.
 21 |     ///
 22 |     /// # Returns
 23 |     /// Ok(Self) on success, Err(WhisperError) on failure.
 24 |     ///
 25 |     /// # C++ equivalent
 26 |     /// `struct whisper_context * whisper_init_from_file_with_params_no_state(const char * path_model, struct whisper_context_params params);`
 27 |     pub fn new_with_params(
 28 |         path: &str,
 29 |         parameters: WhisperContextParameters,
 30 |     ) -> Result<Self, WhisperError> {
 31 |         let path_cstr = CString::new(path)?;
 32 |         let ctx = unsafe {
 33 |             whisper_rs_sys::whisper_init_from_file_with_params_no_state(
 34 |                 path_cstr.as_ptr(),
 35 |                 parameters.to_c_struct(),
 36 |             )
 37 |         };
 38 |         if ctx.is_null() {
 39 |             Err(WhisperError::InitError)
 40 |         } else {
 41 |             Ok(Self { ctx })
 42 |         }
 43 |     }
 44 | 
 45 |     /// Create a new WhisperContext from a buffer.
 46 |     ///
 47 |     /// # Arguments
 48 |     /// * buffer: The buffer containing the model.
 49 |     ///
 50 |     /// # Returns
 51 |     /// Ok(Self) on success, Err(WhisperError) on failure.
 52 |     ///
 53 |     /// # C++ equivalent
 54 |     /// `struct whisper_context * whisper_init_from_buffer_with_params_no_state(void * buffer, size_t buffer_size, struct whisper_context_params params);`
 55 |     pub fn new_from_buffer_with_params(
 56 |         buffer: &[u8],
 57 |         parameters: WhisperContextParameters,
 58 |     ) -> Result<Self, WhisperError> {
 59 |         let ctx = unsafe {
 60 |             whisper_rs_sys::whisper_init_from_buffer_with_params_no_state(
 61 |                 buffer.as_ptr() as _,
 62 |                 buffer.len(),
 63 |                 parameters.to_c_struct(),
 64 |             )
 65 |         };
 66 |         if ctx.is_null() {
 67 |             Err(WhisperError::InitError)
 68 |         } else {
 69 |             Ok(Self { ctx })
 70 |         }
 71 |     }
 72 | 
 73 |     /// Convert the provided text into tokens.
 74 |     ///
 75 |     /// # Arguments
 76 |     /// * text: The text to convert.
 77 |     ///
 78 |     /// # Returns
 79 |     /// `Ok(Vec<WhisperToken>)` on success, `Err(WhisperError)` on failure.
 80 |     ///
 81 |     /// # C++ equivalent
 82 |     /// `int whisper_tokenize(struct whisper_context * ctx, const char * text, whisper_token * tokens, int n_max_tokens);`
 83 |     pub fn tokenize(
 84 |         &self,
 85 |         text: &str,
 86 |         max_tokens: usize,
 87 |     ) -> Result<Vec<WhisperToken>, WhisperError> {
 88 |         // convert the text to a nul-terminated C string. Will raise an error if the text contains
 89 |         // any nul bytes.
 90 |         let text = CString::new(text)?;
 91 |         // allocate at least max_tokens to ensure the memory is valid
 92 |         let mut tokens: Vec<WhisperToken> = Vec::with_capacity(max_tokens);
 93 |         let ret = unsafe {
 94 |             whisper_rs_sys::whisper_tokenize(
 95 |                 self.ctx,
 96 |                 text.as_ptr(),
 97 |                 tokens.as_mut_ptr(),
 98 |                 max_tokens as c_int,
 99 |             )
100 |         };
101 |         if ret == -1 {
102 |             Err(WhisperError::InvalidText)
103 |         } else {
104 |             // SAFETY: when ret != -1, we know that the length of the vector is at least ret tokens
105 |             unsafe { tokens.set_len(ret as usize) };
106 |             Ok(tokens)
107 |         }
108 |     }
109 | 
110 |     /// Get n_vocab.
111 |     ///
112 |     /// # Returns
113 |     /// c_int
114 |     ///
115 |     /// # C++ equivalent
116 |     /// `int whisper_n_vocab        (struct whisper_context * ctx)`
117 |     #[inline]
118 |     pub fn n_vocab(&self) -> c_int {
119 |         unsafe { whisper_rs_sys::whisper_n_vocab(self.ctx) }
120 |     }
121 | 
122 |     /// Get n_text_ctx.
123 |     ///
124 |     /// # Returns
125 |     /// c_int
126 |     ///
127 |     /// # C++ equivalent
128 |     /// `int whisper_n_text_ctx     (struct whisper_context * ctx);`
129 |     #[inline]
130 |     pub fn n_text_ctx(&self) -> c_int {
131 |         unsafe { whisper_rs_sys::whisper_n_text_ctx(self.ctx) }
132 |     }
133 | 
134 |     /// Get n_audio_ctx.
135 |     ///
136 |     /// # Returns
137 |     /// c_int
138 |     ///
139 |     /// # C++ equivalent
140 |     /// `int whisper_n_audio_ctx     (struct whisper_context * ctx);`
141 |     #[inline]
142 |     pub fn n_audio_ctx(&self) -> c_int {
143 |         unsafe { whisper_rs_sys::whisper_n_audio_ctx(self.ctx) }
144 |     }
145 | 
146 |     /// Does this model support multiple languages?
147 |     ///
148 |     /// # C++ equivalent
149 |     /// `int whisper_is_multilingual(struct whisper_context * ctx)`
150 |     #[inline]
151 |     pub fn is_multilingual(&self) -> bool {
152 |         unsafe { whisper_rs_sys::whisper_is_multilingual(self.ctx) != 0 }
153 |     }
154 | 
155 |     /// Get model_n_vocab.
156 |     ///
157 |     /// # Returns
158 |     /// c_int
159 |     ///
160 |     /// # C++ equivalent
161 |     /// `int whisper_model_n_vocab      (struct whisper_context * ctx);`
162 |     #[inline]
163 |     pub fn model_n_vocab(&self) -> c_int {
164 |         unsafe { whisper_rs_sys::whisper_model_n_vocab(self.ctx) }
165 |     }
166 | 
167 |     /// Get model_n_audio_ctx.
168 |     ///
169 |     /// # Returns
170 |     /// c_int
171 |     ///
172 |     /// # C++ equivalent
173 |     /// `int whisper_model_n_audio_ctx    (struct whisper_context * ctx)`
174 |     #[inline]
175 |     pub fn model_n_audio_ctx(&self) -> c_int {
176 |         unsafe { whisper_rs_sys::whisper_model_n_audio_ctx(self.ctx) }
177 |     }
178 | 
179 |     /// Get model_n_audio_state.
180 |     ///
181 |     /// # Returns
182 |     /// c_int
183 |     ///
184 |     /// # C++ equivalent
185 |     /// `int whisper_model_n_audio_state(struct whisper_context * ctx);`
186 |     #[inline]
187 |     pub fn model_n_audio_state(&self) -> c_int {
188 |         unsafe { whisper_rs_sys::whisper_model_n_audio_state(self.ctx) }
189 |     }
190 | 
191 |     /// Get model_n_audio_head.
192 |     ///
193 |     /// # Returns
194 |     /// c_int
195 |     ///
196 |     /// # C++ equivalent
197 |     /// `int whisper_model_n_audio_head (struct whisper_context * ctx);`
198 |     #[inline]
199 |     pub fn model_n_audio_head(&self) -> c_int {
200 |         unsafe { whisper_rs_sys::whisper_model_n_audio_head(self.ctx) }
201 |     }
202 | 
203 |     /// Get model_n_audio_layer.
204 |     ///
205 |     /// # Returns
206 |     /// c_int
207 |     ///
208 |     /// # C++ equivalent
209 |     /// `int whisper_model_n_audio_layer(struct whisper_context * ctx);`
210 |     #[inline]
211 |     pub fn model_n_audio_layer(&self) -> c_int {
212 |         unsafe { whisper_rs_sys::whisper_model_n_audio_layer(self.ctx) }
213 |     }
214 | 
215 |     /// Get model_n_text_ctx.
216 |     ///
217 |     /// # Returns
218 |     /// c_int
219 |     ///
220 |     /// # C++ equivalent
221 |     /// `int whisper_model_n_text_ctx     (struct whisper_context * ctx)`
222 |     #[inline]
223 |     pub fn model_n_text_ctx(&self) -> c_int {
224 |         unsafe { whisper_rs_sys::whisper_model_n_text_ctx(self.ctx) }
225 |     }
226 | 
227 |     /// Get model_n_text_state.
228 |     ///
229 |     /// # Returns
230 |     /// c_int
231 |     ///
232 |     /// # C++ equivalent
233 |     /// `int whisper_model_n_text_state (struct whisper_context * ctx);`
234 |     #[inline]
235 |     pub fn model_n_text_state(&self) -> c_int {
236 |         unsafe { whisper_rs_sys::whisper_model_n_text_state(self.ctx) }
237 |     }
238 | 
239 |     /// Get model_n_text_head.
240 |     ///
241 |     /// # Returns
242 |     /// c_int
243 |     ///
244 |     /// # C++ equivalent
245 |     /// `int whisper_model_n_text_head  (struct whisper_context * ctx);`
246 |     #[inline]
247 |     pub fn model_n_text_head(&self) -> c_int {
248 |         unsafe { whisper_rs_sys::whisper_model_n_text_head(self.ctx) }
249 |     }
250 | 
251 |     /// Get model_n_text_layer.
252 |     ///
253 |     /// # Returns
254 |     /// c_int
255 |     ///
256 |     /// # C++ equivalent
257 |     /// `int whisper_model_n_text_layer (struct whisper_context * ctx);`
258 |     #[inline]
259 |     pub fn model_n_text_layer(&self) -> c_int {
260 |         unsafe { whisper_rs_sys::whisper_model_n_text_layer(self.ctx) }
261 |     }
262 | 
263 |     /// Get model_n_mels.
264 |     ///
265 |     /// # Returns
266 |     /// c_int
267 |     ///
268 |     /// # C++ equivalent
269 |     /// `int whisper_model_n_mels       (struct whisper_context * ctx);`
270 |     #[inline]
271 |     pub fn model_n_mels(&self) -> c_int {
272 |         unsafe { whisper_rs_sys::whisper_model_n_mels(self.ctx) }
273 |     }
274 | 
275 |     /// Get model_ftype.
276 |     ///
277 |     /// # Returns
278 |     /// c_int
279 |     ///
280 |     /// # C++ equivalent
281 |     /// `int whisper_model_ftype          (struct whisper_context * ctx);`
282 |     #[inline]
283 |     pub fn model_ftype(&self) -> c_int {
284 |         unsafe { whisper_rs_sys::whisper_model_ftype(self.ctx) }
285 |     }
286 | 
287 |     /// Get model_type.
288 |     ///
289 |     /// # Returns
290 |     /// c_int
291 |     ///
292 |     /// # C++ equivalent
293 |     /// `int whisper_model_type         (struct whisper_context * ctx);`
294 |     #[inline]
295 |     pub fn model_type(&self) -> c_int {
296 |         unsafe { whisper_rs_sys::whisper_model_type(self.ctx) }
297 |     }
298 | 
299 |     // token functions
300 |     /// Convert a token ID to a string.
301 |     ///
302 |     /// # Arguments
303 |     /// * token_id: ID of the token.
304 |     ///
305 |     /// # Returns
306 |     /// Ok(&str) on success, Err(WhisperError) on failure.
307 |     ///
308 |     /// # C++ equivalent
309 |     /// `const char * whisper_token_to_str(struct whisper_context * ctx, whisper_token token)`
310 |     pub fn token_to_str(&self, token_id: WhisperToken) -> Result<&str, WhisperError> {
311 |         let c_str = self.token_to_cstr(token_id)?;
312 |         let r_str = c_str.to_str()?;
313 |         Ok(r_str)
314 |     }
315 | 
316 |     /// Convert a token ID to a &CStr.
317 |     ///
318 |     /// # Arguments
319 |     /// * token_id: ID of the token.
320 |     ///
321 |     /// # Returns
322 |     /// Ok(String) on success, Err(WhisperError) on failure.
323 |     ///
324 |     /// # C++ equivalent
325 |     /// `const char * whisper_token_to_str(struct whisper_context * ctx, whisper_token token)`
326 |     pub fn token_to_cstr(&self, token_id: WhisperToken) -> Result<&CStr, WhisperError> {
327 |         let ret = unsafe { whisper_rs_sys::whisper_token_to_str(self.ctx, token_id) };
328 |         if ret.is_null() {
329 |             return Err(WhisperError::NullPointer);
330 |         }
331 |         Ok(unsafe { CStr::from_ptr(ret) })
332 |     }
333 | 
334 |     /// Undocumented but exposed function in the C++ API.
335 |     /// `const char * whisper_model_type_readable(struct whisper_context * ctx);`
336 |     ///
337 |     /// # Returns
338 |     /// Ok(String) on success, Err(WhisperError) on failure.
339 |     pub fn model_type_readable(&self) -> Result<String, WhisperError> {
340 |         let ret = unsafe { whisper_rs_sys::whisper_model_type_readable(self.ctx) };
341 |         if ret.is_null() {
342 |             return Err(WhisperError::NullPointer);
343 |         }
344 |         let c_str = unsafe { CStr::from_ptr(ret) };
345 |         let r_str = c_str.to_str()?;
346 |         Ok(r_str.to_string())
347 |     }
348 | 
349 |     /// Get the ID of the eot token.
350 |     ///
351 |     /// # C++ equivalent
352 |     /// `whisper_token whisper_token_eot (struct whisper_context * ctx)`
353 |     #[inline]
354 |     pub fn token_eot(&self) -> WhisperToken {
355 |         unsafe { whisper_rs_sys::whisper_token_eot(self.ctx) }
356 |     }
357 | 
358 |     /// Get the ID of the sot token.
359 |     ///
360 |     /// # C++ equivalent
361 |     /// `whisper_token whisper_token_sot (struct whisper_context * ctx)`
362 |     #[inline]
363 |     pub fn token_sot(&self) -> WhisperToken {
364 |         unsafe { whisper_rs_sys::whisper_token_sot(self.ctx) }
365 |     }
366 | 
367 |     /// Get the ID of the solm token.
368 |     ///
369 |     /// # C++ equivalent
370 |     /// `whisper_token whisper_token_solm(struct whisper_context * ctx)`
371 |     #[inline]
372 |     pub fn token_solm(&self) -> WhisperToken {
373 |         unsafe { whisper_rs_sys::whisper_token_solm(self.ctx) }
374 |     }
375 | 
376 |     /// Get the ID of the prev token.
377 |     ///
378 |     /// # C++ equivalent
379 |     /// `whisper_token whisper_token_prev(struct whisper_context * ctx)`
380 |     #[inline]
381 |     pub fn token_prev(&self) -> WhisperToken {
382 |         unsafe { whisper_rs_sys::whisper_token_prev(self.ctx) }
383 |     }
384 | 
385 |     /// Get the ID of the nosp token.
386 |     ///
387 |     /// # C++ equivalent
388 |     /// `whisper_token whisper_token_nosp(struct whisper_context * ctx)`
389 |     #[inline]
390 |     pub fn token_nosp(&self) -> WhisperToken {
391 |         unsafe { whisper_rs_sys::whisper_token_nosp(self.ctx) }
392 |     }
393 | 
394 |     /// Get the ID of the not token.
395 |     ///
396 |     /// # C++ equivalent
397 |     /// `whisper_token whisper_token_not (struct whisper_context * ctx)`
398 |     #[inline]
399 |     pub fn token_not(&self) -> WhisperToken {
400 |         unsafe { whisper_rs_sys::whisper_token_not(self.ctx) }
401 |     }
402 | 
403 |     /// Get the ID of the beg token.
404 |     ///
405 |     /// # C++ equivalent
406 |     /// `whisper_token whisper_token_beg (struct whisper_context * ctx)`
407 |     #[inline]
408 |     pub fn token_beg(&self) -> WhisperToken {
409 |         unsafe { whisper_rs_sys::whisper_token_beg(self.ctx) }
410 |     }
411 | 
412 |     /// Get the ID of a specified language token
413 |     ///
414 |     /// # Arguments
415 |     /// * lang_id: ID of the language
416 |     ///
417 |     /// # C++ equivalent
418 |     /// `whisper_token whisper_token_lang(struct whisper_context * ctx, int lang_id)`
419 |     #[inline]
420 |     pub fn token_lang(&self, lang_id: c_int) -> WhisperToken {
421 |         unsafe { whisper_rs_sys::whisper_token_lang(self.ctx, lang_id) }
422 |     }
423 | 
424 |     /// Print performance statistics to stderr.
425 |     ///
426 |     /// # C++ equivalent
427 |     /// `void whisper_print_timings(struct whisper_context * ctx)`
428 |     #[inline]
429 |     pub fn print_timings(&self) {
430 |         unsafe { whisper_rs_sys::whisper_print_timings(self.ctx) }
431 |     }
432 | 
433 |     /// Reset performance statistics.
434 |     ///
435 |     /// # C++ equivalent
436 |     /// `void whisper_reset_timings(struct whisper_context * ctx)`
437 |     #[inline]
438 |     pub fn reset_timings(&self) {
439 |         unsafe { whisper_rs_sys::whisper_reset_timings(self.ctx) }
440 |     }
441 | 
442 |     // task tokens
443 |     /// Get the ID of the translate task token.
444 |     ///
445 |     /// # C++ equivalent
446 |     /// `whisper_token whisper_token_translate ()`
447 |     pub fn token_translate(&self) -> WhisperToken {
448 |         unsafe { whisper_rs_sys::whisper_token_translate(self.ctx) }
449 |     }
450 | 
451 |     /// Get the ID of the transcribe task token.
452 |     ///
453 |     /// # C++ equivalent
454 |     /// `whisper_token whisper_token_transcribe()`
455 |     pub fn token_transcribe(&self) -> WhisperToken {
456 |         unsafe { whisper_rs_sys::whisper_token_transcribe(self.ctx) }
457 |     }
458 | }
459 | 
460 | impl Drop for WhisperInnerContext {
461 |     #[inline]
462 |     fn drop(&mut self) {
463 |         unsafe { whisper_rs_sys::whisper_free(self.ctx) };
464 |     }
465 | }
466 | 
467 | // following implementations are safe
468 | // see https://github.com/ggerganov/whisper.cpp/issues/32#issuecomment-1272790388
469 | unsafe impl Send for WhisperInnerContext {}
470 | unsafe impl Sync for WhisperInnerContext {}
471 | 
472 | pub struct WhisperContextParameters<'a> {
473 |     /// Use GPU if available.
474 |     pub use_gpu: bool,
475 |     /// Enable flash attention, default false
476 |     ///
477 |     /// **Warning** Can't be used with DTW. DTW will be disabled if flash_attn is true
478 |     pub flash_attn: bool,
479 |     /// GPU device id, default 0
480 |     pub gpu_device: c_int,
481 |     /// DTW token level timestamp parameters
482 |     pub dtw_parameters: DtwParameters<'a>,
483 | }
484 | 
485 | #[allow(clippy::derivable_impls)] // this impl cannot be derived
486 | impl<'a> Default for WhisperContextParameters<'a> {
487 |     fn default() -> Self {
488 |         Self {
489 |             use_gpu: cfg!(feature = "_gpu"),
490 |             flash_attn: false,
491 |             gpu_device: 0,
492 |             dtw_parameters: DtwParameters::default(),
493 |         }
494 |     }
495 | }
496 | impl<'a> WhisperContextParameters<'a> {
497 |     pub fn new() -> Self {
498 |         Self::default()
499 |     }
500 |     pub fn use_gpu(&mut self, use_gpu: bool) -> &mut Self {
501 |         self.use_gpu = use_gpu;
502 |         self
503 |     }
504 |     pub fn flash_attn(&mut self, flash_attn: bool) -> &mut Self {
505 |         self.flash_attn = flash_attn;
506 |         self
507 |     }
508 |     pub fn gpu_device(&mut self, gpu_device: c_int) -> &mut Self {
509 |         self.gpu_device = gpu_device;
510 |         self
511 |     }
512 |     pub fn dtw_parameters(&mut self, dtw_parameters: DtwParameters<'a>) -> &mut Self {
513 |         self.dtw_parameters = dtw_parameters;
514 |         self
515 |     }
516 | 
517 |     fn to_c_struct(&self) -> whisper_rs_sys::whisper_context_params {
518 |         let dtw_token_timestamps = !matches!(self.dtw_parameters.mode, DtwMode::None);
519 |         let mut dtw_aheads_preset =
520 |             whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_NONE;
521 |         let mut dtw_n_top: c_int = -1;
522 |         let mut dtw_aheads = whisper_rs_sys::whisper_aheads {
523 |             n_heads: 0,
524 |             heads: std::ptr::null(),
525 |         };
526 | 
527 |         match &self.dtw_parameters.mode {
528 |             DtwMode::None => {}
529 |             DtwMode::TopMost { n_top } => {
530 |                 dtw_aheads_preset =
531 |                     whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_N_TOP_MOST;
532 |                 dtw_n_top = *n_top;
533 |             }
534 |             DtwMode::Custom { aheads } => {
535 |                 dtw_aheads_preset =
536 |                     whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_CUSTOM;
537 | 
538 |                 dtw_aheads = whisper_rs_sys::whisper_aheads {
539 |                     n_heads: aheads.len(),
540 |                     heads: aheads.as_ptr(),
541 |                 };
542 |             }
543 |             DtwMode::ModelPreset { model_preset } => match model_preset {
544 |                 DtwModelPreset::TinyEn => {
545 |                     dtw_aheads_preset =
546 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_TINY_EN;
547 |                 }
548 |                 DtwModelPreset::Tiny => {
549 |                     dtw_aheads_preset =
550 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_TINY;
551 |                 }
552 |                 DtwModelPreset::BaseEn => {
553 |                     dtw_aheads_preset =
554 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_BASE_EN;
555 |                 }
556 |                 DtwModelPreset::Base => {
557 |                     dtw_aheads_preset =
558 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_BASE;
559 |                 }
560 |                 DtwModelPreset::SmallEn => {
561 |                     dtw_aheads_preset =
562 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_SMALL_EN;
563 |                 }
564 |                 DtwModelPreset::Small => {
565 |                     dtw_aheads_preset =
566 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_SMALL;
567 |                 }
568 |                 DtwModelPreset::MediumEn => {
569 |                     dtw_aheads_preset =
570 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_MEDIUM_EN;
571 |                 }
572 |                 DtwModelPreset::Medium => {
573 |                     dtw_aheads_preset =
574 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_MEDIUM;
575 |                 }
576 |                 DtwModelPreset::LargeV1 => {
577 |                     dtw_aheads_preset =
578 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V1;
579 |                 }
580 |                 DtwModelPreset::LargeV2 => {
581 |                     dtw_aheads_preset =
582 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V2;
583 |                 }
584 |                 DtwModelPreset::LargeV3 => {
585 |                     dtw_aheads_preset =
586 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V3;
587 |                 }
588 |                 DtwModelPreset::LargeV3Turbo => {
589 |                     dtw_aheads_preset =
590 |                         whisper_rs_sys::whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V3_TURBO;
591 |                 }
592 |             },
593 |         }
594 | 
595 |         whisper_rs_sys::whisper_context_params {
596 |             use_gpu: self.use_gpu,
597 |             flash_attn: self.flash_attn,
598 |             gpu_device: self.gpu_device,
599 |             dtw_token_timestamps,
600 |             dtw_aheads_preset,
601 |             dtw_n_top,
602 |             dtw_aheads,
603 |             dtw_mem_size: self.dtw_parameters.dtw_mem_size,
604 |         }
605 |     }
606 | }
607 | 
608 | /// [EXPERIMENTAL] Enable Token-level timestamps with DTW, default Disabled
609 | #[derive(Debug, Clone)]
610 | pub struct DtwParameters<'a> {
611 |     pub mode: DtwMode<'a>,
612 |     pub dtw_mem_size: usize,
613 | }
614 | 
615 | impl Default for DtwParameters<'_> {
616 |     fn default() -> Self {
617 |         Self {
618 |             mode: DtwMode::None,
619 |             dtw_mem_size: 1024 * 1024 * 128,
620 |         }
621 |     }
622 | }
623 | 
624 | #[derive(Debug, Clone)]
625 | pub enum DtwMode<'a> {
626 |     /// DTW token level timestamps disabled
627 |     None,
628 |     /// Use N Top Most layers from loaded model
629 |     TopMost {
630 |         /// Number of top text layers used from model, should be 0 < n_top <= model n_text_layer
631 |         n_top: c_int,
632 |     },
633 |     /// Use custom aheads, non-empty list of whisper_ahead.
634 |     /// 0 < n_text_layer < model n_text_layer, 0 < n_head < model n_text_head for each element
635 |     /// See details https://github.com/ggerganov/whisper.cpp/pull/1485#discussion_r1519681143
636 |     Custom {
637 |         aheads: &'a [whisper_rs_sys::whisper_ahead],
638 |     },
639 |     /// Use predefined preset for standard models
640 |     ModelPreset { model_preset: DtwModelPreset },
641 | }
642 | 
643 | #[derive(Debug, Clone)]
644 | pub enum DtwModelPreset {
645 |     TinyEn,
646 |     Tiny,
647 |     BaseEn,
648 |     Base,
649 |     SmallEn,
650 |     Small,
651 |     MediumEn,
652 |     Medium,
653 |     LargeV1,
654 |     LargeV2,
655 |     LargeV3,
656 |     LargeV3Turbo,
657 | }
658 | 
659 | #[cfg(test)]
660 | #[cfg(feature = "test-with-tiny-model")]
661 | mod test_with_tiny_model {
662 |     use super::*;
663 |     const MODEL_PATH: &str = "./sys/whisper.cpp/models/ggml-tiny.en.bin";
664 | 
665 |     // These tests expect that the tiny.en model has been downloaded
666 |     // using the script `sys/whisper.cpp/models/download-ggml-model.sh tiny.en`
667 | 
668 |     #[test]
669 |     fn test_tokenize_round_trip() {
670 |         let ctx = WhisperInnerContext::new(MODEL_PATH).expect("Download the ggml-tiny.en model using 'sys/whisper.cpp/models/download-ggml-model.sh tiny.en'");
671 |         let text_in = " And so my fellow Americans, ask not what your country can do for you, ask what you can do for your country.";
672 |         let tokens = ctx.tokenize(text_in, 1024).unwrap();
673 |         let text_out = tokens
674 |             .into_iter()
675 |             .map(|t| ctx.token_to_str(t).unwrap())
676 |             .collect::<Vec<_>>()
677 |             .join("");
678 |         assert_eq!(text_in, text_out);
679 |     }
680 | }
681 | 


--------------------------------------------------------------------------------
/src/whisper_ctx_wrapper.rs:
--------------------------------------------------------------------------------
  1 | use std::ffi::{c_int, CStr};
  2 | use std::sync::Arc;
  3 | 
  4 | use crate::{
  5 |     WhisperContextParameters, WhisperError, WhisperInnerContext, WhisperState, WhisperToken,
  6 | };
  7 | 
  8 | pub struct WhisperContext {
  9 |     ctx: Arc<WhisperInnerContext>,
 10 | }
 11 | 
 12 | impl WhisperContext {
 13 |     fn wrap(ctx: WhisperInnerContext) -> Self {
 14 |         Self { ctx: Arc::new(ctx) }
 15 |     }
 16 | 
 17 |     /// Create a new WhisperContext from a file, with parameters.
 18 |     ///
 19 |     /// # Arguments
 20 |     /// * path: The path to the model file.
 21 |     /// * parameters: A parameter struct containing the parameters to use.
 22 |     ///
 23 |     /// # Returns
 24 |     /// Ok(Self) on success, Err(WhisperError) on failure.
 25 |     ///
 26 |     /// # C++ equivalent
 27 |     /// `struct whisper_context * whisper_init_from_file_with_params_no_state(const char * path_model, struct whisper_context_params params);`
 28 |     pub fn new_with_params(
 29 |         path: &str,
 30 |         parameters: WhisperContextParameters,
 31 |     ) -> Result<Self, WhisperError> {
 32 |         let ctx = WhisperInnerContext::new_with_params(path, parameters)?;
 33 |         Ok(Self::wrap(ctx))
 34 |     }
 35 | 
 36 |     /// Create a new WhisperContext from a buffer.
 37 |     ///
 38 |     /// # Arguments
 39 |     /// * buffer: The buffer containing the model.
 40 |     ///
 41 |     /// # Returns
 42 |     /// Ok(Self) on success, Err(WhisperError) on failure.
 43 |     ///
 44 |     /// # C++ equivalent
 45 |     /// `struct whisper_context * whisper_init_from_buffer_with_params_no_state(void * buffer, size_t buffer_size, struct whisper_context_params params);`
 46 |     pub fn new_from_buffer_with_params(
 47 |         buffer: &[u8],
 48 |         parameters: WhisperContextParameters,
 49 |     ) -> Result<Self, WhisperError> {
 50 |         let ctx = WhisperInnerContext::new_from_buffer_with_params(buffer, parameters)?;
 51 |         Ok(Self::wrap(ctx))
 52 |     }
 53 | 
 54 |     /// Convert the provided text into tokens.
 55 |     ///
 56 |     /// # Arguments
 57 |     /// * text: The text to convert.
 58 |     ///
 59 |     /// # Returns
 60 |     /// `Ok(Vec<WhisperToken>)` on success, `Err(WhisperError)` on failure.
 61 |     ///
 62 |     /// # C++ equivalent
 63 |     /// `int whisper_tokenize(struct whisper_context * ctx, const char * text, whisper_token * tokens, int n_max_tokens);`
 64 |     pub fn tokenize(
 65 |         &self,
 66 |         text: &str,
 67 |         max_tokens: usize,
 68 |     ) -> Result<Vec<WhisperToken>, WhisperError> {
 69 |         self.ctx.tokenize(text, max_tokens)
 70 |     }
 71 | 
 72 |     /// Get n_vocab.
 73 |     ///
 74 |     /// # Returns
 75 |     /// c_int
 76 |     ///
 77 |     /// # C++ equivalent
 78 |     /// `int whisper_n_vocab        (struct whisper_context * ctx)`
 79 |     #[inline]
 80 |     pub fn n_vocab(&self) -> c_int {
 81 |         self.ctx.n_vocab()
 82 |     }
 83 | 
 84 |     /// Get n_text_ctx.
 85 |     ///
 86 |     /// # Returns
 87 |     /// c_int
 88 |     ///
 89 |     /// # C++ equivalent
 90 |     /// `int whisper_n_text_ctx     (struct whisper_context * ctx);`
 91 |     #[inline]
 92 |     pub fn n_text_ctx(&self) -> c_int {
 93 |         self.ctx.n_text_ctx()
 94 |     }
 95 | 
 96 |     /// Get n_audio_ctx.
 97 |     ///
 98 |     /// # Returns
 99 |     /// c_int
100 |     ///
101 |     /// # C++ equivalent
102 |     /// `int whisper_n_audio_ctx     (struct whisper_context * ctx);`
103 |     #[inline]
104 |     pub fn n_audio_ctx(&self) -> c_int {
105 |         self.ctx.n_audio_ctx()
106 |     }
107 | 
108 |     /// Does this model support multiple languages?
109 |     ///
110 |     /// # C++ equivalent
111 |     /// `int whisper_is_multilingual(struct whisper_context * ctx)`
112 |     #[inline]
113 |     pub fn is_multilingual(&self) -> bool {
114 |         self.ctx.is_multilingual()
115 |     }
116 | 
117 |     /// Get model_n_vocab.
118 |     ///
119 |     /// # Returns
120 |     /// c_int
121 |     ///
122 |     /// # C++ equivalent
123 |     /// `int whisper_model_n_vocab      (struct whisper_context * ctx);`
124 |     #[inline]
125 |     pub fn model_n_vocab(&self) -> c_int {
126 |         self.ctx.model_n_vocab()
127 |     }
128 | 
129 |     /// Get model_n_audio_ctx.
130 |     ///
131 |     /// # Returns
132 |     /// c_int
133 |     ///
134 |     /// # C++ equivalent
135 |     /// `int whisper_model_n_audio_ctx    (struct whisper_context * ctx)`
136 |     #[inline]
137 |     pub fn model_n_audio_ctx(&self) -> c_int {
138 |         self.ctx.model_n_audio_ctx()
139 |     }
140 | 
141 |     /// Get model_n_audio_state.
142 |     ///
143 |     /// # Returns
144 |     /// c_int
145 |     ///
146 |     /// # C++ equivalent
147 |     /// `int whisper_model_n_audio_state(struct whisper_context * ctx);`
148 |     #[inline]
149 |     pub fn model_n_audio_state(&self) -> c_int {
150 |         self.ctx.model_n_audio_state()
151 |     }
152 | 
153 |     /// Get model_n_audio_head.
154 |     ///
155 |     /// # Returns
156 |     /// c_int
157 |     ///
158 |     /// # C++ equivalent
159 |     /// `int whisper_model_n_audio_head (struct whisper_context * ctx);`
160 |     #[inline]
161 |     pub fn model_n_audio_head(&self) -> c_int {
162 |         self.ctx.model_n_audio_head()
163 |     }
164 | 
165 |     /// Get model_n_audio_layer.
166 |     ///
167 |     /// # Returns
168 |     /// c_int
169 |     ///
170 |     /// # C++ equivalent
171 |     /// `int whisper_model_n_audio_layer(struct whisper_context * ctx);`
172 |     #[inline]
173 |     pub fn model_n_audio_layer(&self) -> c_int {
174 |         self.ctx.model_n_audio_layer()
175 |     }
176 | 
177 |     /// Get model_n_text_ctx.
178 |     ///
179 |     /// # Returns
180 |     /// c_int
181 |     ///
182 |     /// # C++ equivalent
183 |     /// `int whisper_model_n_text_ctx     (struct whisper_context * ctx)`
184 |     #[inline]
185 |     pub fn model_n_text_ctx(&self) -> c_int {
186 |         self.ctx.model_n_text_ctx()
187 |     }
188 | 
189 |     /// Get model_n_text_state.
190 |     ///
191 |     /// # Returns
192 |     /// c_int
193 |     ///
194 |     /// # C++ equivalent
195 |     /// `int whisper_model_n_text_state (struct whisper_context * ctx);`
196 |     #[inline]
197 |     pub fn model_n_text_state(&self) -> c_int {
198 |         self.ctx.model_n_text_state()
199 |     }
200 | 
201 |     /// Get model_n_text_head.
202 |     ///
203 |     /// # Returns
204 |     /// c_int
205 |     ///
206 |     /// # C++ equivalent
207 |     /// `int whisper_model_n_text_head  (struct whisper_context * ctx);`
208 |     #[inline]
209 |     pub fn model_n_text_head(&self) -> c_int {
210 |         self.ctx.model_n_text_head()
211 |     }
212 | 
213 |     /// Get model_n_text_layer.
214 |     ///
215 |     /// # Returns
216 |     /// c_int
217 |     ///
218 |     /// # C++ equivalent
219 |     /// `int whisper_model_n_text_layer (struct whisper_context * ctx);`
220 |     #[inline]
221 |     pub fn model_n_text_layer(&self) -> c_int {
222 |         self.ctx.model_n_text_layer()
223 |     }
224 | 
225 |     /// Get model_n_mels.
226 |     ///
227 |     /// # Returns
228 |     /// c_int
229 |     ///
230 |     /// # C++ equivalent
231 |     /// `int whisper_model_n_mels       (struct whisper_context * ctx);`
232 |     #[inline]
233 |     pub fn model_n_mels(&self) -> c_int {
234 |         self.ctx.model_n_mels()
235 |     }
236 | 
237 |     /// Get model_ftype.
238 |     ///
239 |     /// # Returns
240 |     /// c_int
241 |     ///
242 |     /// # C++ equivalent
243 |     /// `int whisper_model_ftype          (struct whisper_context * ctx);`
244 |     #[inline]
245 |     pub fn model_ftype(&self) -> c_int {
246 |         self.ctx.model_ftype()
247 |     }
248 | 
249 |     /// Get model_type.
250 |     ///
251 |     /// # Returns
252 |     /// c_int
253 |     ///
254 |     /// # C++ equivalent
255 |     /// `int whisper_model_type         (struct whisper_context * ctx);`
256 |     #[inline]
257 |     pub fn model_type(&self) -> c_int {
258 |         self.ctx.model_type()
259 |     }
260 | 
261 |     // token functions
262 |     /// Convert a token ID to a string.
263 |     ///
264 |     /// # Arguments
265 |     /// * token_id: ID of the token.
266 |     ///
267 |     /// # Returns
268 |     /// Ok(&str) on success, Err(WhisperError) on failure.
269 |     ///
270 |     /// # C++ equivalent
271 |     /// `const char * whisper_token_to_str(struct whisper_context * ctx, whisper_token token)`
272 |     pub fn token_to_str(&self, token_id: WhisperToken) -> Result<&str, WhisperError> {
273 |         self.ctx.token_to_str(token_id)
274 |     }
275 | 
276 |     /// Convert a token ID to a &CStr.
277 |     ///
278 |     /// # Arguments
279 |     /// * token_id: ID of the token.
280 |     ///
281 |     /// # Returns
282 |     /// Ok(String) on success, Err(WhisperError) on failure.
283 |     ///
284 |     /// # C++ equivalent
285 |     /// `const char * whisper_token_to_str(struct whisper_context * ctx, whisper_token token)`
286 |     pub fn token_to_cstr(&self, token_id: WhisperToken) -> Result<&CStr, WhisperError> {
287 |         self.ctx.token_to_cstr(token_id)
288 |     }
289 | 
290 |     /// Undocumented but exposed function in the C++ API.
291 |     /// `const char * whisper_model_type_readable(struct whisper_context * ctx);`
292 |     ///
293 |     /// # Returns
294 |     /// Ok(String) on success, Err(WhisperError) on failure.
295 |     pub fn model_type_readable(&self) -> Result<String, WhisperError> {
296 |         self.ctx.model_type_readable()
297 |     }
298 | 
299 |     /// Get the ID of the eot token.
300 |     ///
301 |     /// # C++ equivalent
302 |     /// `whisper_token whisper_token_eot (struct whisper_context * ctx)`
303 |     #[inline]
304 |     pub fn token_eot(&self) -> WhisperToken {
305 |         self.ctx.token_eot()
306 |     }
307 | 
308 |     /// Get the ID of the sot token.
309 |     ///
310 |     /// # C++ equivalent
311 |     /// `whisper_token whisper_token_sot (struct whisper_context * ctx)`
312 |     #[inline]
313 |     pub fn token_sot(&self) -> WhisperToken {
314 |         self.ctx.token_sot()
315 |     }
316 | 
317 |     /// Get the ID of the solm token.
318 |     ///
319 |     /// # C++ equivalent
320 |     /// `whisper_token whisper_token_solm(struct whisper_context * ctx)`
321 |     #[inline]
322 |     pub fn token_solm(&self) -> WhisperToken {
323 |         self.ctx.token_solm()
324 |     }
325 | 
326 |     /// Get the ID of the prev token.
327 |     ///
328 |     /// # C++ equivalent
329 |     /// `whisper_token whisper_token_prev(struct whisper_context * ctx)`
330 |     #[inline]
331 |     pub fn token_prev(&self) -> WhisperToken {
332 |         self.ctx.token_prev()
333 |     }
334 | 
335 |     /// Get the ID of the nosp token.
336 |     ///
337 |     /// # C++ equivalent
338 |     /// `whisper_token whisper_token_nosp(struct whisper_context * ctx)`
339 |     #[inline]
340 |     pub fn token_nosp(&self) -> WhisperToken {
341 |         self.ctx.token_nosp()
342 |     }
343 | 
344 |     /// Get the ID of the not token.
345 |     ///
346 |     /// # C++ equivalent
347 |     /// `whisper_token whisper_token_not (struct whisper_context * ctx)`
348 |     #[inline]
349 |     pub fn token_not(&self) -> WhisperToken {
350 |         self.ctx.token_not()
351 |     }
352 | 
353 |     /// Get the ID of the beg token.
354 |     ///
355 |     /// # C++ equivalent
356 |     /// `whisper_token whisper_token_beg (struct whisper_context * ctx)`
357 |     #[inline]
358 |     pub fn token_beg(&self) -> WhisperToken {
359 |         self.ctx.token_beg()
360 |     }
361 | 
362 |     /// Get the ID of a specified language token
363 |     ///
364 |     /// # Arguments
365 |     /// * lang_id: ID of the language
366 |     ///
367 |     /// # C++ equivalent
368 |     /// `whisper_token whisper_token_lang(struct whisper_context * ctx, int lang_id)`
369 |     #[inline]
370 |     pub fn token_lang(&self, lang_id: c_int) -> WhisperToken {
371 |         self.ctx.token_lang(lang_id)
372 |     }
373 | 
374 |     /// Print performance statistics to stderr.
375 |     ///
376 |     /// # C++ equivalent
377 |     /// `void whisper_print_timings(struct whisper_context * ctx)`
378 |     #[inline]
379 |     pub fn print_timings(&self) {
380 |         self.ctx.print_timings()
381 |     }
382 | 
383 |     /// Reset performance statistics.
384 |     ///
385 |     /// # C++ equivalent
386 |     /// `void whisper_reset_timings(struct whisper_context * ctx)`
387 |     #[inline]
388 |     pub fn reset_timings(&self) {
389 |         self.ctx.reset_timings()
390 |     }
391 | 
392 |     // task tokens
393 |     /// Get the ID of the translate task token.
394 |     ///
395 |     /// # C++ equivalent
396 |     /// `whisper_token whisper_token_translate ()`
397 |     pub fn token_translate(&self) -> WhisperToken {
398 |         self.ctx.token_translate()
399 |     }
400 | 
401 |     /// Get the ID of the transcribe task token.
402 |     ///
403 |     /// # C++ equivalent
404 |     /// `whisper_token whisper_token_transcribe()`
405 |     pub fn token_transcribe(&self) -> WhisperToken {
406 |         self.ctx.token_transcribe()
407 |     }
408 | 
409 |     // we don't implement `whisper_init()` here since i have zero clue what `whisper_model_loader` does
410 | 
411 |     /// Create a new state object, ready for use.
412 |     ///
413 |     /// # Returns
414 |     /// Ok(WhisperState) on success, Err(WhisperError) on failure.
415 |     ///
416 |     /// # C++ equivalent
417 |     /// `struct whisper_state * whisper_init_state(struct whisper_context * ctx);`
418 |     pub fn create_state(&self) -> Result<WhisperState, WhisperError> {
419 |         let state = unsafe { whisper_rs_sys::whisper_init_state(self.ctx.ctx) };
420 |         if state.is_null() {
421 |             Err(WhisperError::InitError)
422 |         } else {
423 |             // SAFETY: this is known to be a valid pointer to a `whisper_state` struct
424 |             Ok(WhisperState::new(self.ctx.clone(), state))
425 |         }
426 |     }
427 | }
428 | 


--------------------------------------------------------------------------------
/src/whisper_grammar.rs:
--------------------------------------------------------------------------------
 1 | use whisper_rs_sys::{
 2 |     whisper_gretype_WHISPER_GRETYPE_ALT, whisper_gretype_WHISPER_GRETYPE_CHAR,
 3 |     whisper_gretype_WHISPER_GRETYPE_CHAR_ALT, whisper_gretype_WHISPER_GRETYPE_CHAR_NOT,
 4 |     whisper_gretype_WHISPER_GRETYPE_CHAR_RNG_UPPER, whisper_gretype_WHISPER_GRETYPE_END,
 5 |     whisper_gretype_WHISPER_GRETYPE_RULE_REF,
 6 | };
 7 | 
 8 | #[cfg_attr(any(not(windows), target_env = "gnu"), repr(u32))] // include windows-gnu
 9 | #[cfg_attr(all(windows, not(target_env = "gnu")), repr(i32))] // msvc being *special* again
10 | #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
11 | pub enum WhisperGrammarElementType {
12 |     /// End of rule definition
13 |     End = whisper_gretype_WHISPER_GRETYPE_END,
14 |     /// Start of alternate definition for a rule
15 |     Alternate = whisper_gretype_WHISPER_GRETYPE_ALT,
16 |     /// Non-terminal element: reference to another rule
17 |     RuleReference = whisper_gretype_WHISPER_GRETYPE_RULE_REF,
18 |     /// Terminal element: character (code point)
19 |     Character = whisper_gretype_WHISPER_GRETYPE_CHAR,
20 |     /// Inverse of a character(s)
21 |     NotCharacter = whisper_gretype_WHISPER_GRETYPE_CHAR_NOT,
22 |     /// Modifies a preceding [Self::Character] to be an inclusive range
23 |     CharacterRangeUpper = whisper_gretype_WHISPER_GRETYPE_CHAR_RNG_UPPER,
24 |     /// Modifies a preceding [Self::Character] to add an alternate character to match
25 |     CharacterAlternate = whisper_gretype_WHISPER_GRETYPE_CHAR_ALT,
26 | }
27 | 
28 | impl From<whisper_rs_sys::whisper_gretype> for WhisperGrammarElementType {
29 |     fn from(value: whisper_rs_sys::whisper_gretype) -> Self {
30 |         assert!(
31 |             (0..=6).contains(&value),
32 |             "Invalid WhisperGrammarElementType value: {}",
33 |             value
34 |         );
35 | 
36 |         #[allow(non_upper_case_globals)] // weird place to trigger this
37 |         match value {
38 |             whisper_gretype_WHISPER_GRETYPE_END => WhisperGrammarElementType::End,
39 |             whisper_gretype_WHISPER_GRETYPE_ALT => WhisperGrammarElementType::Alternate,
40 |             whisper_gretype_WHISPER_GRETYPE_RULE_REF => WhisperGrammarElementType::RuleReference,
41 |             whisper_gretype_WHISPER_GRETYPE_CHAR => WhisperGrammarElementType::Character,
42 |             whisper_gretype_WHISPER_GRETYPE_CHAR_NOT => WhisperGrammarElementType::NotCharacter,
43 |             whisper_gretype_WHISPER_GRETYPE_CHAR_RNG_UPPER => {
44 |                 WhisperGrammarElementType::CharacterRangeUpper
45 |             }
46 |             whisper_gretype_WHISPER_GRETYPE_CHAR_ALT => {
47 |                 WhisperGrammarElementType::CharacterAlternate
48 |             }
49 |             _ => unreachable!(),
50 |         }
51 |     }
52 | }
53 | 
54 | impl From<WhisperGrammarElementType> for whisper_rs_sys::whisper_gretype {
55 |     fn from(value: WhisperGrammarElementType) -> Self {
56 |         value as Self
57 |     }
58 | }
59 | 
60 | #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
61 | pub struct WhisperGrammarElement {
62 |     pub element_type: WhisperGrammarElementType,
63 |     pub value: u32,
64 | }
65 | 
66 | impl WhisperGrammarElement {
67 |     pub fn new(element_type: WhisperGrammarElementType, value: u32) -> Self {
68 |         Self {
69 |             element_type,
70 |             value,
71 |         }
72 |     }
73 | 
74 |     pub fn to_c_type(self) -> whisper_rs_sys::whisper_grammar_element {
75 |         whisper_rs_sys::whisper_grammar_element {
76 |             type_: self.element_type.into(),
77 |             value: self.value,
78 |         }
79 |     }
80 | }
81 | 


--------------------------------------------------------------------------------
/src/whisper_logging_hook.rs:
--------------------------------------------------------------------------------
 1 | use crate::common_logging::{
 2 |     generic_debug, generic_error, generic_info, generic_trace, generic_warn, GGMLLogLevel,
 3 | };
 4 | use core::ffi::{c_char, c_void};
 5 | use std::borrow::Cow;
 6 | use std::ffi::CStr;
 7 | use std::sync::Once;
 8 | use whisper_rs_sys::ggml_log_level;
 9 | 
10 | static WHISPER_LOG_TRAMPOLINE_INSTALL: Once = Once::new();
11 | pub(crate) fn install_whisper_logging_hook() {
12 |     WHISPER_LOG_TRAMPOLINE_INSTALL.call_once(|| unsafe {
13 |         whisper_rs_sys::whisper_log_set(Some(whisper_logging_trampoline), std::ptr::null_mut())
14 |     });
15 | }
16 | 
17 | unsafe extern "C" fn whisper_logging_trampoline(
18 |     level: ggml_log_level,
19 |     text: *const c_char,
20 |     _: *mut c_void, // user_data
21 | ) {
22 |     if text.is_null() {
23 |         generic_error!("whisper_logging_trampoline: text is nullptr");
24 |     }
25 |     let level = GGMLLogLevel::from(level);
26 | 
27 |     // SAFETY: we must trust whisper.cpp that it will not pass us a string that does not satisfy
28 |     // from_ptr's requirements.
29 |     let log_str = unsafe { CStr::from_ptr(text) }.to_string_lossy();
30 | 
31 |     whisper_logging_trampoline_safe(level, log_str)
32 | }
33 | 
34 | // this code essentially compiles down to a noop if neither feature is enabled
35 | #[cfg_attr(
36 |     not(any(feature = "log_backend", feature = "tracing_backend")),
37 |     allow(unused_variables)
38 | )]
39 | fn whisper_logging_trampoline_safe(level: GGMLLogLevel, text: Cow<str>) {
40 |     match level {
41 |         GGMLLogLevel::None => {
42 |             // no clue what to do here, trace it?
43 |             generic_trace!("{}", text.trim());
44 |         }
45 |         GGMLLogLevel::Info => {
46 |             generic_info!("{}", text.trim());
47 |         }
48 |         GGMLLogLevel::Warn => {
49 |             generic_warn!("{}", text.trim());
50 |         }
51 |         GGMLLogLevel::Error => {
52 |             generic_error!("{}", text.trim());
53 |         }
54 |         GGMLLogLevel::Debug => {
55 |             generic_debug!("{}", text.trim());
56 |         }
57 |         GGMLLogLevel::Cont => {
58 |             // this means continue previous log
59 |             // storing state to do this is a massive pain so it's just a lot easier to not
60 |             // plus as far as i can tell it's not actually *used* anywhere
61 |             // whisper splits at 1024 chars and doesn't actually change the kind
62 |             // so technically this is unused
63 |             generic_trace!("{}", text.trim());
64 |         }
65 |         GGMLLogLevel::Unknown(level) => {
66 |             generic_warn!(
67 |                 "whisper_logging_trampoline: unknown log level {}: message: {}",
68 |                 level,
69 |                 text.trim()
70 |             );
71 |         }
72 |     }
73 | }
74 | 


--------------------------------------------------------------------------------
/src/whisper_params.rs:
--------------------------------------------------------------------------------
  1 | use crate::whisper_grammar::WhisperGrammarElement;
  2 | use std::ffi::{c_char, c_float, c_int, CString};
  3 | use std::marker::PhantomData;
  4 | use std::sync::Arc;
  5 | use whisper_rs_sys::whisper_token;
  6 | 
  7 | #[derive(Debug, Clone)]
  8 | pub enum SamplingStrategy {
  9 |     Greedy {
 10 |         best_of: c_int,
 11 |     },
 12 |     BeamSearch {
 13 |         beam_size: c_int,
 14 |         // not implemented in whisper.cpp as of this writing (v1.2.0)
 15 |         patience: c_float,
 16 |     },
 17 | }
 18 | 
 19 | impl Default for SamplingStrategy {
 20 |     fn default() -> Self {
 21 |         Self::Greedy { best_of: 1 }
 22 |     }
 23 | }
 24 | 
 25 | #[derive(Debug, Clone)]
 26 | pub struct SegmentCallbackData {
 27 |     pub segment: i32,
 28 |     pub start_timestamp: i64,
 29 |     pub end_timestamp: i64,
 30 |     pub text: String,
 31 | }
 32 | 
 33 | type SegmentCallbackFn = Box<dyn FnMut(SegmentCallbackData)>;
 34 | 
 35 | #[derive(Clone)]
 36 | pub struct FullParams<'a, 'b> {
 37 |     pub(crate) fp: whisper_rs_sys::whisper_full_params,
 38 |     phantom_lang: PhantomData<&'a str>,
 39 |     phantom_tokens: PhantomData<&'b [c_int]>,
 40 |     grammar: Option<Vec<whisper_rs_sys::whisper_grammar_element>>,
 41 |     progess_callback_safe: Option<Arc<Box<dyn FnMut(i32)>>>,
 42 |     abort_callback_safe: Option<Arc<Box<dyn FnMut() -> bool>>>,
 43 |     segment_calllback_safe: Option<Arc<SegmentCallbackFn>>,
 44 | }
 45 | 
 46 | impl<'a, 'b> FullParams<'a, 'b> {
 47 |     /// Create a new set of parameters for the decoder.
 48 |     pub fn new(sampling_strategy: SamplingStrategy) -> FullParams<'a, 'b> {
 49 |         let mut fp = unsafe {
 50 |             whisper_rs_sys::whisper_full_default_params(match sampling_strategy {
 51 |                 SamplingStrategy::Greedy { .. } => {
 52 |                     whisper_rs_sys::whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY
 53 |                 }
 54 |                 SamplingStrategy::BeamSearch { .. } => {
 55 |                     whisper_rs_sys::whisper_sampling_strategy_WHISPER_SAMPLING_BEAM_SEARCH
 56 |                 }
 57 |             } as _)
 58 |         };
 59 | 
 60 |         match sampling_strategy {
 61 |             SamplingStrategy::Greedy { best_of } => {
 62 |                 fp.greedy.best_of = best_of;
 63 |             }
 64 |             SamplingStrategy::BeamSearch {
 65 |                 beam_size,
 66 |                 patience,
 67 |             } => {
 68 |                 fp.beam_search.beam_size = beam_size;
 69 |                 fp.beam_search.patience = patience;
 70 |             }
 71 |         }
 72 | 
 73 |         Self {
 74 |             fp,
 75 |             phantom_lang: PhantomData,
 76 |             phantom_tokens: PhantomData,
 77 |             grammar: None,
 78 |             progess_callback_safe: None,
 79 |             abort_callback_safe: None,
 80 |             segment_calllback_safe: None,
 81 |         }
 82 |     }
 83 | 
 84 |     /// Set the number of threads to use for decoding.
 85 |     ///
 86 |     /// Defaults to min(4, std::thread::hardware_concurrency()).
 87 |     pub fn set_n_threads(&mut self, n_threads: c_int) {
 88 |         self.fp.n_threads = n_threads;
 89 |     }
 90 | 
 91 |     /// Max tokens to use from past text as prompt for the decoder
 92 |     ///
 93 |     /// Defaults to 16384.
 94 |     pub fn set_n_max_text_ctx(&mut self, n_max_text_ctx: c_int) {
 95 |         self.fp.n_max_text_ctx = n_max_text_ctx;
 96 |     }
 97 | 
 98 |     /// Set the start offset in milliseconds to use for decoding.
 99 |     ///
100 |     /// Defaults to 0.
101 |     pub fn set_offset_ms(&mut self, offset_ms: c_int) {
102 |         self.fp.offset_ms = offset_ms;
103 |     }
104 | 
105 |     /// Set the audio duration to process in milliseconds.
106 |     ///
107 |     /// Defaults to 0.
108 |     pub fn set_duration_ms(&mut self, duration_ms: c_int) {
109 |         self.fp.duration_ms = duration_ms;
110 |     }
111 | 
112 |     /// Set whether to translate the output to the language specified by `language`.
113 |     ///
114 |     /// Defaults to false.
115 |     pub fn set_translate(&mut self, translate: bool) {
116 |         self.fp.translate = translate;
117 |     }
118 | 
119 |     /// Do not use past transcription (if any) as initial prompt for the decoder.
120 |     ///
121 |     /// Defaults to false.
122 |     pub fn set_no_context(&mut self, no_context: bool) {
123 |         self.fp.no_context = no_context;
124 |     }
125 | 
126 |     /// Do not generate timestamps.
127 |     ///
128 |     /// Defaults to false.
129 |     pub fn set_no_timestamps(&mut self, no_timestamps: bool) {
130 |         self.fp.no_timestamps = no_timestamps;
131 |     }
132 | 
133 |     /// Force single segment output. This may be useful for streaming.
134 |     ///
135 |     /// Defaults to false.
136 |     pub fn set_single_segment(&mut self, single_segment: bool) {
137 |         self.fp.single_segment = single_segment;
138 |     }
139 | 
140 |     /// Print special tokens (e.g. `<SOT>`, `<EOT>`, `<BEG>`, etc.)
141 |     ///
142 |     /// Defaults to false.
143 |     pub fn set_print_special(&mut self, print_special: bool) {
144 |         self.fp.print_special = print_special;
145 |     }
146 | 
147 |     /// Set whether to print progress.
148 |     ///
149 |     /// Defaults to true.
150 |     pub fn set_print_progress(&mut self, print_progress: bool) {
151 |         self.fp.print_progress = print_progress;
152 |     }
153 | 
154 |     /// Print results from within whisper.cpp.
155 |     /// Try to use the callback methods instead: [set_new_segment_callback](FullParams::set_new_segment_callback),
156 |     /// [set_new_segment_callback_user_data](FullParams::set_new_segment_callback_user_data).
157 |     ///
158 |     /// Defaults to false.
159 |     pub fn set_print_realtime(&mut self, print_realtime: bool) {
160 |         self.fp.print_realtime = print_realtime;
161 |     }
162 | 
163 |     /// Print timestamps for each text segment when printing realtime. Only has an effect if
164 |     /// [set_print_realtime](FullParams::set_print_realtime) is set to true.
165 |     ///
166 |     /// Defaults to true.
167 |     pub fn set_print_timestamps(&mut self, print_timestamps: bool) {
168 |         self.fp.print_timestamps = print_timestamps;
169 |     }
170 | 
171 |     /// # EXPERIMENTAL
172 |     ///
173 |     /// Enable token-level timestamps.
174 |     ///
175 |     /// Defaults to false.
176 |     pub fn set_token_timestamps(&mut self, token_timestamps: bool) {
177 |         self.fp.token_timestamps = token_timestamps;
178 |     }
179 | 
180 |     /// # EXPERIMENTAL
181 |     ///
182 |     /// Set timestamp token probability threshold.
183 |     ///
184 |     /// Defaults to 0.01.
185 |     pub fn set_thold_pt(&mut self, thold_pt: f32) {
186 |         self.fp.thold_pt = thold_pt;
187 |     }
188 | 
189 |     /// # EXPERIMENTAL
190 |     ///
191 |     /// Set timestamp token sum probability threshold.
192 |     ///
193 |     /// Defaults to 0.01.
194 |     pub fn set_thold_ptsum(&mut self, thold_ptsum: f32) {
195 |         self.fp.thold_ptsum = thold_ptsum;
196 |     }
197 | 
198 |     /// # EXPERIMENTAL
199 |     ///
200 |     /// Set maximum segment length in characters.
201 |     ///
202 |     /// Defaults to 0.
203 |     pub fn set_max_len(&mut self, max_len: c_int) {
204 |         self.fp.max_len = max_len;
205 |     }
206 | 
207 |     /// # EXPERIMENTAL
208 |     ///
209 |     /// Should the timestamps be split on words instead of characters?
210 |     ///
211 |     /// Defaults to false.
212 |     pub fn set_split_on_word(&mut self, split_on_word: bool) {
213 |         self.fp.split_on_word = split_on_word;
214 |     }
215 | 
216 |     /// # EXPERIMENTAL
217 |     ///
218 |     /// Set maximum tokens per segment. 0 means no limit.
219 |     ///
220 |     /// Defaults to 0.
221 |     pub fn set_max_tokens(&mut self, max_tokens: c_int) {
222 |         self.fp.max_tokens = max_tokens;
223 |     }
224 | 
225 |     /// # EXPERIMENTAL
226 |     ///
227 |     /// Enables debug mode, such as dumping the log mel spectrogram.
228 |     ///
229 |     /// Defaults to false.
230 |     pub fn set_debug_mode(&mut self, debug: bool) {
231 |         self.fp.debug_mode = debug;
232 |     }
233 | 
234 |     /// # EXPERIMENTAL
235 |     ///
236 |     /// Overwrite the audio context size. 0 = default.
237 |     ///
238 |     /// Defaults to 0.
239 |     pub fn set_audio_ctx(&mut self, audio_ctx: c_int) {
240 |         self.fp.audio_ctx = audio_ctx;
241 |     }
242 | 
243 |     /// # EXPERIMENTAL
244 |     ///
245 |     /// Enable tinydiarize support.
246 |     /// Experimental speaker turn detection.
247 |     ///
248 |     /// Defaults to false.
249 |     pub fn set_tdrz_enable(&mut self, tdrz_enable: bool) {
250 |         self.fp.tdrz_enable = tdrz_enable;
251 |     }
252 | 
253 |     /// Set tokens to provide the model as initial input.
254 |     ///
255 |     /// These tokens are prepended to any existing text content from a previous call.
256 |     ///
257 |     /// Calling this more than once will overwrite the previous tokens.
258 |     ///
259 |     /// Defaults to an empty vector.
260 |     pub fn set_tokens(&mut self, tokens: &'b [c_int]) {
261 |         // turn into ptr and len
262 |         let tokens_ptr: *const whisper_token = tokens.as_ptr();
263 |         let tokens_len: c_int = tokens.len() as c_int;
264 | 
265 |         // set the tokens
266 |         self.fp.prompt_tokens = tokens_ptr;
267 |         self.fp.prompt_n_tokens = tokens_len;
268 |     }
269 | 
270 |     /// Set the target language.
271 |     ///
272 |     /// For auto-detection, set this to either "auto" or None.
273 |     ///
274 |     /// Defaults to "en".
275 |     pub fn set_language(&mut self, language: Option<&'a str>) {
276 |         self.fp.language = match language {
277 |             Some(language) => CString::new(language)
278 |                 .expect("Language contains null byte")
279 |                 .into_raw() as *const _,
280 |             None => std::ptr::null(),
281 |         };
282 |     }
283 | 
284 |     /// Set `detect_language`.
285 |     ///
286 |     /// Has the same effect as setting the language to "auto" or None.
287 |     ///
288 |     /// Defaults to false.
289 |     pub fn set_detect_language(&mut self, detect_language: bool) {
290 |         self.fp.detect_language = detect_language;
291 |     }
292 | 
293 |     /// Set suppress_blank.
294 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/decoding.py#L89>
295 |     /// for more information.
296 |     ///
297 |     /// Defaults to true.
298 |     pub fn set_suppress_blank(&mut self, suppress_blank: bool) {
299 |         self.fp.suppress_blank = suppress_blank;
300 |     }
301 | 
302 |     /// Set suppress_non_speech_tokens.
303 |     /// See <https://github.com/openai/whisper/blob/7858aa9c08d98f75575035ecd6481f462d66ca27/whisper/tokenizer.py#L224-L253>
304 |     /// for more information.
305 |     ///
306 |     /// Defaults to false.
307 |     pub fn set_suppress_nst(&mut self, suppress_nst: bool) {
308 |         self.fp.suppress_nst = suppress_nst;
309 |     }
310 | 
311 |     /// Set initial decoding temperature.
312 |     /// See <https://ai.stackexchange.com/a/32478> for more information.
313 |     ///
314 |     /// Defaults to 0.0.
315 |     pub fn set_temperature(&mut self, temperature: f32) {
316 |         self.fp.temperature = temperature;
317 |     }
318 | 
319 |     /// Set max_initial_ts.
320 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/decoding.py#L97>
321 |     /// for more information.
322 |     ///
323 |     /// Defaults to 1.0.
324 |     pub fn set_max_initial_ts(&mut self, max_initial_ts: f32) {
325 |         self.fp.max_initial_ts = max_initial_ts;
326 |     }
327 | 
328 |     /// Set length_penalty.
329 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/transcribe.py#L267>
330 |     /// for more information.
331 |     ///
332 |     /// Defaults to -1.0.
333 |     pub fn set_length_penalty(&mut self, length_penalty: f32) {
334 |         self.fp.length_penalty = length_penalty;
335 |     }
336 | 
337 |     /// Set temperature_inc.
338 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/transcribe.py#L274-L278>
339 |     /// for more information.
340 |     ///
341 |     /// Defaults to 0.2.
342 |     pub fn set_temperature_inc(&mut self, temperature_inc: f32) {
343 |         self.fp.temperature_inc = temperature_inc;
344 |     }
345 | 
346 |     /// Set entropy_thold. Similar to OpenAI's compression_ratio_threshold.
347 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/transcribe.py#L274-L278> for more information.
348 |     ///
349 |     /// Defaults to 2.4.
350 |     pub fn set_entropy_thold(&mut self, entropy_thold: f32) {
351 |         self.fp.entropy_thold = entropy_thold;
352 |     }
353 | 
354 |     /// Set logprob_thold.
355 |     /// See <https://github.com/openai/whisper/blob/f82bc59f5ea234d4b97fb2860842ed38519f7e65/whisper/transcribe.py#L274-L278>
356 |     /// for more information.
357 |     ///
358 |     /// Defaults to -1.0.
359 |     pub fn set_logprob_thold(&mut self, logprob_thold: f32) {
360 |         self.fp.logprob_thold = logprob_thold;
361 |     }
362 | 
363 |     /// Set no_speech_thold. Currently (as of v1.3.0) not implemented.
364 |     ///
365 |     /// Defaults to 0.6.
366 |     pub fn set_no_speech_thold(&mut self, no_speech_thold: f32) {
367 |         self.fp.no_speech_thold = no_speech_thold;
368 |     }
369 | 
370 |     /// Set the callback for new segments.
371 |     ///
372 |     /// Note that this callback has not been Rustified yet (and likely never will be, unless someone else feels the need to do so).
373 |     /// It is still a C callback.
374 |     ///
375 |     /// # Safety
376 |     /// Do not use this function unless you know what you are doing.
377 |     /// * Be careful not to mutate the state of the whisper_context pointer returned in the callback.
378 |     ///   This could cause undefined behavior, as this violates the thread-safety guarantees of the underlying C library.
379 |     /// **Warning** Can't be used with DTW. DTW will produce inconsistent callback invocation
380 |     ///
381 |     /// Defaults to None.
382 |     pub unsafe fn set_new_segment_callback(
383 |         &mut self,
384 |         new_segment_callback: crate::WhisperNewSegmentCallback,
385 |     ) {
386 |         self.fp.new_segment_callback = new_segment_callback;
387 |     }
388 | 
389 |     /// Set the user data to be passed to the new segment callback.
390 |     ///
391 |     /// # Safety
392 |     /// See the safety notes for `set_new_segment_callback`.
393 |     /// **Warning** Can't be used with DTW. DTW will produce inconsistent callback invocation
394 |     ///
395 |     /// Defaults to None.
396 |     pub unsafe fn set_new_segment_callback_user_data(&mut self, user_data: *mut std::ffi::c_void) {
397 |         self.fp.new_segment_callback_user_data = user_data;
398 |     }
399 | 
400 |     /// Set the callback for segment updates.
401 |     ///
402 |     /// Provides a limited segment_callback to ensure safety.
403 |     /// See `set_new_segment_callback` if you need to use `whisper_context` and `whisper_state`
404 |     /// **Warning** Can't be used with DTW. DTW will produce inconsistent callback invocation
405 |     ///
406 |     /// Defaults to None.
407 |     pub fn set_segment_callback_safe<O, F>(&mut self, closure: O)
408 |     where
409 |         F: FnMut(SegmentCallbackData) + 'static,
410 |         O: Into<Option<F>>,
411 |     {
412 |         use std::ffi::{c_void, CStr};
413 |         use whisper_rs_sys::{whisper_context, whisper_state};
414 | 
415 |         extern "C" fn trampoline<F>(
416 |             _: *mut whisper_context,
417 |             state: *mut whisper_state,
418 |             n_new: i32,
419 |             user_data: *mut c_void,
420 |         ) where
421 |             F: FnMut(SegmentCallbackData) + 'static,
422 |         {
423 |             unsafe {
424 |                 let user_data = &mut *(user_data as *mut SegmentCallbackFn);
425 |                 let n_segments = whisper_rs_sys::whisper_full_n_segments_from_state(state);
426 |                 let s0 = n_segments - n_new;
427 |                 //let user_data = user_data as *mut Box<dyn FnMut(SegmentCallbackData)>;
428 | 
429 |                 for i in s0..n_segments {
430 |                     let text = whisper_rs_sys::whisper_full_get_segment_text_from_state(state, i);
431 |                     let text = CStr::from_ptr(text);
432 | 
433 |                     let t0 = whisper_rs_sys::whisper_full_get_segment_t0_from_state(state, i);
434 |                     let t1 = whisper_rs_sys::whisper_full_get_segment_t1_from_state(state, i);
435 | 
436 |                     match text.to_str() {
437 |                         Ok(n) => user_data(SegmentCallbackData {
438 |                             segment: i,
439 |                             start_timestamp: t0,
440 |                             end_timestamp: t1,
441 |                             text: n.to_string(),
442 |                         }),
443 |                         Err(_) => {}
444 |                     }
445 |                 }
446 |             }
447 |         }
448 | 
449 |         match closure.into() {
450 |             Some(closure) => {
451 |                 // Stable address
452 |                 let closure = Box::new(closure) as SegmentCallbackFn;
453 |                 // Thin pointer
454 |                 let closure = Box::new(closure);
455 |                 // Raw pointer
456 |                 let closure = Box::into_raw(closure);
457 | 
458 |                 self.fp.new_segment_callback_user_data = closure as *mut c_void;
459 |                 self.fp.new_segment_callback = Some(trampoline::<SegmentCallbackFn>);
460 |                 self.segment_calllback_safe = None;
461 |             }
462 |             None => {
463 |                 self.segment_calllback_safe = None;
464 |                 self.fp.new_segment_callback = None;
465 |                 self.fp.new_segment_callback_user_data = std::ptr::null_mut::<c_void>();
466 |             }
467 |         }
468 |     }
469 | 
470 |     /// Set the callback for segment updates.
471 |     ///
472 |     /// Provides a limited segment_callback to ensure safety with lossy handling of bad UTF-8 characters.
473 |     /// See `set_new_segment_callback` if you need to use `whisper_context` and `whisper_state`.
474 |     /// **Warning** Can't be used with DTW. DTW will produce inconsistent callback invocation
475 |     ///
476 |     /// Defaults to None.
477 |     pub fn set_segment_callback_safe_lossy<O, F>(&mut self, closure: O)
478 |     where
479 |         F: FnMut(SegmentCallbackData) + 'static,
480 |         O: Into<Option<F>>,
481 |     {
482 |         use std::ffi::{c_void, CStr};
483 |         use whisper_rs_sys::{whisper_context, whisper_state};
484 | 
485 |         extern "C" fn trampoline<F>(
486 |             _: *mut whisper_context,
487 |             state: *mut whisper_state,
488 |             n_new: i32,
489 |             user_data: *mut c_void,
490 |         ) where
491 |             F: FnMut(SegmentCallbackData) + 'static,
492 |         {
493 |             unsafe {
494 |                 let user_data = &mut *(user_data as *mut SegmentCallbackFn);
495 |                 let n_segments = whisper_rs_sys::whisper_full_n_segments_from_state(state);
496 |                 let s0 = n_segments - n_new;
497 |                 //let user_data = user_data as *mut Box<dyn FnMut(SegmentCallbackData)>;
498 | 
499 |                 for i in s0..n_segments {
500 |                     let text = whisper_rs_sys::whisper_full_get_segment_text_from_state(state, i);
501 |                     let text = CStr::from_ptr(text);
502 | 
503 |                     let t0 = whisper_rs_sys::whisper_full_get_segment_t0_from_state(state, i);
504 |                     let t1 = whisper_rs_sys::whisper_full_get_segment_t1_from_state(state, i);
505 |                     user_data(SegmentCallbackData {
506 |                         segment: i,
507 |                         start_timestamp: t0,
508 |                         end_timestamp: t1,
509 |                         text: text.to_string_lossy().to_string(),
510 |                     });
511 |                 }
512 |             }
513 |         }
514 | 
515 |         match closure.into() {
516 |             Some(closure) => {
517 |                 // Stable address
518 |                 let closure = Box::new(closure) as SegmentCallbackFn;
519 |                 // Thin pointer
520 |                 let closure = Box::new(closure);
521 |                 // Raw pointer
522 |                 let closure = Box::into_raw(closure);
523 | 
524 |                 self.fp.new_segment_callback_user_data = closure as *mut c_void;
525 |                 self.fp.new_segment_callback = Some(trampoline::<SegmentCallbackFn>);
526 |                 self.segment_calllback_safe = None;
527 |             }
528 |             None => {
529 |                 self.segment_calllback_safe = None;
530 |                 self.fp.new_segment_callback = None;
531 |                 self.fp.new_segment_callback_user_data = std::ptr::null_mut::<c_void>();
532 |             }
533 |         }
534 |     }
535 | 
536 |     /// Set the callback for progress updates.
537 |     ///
538 |     /// Note that is still a C callback.
539 |     /// See `set_progress_callback_safe` for a limited yet safe version.
540 |     ///
541 |     /// # Safety
542 |     /// Do not use this function unless you know what you are doing.
543 |     /// * Be careful not to mutate the state of the whisper_context pointer returned in the callback.
544 |     ///   This could cause undefined behavior, as this violates the thread-safety guarantees of the underlying C library.
545 |     ///
546 |     /// Defaults to None.
547 |     pub unsafe fn set_progress_callback(
548 |         &mut self,
549 |         progress_callback: crate::WhisperProgressCallback,
550 |     ) {
551 |         self.fp.progress_callback = progress_callback;
552 |     }
553 | 
554 |     /// Set the callback for progress updates, potentially using a closure.
555 |     ///
556 |     /// Note that, in order to ensure safety, the callback only accepts the progress in percent.
557 |     /// See `set_progress_callback` if you need to use `whisper_context` and `whisper_state`
558 |     /// (or extend this one to support their use).
559 |     ///
560 |     /// Defaults to None.
561 |     pub fn set_progress_callback_safe<O, F>(&mut self, closure: O)
562 |     where
563 |         F: FnMut(i32) + 'static,
564 |         O: Into<Option<F>>,
565 |     {
566 |         use std::ffi::c_void;
567 |         use whisper_rs_sys::{whisper_context, whisper_state};
568 | 
569 |         unsafe extern "C" fn trampoline<F>(
570 |             _: *mut whisper_context,
571 |             _: *mut whisper_state,
572 |             progress: c_int,
573 |             user_data: *mut c_void,
574 |         ) where
575 |             F: FnMut(i32),
576 |         {
577 |             let user_data = &mut *(user_data as *mut F);
578 |             user_data(progress);
579 |         }
580 | 
581 |         match closure.into() {
582 |             Some(mut closure) => {
583 |                 self.fp.progress_callback = Some(trampoline::<F>);
584 |                 self.fp.progress_callback_user_data = &mut closure as *mut F as *mut c_void;
585 |                 // store the closure internally to make sure that the pointer above remains valid
586 |                 self.progess_callback_safe = Some(Arc::new(Box::new(closure)));
587 |             }
588 |             None => {
589 |                 self.fp.progress_callback = None;
590 |                 self.fp.progress_callback_user_data = std::ptr::null_mut::<c_void>();
591 |                 self.progess_callback_safe = None;
592 |             }
593 |         }
594 |     }
595 | 
596 |     /// Set the callback for abort conditions, potentially using a closure.
597 |     ///
598 |     /// Note that, for safety, the callback only accepts a function that returns a boolean
599 |     /// indicating whether to abort or not.
600 |     ///
601 |     /// See `set_progress_callback` if you need to use `whisper_context` and `whisper_state`,
602 |     /// or extend this one to support their use.
603 |     ///
604 |     /// Defaults to None.
605 |     pub fn set_abort_callback_safe<O, F>(&mut self, closure: O)
606 |     where
607 |         F: FnMut() -> bool + 'static,
608 |         O: Into<Option<F>>,
609 |     {
610 |         use std::ffi::c_void;
611 | 
612 |         unsafe extern "C" fn trampoline<F>(user_data: *mut c_void) -> bool
613 |         where
614 |             F: FnMut() -> bool,
615 |         {
616 |             let user_data = &mut *(user_data as *mut F);
617 |             user_data()
618 |         }
619 | 
620 |         match closure.into() {
621 |             Some(closure) => {
622 |                 // Stable address
623 |                 let closure = Box::new(closure) as Box<dyn FnMut() -> bool>;
624 |                 // Thin pointer
625 |                 let closure = Box::new(closure);
626 |                 // Raw pointer
627 |                 let closure = Box::into_raw(closure);
628 | 
629 |                 self.fp.abort_callback = Some(trampoline::<F>);
630 |                 self.fp.abort_callback_user_data = closure as *mut c_void;
631 |                 self.abort_callback_safe = None;
632 |             }
633 |             None => {
634 |                 self.fp.abort_callback = None;
635 |                 self.fp.abort_callback_user_data = std::ptr::null_mut::<c_void>();
636 |                 self.abort_callback_safe = None;
637 |             }
638 |         }
639 |     }
640 | 
641 |     /// Set the user data to be passed to the progress callback.
642 |     ///
643 |     /// # Safety
644 |     /// See the safety notes for `set_progress_callback`.
645 |     ///
646 |     /// Defaults to None.
647 |     pub unsafe fn set_progress_callback_user_data(&mut self, user_data: *mut std::ffi::c_void) {
648 |         self.fp.progress_callback_user_data = user_data;
649 |     }
650 | 
651 |     /// Set the callback that is called each time before the encoder begins.
652 |     ///
653 |     /// Note that this callback has not been Rustified yet (and likely never will be, unless someone else feels the need to do so).
654 |     /// It is still a C callback.
655 |     ///
656 |     /// # Safety
657 |     /// Do not use this function unless you know what you are doing.
658 |     /// * Be careful not to mutate the state of the whisper_context pointer returned in the callback.
659 |     ///   This could cause undefined behavior, as this violates the thread-safety guarantees of the underlying C library.
660 |     ///
661 |     /// Defaults to None.
662 |     pub unsafe fn set_start_encoder_callback(
663 |         &mut self,
664 |         start_encoder_callback: crate::WhisperStartEncoderCallback,
665 |     ) {
666 |         self.fp.encoder_begin_callback = start_encoder_callback;
667 |     }
668 | 
669 |     /// Set the user data to be passed to the start encoder callback.
670 |     ///
671 |     /// # Safety
672 |     /// See the safety notes for `set_start_encoder_callback`.
673 |     ///
674 |     /// Defaults to None.
675 |     pub unsafe fn set_start_encoder_callback_user_data(
676 |         &mut self,
677 |         user_data: *mut std::ffi::c_void,
678 |     ) {
679 |         self.fp.encoder_begin_callback_user_data = user_data;
680 |     }
681 | 
682 |     /// Set the callback that is called by each decoder to filter obtained logits.
683 |     ///
684 |     /// Note that this callback has not been Rustified yet (and likely never will be, unless someone else feels the need to do so).
685 |     /// It is still a C callback.
686 |     ///
687 |     /// # Safety
688 |     /// Do not use this function unless you know what you are doing.
689 |     /// * Be careful not to mutate the state of the whisper_context pointer returned in the callback.
690 |     ///   This could cause undefined behavior, as this violates the thread-safety guarantees of the underlying C library.
691 |     ///
692 |     /// Defaults to None.
693 |     pub unsafe fn set_filter_logits_callback(
694 |         &mut self,
695 |         logits_filter_callback: crate::WhisperLogitsFilterCallback,
696 |     ) {
697 |         self.fp.logits_filter_callback = logits_filter_callback;
698 |     }
699 | 
700 |     /// Set the user data to be passed to the logits filter callback.
701 |     ///
702 |     /// # Safety
703 |     /// See the safety notes for `set_filter_logits_callback`.
704 |     ///
705 |     /// Defaults to None.
706 |     pub unsafe fn set_filter_logits_callback_user_data(
707 |         &mut self,
708 |         user_data: *mut std::ffi::c_void,
709 |     ) {
710 |         self.fp.logits_filter_callback_user_data = user_data;
711 |     }
712 | 
713 |     /// Set the callback that is called each time before ggml computation starts.
714 |     ///
715 |     /// Note that this callback has not been Rustified yet (and likely never will be, unless someone else feels the need to do so).
716 |     /// It is still a C callback.
717 |     ///
718 |     /// # Safety
719 |     /// Do not use this function unless you know what you are doing.
720 |     /// * Be careful not to mutate the state of the whisper_context pointer returned in the callback.
721 |     ///   This could cause undefined behavior, as this violates the thread-safety guarantees of the underlying C library.
722 |     ///
723 |     /// Defaults to None.
724 |     pub unsafe fn set_abort_callback(&mut self, abort_callback: crate::WhisperAbortCallback) {
725 |         self.fp.abort_callback = abort_callback;
726 |     }
727 | 
728 |     /// Set the user data to be passed to the abort callback.
729 |     ///
730 |     /// # Safety
731 |     /// See the safety notes for `set_abort_callback`.
732 |     ///
733 |     /// Defaults to None.
734 |     pub unsafe fn set_abort_callback_user_data(&mut self, user_data: *mut std::ffi::c_void) {
735 |         self.fp.abort_callback_user_data = user_data;
736 |     }
737 | 
738 |     /// Enable an array of grammar elements to be passed to the whisper model.
739 |     ///
740 |     /// Defaults to an empty vector.
741 |     pub fn set_grammar(&mut self, grammar: Option<&[WhisperGrammarElement]>) {
742 |         if let Some(grammar) = grammar {
743 |             // convert to c types
744 |             let inner = grammar.iter().map(|e| e.to_c_type()).collect::<Vec<_>>();
745 |             // turn into ptr and len
746 |             let grammar_ptr = inner.as_ptr() as *mut _;
747 |             let grammar_len = inner.len();
748 | 
749 |             self.grammar = Some(inner);
750 | 
751 |             // set the grammar
752 |             self.fp.grammar_rules = grammar_ptr;
753 |             self.fp.n_grammar_rules = grammar_len;
754 |         } else {
755 |             self.grammar = None;
756 |             self.fp.grammar_rules = std::ptr::null_mut();
757 |             self.fp.n_grammar_rules = 0;
758 |             self.fp.i_start_rule = 0;
759 |         }
760 |     }
761 | 
762 |     /// Set the start grammar rule. Does nothing if no grammar is set.
763 |     ///
764 |     /// Defaults to 0.
765 |     pub fn set_start_rule(&mut self, start_rule: usize) {
766 |         if self.grammar.is_some() {
767 |             self.fp.i_start_rule = start_rule;
768 |         }
769 |     }
770 | 
771 |     /// Set grammar penalty.
772 |     ///
773 |     /// Defaults to 100.0.
774 |     pub fn set_grammar_penalty(&mut self, grammar_penalty: f32) {
775 |         self.fp.grammar_penalty = grammar_penalty;
776 |     }
777 | 
778 |     /// Set the initial prompt for the model.
779 |     ///
780 |     /// This is the text that will be used as the starting point for the model's decoding.
781 |     /// Calling this more than once will overwrite the previous initial prompt.
782 |     ///
783 |     /// # Arguments
784 |     /// * `initial_prompt` - A string slice representing the initial prompt text.
785 |     ///
786 |     /// # Panics
787 |     /// This method will panic if `initial_prompt` contains a null byte, as it cannot be converted into a `CString`.
788 |     ///
789 |     /// # Examples
790 |     /// ```
791 |     /// # use whisper_rs::{FullParams, SamplingStrategy};
792 |     /// let mut params = FullParams::new(SamplingStrategy::default());
793 |     /// params.set_initial_prompt("Hello, world!");
794 |     /// // ... further usage of params ...
795 |     /// ```
796 |     pub fn set_initial_prompt(&mut self, initial_prompt: &str) {
797 |         self.fp.initial_prompt = CString::new(initial_prompt)
798 |             .expect("Initial prompt contains null byte")
799 |             .into_raw() as *const c_char;
800 |     }
801 | }
802 | 
803 | // following implementations are safe
804 | // see https://github.com/ggerganov/whisper.cpp/issues/32#issuecomment-1272790388
805 | // concurrent usage is prevented by &mut self on methods that modify the struct
806 | unsafe impl Send for FullParams<'_, '_> {}
807 | unsafe impl Sync for FullParams<'_, '_> {}
808 | 
809 | #[cfg(test)]
810 | mod test_whisper_params_initial_prompt {
811 |     use super::*;
812 | 
813 |     impl<'a, 'b> FullParams<'a, 'b> {
814 |         pub fn get_initial_prompt(&self) -> &str {
815 |             // SAFETY: Ensure this is safe and respects the lifetime of the string in self.fp
816 |             unsafe {
817 |                 std::ffi::CStr::from_ptr(self.fp.initial_prompt)
818 |                     .to_str()
819 |                     .unwrap()
820 |             }
821 |         }
822 |     }
823 | 
824 |     #[test]
825 |     fn test_initial_prompt_normal_usage() {
826 |         let mut params = FullParams::new(SamplingStrategy::default());
827 |         let prompt = "Hello, world!";
828 |         params.set_initial_prompt(prompt);
829 |         assert_eq!(params.get_initial_prompt(), prompt);
830 |     }
831 | 
832 |     #[test]
833 |     #[should_panic(expected = "Initial prompt contains null byte")]
834 |     fn test_initial_prompt_null_byte() {
835 |         let mut params = FullParams::new(SamplingStrategy::default());
836 |         let prompt = "Hello\0, world!";
837 |         params.set_initial_prompt(prompt);
838 |         // Should panic
839 |     }
840 | 
841 |     #[test]
842 |     fn test_initial_prompt_empty_string() {
843 |         let mut params = FullParams::new(SamplingStrategy::default());
844 |         let prompt = "";
845 |         params.set_initial_prompt(prompt);
846 | 
847 |         assert_eq!(
848 |             params.get_initial_prompt(),
849 |             prompt,
850 |             "The initial prompt should be an empty string."
851 |         );
852 |     }
853 | 
854 |     #[test]
855 |     fn test_initial_prompt_repeated_calls() {
856 |         let mut params = FullParams::new(SamplingStrategy::default());
857 |         params.set_initial_prompt("First prompt");
858 |         assert_eq!(
859 |             params.get_initial_prompt(),
860 |             "First prompt",
861 |             "The initial prompt should be 'First prompt'."
862 |         );
863 | 
864 |         params.set_initial_prompt("Second prompt");
865 |         assert_eq!(
866 |             params.get_initial_prompt(),
867 |             "Second prompt",
868 |             "The initial prompt should be 'Second prompt' after second set."
869 |         );
870 |     }
871 | 
872 |     #[test]
873 |     fn test_initial_prompt_long_string() {
874 |         let mut params = FullParams::new(SamplingStrategy::default());
875 |         let long_prompt = "a".repeat(10000); // a long string of 10,000 'a' characters
876 |         params.set_initial_prompt(&long_prompt);
877 | 
878 |         assert_eq!(
879 |             params.get_initial_prompt(),
880 |             long_prompt.as_str(),
881 |             "The initial prompt should match the long string provided."
882 |         );
883 |     }
884 | }
885 | 


--------------------------------------------------------------------------------
/src/whisper_state.rs:
--------------------------------------------------------------------------------
  1 | use std::ffi::{c_int, CStr};
  2 | use std::sync::Arc;
  3 | 
  4 | use crate::{FullParams, WhisperError, WhisperInnerContext, WhisperToken, WhisperTokenData};
  5 | 
  6 | /// Rustified pointer to a Whisper state.
  7 | #[derive(Debug)]
  8 | pub struct WhisperState {
  9 |     ctx: Arc<WhisperInnerContext>,
 10 |     ptr: *mut whisper_rs_sys::whisper_state,
 11 | }
 12 | 
 13 | unsafe impl Send for WhisperState {}
 14 | 
 15 | unsafe impl Sync for WhisperState {}
 16 | 
 17 | impl Drop for WhisperState {
 18 |     fn drop(&mut self) {
 19 |         unsafe {
 20 |             whisper_rs_sys::whisper_free_state(self.ptr);
 21 |         }
 22 |     }
 23 | }
 24 | 
 25 | impl WhisperState {
 26 |     pub(crate) fn new(
 27 |         ctx: Arc<WhisperInnerContext>,
 28 |         ptr: *mut whisper_rs_sys::whisper_state,
 29 |     ) -> Self {
 30 |         Self { ctx, ptr }
 31 |     }
 32 | 
 33 |     /// Convert raw PCM audio (floating point 32 bit) to log mel spectrogram.
 34 |     /// The resulting spectrogram is stored in the context transparently.
 35 |     ///
 36 |     /// # Arguments
 37 |     /// * pcm: The raw PCM audio.
 38 |     /// * threads: How many threads to use. Defaults to 1. Must be at least 1, returns an error otherwise.
 39 |     ///
 40 |     /// # Returns
 41 |     /// Ok(()) on success, Err(WhisperError) on failure.
 42 |     ///
 43 |     /// # C++ equivalent
 44 |     /// `int whisper_pcm_to_mel(struct whisper_context * ctx, const float * samples, int n_samples, int n_threads)`
 45 |     pub fn pcm_to_mel(&mut self, pcm: &[f32], threads: usize) -> Result<(), WhisperError> {
 46 |         if threads < 1 {
 47 |             return Err(WhisperError::InvalidThreadCount);
 48 |         }
 49 |         let ret = unsafe {
 50 |             whisper_rs_sys::whisper_pcm_to_mel_with_state(
 51 |                 self.ctx.ctx,
 52 |                 self.ptr,
 53 |                 pcm.as_ptr(),
 54 |                 pcm.len() as c_int,
 55 |                 threads as c_int,
 56 |             )
 57 |         };
 58 |         if ret == -1 {
 59 |             Err(WhisperError::UnableToCalculateSpectrogram)
 60 |         } else if ret == 0 {
 61 |             Ok(())
 62 |         } else {
 63 |             Err(WhisperError::GenericError(ret))
 64 |         }
 65 |     }
 66 | 
 67 |     /// This can be used to set a custom log mel spectrogram inside the provided whisper state.
 68 |     /// Use this instead of whisper_pcm_to_mel() if you want to provide your own log mel spectrogram.
 69 |     ///
 70 |     /// # Note
 71 |     /// This is a low-level function.
 72 |     /// If you're a typical user, you probably don't want to use this function.
 73 |     /// See instead [WhisperState::pcm_to_mel].
 74 |     ///
 75 |     /// # Arguments
 76 |     /// * data: The log mel spectrogram.
 77 |     ///
 78 |     /// # Returns
 79 |     /// Ok(()) on success, Err(WhisperError) on failure.
 80 |     ///
 81 |     /// # C++ equivalent
 82 |     /// `int whisper_set_mel(struct whisper_context * ctx, const float * data, int n_len, int n_mel)`
 83 |     pub fn set_mel(&mut self, data: &[f32]) -> Result<(), WhisperError> {
 84 |         let hop_size = 160;
 85 |         let n_len = (data.len() / hop_size) * 2;
 86 |         let ret = unsafe {
 87 |             whisper_rs_sys::whisper_set_mel_with_state(
 88 |                 self.ctx.ctx,
 89 |                 self.ptr,
 90 |                 data.as_ptr(),
 91 |                 n_len as c_int,
 92 |                 80 as c_int,
 93 |             )
 94 |         };
 95 |         if ret == -1 {
 96 |             Err(WhisperError::InvalidMelBands)
 97 |         } else if ret == 0 {
 98 |             Ok(())
 99 |         } else {
100 |             Err(WhisperError::GenericError(ret))
101 |         }
102 |     }
103 | 
104 |     /// Run the Whisper encoder on the log mel spectrogram stored inside the provided whisper state.
105 |     /// Make sure to call [WhisperState::pcm_to_mel] or [WhisperState::set_mel] first.
106 |     ///
107 |     /// # Arguments
108 |     /// * offset: Can be used to specify the offset of the first frame in the spectrogram. Usually 0.
109 |     /// * threads: How many threads to use. Defaults to 1. Must be at least 1, returns an error otherwise.
110 |     ///
111 |     /// # Returns
112 |     /// Ok(()) on success, Err(WhisperError) on failure.
113 |     ///
114 |     /// # C++ equivalent
115 |     /// `int whisper_encode(struct whisper_context * ctx, int offset, int n_threads)`
116 |     pub fn encode(&mut self, offset: usize, threads: usize) -> Result<(), WhisperError> {
117 |         if threads < 1 {
118 |             return Err(WhisperError::InvalidThreadCount);
119 |         }
120 |         let ret = unsafe {
121 |             whisper_rs_sys::whisper_encode_with_state(
122 |                 self.ctx.ctx,
123 |                 self.ptr,
124 |                 offset as c_int,
125 |                 threads as c_int,
126 |             )
127 |         };
128 |         if ret == -1 {
129 |             Err(WhisperError::UnableToCalculateEvaluation)
130 |         } else if ret == 0 {
131 |             Ok(())
132 |         } else {
133 |             Err(WhisperError::GenericError(ret))
134 |         }
135 |     }
136 | 
137 |     /// Run the Whisper decoder to obtain the logits and probabilities for the next token.
138 |     /// Make sure to call [WhisperState::encode] first.
139 |     /// tokens + n_tokens is the provided context for the decoder.
140 |     ///
141 |     /// # Arguments
142 |     /// * tokens: The tokens to decode.
143 |     /// * n_tokens: The number of tokens to decode.
144 |     /// * n_past: The number of past tokens to use for the decoding.
145 |     /// * n_threads: How many threads to use. Defaults to 1. Must be at least 1, returns an error otherwise.
146 |     ///
147 |     /// # Returns
148 |     /// Ok(()) on success, Err(WhisperError) on failure.
149 |     ///
150 |     /// # C++ equivalent
151 |     /// `int whisper_decode(struct whisper_context * ctx, const whisper_token * tokens, int n_tokens, int n_past, int n_threads)`
152 |     pub fn decode(
153 |         &mut self,
154 |         tokens: &[WhisperToken],
155 |         n_past: usize,
156 |         threads: usize,
157 |     ) -> Result<(), WhisperError> {
158 |         if threads < 1 {
159 |             return Err(WhisperError::InvalidThreadCount);
160 |         }
161 |         let ret = unsafe {
162 |             whisper_rs_sys::whisper_decode_with_state(
163 |                 self.ctx.ctx,
164 |                 self.ptr,
165 |                 tokens.as_ptr(),
166 |                 tokens.len() as c_int,
167 |                 n_past as c_int,
168 |                 threads as c_int,
169 |             )
170 |         };
171 |         if ret == -1 {
172 |             Err(WhisperError::UnableToCalculateEvaluation)
173 |         } else if ret == 0 {
174 |             Ok(())
175 |         } else {
176 |             Err(WhisperError::GenericError(ret))
177 |         }
178 |     }
179 | 
180 |     // Language functions
181 |     /// Use mel data at offset_ms to try and auto-detect the spoken language
182 |     /// Make sure to call pcm_to_mel() or set_mel() first
183 |     ///
184 |     /// # Arguments
185 |     /// * offset_ms: The offset in milliseconds to use for the language detection.
186 |     /// * n_threads: How many threads to use. Defaults to 1. Must be at least 1, returns an error otherwise.
187 |     ///
188 |     /// # Returns
189 |     /// `Ok((i32, Vec<f32>))` on success where the i32 is detected language id and Vec<f32>
190 |     /// is array with the probabilities of all languages, `Err(WhisperError)` on failure.
191 |     ///
192 |     /// # C++ equivalent
193 |     /// `int whisper_lang_auto_detect(struct whisper_context * ctx, int offset_ms, int n_threads, float * lang_probs)`
194 |     pub fn lang_detect(
195 |         &self,
196 |         offset_ms: usize,
197 |         threads: usize,
198 |     ) -> Result<(i32, Vec<f32>), WhisperError> {
199 |         if threads < 1 {
200 |             return Err(WhisperError::InvalidThreadCount);
201 |         }
202 | 
203 |         let mut lang_probs: Vec<f32> = vec![0.0; crate::standalone::get_lang_max_id() as usize + 1];
204 |         let ret = unsafe {
205 |             whisper_rs_sys::whisper_lang_auto_detect_with_state(
206 |                 self.ctx.ctx,
207 |                 self.ptr,
208 |                 offset_ms as c_int,
209 |                 threads as c_int,
210 |                 lang_probs.as_mut_ptr(),
211 |             )
212 |         };
213 |         if ret < 0 {
214 |             Err(WhisperError::GenericError(ret))
215 |         } else {
216 |             Ok((ret as i32, lang_probs))
217 |         }
218 |     }
219 | 
220 |     // logit functions
221 |     /// Gets logits obtained from the last call to [WhisperState::decode].
222 |     /// As of whisper.cpp 1.4.1, only a single row of logits is available, corresponding to the last token in the input.
223 |     ///
224 |     /// # Returns
225 |     /// A slice of logits with length equal to n_vocab.
226 |     ///
227 |     /// # C++ equivalent
228 |     /// `float * whisper_get_logits(struct whisper_context * ctx)`
229 |     pub fn get_logits(&self) -> Result<&[f32], WhisperError> {
230 |         let ret = unsafe { whisper_rs_sys::whisper_get_logits_from_state(self.ptr) };
231 |         if ret.is_null() {
232 |             return Err(WhisperError::NullPointer);
233 |         }
234 |         let n_vocab = self.n_vocab();
235 |         Ok(unsafe { std::slice::from_raw_parts(ret, n_vocab as usize) })
236 |     }
237 | 
238 |     // model attributes
239 |     /// Get the mel spectrogram length.
240 |     ///
241 |     /// # Returns
242 |     /// Ok(c_int) on success, Err(WhisperError) on failure.
243 |     ///
244 |     /// # C++ equivalent
245 |     /// `int whisper_n_len_from_state(struct whisper_context * ctx)`
246 |     #[inline]
247 |     pub fn n_len(&self) -> Result<c_int, WhisperError> {
248 |         Ok(unsafe { whisper_rs_sys::whisper_n_len_from_state(self.ptr) })
249 |     }
250 | 
251 |     /// Get n_vocab.
252 |     ///
253 |     /// # Returns
254 |     /// c_int
255 |     ///
256 |     /// # C++ equivalent
257 |     /// `int whisper_n_vocab        (struct whisper_context * ctx)`
258 |     #[inline]
259 |     pub fn n_vocab(&self) -> c_int {
260 |         unsafe { whisper_rs_sys::whisper_n_vocab(self.ctx.ctx) }
261 |     }
262 | 
263 |     /// Run the entire model: PCM -> log mel spectrogram -> encoder -> decoder -> text
264 |     /// Uses the specified decoding strategy to obtain the text.
265 |     ///
266 |     /// This is usually the only function you need to call as an end user.
267 |     ///
268 |     /// # Arguments
269 |     /// * params: [crate::FullParams] struct.
270 |     /// * pcm: raw PCM audio data, 32 bit floating point at a sample rate of 16 kHz, 1 channel.
271 |     ///   See utilities in the root of this crate for functions to convert audio to this format.
272 |     ///
273 |     /// # Returns
274 |     /// Ok(c_int) on success, Err(WhisperError) on failure.
275 |     ///
276 |     /// # C++ equivalent
277 |     /// `int whisper_full(struct whisper_context * ctx, struct whisper_full_params params, const float * samples, int n_samples)`
278 |     pub fn full(&mut self, params: FullParams, data: &[f32]) -> Result<c_int, WhisperError> {
279 |         if data.is_empty() {
280 |             // can randomly trigger segmentation faults if we don't check this
281 |             return Err(WhisperError::NoSamples);
282 |         }
283 | 
284 |         let ret = unsafe {
285 |             whisper_rs_sys::whisper_full_with_state(
286 |                 self.ctx.ctx,
287 |                 self.ptr,
288 |                 params.fp,
289 |                 data.as_ptr(),
290 |                 data.len() as c_int,
291 |             )
292 |         };
293 |         if ret == -1 {
294 |             Err(WhisperError::UnableToCalculateSpectrogram)
295 |         } else if ret == 7 {
296 |             Err(WhisperError::FailedToEncode)
297 |         } else if ret == 8 {
298 |             Err(WhisperError::FailedToDecode)
299 |         } else if ret == 0 {
300 |             Ok(ret)
301 |         } else {
302 |             Err(WhisperError::GenericError(ret))
303 |         }
304 |     }
305 | 
306 |     /// Number of generated text segments.
307 |     /// A segment can be a few words, a sentence, or even a paragraph.
308 |     ///
309 |     /// # C++ equivalent
310 |     /// `int whisper_full_n_segments(struct whisper_context * ctx)`
311 |     #[inline]
312 |     pub fn full_n_segments(&self) -> Result<c_int, WhisperError> {
313 |         Ok(unsafe { whisper_rs_sys::whisper_full_n_segments_from_state(self.ptr) })
314 |     }
315 | 
316 |     /// Language ID associated with the provided state.
317 |     ///
318 |     /// # C++ equivalent
319 |     /// `int whisper_full_lang_id_from_state(struct whisper_state * state);`
320 |     #[inline]
321 |     pub fn full_lang_id_from_state(&self) -> Result<c_int, WhisperError> {
322 |         Ok(unsafe { whisper_rs_sys::whisper_full_lang_id_from_state(self.ptr) })
323 |     }
324 | 
325 |     /// Get the start time of the specified segment.
326 |     ///
327 |     /// # Arguments
328 |     /// * segment: Segment index.
329 |     ///
330 |     /// # C++ equivalent
331 |     /// `int64_t whisper_full_get_segment_t0(struct whisper_context * ctx, int i_segment)`
332 |     #[inline]
333 |     pub fn full_get_segment_t0(&self, segment: c_int) -> Result<i64, WhisperError> {
334 |         Ok(unsafe { whisper_rs_sys::whisper_full_get_segment_t0_from_state(self.ptr, segment) })
335 |     }
336 | 
337 |     /// Get the end time of the specified segment.
338 |     ///
339 |     /// # Arguments
340 |     /// * segment: Segment index.
341 |     ///
342 |     /// # C++ equivalent
343 |     /// `int64_t whisper_full_get_segment_t1(struct whisper_context * ctx, int i_segment)`
344 |     #[inline]
345 |     pub fn full_get_segment_t1(&self, segment: c_int) -> Result<i64, WhisperError> {
346 |         Ok(unsafe { whisper_rs_sys::whisper_full_get_segment_t1_from_state(self.ptr, segment) })
347 |     }
348 | 
349 |     fn full_get_segment_raw(&self, segment: c_int) -> Result<&CStr, WhisperError> {
350 |         let ret =
351 |             unsafe { whisper_rs_sys::whisper_full_get_segment_text_from_state(self.ptr, segment) };
352 |         if ret.is_null() {
353 |             return Err(WhisperError::NullPointer);
354 |         }
355 |         unsafe { Ok(CStr::from_ptr(ret)) }
356 |     }
357 | 
358 |     /// Get the raw bytes of the specified segment.
359 |     ///
360 |     /// # Arguments
361 |     /// * segment: Segment index.
362 |     ///
363 |     /// # Returns
364 |     /// `Ok(Vec<u8>)` on success, with the returned bytes or
365 |     /// `Err(WhisperError::NullPointer)` on failure (this is the only possible error)
366 |     ///
367 |     /// # C++ equivalent
368 |     /// `const char * whisper_full_get_segment_text(struct whisper_context * ctx, int i_segment)`
369 |     pub fn full_get_segment_bytes(&self, segment: c_int) -> Result<Vec<u8>, WhisperError> {
370 |         Ok(self.full_get_segment_raw(segment)?.to_bytes().to_vec())
371 |     }
372 | 
373 |     /// Get the text of the specified segment.
374 |     ///
375 |     /// # Arguments
376 |     /// * segment: Segment index.
377 |     ///
378 |     /// # Returns
379 |     /// `Ok(String)` on success, with the UTF-8 validated string, or
380 |     /// `Err(WhisperError)` on failure (either `NullPointer` or `InvalidUtf8`)
381 |     ///
382 |     /// # C++ equivalent
383 |     /// `const char * whisper_full_get_segment_text(struct whisper_context * ctx, int i_segment)`
384 |     pub fn full_get_segment_text(&self, segment: c_int) -> Result<String, WhisperError> {
385 |         Ok(self.full_get_segment_raw(segment)?.to_str()?.to_string())
386 |     }
387 | 
388 |     /// Get the text of the specified segment.
389 |     /// This function differs from [WhisperState::full_get_segment_text]
390 |     /// in that it ignores invalid UTF-8 in whisper strings,
391 |     /// instead opting to replace it with the replacement character.
392 |     ///
393 |     /// # Arguments
394 |     /// * segment: Segment index.
395 |     ///
396 |     /// # Returns
397 |     /// `Ok(String)` on success, or
398 |     /// `Err(WhisperError::NullPointer)` on failure (this is the only possible error)
399 |     ///
400 |     /// # C++ equivalent
401 |     /// `const char * whisper_full_get_segment_text(struct whisper_context * ctx, int i_segment)`
402 |     pub fn full_get_segment_text_lossy(&self, segment: c_int) -> Result<String, WhisperError> {
403 |         Ok(self
404 |             .full_get_segment_raw(segment)?
405 |             .to_string_lossy()
406 |             .to_string())
407 |     }
408 | 
409 |     /// Get number of tokens in the specified segment.
410 |     ///
411 |     /// # Arguments
412 |     /// * segment: Segment index.
413 |     ///
414 |     /// # Returns
415 |     /// c_int
416 |     ///
417 |     /// # C++ equivalent
418 |     /// `int whisper_full_n_tokens(struct whisper_context * ctx, int i_segment)`
419 |     #[inline]
420 |     pub fn full_n_tokens(&self, segment: c_int) -> Result<c_int, WhisperError> {
421 |         Ok(unsafe { whisper_rs_sys::whisper_full_n_tokens_from_state(self.ptr, segment) })
422 |     }
423 | 
424 |     fn full_get_token_raw(&self, segment: c_int, token: c_int) -> Result<&CStr, WhisperError> {
425 |         let ret = unsafe {
426 |             whisper_rs_sys::whisper_full_get_token_text_from_state(
427 |                 self.ctx.ctx,
428 |                 self.ptr,
429 |                 segment,
430 |                 token,
431 |             )
432 |         };
433 |         if ret.is_null() {
434 |             return Err(WhisperError::NullPointer);
435 |         }
436 |         unsafe { Ok(CStr::from_ptr(ret)) }
437 |     }
438 | 
439 |     /// Get the raw token bytes of the specified token in the specified segment.
440 |     ///
441 |     /// Useful if you're using a language for which whisper is known to split tokens
442 |     /// away from UTF-8 character boundaries.
443 |     ///
444 |     /// # Arguments
445 |     /// * segment: Segment index.
446 |     /// * token: Token index.
447 |     ///
448 |     /// # Returns
449 |     /// `Ok(Vec<u8>)` on success, with the returned bytes or
450 |     /// `Err(WhisperError::NullPointer)` on failure (this is the only possible error)
451 |     ///
452 |     /// # C++ equivalent
453 |     /// `const char * whisper_full_get_token_text(struct whisper_context * ctx, int i_segment, int i_token)`
454 |     pub fn full_get_token_bytes(
455 |         &self,
456 |         segment: c_int,
457 |         token: c_int,
458 |     ) -> Result<Vec<u8>, WhisperError> {
459 |         Ok(self.full_get_token_raw(segment, token)?.to_bytes().to_vec())
460 |     }
461 | 
462 |     /// Get the token text of the specified token in the specified segment.
463 |     ///
464 |     /// # Arguments
465 |     /// * segment: Segment index.
466 |     /// * token: Token index.
467 |     ///
468 |     /// # Returns
469 |     /// `Ok(String)` on success, with the UTF-8 validated string, or
470 |     /// `Err(WhisperError)` on failure (either `NullPointer` or `InvalidUtf8`)
471 |     ///
472 |     /// # C++ equivalent
473 |     /// `const char * whisper_full_get_token_text(struct whisper_context * ctx, int i_segment, int i_token)`
474 |     pub fn full_get_token_text(
475 |         &self,
476 |         segment: c_int,
477 |         token: c_int,
478 |     ) -> Result<String, WhisperError> {
479 |         Ok(self
480 |             .full_get_token_raw(segment, token)?
481 |             .to_str()?
482 |             .to_string())
483 |     }
484 | 
485 |     /// Get the token text of the specified token in the specified segment.
486 |     /// This function differs from [WhisperState::full_get_token_text]
487 |     /// in that it ignores invalid UTF-8 in whisper strings,
488 |     /// instead opting to replace it with the replacement character.
489 |     ///
490 |     /// # Arguments
491 |     /// * segment: Segment index.
492 |     /// * token: Token index.
493 |     ///
494 |     /// # Returns
495 |     /// `Ok(String)` on success, or
496 |     /// `Err(WhisperError::NullPointer)` on failure (this is the only possible error)
497 |     ///
498 |     /// # C++ equivalent
499 |     /// `const char * whisper_full_get_token_text(struct whisper_context * ctx, int i_segment, int i_token)`
500 |     pub fn full_get_token_text_lossy(
501 |         &self,
502 |         segment: c_int,
503 |         token: c_int,
504 |     ) -> Result<String, WhisperError> {
505 |         Ok(self
506 |             .full_get_token_raw(segment, token)?
507 |             .to_string_lossy()
508 |             .to_string())
509 |     }
510 | 
511 |     /// Get the token ID of the specified token in the specified segment.
512 |     ///
513 |     /// # Arguments
514 |     /// * segment: Segment index.
515 |     /// * token: Token index.
516 |     ///
517 |     /// # Returns
518 |     /// [crate::WhisperToken]
519 |     ///
520 |     /// # C++ equivalent
521 |     /// `whisper_token whisper_full_get_token_id (struct whisper_context * ctx, int i_segment, int i_token)`
522 |     pub fn full_get_token_id(
523 |         &self,
524 |         segment: c_int,
525 |         token: c_int,
526 |     ) -> Result<WhisperToken, WhisperError> {
527 |         Ok(unsafe {
528 |             whisper_rs_sys::whisper_full_get_token_id_from_state(self.ptr, segment, token)
529 |         })
530 |     }
531 | 
532 |     /// Get token data for the specified token in the specified segment.
533 |     ///
534 |     /// # Arguments
535 |     /// * segment: Segment index.
536 |     /// * token: Token index.
537 |     ///
538 |     /// # Returns
539 |     /// [crate::WhisperTokenData]
540 |     ///
541 |     /// # C++ equivalent
542 |     /// `whisper_token_data whisper_full_get_token_data(struct whisper_context * ctx, int i_segment, int i_token)`
543 |     #[inline]
544 |     pub fn full_get_token_data(
545 |         &self,
546 |         segment: c_int,
547 |         token: c_int,
548 |     ) -> Result<WhisperTokenData, WhisperError> {
549 |         Ok(unsafe {
550 |             whisper_rs_sys::whisper_full_get_token_data_from_state(self.ptr, segment, token)
551 |         })
552 |     }
553 | 
554 |     /// Get the probability of the specified token in the specified segment.
555 |     ///
556 |     /// # Arguments
557 |     /// * segment: Segment index.
558 |     /// * token: Token index.
559 |     ///
560 |     /// # Returns
561 |     /// f32
562 |     ///
563 |     /// # C++ equivalent
564 |     /// `float whisper_full_get_token_p(struct whisper_context * ctx, int i_segment, int i_token)`
565 |     #[inline]
566 |     pub fn full_get_token_prob(&self, segment: c_int, token: c_int) -> Result<f32, WhisperError> {
567 |         Ok(
568 |             unsafe {
569 |                 whisper_rs_sys::whisper_full_get_token_p_from_state(self.ptr, segment, token)
570 |             },
571 |         )
572 |     }
573 | 
574 |     /// Get whether the next segment is predicted as a speaker turn.
575 |     ///
576 |     /// # Arguments
577 |     /// * i_segment: Segment index.
578 |     ///
579 |     /// # Returns
580 |     /// bool
581 |     ///
582 |     /// # C++ equivalent
583 |     /// `bool whisper_full_get_segment_speaker_turn_next_from_state(struct whisper_state * state, int i_segment)`
584 |     pub fn full_get_segment_speaker_turn_next(&mut self, i_segment: c_int) -> bool {
585 |         unsafe {
586 |             whisper_rs_sys::whisper_full_get_segment_speaker_turn_next_from_state(
587 |                 self.ptr, i_segment,
588 |             )
589 |         }
590 |     }
591 | }
592 | 


--------------------------------------------------------------------------------
/sys/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "whisper-rs-sys"
 3 | version = "0.12.1"
 4 | edition = "2021"
 5 | description = "Rust bindings for whisper.cpp (FFI bindings)"
 6 | license = "Unlicense"
 7 | documentation = "https://docs.rs/whisper-rs-sys"
 8 | repository = "https://github.com/tazz4843/whisper-rs"
 9 | links = "whisper"
10 | include = [
11 |     "whisper.cpp/bindings/javascript/package-tmpl.json",
12 |     "whisper.cpp/bindings/CMakeLists.txt",
13 |     "whisper.cpp/CMakeLists.txt",
14 |     "whisper.cpp/cmake",
15 |     "whisper.cpp/src/**",
16 |     "whisper.cpp/include/whisper.h",
17 |     "whisper.cpp/ggml/cmake",
18 |     "whisper.cpp/ggml/CMakeLists.txt",
19 |     "whisper.cpp/ggml/src/**",
20 |     "whisper.cpp/ggml/include/*.h",
21 |     "whisper.cpp/LICENSE",
22 |     "src/*.rs",
23 |     "build.rs",
24 |     "wrapper.h",
25 | ]
26 | 
27 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
28 | 
29 | [features]
30 | coreml = []
31 | cuda = []
32 | hipblas = []
33 | openblas = []
34 | metal = []
35 | vulkan = []
36 | force-debug = []
37 | openmp = []
38 | 
39 | [build-dependencies]
40 | cmake = "0.1"
41 | bindgen = "0.71"
42 | cfg-if = "1"
43 | fs_extra = "1.3"
44 | 


--------------------------------------------------------------------------------
/sys/build.rs:
--------------------------------------------------------------------------------
  1 | #![allow(clippy::uninlined_format_args)]
  2 | 
  3 | extern crate bindgen;
  4 | 
  5 | use cmake::Config;
  6 | use std::env;
  7 | use std::fs::File;
  8 | use std::io::{BufRead, BufReader};
  9 | use std::path::PathBuf;
 10 | 
 11 | fn main() {
 12 |     let target = env::var("TARGET").unwrap();
 13 |     // Link C++ standard library
 14 |     if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
 15 |         println!("cargo:rustc-link-lib=dylib={}", cpp_stdlib);
 16 |     }
 17 |     // Link macOS Accelerate framework for matrix calculations
 18 |     if target.contains("apple") {
 19 |         println!("cargo:rustc-link-lib=framework=Accelerate");
 20 |         #[cfg(feature = "coreml")]
 21 |         {
 22 |             println!("cargo:rustc-link-lib=framework=Foundation");
 23 |             println!("cargo:rustc-link-lib=framework=CoreML");
 24 |         }
 25 |         #[cfg(feature = "metal")]
 26 |         {
 27 |             println!("cargo:rustc-link-lib=framework=Foundation");
 28 |             println!("cargo:rustc-link-lib=framework=Metal");
 29 |             println!("cargo:rustc-link-lib=framework=MetalKit");
 30 |         }
 31 |     }
 32 | 
 33 |     #[cfg(feature = "coreml")]
 34 |     println!("cargo:rustc-link-lib=static=whisper.coreml");
 35 | 
 36 |     #[cfg(feature = "openblas")]
 37 |     {
 38 |         if let Ok(openblas_path) = env::var("OPENBLAS_PATH") {
 39 |             println!(
 40 |                 "cargo::rustc-link-search={}",
 41 |                 PathBuf::from(openblas_path).join("lib").display()
 42 |             );
 43 |         }
 44 |         if cfg!(windows) {
 45 |             println!("cargo:rustc-link-lib=libopenblas");
 46 |         } else {
 47 |             println!("cargo:rustc-link-lib=openblas");
 48 |         }
 49 |     }
 50 |     #[cfg(feature = "cuda")]
 51 |     {
 52 |         println!("cargo:rustc-link-lib=cublas");
 53 |         println!("cargo:rustc-link-lib=cudart");
 54 |         println!("cargo:rustc-link-lib=cublasLt");
 55 |         println!("cargo:rustc-link-lib=cuda");
 56 |         cfg_if::cfg_if! {
 57 |             if #[cfg(target_os = "windows")] {
 58 |                 let cuda_path = PathBuf::from(env::var("CUDA_PATH").unwrap()).join("lib/x64");
 59 |                 println!("cargo:rustc-link-search={}", cuda_path.display());
 60 |             } else {
 61 |                 println!("cargo:rustc-link-lib=culibos");
 62 |                 println!("cargo:rustc-link-search=/usr/local/cuda/lib64");
 63 |                 println!("cargo:rustc-link-search=/usr/local/cuda/lib64/stubs");
 64 |                 println!("cargo:rustc-link-search=/opt/cuda/lib64");
 65 |                 println!("cargo:rustc-link-search=/opt/cuda/lib64/stubs");
 66 |             }
 67 |         }
 68 |     }
 69 |     #[cfg(feature = "hipblas")]
 70 |     {
 71 |         println!("cargo:rustc-link-lib=hipblas");
 72 |         println!("cargo:rustc-link-lib=rocblas");
 73 |         println!("cargo:rustc-link-lib=amdhip64");
 74 | 
 75 |         cfg_if::cfg_if! {
 76 |             if #[cfg(target_os = "windows")] {
 77 |                 panic!("Due to a problem with the last revision of the ROCm 5.7 library, it is not possible to compile the library for the windows environment.\nSee https://github.com/ggerganov/whisper.cpp/issues/2202 for more details.")
 78 |             } else {
 79 |                 println!("cargo:rerun-if-env-changed=HIP_PATH");
 80 | 
 81 |                 let hip_path = match env::var("HIP_PATH") {
 82 |                     Ok(path) =>PathBuf::from(path),
 83 |                     Err(_) => PathBuf::from("/opt/rocm"),
 84 |                 };
 85 |                 let hip_lib_path = hip_path.join("lib");
 86 | 
 87 |                 println!("cargo:rustc-link-search={}",hip_lib_path.display());
 88 |             }
 89 |         }
 90 |     }
 91 | 
 92 |     #[cfg(feature = "openmp")]
 93 |     {
 94 |         if target.contains("gnu") {
 95 |             println!("cargo:rustc-link-lib=gomp");
 96 |         } else if target.contains("apple") {
 97 |             println!("cargo:rustc-link-lib=omp");
 98 |             println!("cargo:rustc-link-search=/opt/homebrew/opt/libomp/lib");
 99 |         }
100 |     }
101 | 
102 |     println!("cargo:rerun-if-changed=wrapper.h");
103 | 
104 |     let out = PathBuf::from(env::var("OUT_DIR").unwrap());
105 |     let whisper_root = out.join("whisper.cpp/");
106 | 
107 |     if !whisper_root.exists() {
108 |         std::fs::create_dir_all(&whisper_root).unwrap();
109 |         fs_extra::dir::copy("./whisper.cpp", &out, &Default::default()).unwrap_or_else(|e| {
110 |             panic!(
111 |                 "Failed to copy whisper sources into {}: {}",
112 |                 whisper_root.display(),
113 |                 e
114 |             )
115 |         });
116 |     }
117 | 
118 |     if env::var("WHISPER_DONT_GENERATE_BINDINGS").is_ok() {
119 |         let _: u64 = std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
120 |             .expect("Failed to copy bindings.rs");
121 |     } else {
122 |         let bindings = bindgen::Builder::default().header("wrapper.h");
123 | 
124 |         #[cfg(feature = "metal")]
125 |         let bindings = bindings.header("whisper.cpp/ggml/include/ggml-metal.h");
126 | 
127 |         let bindings = bindings
128 |             .clang_arg("-I./whisper.cpp/")
129 |             .clang_arg("-I./whisper.cpp/include")
130 |             .clang_arg("-I./whisper.cpp/ggml/include")
131 |             .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
132 |             .generate();
133 | 
134 |         match bindings {
135 |             Ok(b) => {
136 |                 let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
137 |                 b.write_to_file(out_path.join("bindings.rs"))
138 |                     .expect("Couldn't write bindings!");
139 |             }
140 |             Err(e) => {
141 |                 println!("cargo:warning=Unable to generate bindings: {}", e);
142 |                 println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
143 |                 // copy src/bindings.rs to OUT_DIR
144 |                 std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
145 |                     .expect("Unable to copy bindings.rs");
146 |             }
147 |         }
148 |     };
149 | 
150 |     // stop if we're on docs.rs
151 |     if env::var("DOCS_RS").is_ok() {
152 |         return;
153 |     }
154 | 
155 |     let mut config = Config::new(&whisper_root);
156 | 
157 |     config
158 |         .profile("Release")
159 |         .define("BUILD_SHARED_LIBS", "OFF")
160 |         .define("WHISPER_ALL_WARNINGS", "OFF")
161 |         .define("WHISPER_ALL_WARNINGS_3RD_PARTY", "OFF")
162 |         .define("WHISPER_BUILD_TESTS", "OFF")
163 |         .define("WHISPER_BUILD_EXAMPLES", "OFF")
164 |         .very_verbose(true)
165 |         .pic(true);
166 | 
167 |     if cfg!(target_os = "windows") {
168 |         config.cxxflag("/utf-8");
169 |     }
170 | 
171 |     if cfg!(feature = "coreml") {
172 |         config.define("WHISPER_COREML", "ON");
173 |         config.define("WHISPER_COREML_ALLOW_FALLBACK", "1");
174 |     }
175 | 
176 |     if cfg!(feature = "cuda") {
177 |         config.define("GGML_CUDA", "ON");
178 |     }
179 | 
180 |     if cfg!(feature = "hipblas") {
181 |         config.define("GGML_HIPBLAS", "ON");
182 |         config.define("CMAKE_C_COMPILER", "hipcc");
183 |         config.define("CMAKE_CXX_COMPILER", "hipcc");
184 |         println!("cargo:rerun-if-env-changed=AMDGPU_TARGETS");
185 |         if let Ok(gpu_targets) = env::var("AMDGPU_TARGETS") {
186 |             config.define("AMDGPU_TARGETS", gpu_targets);
187 |         }
188 |     }
189 | 
190 |     if cfg!(feature = "vulkan") {
191 |         config.define("GGML_VULKAN", "ON");
192 |         if cfg!(windows) {
193 |             println!("cargo:rerun-if-env-changed=VULKAN_SDK");
194 |             println!("cargo:rustc-link-lib=vulkan-1");
195 |             let vulkan_path = match env::var("VULKAN_SDK") {
196 |                 Ok(path) => PathBuf::from(path),
197 |                 Err(_) => panic!(
198 |                     "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
199 |                 ),
200 |             };
201 |             let vulkan_lib_path = vulkan_path.join("Lib");
202 |             println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
203 |         } else if cfg!(target_os = "macos") {
204 |             println!("cargo:rerun-if-env-changed=VULKAN_SDK");
205 |             println!("cargo:rustc-link-lib=vulkan");
206 |             let vulkan_path = match env::var("VULKAN_SDK") {
207 |                 Ok(path) => PathBuf::from(path),
208 |                 Err(_) => panic!(
209 |                     "Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set"
210 |                 ),
211 |             };
212 |             let vulkan_lib_path = vulkan_path.join("lib");
213 |             println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
214 |         } else {
215 |             println!("cargo:rustc-link-lib=vulkan");
216 |         }
217 |     }
218 | 
219 |     if cfg!(feature = "openblas") {
220 |         config.define("GGML_BLAS", "ON");
221 |         config.define("GGML_BLAS_VENDOR", "OpenBLAS");
222 |         if env::var("BLAS_INCLUDE_DIRS").is_err() {
223 |             panic!("BLAS_INCLUDE_DIRS environment variable must be set when using OpenBLAS");
224 |         }
225 |         config.define("BLAS_INCLUDE_DIRS", env::var("BLAS_INCLUDE_DIRS").unwrap());
226 |         println!("cargo:rerun-if-env-changed=BLAS_INCLUDE_DIRS");
227 |     }
228 | 
229 |     if cfg!(feature = "metal") {
230 |         config.define("GGML_METAL", "ON");
231 |         config.define("GGML_METAL_NDEBUG", "ON");
232 |         config.define("GGML_METAL_EMBED_LIBRARY", "ON");
233 |     } else {
234 |         // Metal is enabled by default, so we need to explicitly disable it
235 |         config.define("GGML_METAL", "OFF");
236 |     }
237 | 
238 |     if cfg!(debug_assertions) || cfg!(feature = "force-debug") {
239 |         // debug builds are too slow to even remotely be usable,
240 |         // so we build with optimizations even in debug mode
241 |         config.define("CMAKE_BUILD_TYPE", "RelWithDebInfo");
242 |         config.cxxflag("-DWHISPER_DEBUG");
243 |     }
244 | 
245 |     // Allow passing any WHISPER or CMAKE compile flags
246 |     for (key, value) in env::vars() {
247 |         let is_whisper_flag =
248 |             key.starts_with("WHISPER_") && key != "WHISPER_DONT_GENERATE_BINDINGS";
249 |         let is_cmake_flag = key.starts_with("CMAKE_");
250 |         if is_whisper_flag || is_cmake_flag {
251 |             config.define(&key, &value);
252 |         }
253 |     }
254 | 
255 |     if cfg!(not(feature = "openmp")) {
256 |         config.define("GGML_OPENMP", "OFF");
257 |     }
258 | 
259 |     let destination = config.build();
260 | 
261 |     add_link_search_path(&out.join("build")).unwrap();
262 | 
263 |     println!("cargo:rustc-link-search=native={}", destination.display());
264 |     println!("cargo:rustc-link-lib=static=whisper");
265 |     println!("cargo:rustc-link-lib=static=ggml");
266 |     println!("cargo:rustc-link-lib=static=ggml-base");
267 |     println!("cargo:rustc-link-lib=static=ggml-cpu");
268 |     if cfg!(target_os = "macos") || cfg!(feature = "openblas") {
269 |         println!("cargo:rustc-link-lib=static=ggml-blas");
270 |     }
271 |     if cfg!(feature = "vulkan") {
272 |         println!("cargo:rustc-link-lib=static=ggml-vulkan");
273 |     }
274 | 
275 |     if cfg!(feature = "metal") {
276 |         println!("cargo:rustc-link-lib=static=ggml-metal");
277 |     }
278 | 
279 |     if cfg!(feature = "cuda") {
280 |         println!("cargo:rustc-link-lib=static=ggml-cuda");
281 |     }
282 | 
283 |     println!(
284 |         "cargo:WHISPER_CPP_VERSION={}",
285 |         get_whisper_cpp_version(&whisper_root)
286 |             .expect("Failed to read whisper.cpp CMake config")
287 |             .expect("Could not find whisper.cpp version declaration"),
288 |     );
289 | 
290 |     // for whatever reason this file is generated during build and triggers cargo complaining
291 |     _ = std::fs::remove_file("bindings/javascript/package.json");
292 | }
293 | 
294 | // From https://github.com/alexcrichton/cc-rs/blob/fba7feded71ee4f63cfe885673ead6d7b4f2f454/src/lib.rs#L2462
295 | fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
296 |     if target.contains("msvc") {
297 |         None
298 |     } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
299 |         Some("c++")
300 |     } else if target.contains("android") {
301 |         Some("c++_shared")
302 |     } else {
303 |         Some("stdc++")
304 |     }
305 | }
306 | 
307 | fn add_link_search_path(dir: &std::path::Path) -> std::io::Result<()> {
308 |     if dir.is_dir() {
309 |         println!("cargo:rustc-link-search={}", dir.display());
310 |         for entry in std::fs::read_dir(dir)? {
311 |             add_link_search_path(&entry?.path())?;
312 |         }
313 |     }
314 |     Ok(())
315 | }
316 | 
317 | fn get_whisper_cpp_version(whisper_root: &std::path::Path) -> std::io::Result<Option<String>> {
318 |     let cmake_lists = BufReader::new(File::open(whisper_root.join("CMakeLists.txt"))?);
319 | 
320 |     for line in cmake_lists.lines() {
321 |         let line = line?;
322 | 
323 |         if let Some(suffix) = line.strip_prefix(r#"project("whisper.cpp" VERSION "#) {
324 |             let whisper_cpp_version = suffix.trim_end_matches(')');
325 |             return Ok(Some(whisper_cpp_version.into()));
326 |         }
327 |     }
328 | 
329 |     Ok(None)
330 | }
331 | 


--------------------------------------------------------------------------------
/sys/src/bindings.rs:
--------------------------------------------------------------------------------
   1 | /* automatically generated by rust-bindgen 0.70.1 */
   2 | 
   3 | pub const __bool_true_false_are_defined: u32 = 1;
   4 | pub const true_: u32 = 1;
   5 | pub const false_: u32 = 0;
   6 | pub const _STDINT_H: u32 = 1;
   7 | pub const _FEATURES_H: u32 = 1;
   8 | pub const _DEFAULT_SOURCE: u32 = 1;
   9 | pub const __GLIBC_USE_ISOC23: u32 = 0;
  10 | pub const __USE_ISOC11: u32 = 1;
  11 | pub const __USE_ISOC99: u32 = 1;
  12 | pub const __USE_ISOC95: u32 = 1;
  13 | pub const __USE_POSIX_IMPLICITLY: u32 = 1;
  14 | pub const _POSIX_SOURCE: u32 = 1;
  15 | pub const _POSIX_C_SOURCE: u32 = 200809;
  16 | pub const __USE_POSIX: u32 = 1;
  17 | pub const __USE_POSIX2: u32 = 1;
  18 | pub const __USE_POSIX199309: u32 = 1;
  19 | pub const __USE_POSIX199506: u32 = 1;
  20 | pub const __USE_XOPEN2K: u32 = 1;
  21 | pub const __USE_XOPEN2K8: u32 = 1;
  22 | pub const _ATFILE_SOURCE: u32 = 1;
  23 | pub const __WORDSIZE: u32 = 64;
  24 | pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
  25 | pub const __SYSCALL_WORDSIZE: u32 = 64;
  26 | pub const __TIMESIZE: u32 = 64;
  27 | pub const __USE_TIME_BITS64: u32 = 1;
  28 | pub const __USE_MISC: u32 = 1;
  29 | pub const __USE_ATFILE: u32 = 1;
  30 | pub const __USE_FORTIFY_LEVEL: u32 = 0;
  31 | pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
  32 | pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
  33 | pub const __GLIBC_USE_C23_STRTOL: u32 = 0;
  34 | pub const _STDC_PREDEF_H: u32 = 1;
  35 | pub const __STDC_IEC_559__: u32 = 1;
  36 | pub const __STDC_IEC_60559_BFP__: u32 = 201404;
  37 | pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
  38 | pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
  39 | pub const __STDC_ISO_10646__: u32 = 201706;
  40 | pub const __GNU_LIBRARY__: u32 = 6;
  41 | pub const __GLIBC__: u32 = 2;
  42 | pub const __GLIBC_MINOR__: u32 = 40;
  43 | pub const _SYS_CDEFS_H: u32 = 1;
  44 | pub const __glibc_c99_flexarr_available: u32 = 1;
  45 | pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
  46 | pub const __HAVE_GENERIC_SELECTION: u32 = 1;
  47 | pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
  48 | pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
  49 | pub const __GLIBC_USE_IEC_60559_BFP_EXT_C23: u32 = 0;
  50 | pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
  51 | pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
  52 | pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C23: u32 = 0;
  53 | pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
  54 | pub const _BITS_TYPES_H: u32 = 1;
  55 | pub const _BITS_TYPESIZES_H: u32 = 1;
  56 | pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
  57 | pub const __INO_T_MATCHES_INO64_T: u32 = 1;
  58 | pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
  59 | pub const __STATFS_MATCHES_STATFS64: u32 = 1;
  60 | pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
  61 | pub const __FD_SETSIZE: u32 = 1024;
  62 | pub const _BITS_TIME64_H: u32 = 1;
  63 | pub const _BITS_WCHAR_H: u32 = 1;
  64 | pub const _BITS_STDINT_INTN_H: u32 = 1;
  65 | pub const _BITS_STDINT_UINTN_H: u32 = 1;
  66 | pub const _BITS_STDINT_LEAST_H: u32 = 1;
  67 | pub const INT8_MIN: i32 = -128;
  68 | pub const INT16_MIN: i32 = -32768;
  69 | pub const INT32_MIN: i32 = -2147483648;
  70 | pub const INT8_MAX: u32 = 127;
  71 | pub const INT16_MAX: u32 = 32767;
  72 | pub const INT32_MAX: u32 = 2147483647;
  73 | pub const UINT8_MAX: u32 = 255;
  74 | pub const UINT16_MAX: u32 = 65535;
  75 | pub const UINT32_MAX: u32 = 4294967295;
  76 | pub const INT_LEAST8_MIN: i32 = -128;
  77 | pub const INT_LEAST16_MIN: i32 = -32768;
  78 | pub const INT_LEAST32_MIN: i32 = -2147483648;
  79 | pub const INT_LEAST8_MAX: u32 = 127;
  80 | pub const INT_LEAST16_MAX: u32 = 32767;
  81 | pub const INT_LEAST32_MAX: u32 = 2147483647;
  82 | pub const UINT_LEAST8_MAX: u32 = 255;
  83 | pub const UINT_LEAST16_MAX: u32 = 65535;
  84 | pub const UINT_LEAST32_MAX: u32 = 4294967295;
  85 | pub const INT_FAST8_MIN: i32 = -128;
  86 | pub const INT_FAST16_MIN: i64 = -9223372036854775808;
  87 | pub const INT_FAST32_MIN: i64 = -9223372036854775808;
  88 | pub const INT_FAST8_MAX: u32 = 127;
  89 | pub const INT_FAST16_MAX: u64 = 9223372036854775807;
  90 | pub const INT_FAST32_MAX: u64 = 9223372036854775807;
  91 | pub const UINT_FAST8_MAX: u32 = 255;
  92 | pub const UINT_FAST16_MAX: i32 = -1;
  93 | pub const UINT_FAST32_MAX: i32 = -1;
  94 | pub const INTPTR_MIN: i64 = -9223372036854775808;
  95 | pub const INTPTR_MAX: u64 = 9223372036854775807;
  96 | pub const UINTPTR_MAX: i32 = -1;
  97 | pub const PTRDIFF_MIN: i64 = -9223372036854775808;
  98 | pub const PTRDIFF_MAX: u64 = 9223372036854775807;
  99 | pub const SIG_ATOMIC_MIN: i32 = -2147483648;
 100 | pub const SIG_ATOMIC_MAX: u32 = 2147483647;
 101 | pub const SIZE_MAX: i32 = -1;
 102 | pub const WINT_MIN: u32 = 0;
 103 | pub const WINT_MAX: u32 = 4294967295;
 104 | pub const _STDIO_H: u32 = 1;
 105 | pub const _____fpos_t_defined: u32 = 1;
 106 | pub const ____mbstate_t_defined: u32 = 1;
 107 | pub const _____fpos64_t_defined: u32 = 1;
 108 | pub const ____FILE_defined: u32 = 1;
 109 | pub const __FILE_defined: u32 = 1;
 110 | pub const __struct_FILE_defined: u32 = 1;
 111 | pub const _IO_EOF_SEEN: u32 = 16;
 112 | pub const _IO_ERR_SEEN: u32 = 32;
 113 | pub const _IO_USER_LOCK: u32 = 32768;
 114 | pub const __cookie_io_functions_t_defined: u32 = 1;
 115 | pub const _IOFBF: u32 = 0;
 116 | pub const _IOLBF: u32 = 1;
 117 | pub const _IONBF: u32 = 2;
 118 | pub const BUFSIZ: u32 = 8192;
 119 | pub const EOF: i32 = -1;
 120 | pub const SEEK_SET: u32 = 0;
 121 | pub const SEEK_CUR: u32 = 1;
 122 | pub const SEEK_END: u32 = 2;
 123 | pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
 124 | pub const L_tmpnam: u32 = 20;
 125 | pub const TMP_MAX: u32 = 238328;
 126 | pub const _BITS_STDIO_LIM_H: u32 = 1;
 127 | pub const FILENAME_MAX: u32 = 4096;
 128 | pub const L_ctermid: u32 = 9;
 129 | pub const FOPEN_MAX: u32 = 16;
 130 | pub const __HAVE_FLOAT128: u32 = 0;
 131 | pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
 132 | pub const __HAVE_FLOAT64X: u32 = 1;
 133 | pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
 134 | pub const __HAVE_FLOAT16: u32 = 0;
 135 | pub const __HAVE_FLOAT32: u32 = 1;
 136 | pub const __HAVE_FLOAT64: u32 = 1;
 137 | pub const __HAVE_FLOAT32X: u32 = 1;
 138 | pub const __HAVE_FLOAT128X: u32 = 0;
 139 | pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
 140 | pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
 141 | pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
 142 | pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
 143 | pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
 144 | pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
 145 | pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
 146 | pub const GGML_FILE_MAGIC: u32 = 1734831468;
 147 | pub const GGML_FILE_VERSION: u32 = 2;
 148 | pub const GGML_QNT_VERSION: u32 = 2;
 149 | pub const GGML_QNT_VERSION_FACTOR: u32 = 1000;
 150 | pub const GGML_MAX_DIMS: u32 = 4;
 151 | pub const GGML_MAX_PARAMS: u32 = 2048;
 152 | pub const GGML_MAX_SRC: u32 = 10;
 153 | pub const GGML_MAX_N_THREADS: u32 = 512;
 154 | pub const GGML_MAX_OP_PARAMS: u32 = 64;
 155 | pub const GGML_MAX_NAME: u32 = 64;
 156 | pub const GGML_DEFAULT_N_THREADS: u32 = 4;
 157 | pub const GGML_DEFAULT_GRAPH_SIZE: u32 = 2048;
 158 | pub const GGML_MEM_ALIGN: u32 = 16;
 159 | pub const GGML_EXIT_SUCCESS: u32 = 0;
 160 | pub const GGML_EXIT_ABORTED: u32 = 1;
 161 | pub const GGML_ROPE_TYPE_NEOX: u32 = 2;
 162 | pub const GGUF_MAGIC: &[u8; 5] = b"GGUF\0";
 163 | pub const GGUF_VERSION: u32 = 3;
 164 | pub const GGUF_DEFAULT_ALIGNMENT: u32 = 32;
 165 | pub const GGML_KQ_MASK_PAD: u32 = 32;
 166 | pub const GGML_N_TASKS_MAX: i32 = -1;
 167 | pub const WHISPER_SAMPLE_RATE: u32 = 16000;
 168 | pub const WHISPER_N_FFT: u32 = 400;
 169 | pub const WHISPER_HOP_LENGTH: u32 = 160;
 170 | pub const WHISPER_CHUNK_SIZE: u32 = 30;
 171 | pub type wchar_t = ::std::os::raw::c_int;
 172 | #[repr(C)]
 173 | #[repr(align(16))]
 174 | #[derive(Debug, Copy, Clone)]
 175 | pub struct max_align_t {
 176 |     pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
 177 |     pub __bindgen_padding_0: u64,
 178 |     pub __clang_max_align_nonce2: u128,
 179 | }
 180 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 181 | const _: () = {
 182 |     ["Size of max_align_t"][::std::mem::size_of::<max_align_t>() - 32usize];
 183 |     ["Alignment of max_align_t"][::std::mem::align_of::<max_align_t>() - 16usize];
 184 |     ["Offset of field: max_align_t::__clang_max_align_nonce1"]
 185 |         [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
 186 |     ["Offset of field: max_align_t::__clang_max_align_nonce2"]
 187 |         [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
 188 | };
 189 | pub type __u_char = ::std::os::raw::c_uchar;
 190 | pub type __u_short = ::std::os::raw::c_ushort;
 191 | pub type __u_int = ::std::os::raw::c_uint;
 192 | pub type __u_long = ::std::os::raw::c_ulong;
 193 | pub type __int8_t = ::std::os::raw::c_schar;
 194 | pub type __uint8_t = ::std::os::raw::c_uchar;
 195 | pub type __int16_t = ::std::os::raw::c_short;
 196 | pub type __uint16_t = ::std::os::raw::c_ushort;
 197 | pub type __int32_t = ::std::os::raw::c_int;
 198 | pub type __uint32_t = ::std::os::raw::c_uint;
 199 | pub type __int64_t = ::std::os::raw::c_long;
 200 | pub type __uint64_t = ::std::os::raw::c_ulong;
 201 | pub type __int_least8_t = __int8_t;
 202 | pub type __uint_least8_t = __uint8_t;
 203 | pub type __int_least16_t = __int16_t;
 204 | pub type __uint_least16_t = __uint16_t;
 205 | pub type __int_least32_t = __int32_t;
 206 | pub type __uint_least32_t = __uint32_t;
 207 | pub type __int_least64_t = __int64_t;
 208 | pub type __uint_least64_t = __uint64_t;
 209 | pub type __quad_t = ::std::os::raw::c_long;
 210 | pub type __u_quad_t = ::std::os::raw::c_ulong;
 211 | pub type __intmax_t = ::std::os::raw::c_long;
 212 | pub type __uintmax_t = ::std::os::raw::c_ulong;
 213 | pub type __dev_t = ::std::os::raw::c_ulong;
 214 | pub type __uid_t = ::std::os::raw::c_uint;
 215 | pub type __gid_t = ::std::os::raw::c_uint;
 216 | pub type __ino_t = ::std::os::raw::c_ulong;
 217 | pub type __ino64_t = ::std::os::raw::c_ulong;
 218 | pub type __mode_t = ::std::os::raw::c_uint;
 219 | pub type __nlink_t = ::std::os::raw::c_ulong;
 220 | pub type __off_t = ::std::os::raw::c_long;
 221 | pub type __off64_t = ::std::os::raw::c_long;
 222 | pub type __pid_t = ::std::os::raw::c_int;
 223 | #[repr(C)]
 224 | #[derive(Debug, Copy, Clone)]
 225 | pub struct __fsid_t {
 226 |     pub __val: [::std::os::raw::c_int; 2usize],
 227 | }
 228 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 229 | const _: () = {
 230 |     ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
 231 |     ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
 232 |     ["Offset of field: __fsid_t::__val"][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
 233 | };
 234 | pub type __clock_t = ::std::os::raw::c_long;
 235 | pub type __rlim_t = ::std::os::raw::c_ulong;
 236 | pub type __rlim64_t = ::std::os::raw::c_ulong;
 237 | pub type __id_t = ::std::os::raw::c_uint;
 238 | pub type __time_t = ::std::os::raw::c_long;
 239 | pub type __useconds_t = ::std::os::raw::c_uint;
 240 | pub type __suseconds_t = ::std::os::raw::c_long;
 241 | pub type __suseconds64_t = ::std::os::raw::c_long;
 242 | pub type __daddr_t = ::std::os::raw::c_int;
 243 | pub type __key_t = ::std::os::raw::c_int;
 244 | pub type __clockid_t = ::std::os::raw::c_int;
 245 | pub type __timer_t = *mut ::std::os::raw::c_void;
 246 | pub type __blksize_t = ::std::os::raw::c_long;
 247 | pub type __blkcnt_t = ::std::os::raw::c_long;
 248 | pub type __blkcnt64_t = ::std::os::raw::c_long;
 249 | pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
 250 | pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
 251 | pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
 252 | pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
 253 | pub type __fsword_t = ::std::os::raw::c_long;
 254 | pub type __ssize_t = ::std::os::raw::c_long;
 255 | pub type __syscall_slong_t = ::std::os::raw::c_long;
 256 | pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
 257 | pub type __loff_t = __off64_t;
 258 | pub type __caddr_t = *mut ::std::os::raw::c_char;
 259 | pub type __intptr_t = ::std::os::raw::c_long;
 260 | pub type __socklen_t = ::std::os::raw::c_uint;
 261 | pub type __sig_atomic_t = ::std::os::raw::c_int;
 262 | pub type int_least8_t = __int_least8_t;
 263 | pub type int_least16_t = __int_least16_t;
 264 | pub type int_least32_t = __int_least32_t;
 265 | pub type int_least64_t = __int_least64_t;
 266 | pub type uint_least8_t = __uint_least8_t;
 267 | pub type uint_least16_t = __uint_least16_t;
 268 | pub type uint_least32_t = __uint_least32_t;
 269 | pub type uint_least64_t = __uint_least64_t;
 270 | pub type int_fast8_t = ::std::os::raw::c_schar;
 271 | pub type int_fast16_t = ::std::os::raw::c_long;
 272 | pub type int_fast32_t = ::std::os::raw::c_long;
 273 | pub type int_fast64_t = ::std::os::raw::c_long;
 274 | pub type uint_fast8_t = ::std::os::raw::c_uchar;
 275 | pub type uint_fast16_t = ::std::os::raw::c_ulong;
 276 | pub type uint_fast32_t = ::std::os::raw::c_ulong;
 277 | pub type uint_fast64_t = ::std::os::raw::c_ulong;
 278 | pub type intmax_t = __intmax_t;
 279 | pub type uintmax_t = __uintmax_t;
 280 | pub type __gnuc_va_list = __builtin_va_list;
 281 | #[repr(C)]
 282 | #[derive(Copy, Clone)]
 283 | pub struct __mbstate_t {
 284 |     pub __count: ::std::os::raw::c_int,
 285 |     pub __value: __mbstate_t__bindgen_ty_1,
 286 | }
 287 | #[repr(C)]
 288 | #[derive(Copy, Clone)]
 289 | pub union __mbstate_t__bindgen_ty_1 {
 290 |     pub __wch: ::std::os::raw::c_uint,
 291 |     pub __wchb: [::std::os::raw::c_char; 4usize],
 292 | }
 293 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 294 | const _: () = {
 295 |     ["Size of __mbstate_t__bindgen_ty_1"]
 296 |         [::std::mem::size_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
 297 |     ["Alignment of __mbstate_t__bindgen_ty_1"]
 298 |         [::std::mem::align_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
 299 |     ["Offset of field: __mbstate_t__bindgen_ty_1::__wch"]
 300 |         [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wch) - 0usize];
 301 |     ["Offset of field: __mbstate_t__bindgen_ty_1::__wchb"]
 302 |         [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wchb) - 0usize];
 303 | };
 304 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 305 | const _: () = {
 306 |     ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 8usize];
 307 |     ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 4usize];
 308 |     ["Offset of field: __mbstate_t::__count"]
 309 |         [::std::mem::offset_of!(__mbstate_t, __count) - 0usize];
 310 |     ["Offset of field: __mbstate_t::__value"]
 311 |         [::std::mem::offset_of!(__mbstate_t, __value) - 4usize];
 312 | };
 313 | #[repr(C)]
 314 | #[derive(Copy, Clone)]
 315 | pub struct _G_fpos_t {
 316 |     pub __pos: __off_t,
 317 |     pub __state: __mbstate_t,
 318 | }
 319 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 320 | const _: () = {
 321 |     ["Size of _G_fpos_t"][::std::mem::size_of::<_G_fpos_t>() - 16usize];
 322 |     ["Alignment of _G_fpos_t"][::std::mem::align_of::<_G_fpos_t>() - 8usize];
 323 |     ["Offset of field: _G_fpos_t::__pos"][::std::mem::offset_of!(_G_fpos_t, __pos) - 0usize];
 324 |     ["Offset of field: _G_fpos_t::__state"][::std::mem::offset_of!(_G_fpos_t, __state) - 8usize];
 325 | };
 326 | pub type __fpos_t = _G_fpos_t;
 327 | #[repr(C)]
 328 | #[derive(Copy, Clone)]
 329 | pub struct _G_fpos64_t {
 330 |     pub __pos: __off64_t,
 331 |     pub __state: __mbstate_t,
 332 | }
 333 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 334 | const _: () = {
 335 |     ["Size of _G_fpos64_t"][::std::mem::size_of::<_G_fpos64_t>() - 16usize];
 336 |     ["Alignment of _G_fpos64_t"][::std::mem::align_of::<_G_fpos64_t>() - 8usize];
 337 |     ["Offset of field: _G_fpos64_t::__pos"][::std::mem::offset_of!(_G_fpos64_t, __pos) - 0usize];
 338 |     ["Offset of field: _G_fpos64_t::__state"]
 339 |         [::std::mem::offset_of!(_G_fpos64_t, __state) - 8usize];
 340 | };
 341 | pub type __fpos64_t = _G_fpos64_t;
 342 | pub type __FILE = _IO_FILE;
 343 | pub type FILE = _IO_FILE;
 344 | #[repr(C)]
 345 | #[derive(Debug, Copy, Clone)]
 346 | pub struct _IO_marker {
 347 |     _unused: [u8; 0],
 348 | }
 349 | #[repr(C)]
 350 | #[derive(Debug, Copy, Clone)]
 351 | pub struct _IO_codecvt {
 352 |     _unused: [u8; 0],
 353 | }
 354 | #[repr(C)]
 355 | #[derive(Debug, Copy, Clone)]
 356 | pub struct _IO_wide_data {
 357 |     _unused: [u8; 0],
 358 | }
 359 | pub type _IO_lock_t = ::std::os::raw::c_void;
 360 | #[repr(C)]
 361 | #[derive(Debug, Copy, Clone)]
 362 | pub struct _IO_FILE {
 363 |     pub _flags: ::std::os::raw::c_int,
 364 |     pub _IO_read_ptr: *mut ::std::os::raw::c_char,
 365 |     pub _IO_read_end: *mut ::std::os::raw::c_char,
 366 |     pub _IO_read_base: *mut ::std::os::raw::c_char,
 367 |     pub _IO_write_base: *mut ::std::os::raw::c_char,
 368 |     pub _IO_write_ptr: *mut ::std::os::raw::c_char,
 369 |     pub _IO_write_end: *mut ::std::os::raw::c_char,
 370 |     pub _IO_buf_base: *mut ::std::os::raw::c_char,
 371 |     pub _IO_buf_end: *mut ::std::os::raw::c_char,
 372 |     pub _IO_save_base: *mut ::std::os::raw::c_char,
 373 |     pub _IO_backup_base: *mut ::std::os::raw::c_char,
 374 |     pub _IO_save_end: *mut ::std::os::raw::c_char,
 375 |     pub _markers: *mut _IO_marker,
 376 |     pub _chain: *mut _IO_FILE,
 377 |     pub _fileno: ::std::os::raw::c_int,
 378 |     pub _flags2: ::std::os::raw::c_int,
 379 |     pub _old_offset: __off_t,
 380 |     pub _cur_column: ::std::os::raw::c_ushort,
 381 |     pub _vtable_offset: ::std::os::raw::c_schar,
 382 |     pub _shortbuf: [::std::os::raw::c_char; 1usize],
 383 |     pub _lock: *mut _IO_lock_t,
 384 |     pub _offset: __off64_t,
 385 |     pub _codecvt: *mut _IO_codecvt,
 386 |     pub _wide_data: *mut _IO_wide_data,
 387 |     pub _freeres_list: *mut _IO_FILE,
 388 |     pub _freeres_buf: *mut ::std::os::raw::c_void,
 389 |     pub _prevchain: *mut *mut _IO_FILE,
 390 |     pub _mode: ::std::os::raw::c_int,
 391 |     pub _unused2: [::std::os::raw::c_char; 20usize],
 392 | }
 393 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 394 | const _: () = {
 395 |     ["Size of _IO_FILE"][::std::mem::size_of::<_IO_FILE>() - 216usize];
 396 |     ["Alignment of _IO_FILE"][::std::mem::align_of::<_IO_FILE>() - 8usize];
 397 |     ["Offset of field: _IO_FILE::_flags"][::std::mem::offset_of!(_IO_FILE, _flags) - 0usize];
 398 |     ["Offset of field: _IO_FILE::_IO_read_ptr"]
 399 |         [::std::mem::offset_of!(_IO_FILE, _IO_read_ptr) - 8usize];
 400 |     ["Offset of field: _IO_FILE::_IO_read_end"]
 401 |         [::std::mem::offset_of!(_IO_FILE, _IO_read_end) - 16usize];
 402 |     ["Offset of field: _IO_FILE::_IO_read_base"]
 403 |         [::std::mem::offset_of!(_IO_FILE, _IO_read_base) - 24usize];
 404 |     ["Offset of field: _IO_FILE::_IO_write_base"]
 405 |         [::std::mem::offset_of!(_IO_FILE, _IO_write_base) - 32usize];
 406 |     ["Offset of field: _IO_FILE::_IO_write_ptr"]
 407 |         [::std::mem::offset_of!(_IO_FILE, _IO_write_ptr) - 40usize];
 408 |     ["Offset of field: _IO_FILE::_IO_write_end"]
 409 |         [::std::mem::offset_of!(_IO_FILE, _IO_write_end) - 48usize];
 410 |     ["Offset of field: _IO_FILE::_IO_buf_base"]
 411 |         [::std::mem::offset_of!(_IO_FILE, _IO_buf_base) - 56usize];
 412 |     ["Offset of field: _IO_FILE::_IO_buf_end"]
 413 |         [::std::mem::offset_of!(_IO_FILE, _IO_buf_end) - 64usize];
 414 |     ["Offset of field: _IO_FILE::_IO_save_base"]
 415 |         [::std::mem::offset_of!(_IO_FILE, _IO_save_base) - 72usize];
 416 |     ["Offset of field: _IO_FILE::_IO_backup_base"]
 417 |         [::std::mem::offset_of!(_IO_FILE, _IO_backup_base) - 80usize];
 418 |     ["Offset of field: _IO_FILE::_IO_save_end"]
 419 |         [::std::mem::offset_of!(_IO_FILE, _IO_save_end) - 88usize];
 420 |     ["Offset of field: _IO_FILE::_markers"][::std::mem::offset_of!(_IO_FILE, _markers) - 96usize];
 421 |     ["Offset of field: _IO_FILE::_chain"][::std::mem::offset_of!(_IO_FILE, _chain) - 104usize];
 422 |     ["Offset of field: _IO_FILE::_fileno"][::std::mem::offset_of!(_IO_FILE, _fileno) - 112usize];
 423 |     ["Offset of field: _IO_FILE::_flags2"][::std::mem::offset_of!(_IO_FILE, _flags2) - 116usize];
 424 |     ["Offset of field: _IO_FILE::_old_offset"]
 425 |         [::std::mem::offset_of!(_IO_FILE, _old_offset) - 120usize];
 426 |     ["Offset of field: _IO_FILE::_cur_column"]
 427 |         [::std::mem::offset_of!(_IO_FILE, _cur_column) - 128usize];
 428 |     ["Offset of field: _IO_FILE::_vtable_offset"]
 429 |         [::std::mem::offset_of!(_IO_FILE, _vtable_offset) - 130usize];
 430 |     ["Offset of field: _IO_FILE::_shortbuf"]
 431 |         [::std::mem::offset_of!(_IO_FILE, _shortbuf) - 131usize];
 432 |     ["Offset of field: _IO_FILE::_lock"][::std::mem::offset_of!(_IO_FILE, _lock) - 136usize];
 433 |     ["Offset of field: _IO_FILE::_offset"][::std::mem::offset_of!(_IO_FILE, _offset) - 144usize];
 434 |     ["Offset of field: _IO_FILE::_codecvt"][::std::mem::offset_of!(_IO_FILE, _codecvt) - 152usize];
 435 |     ["Offset of field: _IO_FILE::_wide_data"]
 436 |         [::std::mem::offset_of!(_IO_FILE, _wide_data) - 160usize];
 437 |     ["Offset of field: _IO_FILE::_freeres_list"]
 438 |         [::std::mem::offset_of!(_IO_FILE, _freeres_list) - 168usize];
 439 |     ["Offset of field: _IO_FILE::_freeres_buf"]
 440 |         [::std::mem::offset_of!(_IO_FILE, _freeres_buf) - 176usize];
 441 |     ["Offset of field: _IO_FILE::_prevchain"]
 442 |         [::std::mem::offset_of!(_IO_FILE, _prevchain) - 184usize];
 443 |     ["Offset of field: _IO_FILE::_mode"][::std::mem::offset_of!(_IO_FILE, _mode) - 192usize];
 444 |     ["Offset of field: _IO_FILE::_unused2"][::std::mem::offset_of!(_IO_FILE, _unused2) - 196usize];
 445 | };
 446 | pub type cookie_read_function_t = ::std::option::Option<
 447 |     unsafe extern "C" fn(
 448 |         __cookie: *mut ::std::os::raw::c_void,
 449 |         __buf: *mut ::std::os::raw::c_char,
 450 |         __nbytes: usize,
 451 |     ) -> __ssize_t,
 452 | >;
 453 | pub type cookie_write_function_t = ::std::option::Option<
 454 |     unsafe extern "C" fn(
 455 |         __cookie: *mut ::std::os::raw::c_void,
 456 |         __buf: *const ::std::os::raw::c_char,
 457 |         __nbytes: usize,
 458 |     ) -> __ssize_t,
 459 | >;
 460 | pub type cookie_seek_function_t = ::std::option::Option<
 461 |     unsafe extern "C" fn(
 462 |         __cookie: *mut ::std::os::raw::c_void,
 463 |         __pos: *mut __off64_t,
 464 |         __w: ::std::os::raw::c_int,
 465 |     ) -> ::std::os::raw::c_int,
 466 | >;
 467 | pub type cookie_close_function_t = ::std::option::Option<
 468 |     unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
 469 | >;
 470 | #[repr(C)]
 471 | #[derive(Debug, Copy, Clone)]
 472 | pub struct _IO_cookie_io_functions_t {
 473 |     pub read: cookie_read_function_t,
 474 |     pub write: cookie_write_function_t,
 475 |     pub seek: cookie_seek_function_t,
 476 |     pub close: cookie_close_function_t,
 477 | }
 478 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
 479 | const _: () = {
 480 |     ["Size of _IO_cookie_io_functions_t"]
 481 |         [::std::mem::size_of::<_IO_cookie_io_functions_t>() - 32usize];
 482 |     ["Alignment of _IO_cookie_io_functions_t"]
 483 |         [::std::mem::align_of::<_IO_cookie_io_functions_t>() - 8usize];
 484 |     ["Offset of field: _IO_cookie_io_functions_t::read"]
 485 |         [::std::mem::offset_of!(_IO_cookie_io_functions_t, read) - 0usize];
 486 |     ["Offset of field: _IO_cookie_io_functions_t::write"]
 487 |         [::std::mem::offset_of!(_IO_cookie_io_functions_t, write) - 8usize];
 488 |     ["Offset of field: _IO_cookie_io_functions_t::seek"]
 489 |         [::std::mem::offset_of!(_IO_cookie_io_functions_t, seek) - 16usize];
 490 |     ["Offset of field: _IO_cookie_io_functions_t::close"]
 491 |         [::std::mem::offset_of!(_IO_cookie_io_functions_t, close) - 24usize];
 492 | };
 493 | pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
 494 | pub type va_list = __gnuc_va_list;
 495 | pub type off_t = __off_t;
 496 | pub type fpos_t = __fpos_t;
 497 | extern "C" {
 498 |     pub static mut stdin: *mut FILE;
 499 | }
 500 | extern "C" {
 501 |     pub static mut stdout: *mut FILE;
 502 | }
 503 | extern "C" {
 504 |     pub static mut stderr: *mut FILE;
 505 | }
 506 | extern "C" {
 507 |     pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
 508 | }
 509 | extern "C" {
 510 |     pub fn rename(
 511 |         __old: *const ::std::os::raw::c_char,
 512 |         __new: *const ::std::os::raw::c_char,
 513 |     ) -> ::std::os::raw::c_int;
 514 | }
 515 | extern "C" {
 516 |     pub fn renameat(
 517 |         __oldfd: ::std::os::raw::c_int,
 518 |         __old: *const ::std::os::raw::c_char,
 519 |         __newfd: ::std::os::raw::c_int,
 520 |         __new: *const ::std::os::raw::c_char,
 521 |     ) -> ::std::os::raw::c_int;
 522 | }
 523 | extern "C" {
 524 |     pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
 525 | }
 526 | extern "C" {
 527 |     pub fn tmpfile() -> *mut FILE;
 528 | }
 529 | extern "C" {
 530 |     pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
 531 | }
 532 | extern "C" {
 533 |     pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
 534 | }
 535 | extern "C" {
 536 |     pub fn tempnam(
 537 |         __dir: *const ::std::os::raw::c_char,
 538 |         __pfx: *const ::std::os::raw::c_char,
 539 |     ) -> *mut ::std::os::raw::c_char;
 540 | }
 541 | extern "C" {
 542 |     pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
 543 | }
 544 | extern "C" {
 545 |     pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 546 | }
 547 | extern "C" {
 548 |     pub fn fopen(
 549 |         __filename: *const ::std::os::raw::c_char,
 550 |         __modes: *const ::std::os::raw::c_char,
 551 |     ) -> *mut FILE;
 552 | }
 553 | extern "C" {
 554 |     pub fn freopen(
 555 |         __filename: *const ::std::os::raw::c_char,
 556 |         __modes: *const ::std::os::raw::c_char,
 557 |         __stream: *mut FILE,
 558 |     ) -> *mut FILE;
 559 | }
 560 | extern "C" {
 561 |     pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
 562 |         -> *mut FILE;
 563 | }
 564 | extern "C" {
 565 |     pub fn fopencookie(
 566 |         __magic_cookie: *mut ::std::os::raw::c_void,
 567 |         __modes: *const ::std::os::raw::c_char,
 568 |         __io_funcs: cookie_io_functions_t,
 569 |     ) -> *mut FILE;
 570 | }
 571 | extern "C" {
 572 |     pub fn fmemopen(
 573 |         __s: *mut ::std::os::raw::c_void,
 574 |         __len: usize,
 575 |         __modes: *const ::std::os::raw::c_char,
 576 |     ) -> *mut FILE;
 577 | }
 578 | extern "C" {
 579 |     pub fn open_memstream(
 580 |         __bufloc: *mut *mut ::std::os::raw::c_char,
 581 |         __sizeloc: *mut usize,
 582 |     ) -> *mut FILE;
 583 | }
 584 | extern "C" {
 585 |     pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
 586 | }
 587 | extern "C" {
 588 |     pub fn setvbuf(
 589 |         __stream: *mut FILE,
 590 |         __buf: *mut ::std::os::raw::c_char,
 591 |         __modes: ::std::os::raw::c_int,
 592 |         __n: usize,
 593 |     ) -> ::std::os::raw::c_int;
 594 | }
 595 | extern "C" {
 596 |     pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
 597 | }
 598 | extern "C" {
 599 |     pub fn setlinebuf(__stream: *mut FILE);
 600 | }
 601 | extern "C" {
 602 |     pub fn fprintf(
 603 |         __stream: *mut FILE,
 604 |         __format: *const ::std::os::raw::c_char,
 605 |         ...
 606 |     ) -> ::std::os::raw::c_int;
 607 | }
 608 | extern "C" {
 609 |     pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
 610 | }
 611 | extern "C" {
 612 |     pub fn sprintf(
 613 |         __s: *mut ::std::os::raw::c_char,
 614 |         __format: *const ::std::os::raw::c_char,
 615 |         ...
 616 |     ) -> ::std::os::raw::c_int;
 617 | }
 618 | extern "C" {
 619 |     pub fn vfprintf(
 620 |         __s: *mut FILE,
 621 |         __format: *const ::std::os::raw::c_char,
 622 |         __arg: *mut __va_list_tag,
 623 |     ) -> ::std::os::raw::c_int;
 624 | }
 625 | extern "C" {
 626 |     pub fn vprintf(
 627 |         __format: *const ::std::os::raw::c_char,
 628 |         __arg: *mut __va_list_tag,
 629 |     ) -> ::std::os::raw::c_int;
 630 | }
 631 | extern "C" {
 632 |     pub fn vsprintf(
 633 |         __s: *mut ::std::os::raw::c_char,
 634 |         __format: *const ::std::os::raw::c_char,
 635 |         __arg: *mut __va_list_tag,
 636 |     ) -> ::std::os::raw::c_int;
 637 | }
 638 | extern "C" {
 639 |     pub fn snprintf(
 640 |         __s: *mut ::std::os::raw::c_char,
 641 |         __maxlen: ::std::os::raw::c_ulong,
 642 |         __format: *const ::std::os::raw::c_char,
 643 |         ...
 644 |     ) -> ::std::os::raw::c_int;
 645 | }
 646 | extern "C" {
 647 |     pub fn vsnprintf(
 648 |         __s: *mut ::std::os::raw::c_char,
 649 |         __maxlen: ::std::os::raw::c_ulong,
 650 |         __format: *const ::std::os::raw::c_char,
 651 |         __arg: *mut __va_list_tag,
 652 |     ) -> ::std::os::raw::c_int;
 653 | }
 654 | extern "C" {
 655 |     pub fn vasprintf(
 656 |         __ptr: *mut *mut ::std::os::raw::c_char,
 657 |         __f: *const ::std::os::raw::c_char,
 658 |         __arg: *mut __va_list_tag,
 659 |     ) -> ::std::os::raw::c_int;
 660 | }
 661 | extern "C" {
 662 |     pub fn __asprintf(
 663 |         __ptr: *mut *mut ::std::os::raw::c_char,
 664 |         __fmt: *const ::std::os::raw::c_char,
 665 |         ...
 666 |     ) -> ::std::os::raw::c_int;
 667 | }
 668 | extern "C" {
 669 |     pub fn asprintf(
 670 |         __ptr: *mut *mut ::std::os::raw::c_char,
 671 |         __fmt: *const ::std::os::raw::c_char,
 672 |         ...
 673 |     ) -> ::std::os::raw::c_int;
 674 | }
 675 | extern "C" {
 676 |     pub fn vdprintf(
 677 |         __fd: ::std::os::raw::c_int,
 678 |         __fmt: *const ::std::os::raw::c_char,
 679 |         __arg: *mut __va_list_tag,
 680 |     ) -> ::std::os::raw::c_int;
 681 | }
 682 | extern "C" {
 683 |     pub fn dprintf(
 684 |         __fd: ::std::os::raw::c_int,
 685 |         __fmt: *const ::std::os::raw::c_char,
 686 |         ...
 687 |     ) -> ::std::os::raw::c_int;
 688 | }
 689 | extern "C" {
 690 |     pub fn fscanf(
 691 |         __stream: *mut FILE,
 692 |         __format: *const ::std::os::raw::c_char,
 693 |         ...
 694 |     ) -> ::std::os::raw::c_int;
 695 | }
 696 | extern "C" {
 697 |     pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
 698 | }
 699 | extern "C" {
 700 |     pub fn sscanf(
 701 |         __s: *const ::std::os::raw::c_char,
 702 |         __format: *const ::std::os::raw::c_char,
 703 |         ...
 704 |     ) -> ::std::os::raw::c_int;
 705 | }
 706 | pub type _Float32 = f32;
 707 | pub type _Float64 = f64;
 708 | pub type _Float32x = f64;
 709 | pub type _Float64x = u128;
 710 | extern "C" {
 711 |     #[link_name = "\u{1}__isoc99_fscanf"]
 712 |     pub fn fscanf1(
 713 |         __stream: *mut FILE,
 714 |         __format: *const ::std::os::raw::c_char,
 715 |         ...
 716 |     ) -> ::std::os::raw::c_int;
 717 | }
 718 | extern "C" {
 719 |     #[link_name = "\u{1}__isoc99_scanf"]
 720 |     pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
 721 | }
 722 | extern "C" {
 723 |     #[link_name = "\u{1}__isoc99_sscanf"]
 724 |     pub fn sscanf1(
 725 |         __s: *const ::std::os::raw::c_char,
 726 |         __format: *const ::std::os::raw::c_char,
 727 |         ...
 728 |     ) -> ::std::os::raw::c_int;
 729 | }
 730 | extern "C" {
 731 |     pub fn vfscanf(
 732 |         __s: *mut FILE,
 733 |         __format: *const ::std::os::raw::c_char,
 734 |         __arg: *mut __va_list_tag,
 735 |     ) -> ::std::os::raw::c_int;
 736 | }
 737 | extern "C" {
 738 |     pub fn vscanf(
 739 |         __format: *const ::std::os::raw::c_char,
 740 |         __arg: *mut __va_list_tag,
 741 |     ) -> ::std::os::raw::c_int;
 742 | }
 743 | extern "C" {
 744 |     pub fn vsscanf(
 745 |         __s: *const ::std::os::raw::c_char,
 746 |         __format: *const ::std::os::raw::c_char,
 747 |         __arg: *mut __va_list_tag,
 748 |     ) -> ::std::os::raw::c_int;
 749 | }
 750 | extern "C" {
 751 |     #[link_name = "\u{1}__isoc99_vfscanf"]
 752 |     pub fn vfscanf1(
 753 |         __s: *mut FILE,
 754 |         __format: *const ::std::os::raw::c_char,
 755 |         __arg: *mut __va_list_tag,
 756 |     ) -> ::std::os::raw::c_int;
 757 | }
 758 | extern "C" {
 759 |     #[link_name = "\u{1}__isoc99_vscanf"]
 760 |     pub fn vscanf1(
 761 |         __format: *const ::std::os::raw::c_char,
 762 |         __arg: *mut __va_list_tag,
 763 |     ) -> ::std::os::raw::c_int;
 764 | }
 765 | extern "C" {
 766 |     #[link_name = "\u{1}__isoc99_vsscanf"]
 767 |     pub fn vsscanf1(
 768 |         __s: *const ::std::os::raw::c_char,
 769 |         __format: *const ::std::os::raw::c_char,
 770 |         __arg: *mut __va_list_tag,
 771 |     ) -> ::std::os::raw::c_int;
 772 | }
 773 | extern "C" {
 774 |     pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
 775 | }
 776 | extern "C" {
 777 |     pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
 778 | }
 779 | extern "C" {
 780 |     pub fn getchar() -> ::std::os::raw::c_int;
 781 | }
 782 | extern "C" {
 783 |     pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 784 | }
 785 | extern "C" {
 786 |     pub fn getchar_unlocked() -> ::std::os::raw::c_int;
 787 | }
 788 | extern "C" {
 789 |     pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 790 | }
 791 | extern "C" {
 792 |     pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
 793 | }
 794 | extern "C" {
 795 |     pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
 796 | }
 797 | extern "C" {
 798 |     pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
 799 | }
 800 | extern "C" {
 801 |     pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
 802 |         -> ::std::os::raw::c_int;
 803 | }
 804 | extern "C" {
 805 |     pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
 806 | }
 807 | extern "C" {
 808 |     pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
 809 | }
 810 | extern "C" {
 811 |     pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
 812 | }
 813 | extern "C" {
 814 |     pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
 815 | }
 816 | extern "C" {
 817 |     pub fn fgets(
 818 |         __s: *mut ::std::os::raw::c_char,
 819 |         __n: ::std::os::raw::c_int,
 820 |         __stream: *mut FILE,
 821 |     ) -> *mut ::std::os::raw::c_char;
 822 | }
 823 | extern "C" {
 824 |     pub fn __getdelim(
 825 |         __lineptr: *mut *mut ::std::os::raw::c_char,
 826 |         __n: *mut usize,
 827 |         __delimiter: ::std::os::raw::c_int,
 828 |         __stream: *mut FILE,
 829 |     ) -> __ssize_t;
 830 | }
 831 | extern "C" {
 832 |     pub fn getdelim(
 833 |         __lineptr: *mut *mut ::std::os::raw::c_char,
 834 |         __n: *mut usize,
 835 |         __delimiter: ::std::os::raw::c_int,
 836 |         __stream: *mut FILE,
 837 |     ) -> __ssize_t;
 838 | }
 839 | extern "C" {
 840 |     pub fn getline(
 841 |         __lineptr: *mut *mut ::std::os::raw::c_char,
 842 |         __n: *mut usize,
 843 |         __stream: *mut FILE,
 844 |     ) -> __ssize_t;
 845 | }
 846 | extern "C" {
 847 |     pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
 848 | }
 849 | extern "C" {
 850 |     pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
 851 | }
 852 | extern "C" {
 853 |     pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
 854 | }
 855 | extern "C" {
 856 |     pub fn fread(
 857 |         __ptr: *mut ::std::os::raw::c_void,
 858 |         __size: ::std::os::raw::c_ulong,
 859 |         __n: ::std::os::raw::c_ulong,
 860 |         __stream: *mut FILE,
 861 |     ) -> ::std::os::raw::c_ulong;
 862 | }
 863 | extern "C" {
 864 |     pub fn fwrite(
 865 |         __ptr: *const ::std::os::raw::c_void,
 866 |         __size: ::std::os::raw::c_ulong,
 867 |         __n: ::std::os::raw::c_ulong,
 868 |         __s: *mut FILE,
 869 |     ) -> ::std::os::raw::c_ulong;
 870 | }
 871 | extern "C" {
 872 |     pub fn fread_unlocked(
 873 |         __ptr: *mut ::std::os::raw::c_void,
 874 |         __size: usize,
 875 |         __n: usize,
 876 |         __stream: *mut FILE,
 877 |     ) -> usize;
 878 | }
 879 | extern "C" {
 880 |     pub fn fwrite_unlocked(
 881 |         __ptr: *const ::std::os::raw::c_void,
 882 |         __size: usize,
 883 |         __n: usize,
 884 |         __stream: *mut FILE,
 885 |     ) -> usize;
 886 | }
 887 | extern "C" {
 888 |     pub fn fseek(
 889 |         __stream: *mut FILE,
 890 |         __off: ::std::os::raw::c_long,
 891 |         __whence: ::std::os::raw::c_int,
 892 |     ) -> ::std::os::raw::c_int;
 893 | }
 894 | extern "C" {
 895 |     pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
 896 | }
 897 | extern "C" {
 898 |     pub fn rewind(__stream: *mut FILE);
 899 | }
 900 | extern "C" {
 901 |     pub fn fseeko(
 902 |         __stream: *mut FILE,
 903 |         __off: __off_t,
 904 |         __whence: ::std::os::raw::c_int,
 905 |     ) -> ::std::os::raw::c_int;
 906 | }
 907 | extern "C" {
 908 |     pub fn ftello(__stream: *mut FILE) -> __off_t;
 909 | }
 910 | extern "C" {
 911 |     pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
 912 | }
 913 | extern "C" {
 914 |     pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
 915 | }
 916 | extern "C" {
 917 |     pub fn clearerr(__stream: *mut FILE);
 918 | }
 919 | extern "C" {
 920 |     pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
 921 | }
 922 | extern "C" {
 923 |     pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
 924 | }
 925 | extern "C" {
 926 |     pub fn clearerr_unlocked(__stream: *mut FILE);
 927 | }
 928 | extern "C" {
 929 |     pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 930 | }
 931 | extern "C" {
 932 |     pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 933 | }
 934 | extern "C" {
 935 |     pub fn perror(__s: *const ::std::os::raw::c_char);
 936 | }
 937 | extern "C" {
 938 |     pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
 939 | }
 940 | extern "C" {
 941 |     pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
 942 | }
 943 | extern "C" {
 944 |     pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
 945 | }
 946 | extern "C" {
 947 |     pub fn popen(
 948 |         __command: *const ::std::os::raw::c_char,
 949 |         __modes: *const ::std::os::raw::c_char,
 950 |     ) -> *mut FILE;
 951 | }
 952 | extern "C" {
 953 |     pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
 954 | }
 955 | extern "C" {
 956 |     pub fn flockfile(__stream: *mut FILE);
 957 | }
 958 | extern "C" {
 959 |     pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
 960 | }
 961 | extern "C" {
 962 |     pub fn funlockfile(__stream: *mut FILE);
 963 | }
 964 | extern "C" {
 965 |     pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
 966 | }
 967 | extern "C" {
 968 |     pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
 969 | }
 970 | extern "C" {
 971 |     pub fn ggml_abort(
 972 |         file: *const ::std::os::raw::c_char,
 973 |         line: ::std::os::raw::c_int,
 974 |         fmt: *const ::std::os::raw::c_char,
 975 |         ...
 976 |     );
 977 | }
 978 | pub const ggml_status_GGML_STATUS_ALLOC_FAILED: ggml_status = -2;
 979 | pub const ggml_status_GGML_STATUS_FAILED: ggml_status = -1;
 980 | pub const ggml_status_GGML_STATUS_SUCCESS: ggml_status = 0;
 981 | pub const ggml_status_GGML_STATUS_ABORTED: ggml_status = 1;
 982 | pub type ggml_status = ::std::os::raw::c_int;
 983 | extern "C" {
 984 |     pub fn ggml_status_to_string(status: ggml_status) -> *const ::std::os::raw::c_char;
 985 | }
 986 | pub type ggml_fp16_t = u16;
 987 | extern "C" {
 988 |     pub fn ggml_fp16_to_fp32(arg1: ggml_fp16_t) -> f32;
 989 | }
 990 | extern "C" {
 991 |     pub fn ggml_fp32_to_fp16(arg1: f32) -> ggml_fp16_t;
 992 | }
 993 | extern "C" {
 994 |     pub fn ggml_fp16_to_fp32_row(arg1: *const ggml_fp16_t, arg2: *mut f32, arg3: i64);
 995 | }
 996 | extern "C" {
 997 |     pub fn ggml_fp32_to_fp16_row(arg1: *const f32, arg2: *mut ggml_fp16_t, arg3: i64);
 998 | }
 999 | #[repr(C)]
1000 | #[derive(Debug, Copy, Clone)]
1001 | pub struct ggml_bf16_t {
1002 |     pub bits: u16,
1003 | }
1004 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1005 | const _: () = {
1006 |     ["Size of ggml_bf16_t"][::std::mem::size_of::<ggml_bf16_t>() - 2usize];
1007 |     ["Alignment of ggml_bf16_t"][::std::mem::align_of::<ggml_bf16_t>() - 2usize];
1008 |     ["Offset of field: ggml_bf16_t::bits"][::std::mem::offset_of!(ggml_bf16_t, bits) - 0usize];
1009 | };
1010 | extern "C" {
1011 |     pub fn ggml_fp32_to_bf16(arg1: f32) -> ggml_bf16_t;
1012 | }
1013 | extern "C" {
1014 |     pub fn ggml_bf16_to_fp32(arg1: ggml_bf16_t) -> f32;
1015 | }
1016 | extern "C" {
1017 |     pub fn ggml_bf16_to_fp32_row(arg1: *const ggml_bf16_t, arg2: *mut f32, arg3: i64);
1018 | }
1019 | extern "C" {
1020 |     pub fn ggml_fp32_to_bf16_row_ref(arg1: *const f32, arg2: *mut ggml_bf16_t, arg3: i64);
1021 | }
1022 | extern "C" {
1023 |     pub fn ggml_fp32_to_bf16_row(arg1: *const f32, arg2: *mut ggml_bf16_t, arg3: i64);
1024 | }
1025 | #[repr(C)]
1026 | #[derive(Debug, Copy, Clone)]
1027 | pub struct ggml_object {
1028 |     _unused: [u8; 0],
1029 | }
1030 | #[repr(C)]
1031 | #[derive(Debug, Copy, Clone)]
1032 | pub struct ggml_context {
1033 |     _unused: [u8; 0],
1034 | }
1035 | #[repr(C)]
1036 | #[derive(Debug, Copy, Clone)]
1037 | pub struct ggml_cgraph {
1038 |     _unused: [u8; 0],
1039 | }
1040 | pub const ggml_type_GGML_TYPE_F32: ggml_type = 0;
1041 | pub const ggml_type_GGML_TYPE_F16: ggml_type = 1;
1042 | pub const ggml_type_GGML_TYPE_Q4_0: ggml_type = 2;
1043 | pub const ggml_type_GGML_TYPE_Q4_1: ggml_type = 3;
1044 | pub const ggml_type_GGML_TYPE_Q5_0: ggml_type = 6;
1045 | pub const ggml_type_GGML_TYPE_Q5_1: ggml_type = 7;
1046 | pub const ggml_type_GGML_TYPE_Q8_0: ggml_type = 8;
1047 | pub const ggml_type_GGML_TYPE_Q8_1: ggml_type = 9;
1048 | pub const ggml_type_GGML_TYPE_Q2_K: ggml_type = 10;
1049 | pub const ggml_type_GGML_TYPE_Q3_K: ggml_type = 11;
1050 | pub const ggml_type_GGML_TYPE_Q4_K: ggml_type = 12;
1051 | pub const ggml_type_GGML_TYPE_Q5_K: ggml_type = 13;
1052 | pub const ggml_type_GGML_TYPE_Q6_K: ggml_type = 14;
1053 | pub const ggml_type_GGML_TYPE_Q8_K: ggml_type = 15;
1054 | pub const ggml_type_GGML_TYPE_IQ2_XXS: ggml_type = 16;
1055 | pub const ggml_type_GGML_TYPE_IQ2_XS: ggml_type = 17;
1056 | pub const ggml_type_GGML_TYPE_IQ3_XXS: ggml_type = 18;
1057 | pub const ggml_type_GGML_TYPE_IQ1_S: ggml_type = 19;
1058 | pub const ggml_type_GGML_TYPE_IQ4_NL: ggml_type = 20;
1059 | pub const ggml_type_GGML_TYPE_IQ3_S: ggml_type = 21;
1060 | pub const ggml_type_GGML_TYPE_IQ2_S: ggml_type = 22;
1061 | pub const ggml_type_GGML_TYPE_IQ4_XS: ggml_type = 23;
1062 | pub const ggml_type_GGML_TYPE_I8: ggml_type = 24;
1063 | pub const ggml_type_GGML_TYPE_I16: ggml_type = 25;
1064 | pub const ggml_type_GGML_TYPE_I32: ggml_type = 26;
1065 | pub const ggml_type_GGML_TYPE_I64: ggml_type = 27;
1066 | pub const ggml_type_GGML_TYPE_F64: ggml_type = 28;
1067 | pub const ggml_type_GGML_TYPE_IQ1_M: ggml_type = 29;
1068 | pub const ggml_type_GGML_TYPE_BF16: ggml_type = 30;
1069 | pub const ggml_type_GGML_TYPE_Q4_0_4_4: ggml_type = 31;
1070 | pub const ggml_type_GGML_TYPE_Q4_0_4_8: ggml_type = 32;
1071 | pub const ggml_type_GGML_TYPE_Q4_0_8_8: ggml_type = 33;
1072 | pub const ggml_type_GGML_TYPE_TQ1_0: ggml_type = 34;
1073 | pub const ggml_type_GGML_TYPE_TQ2_0: ggml_type = 35;
1074 | pub const ggml_type_GGML_TYPE_COUNT: ggml_type = 36;
1075 | pub type ggml_type = ::std::os::raw::c_uint;
1076 | pub const ggml_prec_GGML_PREC_DEFAULT: ggml_prec = 0;
1077 | pub const ggml_prec_GGML_PREC_F32: ggml_prec = 1;
1078 | pub type ggml_prec = ::std::os::raw::c_uint;
1079 | pub const ggml_backend_type_GGML_BACKEND_TYPE_CPU: ggml_backend_type = 0;
1080 | pub const ggml_backend_type_GGML_BACKEND_TYPE_GPU: ggml_backend_type = 10;
1081 | pub const ggml_backend_type_GGML_BACKEND_TYPE_GPU_SPLIT: ggml_backend_type = 20;
1082 | pub type ggml_backend_type = ::std::os::raw::c_uint;
1083 | pub const ggml_ftype_GGML_FTYPE_UNKNOWN: ggml_ftype = -1;
1084 | pub const ggml_ftype_GGML_FTYPE_ALL_F32: ggml_ftype = 0;
1085 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_F16: ggml_ftype = 1;
1086 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_0: ggml_ftype = 2;
1087 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_1: ggml_ftype = 3;
1088 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_1_SOME_F16: ggml_ftype = 4;
1089 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q8_0: ggml_ftype = 7;
1090 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q5_0: ggml_ftype = 8;
1091 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q5_1: ggml_ftype = 9;
1092 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q2_K: ggml_ftype = 10;
1093 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q3_K: ggml_ftype = 11;
1094 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_K: ggml_ftype = 12;
1095 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q5_K: ggml_ftype = 13;
1096 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q6_K: ggml_ftype = 14;
1097 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ2_XXS: ggml_ftype = 15;
1098 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ2_XS: ggml_ftype = 16;
1099 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ3_XXS: ggml_ftype = 17;
1100 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ1_S: ggml_ftype = 18;
1101 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ4_NL: ggml_ftype = 19;
1102 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ3_S: ggml_ftype = 20;
1103 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ2_S: ggml_ftype = 21;
1104 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ4_XS: ggml_ftype = 22;
1105 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_IQ1_M: ggml_ftype = 23;
1106 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_BF16: ggml_ftype = 24;
1107 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_0_4_4: ggml_ftype = 25;
1108 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_0_4_8: ggml_ftype = 26;
1109 | pub const ggml_ftype_GGML_FTYPE_MOSTLY_Q4_0_8_8: ggml_ftype = 27;
1110 | pub type ggml_ftype = ::std::os::raw::c_int;
1111 | pub const ggml_op_GGML_OP_NONE: ggml_op = 0;
1112 | pub const ggml_op_GGML_OP_DUP: ggml_op = 1;
1113 | pub const ggml_op_GGML_OP_ADD: ggml_op = 2;
1114 | pub const ggml_op_GGML_OP_ADD1: ggml_op = 3;
1115 | pub const ggml_op_GGML_OP_ACC: ggml_op = 4;
1116 | pub const ggml_op_GGML_OP_SUB: ggml_op = 5;
1117 | pub const ggml_op_GGML_OP_MUL: ggml_op = 6;
1118 | pub const ggml_op_GGML_OP_DIV: ggml_op = 7;
1119 | pub const ggml_op_GGML_OP_SQR: ggml_op = 8;
1120 | pub const ggml_op_GGML_OP_SQRT: ggml_op = 9;
1121 | pub const ggml_op_GGML_OP_LOG: ggml_op = 10;
1122 | pub const ggml_op_GGML_OP_SIN: ggml_op = 11;
1123 | pub const ggml_op_GGML_OP_COS: ggml_op = 12;
1124 | pub const ggml_op_GGML_OP_SUM: ggml_op = 13;
1125 | pub const ggml_op_GGML_OP_SUM_ROWS: ggml_op = 14;
1126 | pub const ggml_op_GGML_OP_MEAN: ggml_op = 15;
1127 | pub const ggml_op_GGML_OP_ARGMAX: ggml_op = 16;
1128 | pub const ggml_op_GGML_OP_COUNT_EQUAL: ggml_op = 17;
1129 | pub const ggml_op_GGML_OP_REPEAT: ggml_op = 18;
1130 | pub const ggml_op_GGML_OP_REPEAT_BACK: ggml_op = 19;
1131 | pub const ggml_op_GGML_OP_CONCAT: ggml_op = 20;
1132 | pub const ggml_op_GGML_OP_SILU_BACK: ggml_op = 21;
1133 | pub const ggml_op_GGML_OP_NORM: ggml_op = 22;
1134 | pub const ggml_op_GGML_OP_RMS_NORM: ggml_op = 23;
1135 | pub const ggml_op_GGML_OP_RMS_NORM_BACK: ggml_op = 24;
1136 | pub const ggml_op_GGML_OP_GROUP_NORM: ggml_op = 25;
1137 | pub const ggml_op_GGML_OP_MUL_MAT: ggml_op = 26;
1138 | pub const ggml_op_GGML_OP_MUL_MAT_ID: ggml_op = 27;
1139 | pub const ggml_op_GGML_OP_OUT_PROD: ggml_op = 28;
1140 | pub const ggml_op_GGML_OP_SCALE: ggml_op = 29;
1141 | pub const ggml_op_GGML_OP_SET: ggml_op = 30;
1142 | pub const ggml_op_GGML_OP_CPY: ggml_op = 31;
1143 | pub const ggml_op_GGML_OP_CONT: ggml_op = 32;
1144 | pub const ggml_op_GGML_OP_RESHAPE: ggml_op = 33;
1145 | pub const ggml_op_GGML_OP_VIEW: ggml_op = 34;
1146 | pub const ggml_op_GGML_OP_PERMUTE: ggml_op = 35;
1147 | pub const ggml_op_GGML_OP_TRANSPOSE: ggml_op = 36;
1148 | pub const ggml_op_GGML_OP_GET_ROWS: ggml_op = 37;
1149 | pub const ggml_op_GGML_OP_GET_ROWS_BACK: ggml_op = 38;
1150 | pub const ggml_op_GGML_OP_DIAG: ggml_op = 39;
1151 | pub const ggml_op_GGML_OP_DIAG_MASK_INF: ggml_op = 40;
1152 | pub const ggml_op_GGML_OP_DIAG_MASK_ZERO: ggml_op = 41;
1153 | pub const ggml_op_GGML_OP_SOFT_MAX: ggml_op = 42;
1154 | pub const ggml_op_GGML_OP_SOFT_MAX_BACK: ggml_op = 43;
1155 | pub const ggml_op_GGML_OP_ROPE: ggml_op = 44;
1156 | pub const ggml_op_GGML_OP_ROPE_BACK: ggml_op = 45;
1157 | pub const ggml_op_GGML_OP_CLAMP: ggml_op = 46;
1158 | pub const ggml_op_GGML_OP_CONV_TRANSPOSE_1D: ggml_op = 47;
1159 | pub const ggml_op_GGML_OP_IM2COL: ggml_op = 48;
1160 | pub const ggml_op_GGML_OP_IM2COL_BACK: ggml_op = 49;
1161 | pub const ggml_op_GGML_OP_CONV_TRANSPOSE_2D: ggml_op = 50;
1162 | pub const ggml_op_GGML_OP_POOL_1D: ggml_op = 51;
1163 | pub const ggml_op_GGML_OP_POOL_2D: ggml_op = 52;
1164 | pub const ggml_op_GGML_OP_POOL_2D_BACK: ggml_op = 53;
1165 | pub const ggml_op_GGML_OP_UPSCALE: ggml_op = 54;
1166 | pub const ggml_op_GGML_OP_PAD: ggml_op = 55;
1167 | pub const ggml_op_GGML_OP_ARANGE: ggml_op = 56;
1168 | pub const ggml_op_GGML_OP_TIMESTEP_EMBEDDING: ggml_op = 57;
1169 | pub const ggml_op_GGML_OP_ARGSORT: ggml_op = 58;
1170 | pub const ggml_op_GGML_OP_LEAKY_RELU: ggml_op = 59;
1171 | pub const ggml_op_GGML_OP_FLASH_ATTN_EXT: ggml_op = 60;
1172 | pub const ggml_op_GGML_OP_FLASH_ATTN_BACK: ggml_op = 61;
1173 | pub const ggml_op_GGML_OP_SSM_CONV: ggml_op = 62;
1174 | pub const ggml_op_GGML_OP_SSM_SCAN: ggml_op = 63;
1175 | pub const ggml_op_GGML_OP_WIN_PART: ggml_op = 64;
1176 | pub const ggml_op_GGML_OP_WIN_UNPART: ggml_op = 65;
1177 | pub const ggml_op_GGML_OP_GET_REL_POS: ggml_op = 66;
1178 | pub const ggml_op_GGML_OP_ADD_REL_POS: ggml_op = 67;
1179 | pub const ggml_op_GGML_OP_RWKV_WKV: ggml_op = 68;
1180 | pub const ggml_op_GGML_OP_UNARY: ggml_op = 69;
1181 | pub const ggml_op_GGML_OP_MAP_UNARY: ggml_op = 70;
1182 | pub const ggml_op_GGML_OP_MAP_BINARY: ggml_op = 71;
1183 | pub const ggml_op_GGML_OP_MAP_CUSTOM1_F32: ggml_op = 72;
1184 | pub const ggml_op_GGML_OP_MAP_CUSTOM2_F32: ggml_op = 73;
1185 | pub const ggml_op_GGML_OP_MAP_CUSTOM3_F32: ggml_op = 74;
1186 | pub const ggml_op_GGML_OP_MAP_CUSTOM1: ggml_op = 75;
1187 | pub const ggml_op_GGML_OP_MAP_CUSTOM2: ggml_op = 76;
1188 | pub const ggml_op_GGML_OP_MAP_CUSTOM3: ggml_op = 77;
1189 | pub const ggml_op_GGML_OP_CROSS_ENTROPY_LOSS: ggml_op = 78;
1190 | pub const ggml_op_GGML_OP_CROSS_ENTROPY_LOSS_BACK: ggml_op = 79;
1191 | pub const ggml_op_GGML_OP_OPT_STEP_ADAMW: ggml_op = 80;
1192 | pub const ggml_op_GGML_OP_COUNT: ggml_op = 81;
1193 | pub type ggml_op = ::std::os::raw::c_uint;
1194 | pub const ggml_unary_op_GGML_UNARY_OP_ABS: ggml_unary_op = 0;
1195 | pub const ggml_unary_op_GGML_UNARY_OP_SGN: ggml_unary_op = 1;
1196 | pub const ggml_unary_op_GGML_UNARY_OP_NEG: ggml_unary_op = 2;
1197 | pub const ggml_unary_op_GGML_UNARY_OP_STEP: ggml_unary_op = 3;
1198 | pub const ggml_unary_op_GGML_UNARY_OP_TANH: ggml_unary_op = 4;
1199 | pub const ggml_unary_op_GGML_UNARY_OP_ELU: ggml_unary_op = 5;
1200 | pub const ggml_unary_op_GGML_UNARY_OP_RELU: ggml_unary_op = 6;
1201 | pub const ggml_unary_op_GGML_UNARY_OP_SIGMOID: ggml_unary_op = 7;
1202 | pub const ggml_unary_op_GGML_UNARY_OP_GELU: ggml_unary_op = 8;
1203 | pub const ggml_unary_op_GGML_UNARY_OP_GELU_QUICK: ggml_unary_op = 9;
1204 | pub const ggml_unary_op_GGML_UNARY_OP_SILU: ggml_unary_op = 10;
1205 | pub const ggml_unary_op_GGML_UNARY_OP_HARDSWISH: ggml_unary_op = 11;
1206 | pub const ggml_unary_op_GGML_UNARY_OP_HARDSIGMOID: ggml_unary_op = 12;
1207 | pub const ggml_unary_op_GGML_UNARY_OP_EXP: ggml_unary_op = 13;
1208 | pub const ggml_unary_op_GGML_UNARY_OP_COUNT: ggml_unary_op = 14;
1209 | pub type ggml_unary_op = ::std::os::raw::c_uint;
1210 | pub const ggml_object_type_GGML_OBJECT_TYPE_TENSOR: ggml_object_type = 0;
1211 | pub const ggml_object_type_GGML_OBJECT_TYPE_GRAPH: ggml_object_type = 1;
1212 | pub const ggml_object_type_GGML_OBJECT_TYPE_WORK_BUFFER: ggml_object_type = 2;
1213 | pub type ggml_object_type = ::std::os::raw::c_uint;
1214 | pub const ggml_log_level_GGML_LOG_LEVEL_NONE: ggml_log_level = 0;
1215 | pub const ggml_log_level_GGML_LOG_LEVEL_INFO: ggml_log_level = 1;
1216 | pub const ggml_log_level_GGML_LOG_LEVEL_WARN: ggml_log_level = 2;
1217 | pub const ggml_log_level_GGML_LOG_LEVEL_ERROR: ggml_log_level = 3;
1218 | pub const ggml_log_level_GGML_LOG_LEVEL_DEBUG: ggml_log_level = 4;
1219 | pub const ggml_log_level_GGML_LOG_LEVEL_CONT: ggml_log_level = 5;
1220 | pub type ggml_log_level = ::std::os::raw::c_uint;
1221 | pub const ggml_tensor_flag_GGML_TENSOR_FLAG_INPUT: ggml_tensor_flag = 1;
1222 | pub const ggml_tensor_flag_GGML_TENSOR_FLAG_OUTPUT: ggml_tensor_flag = 2;
1223 | pub const ggml_tensor_flag_GGML_TENSOR_FLAG_PARAM: ggml_tensor_flag = 4;
1224 | pub const ggml_tensor_flag_GGML_TENSOR_FLAG_LOSS: ggml_tensor_flag = 8;
1225 | pub type ggml_tensor_flag = ::std::os::raw::c_uint;
1226 | #[repr(C)]
1227 | #[derive(Debug, Copy, Clone)]
1228 | pub struct ggml_tensor {
1229 |     pub type_: ggml_type,
1230 |     pub backend: ggml_backend_type,
1231 |     pub buffer: *mut ggml_backend_buffer,
1232 |     pub ne: [i64; 4usize],
1233 |     pub nb: [usize; 4usize],
1234 |     pub op: ggml_op,
1235 |     pub op_params: [i32; 16usize],
1236 |     pub flags: i32,
1237 |     pub grad: *mut ggml_tensor,
1238 |     pub src: [*mut ggml_tensor; 10usize],
1239 |     pub view_src: *mut ggml_tensor,
1240 |     pub view_offs: usize,
1241 |     pub data: *mut ::std::os::raw::c_void,
1242 |     pub name: [::std::os::raw::c_char; 64usize],
1243 |     pub extra: *mut ::std::os::raw::c_void,
1244 | }
1245 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1246 | const _: () = {
1247 |     ["Size of ggml_tensor"][::std::mem::size_of::<ggml_tensor>() - 336usize];
1248 |     ["Alignment of ggml_tensor"][::std::mem::align_of::<ggml_tensor>() - 8usize];
1249 |     ["Offset of field: ggml_tensor::type_"][::std::mem::offset_of!(ggml_tensor, type_) - 0usize];
1250 |     ["Offset of field: ggml_tensor::backend"]
1251 |         [::std::mem::offset_of!(ggml_tensor, backend) - 4usize];
1252 |     ["Offset of field: ggml_tensor::buffer"][::std::mem::offset_of!(ggml_tensor, buffer) - 8usize];
1253 |     ["Offset of field: ggml_tensor::ne"][::std::mem::offset_of!(ggml_tensor, ne) - 16usize];
1254 |     ["Offset of field: ggml_tensor::nb"][::std::mem::offset_of!(ggml_tensor, nb) - 48usize];
1255 |     ["Offset of field: ggml_tensor::op"][::std::mem::offset_of!(ggml_tensor, op) - 80usize];
1256 |     ["Offset of field: ggml_tensor::op_params"]
1257 |         [::std::mem::offset_of!(ggml_tensor, op_params) - 84usize];
1258 |     ["Offset of field: ggml_tensor::flags"][::std::mem::offset_of!(ggml_tensor, flags) - 148usize];
1259 |     ["Offset of field: ggml_tensor::grad"][::std::mem::offset_of!(ggml_tensor, grad) - 152usize];
1260 |     ["Offset of field: ggml_tensor::src"][::std::mem::offset_of!(ggml_tensor, src) - 160usize];
1261 |     ["Offset of field: ggml_tensor::view_src"]
1262 |         [::std::mem::offset_of!(ggml_tensor, view_src) - 240usize];
1263 |     ["Offset of field: ggml_tensor::view_offs"]
1264 |         [::std::mem::offset_of!(ggml_tensor, view_offs) - 248usize];
1265 |     ["Offset of field: ggml_tensor::data"][::std::mem::offset_of!(ggml_tensor, data) - 256usize];
1266 |     ["Offset of field: ggml_tensor::name"][::std::mem::offset_of!(ggml_tensor, name) - 264usize];
1267 |     ["Offset of field: ggml_tensor::extra"][::std::mem::offset_of!(ggml_tensor, extra) - 328usize];
1268 | };
1269 | pub const GGML_TENSOR_SIZE: usize = 336;
1270 | pub type ggml_abort_callback =
1271 |     ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> bool>;
1272 | pub const ggml_sched_priority_GGML_SCHED_PRIO_NORMAL: ggml_sched_priority = 0;
1273 | pub const ggml_sched_priority_GGML_SCHED_PRIO_MEDIUM: ggml_sched_priority = 1;
1274 | pub const ggml_sched_priority_GGML_SCHED_PRIO_HIGH: ggml_sched_priority = 2;
1275 | pub const ggml_sched_priority_GGML_SCHED_PRIO_REALTIME: ggml_sched_priority = 3;
1276 | pub type ggml_sched_priority = ::std::os::raw::c_uint;
1277 | #[repr(C)]
1278 | #[derive(Debug, Copy, Clone)]
1279 | pub struct ggml_threadpool_params {
1280 |     pub cpumask: [bool; 512usize],
1281 |     pub n_threads: ::std::os::raw::c_int,
1282 |     pub prio: ggml_sched_priority,
1283 |     pub poll: u32,
1284 |     pub strict_cpu: bool,
1285 |     pub paused: bool,
1286 | }
1287 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1288 | const _: () = {
1289 |     ["Size of ggml_threadpool_params"][::std::mem::size_of::<ggml_threadpool_params>() - 528usize];
1290 |     ["Alignment of ggml_threadpool_params"]
1291 |         [::std::mem::align_of::<ggml_threadpool_params>() - 4usize];
1292 |     ["Offset of field: ggml_threadpool_params::cpumask"]
1293 |         [::std::mem::offset_of!(ggml_threadpool_params, cpumask) - 0usize];
1294 |     ["Offset of field: ggml_threadpool_params::n_threads"]
1295 |         [::std::mem::offset_of!(ggml_threadpool_params, n_threads) - 512usize];
1296 |     ["Offset of field: ggml_threadpool_params::prio"]
1297 |         [::std::mem::offset_of!(ggml_threadpool_params, prio) - 516usize];
1298 |     ["Offset of field: ggml_threadpool_params::poll"]
1299 |         [::std::mem::offset_of!(ggml_threadpool_params, poll) - 520usize];
1300 |     ["Offset of field: ggml_threadpool_params::strict_cpu"]
1301 |         [::std::mem::offset_of!(ggml_threadpool_params, strict_cpu) - 524usize];
1302 |     ["Offset of field: ggml_threadpool_params::paused"]
1303 |         [::std::mem::offset_of!(ggml_threadpool_params, paused) - 525usize];
1304 | };
1305 | #[repr(C)]
1306 | #[derive(Debug, Copy, Clone)]
1307 | pub struct ggml_threadpool {
1308 |     _unused: [u8; 0],
1309 | }
1310 | pub type ggml_threadpool_t = *mut ggml_threadpool;
1311 | #[repr(C)]
1312 | #[derive(Debug, Copy, Clone)]
1313 | pub struct ggml_cplan {
1314 |     pub work_size: usize,
1315 |     pub work_data: *mut u8,
1316 |     pub n_threads: ::std::os::raw::c_int,
1317 |     pub threadpool: *mut ggml_threadpool,
1318 |     pub abort_callback: ggml_abort_callback,
1319 |     pub abort_callback_data: *mut ::std::os::raw::c_void,
1320 | }
1321 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1322 | const _: () = {
1323 |     ["Size of ggml_cplan"][::std::mem::size_of::<ggml_cplan>() - 48usize];
1324 |     ["Alignment of ggml_cplan"][::std::mem::align_of::<ggml_cplan>() - 8usize];
1325 |     ["Offset of field: ggml_cplan::work_size"]
1326 |         [::std::mem::offset_of!(ggml_cplan, work_size) - 0usize];
1327 |     ["Offset of field: ggml_cplan::work_data"]
1328 |         [::std::mem::offset_of!(ggml_cplan, work_data) - 8usize];
1329 |     ["Offset of field: ggml_cplan::n_threads"]
1330 |         [::std::mem::offset_of!(ggml_cplan, n_threads) - 16usize];
1331 |     ["Offset of field: ggml_cplan::threadpool"]
1332 |         [::std::mem::offset_of!(ggml_cplan, threadpool) - 24usize];
1333 |     ["Offset of field: ggml_cplan::abort_callback"]
1334 |         [::std::mem::offset_of!(ggml_cplan, abort_callback) - 32usize];
1335 |     ["Offset of field: ggml_cplan::abort_callback_data"]
1336 |         [::std::mem::offset_of!(ggml_cplan, abort_callback_data) - 40usize];
1337 | };
1338 | #[repr(C)]
1339 | #[derive(Debug, Copy, Clone)]
1340 | pub struct ggml_scratch {
1341 |     pub offs: usize,
1342 |     pub size: usize,
1343 |     pub data: *mut ::std::os::raw::c_void,
1344 | }
1345 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1346 | const _: () = {
1347 |     ["Size of ggml_scratch"][::std::mem::size_of::<ggml_scratch>() - 24usize];
1348 |     ["Alignment of ggml_scratch"][::std::mem::align_of::<ggml_scratch>() - 8usize];
1349 |     ["Offset of field: ggml_scratch::offs"][::std::mem::offset_of!(ggml_scratch, offs) - 0usize];
1350 |     ["Offset of field: ggml_scratch::size"][::std::mem::offset_of!(ggml_scratch, size) - 8usize];
1351 |     ["Offset of field: ggml_scratch::data"][::std::mem::offset_of!(ggml_scratch, data) - 16usize];
1352 | };
1353 | #[repr(C)]
1354 | #[derive(Debug, Copy, Clone)]
1355 | pub struct ggml_init_params {
1356 |     pub mem_size: usize,
1357 |     pub mem_buffer: *mut ::std::os::raw::c_void,
1358 |     pub no_alloc: bool,
1359 | }
1360 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
1361 | const _: () = {
1362 |     ["Size of ggml_init_params"][::std::mem::size_of::<ggml_init_params>() - 24usize];
1363 |     ["Alignment of ggml_init_params"][::std::mem::align_of::<ggml_init_params>() - 8usize];
1364 |     ["Offset of field: ggml_init_params::mem_size"]
1365 |         [::std::mem::offset_of!(ggml_init_params, mem_size) - 0usize];
1366 |     ["Offset of field: ggml_init_params::mem_buffer"]
1367 |         [::std::mem::offset_of!(ggml_init_params, mem_buffer) - 8usize];
1368 |     ["Offset of field: ggml_init_params::no_alloc"]
1369 |         [::std::mem::offset_of!(ggml_init_params, no_alloc) - 16usize];
1370 | };
1371 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_DISABLED: ggml_numa_strategy = 0;
1372 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_DISTRIBUTE: ggml_numa_strategy = 1;
1373 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_ISOLATE: ggml_numa_strategy = 2;
1374 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_NUMACTL: ggml_numa_strategy = 3;
1375 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_MIRROR: ggml_numa_strategy = 4;
1376 | pub const ggml_numa_strategy_GGML_NUMA_STRATEGY_COUNT: ggml_numa_strategy = 5;
1377 | pub type ggml_numa_strategy = ::std::os::raw::c_uint;
1378 | pub type ggml_guid = [u8; 16usize];
1379 | pub type ggml_guid_t = *mut ggml_guid;
1380 | extern "C" {
1381 |     pub fn ggml_guid_matches(guid_a: ggml_guid_t, guid_b: ggml_guid_t) -> bool;
1382 | }
1383 | extern "C" {
1384 |     pub fn ggml_time_init();
1385 | }
1386 | extern "C" {
1387 |     pub fn ggml_time_ms() -> i64;
1388 | }
1389 | extern "C" {
1390 |     pub fn ggml_time_us() -> i64;
1391 | }
1392 | extern "C" {
1393 |     pub fn ggml_cycles() -> i64;
1394 | }
1395 | extern "C" {
1396 |     pub fn ggml_cycles_per_ms() -> i64;
1397 | }
1398 | extern "C" {
1399 |     pub fn ggml_fopen(
1400 |         fname: *const ::std::os::raw::c_char,
1401 |         mode: *const ::std::os::raw::c_char,
1402 |     ) -> *mut FILE;
1403 | }
1404 | extern "C" {
1405 |     pub fn ggml_numa_init(numa: ggml_numa_strategy);
1406 | }
1407 | extern "C" {
1408 |     pub fn ggml_is_numa() -> bool;
1409 | }
1410 | extern "C" {
1411 |     pub fn ggml_print_object(obj: *const ggml_object);
1412 | }
1413 | extern "C" {
1414 |     pub fn ggml_print_objects(ctx: *const ggml_context);
1415 | }
1416 | extern "C" {
1417 |     pub fn ggml_nelements(tensor: *const ggml_tensor) -> i64;
1418 | }
1419 | extern "C" {
1420 |     pub fn ggml_nrows(tensor: *const ggml_tensor) -> i64;
1421 | }
1422 | extern "C" {
1423 |     pub fn ggml_nbytes(tensor: *const ggml_tensor) -> usize;
1424 | }
1425 | extern "C" {
1426 |     pub fn ggml_nbytes_pad(tensor: *const ggml_tensor) -> usize;
1427 | }
1428 | extern "C" {
1429 |     pub fn ggml_blck_size(type_: ggml_type) -> i64;
1430 | }
1431 | extern "C" {
1432 |     pub fn ggml_type_size(type_: ggml_type) -> usize;
1433 | }
1434 | extern "C" {
1435 |     pub fn ggml_row_size(type_: ggml_type, ne: i64) -> usize;
1436 | }
1437 | extern "C" {
1438 |     pub fn ggml_type_sizef(type_: ggml_type) -> f64;
1439 | }
1440 | extern "C" {
1441 |     pub fn ggml_type_name(type_: ggml_type) -> *const ::std::os::raw::c_char;
1442 | }
1443 | extern "C" {
1444 |     pub fn ggml_op_name(op: ggml_op) -> *const ::std::os::raw::c_char;
1445 | }
1446 | extern "C" {
1447 |     pub fn ggml_op_symbol(op: ggml_op) -> *const ::std::os::raw::c_char;
1448 | }
1449 | extern "C" {
1450 |     pub fn ggml_unary_op_name(op: ggml_unary_op) -> *const ::std::os::raw::c_char;
1451 | }
1452 | extern "C" {
1453 |     pub fn ggml_op_desc(t: *const ggml_tensor) -> *const ::std::os::raw::c_char;
1454 | }
1455 | extern "C" {
1456 |     pub fn ggml_element_size(tensor: *const ggml_tensor) -> usize;
1457 | }
1458 | extern "C" {
1459 |     pub fn ggml_is_quantized(type_: ggml_type) -> bool;
1460 | }
1461 | extern "C" {
1462 |     pub fn ggml_ftype_to_ggml_type(ftype: ggml_ftype) -> ggml_type;
1463 | }
1464 | extern "C" {
1465 |     pub fn ggml_is_transposed(tensor: *const ggml_tensor) -> bool;
1466 | }
1467 | extern "C" {
1468 |     pub fn ggml_is_permuted(tensor: *const ggml_tensor) -> bool;
1469 | }
1470 | extern "C" {
1471 |     pub fn ggml_is_empty(tensor: *const ggml_tensor) -> bool;
1472 | }
1473 | extern "C" {
1474 |     pub fn ggml_is_scalar(tensor: *const ggml_tensor) -> bool;
1475 | }
1476 | extern "C" {
1477 |     pub fn ggml_is_vector(tensor: *const ggml_tensor) -> bool;
1478 | }
1479 | extern "C" {
1480 |     pub fn ggml_is_matrix(tensor: *const ggml_tensor) -> bool;
1481 | }
1482 | extern "C" {
1483 |     pub fn ggml_is_3d(tensor: *const ggml_tensor) -> bool;
1484 | }
1485 | extern "C" {
1486 |     pub fn ggml_n_dims(tensor: *const ggml_tensor) -> ::std::os::raw::c_int;
1487 | }
1488 | extern "C" {
1489 |     pub fn ggml_is_contiguous(tensor: *const ggml_tensor) -> bool;
1490 | }
1491 | extern "C" {
1492 |     pub fn ggml_is_contiguous_0(tensor: *const ggml_tensor) -> bool;
1493 | }
1494 | extern "C" {
1495 |     pub fn ggml_is_contiguous_1(tensor: *const ggml_tensor) -> bool;
1496 | }
1497 | extern "C" {
1498 |     pub fn ggml_is_contiguous_2(tensor: *const ggml_tensor) -> bool;
1499 | }
1500 | extern "C" {
1501 |     pub fn ggml_are_same_shape(t0: *const ggml_tensor, t1: *const ggml_tensor) -> bool;
1502 | }
1503 | extern "C" {
1504 |     pub fn ggml_are_same_stride(t0: *const ggml_tensor, t1: *const ggml_tensor) -> bool;
1505 | }
1506 | extern "C" {
1507 |     pub fn ggml_can_repeat(t0: *const ggml_tensor, t1: *const ggml_tensor) -> bool;
1508 | }
1509 | extern "C" {
1510 |     pub fn ggml_tensor_overhead() -> usize;
1511 | }
1512 | extern "C" {
1513 |     pub fn ggml_validate_row_data(
1514 |         type_: ggml_type,
1515 |         data: *const ::std::os::raw::c_void,
1516 |         nbytes: usize,
1517 |     ) -> bool;
1518 | }
1519 | extern "C" {
1520 |     pub fn ggml_init(params: ggml_init_params) -> *mut ggml_context;
1521 | }
1522 | extern "C" {
1523 |     pub fn ggml_reset(ctx: *mut ggml_context);
1524 | }
1525 | extern "C" {
1526 |     pub fn ggml_free(ctx: *mut ggml_context);
1527 | }
1528 | extern "C" {
1529 |     pub fn ggml_used_mem(ctx: *const ggml_context) -> usize;
1530 | }
1531 | extern "C" {
1532 |     pub fn ggml_set_scratch(ctx: *mut ggml_context, scratch: ggml_scratch) -> usize;
1533 | }
1534 | extern "C" {
1535 |     pub fn ggml_get_no_alloc(ctx: *mut ggml_context) -> bool;
1536 | }
1537 | extern "C" {
1538 |     pub fn ggml_set_no_alloc(ctx: *mut ggml_context, no_alloc: bool);
1539 | }
1540 | extern "C" {
1541 |     pub fn ggml_get_mem_buffer(ctx: *const ggml_context) -> *mut ::std::os::raw::c_void;
1542 | }
1543 | extern "C" {
1544 |     pub fn ggml_get_mem_size(ctx: *const ggml_context) -> usize;
1545 | }
1546 | extern "C" {
1547 |     pub fn ggml_get_max_tensor_size(ctx: *const ggml_context) -> usize;
1548 | }
1549 | extern "C" {
1550 |     pub fn ggml_new_tensor(
1551 |         ctx: *mut ggml_context,
1552 |         type_: ggml_type,
1553 |         n_dims: ::std::os::raw::c_int,
1554 |         ne: *const i64,
1555 |     ) -> *mut ggml_tensor;
1556 | }
1557 | extern "C" {
1558 |     pub fn ggml_new_tensor_1d(
1559 |         ctx: *mut ggml_context,
1560 |         type_: ggml_type,
1561 |         ne0: i64,
1562 |     ) -> *mut ggml_tensor;
1563 | }
1564 | extern "C" {
1565 |     pub fn ggml_new_tensor_2d(
1566 |         ctx: *mut ggml_context,
1567 |         type_: ggml_type,
1568 |         ne0: i64,
1569 |         ne1: i64,
1570 |     ) -> *mut ggml_tensor;
1571 | }
1572 | extern "C" {
1573 |     pub fn ggml_new_tensor_3d(
1574 |         ctx: *mut ggml_context,
1575 |         type_: ggml_type,
1576 |         ne0: i64,
1577 |         ne1: i64,
1578 |         ne2: i64,
1579 |     ) -> *mut ggml_tensor;
1580 | }
1581 | extern "C" {
1582 |     pub fn ggml_new_tensor_4d(
1583 |         ctx: *mut ggml_context,
1584 |         type_: ggml_type,
1585 |         ne0: i64,
1586 |         ne1: i64,
1587 |         ne2: i64,
1588 |         ne3: i64,
1589 |     ) -> *mut ggml_tensor;
1590 | }
1591 | extern "C" {
1592 |     pub fn ggml_new_i32(ctx: *mut ggml_context, value: i32) -> *mut ggml_tensor;
1593 | }
1594 | extern "C" {
1595 |     pub fn ggml_new_f32(ctx: *mut ggml_context, value: f32) -> *mut ggml_tensor;
1596 | }
1597 | extern "C" {
1598 |     pub fn ggml_dup_tensor(ctx: *mut ggml_context, src: *const ggml_tensor) -> *mut ggml_tensor;
1599 | }
1600 | extern "C" {
1601 |     pub fn ggml_view_tensor(ctx: *mut ggml_context, src: *mut ggml_tensor) -> *mut ggml_tensor;
1602 | }
1603 | extern "C" {
1604 |     pub fn ggml_get_first_tensor(ctx: *const ggml_context) -> *mut ggml_tensor;
1605 | }
1606 | extern "C" {
1607 |     pub fn ggml_get_next_tensor(
1608 |         ctx: *const ggml_context,
1609 |         tensor: *mut ggml_tensor,
1610 |     ) -> *mut ggml_tensor;
1611 | }
1612 | extern "C" {
1613 |     pub fn ggml_get_tensor(
1614 |         ctx: *mut ggml_context,
1615 |         name: *const ::std::os::raw::c_char,
1616 |     ) -> *mut ggml_tensor;
1617 | }
1618 | extern "C" {
1619 |     pub fn ggml_set_zero(tensor: *mut ggml_tensor) -> *mut ggml_tensor;
1620 | }
1621 | extern "C" {
1622 |     pub fn ggml_set_i32(tensor: *mut ggml_tensor, value: i32) -> *mut ggml_tensor;
1623 | }
1624 | extern "C" {
1625 |     pub fn ggml_set_f32(tensor: *mut ggml_tensor, value: f32) -> *mut ggml_tensor;
1626 | }
1627 | extern "C" {
1628 |     pub fn ggml_unravel_index(
1629 |         tensor: *const ggml_tensor,
1630 |         i: i64,
1631 |         i0: *mut i64,
1632 |         i1: *mut i64,
1633 |         i2: *mut i64,
1634 |         i3: *mut i64,
1635 |     );
1636 | }
1637 | extern "C" {
1638 |     pub fn ggml_get_i32_1d(tensor: *const ggml_tensor, i: ::std::os::raw::c_int) -> i32;
1639 | }
1640 | extern "C" {
1641 |     pub fn ggml_set_i32_1d(tensor: *const ggml_tensor, i: ::std::os::raw::c_int, value: i32);
1642 | }
1643 | extern "C" {
1644 |     pub fn ggml_get_i32_nd(
1645 |         tensor: *const ggml_tensor,
1646 |         i0: ::std::os::raw::c_int,
1647 |         i1: ::std::os::raw::c_int,
1648 |         i2: ::std::os::raw::c_int,
1649 |         i3: ::std::os::raw::c_int,
1650 |     ) -> i32;
1651 | }
1652 | extern "C" {
1653 |     pub fn ggml_set_i32_nd(
1654 |         tensor: *const ggml_tensor,
1655 |         i0: ::std::os::raw::c_int,
1656 |         i1: ::std::os::raw::c_int,
1657 |         i2: ::std::os::raw::c_int,
1658 |         i3: ::std::os::raw::c_int,
1659 |         value: i32,
1660 |     );
1661 | }
1662 | extern "C" {
1663 |     pub fn ggml_get_f32_1d(tensor: *const ggml_tensor, i: ::std::os::raw::c_int) -> f32;
1664 | }
1665 | extern "C" {
1666 |     pub fn ggml_set_f32_1d(tensor: *const ggml_tensor, i: ::std::os::raw::c_int, value: f32);
1667 | }
1668 | extern "C" {
1669 |     pub fn ggml_get_f32_nd(
1670 |         tensor: *const ggml_tensor,
1671 |         i0: ::std::os::raw::c_int,
1672 |         i1: ::std::os::raw::c_int,
1673 |         i2: ::std::os::raw::c_int,
1674 |         i3: ::std::os::raw::c_int,
1675 |     ) -> f32;
1676 | }
1677 | extern "C" {
1678 |     pub fn ggml_set_f32_nd(
1679 |         tensor: *const ggml_tensor,
1680 |         i0: ::std::os::raw::c_int,
1681 |         i1: ::std::os::raw::c_int,
1682 |         i2: ::std::os::raw::c_int,
1683 |         i3: ::std::os::raw::c_int,
1684 |         value: f32,
1685 |     );
1686 | }
1687 | extern "C" {
1688 |     pub fn ggml_get_data(tensor: *const ggml_tensor) -> *mut ::std::os::raw::c_void;
1689 | }
1690 | extern "C" {
1691 |     pub fn ggml_get_data_f32(tensor: *const ggml_tensor) -> *mut f32;
1692 | }
1693 | extern "C" {
1694 |     pub fn ggml_get_unary_op(tensor: *const ggml_tensor) -> ggml_unary_op;
1695 | }
1696 | extern "C" {
1697 |     pub fn ggml_get_name(tensor: *const ggml_tensor) -> *const ::std::os::raw::c_char;
1698 | }
1699 | extern "C" {
1700 |     pub fn ggml_set_name(
1701 |         tensor: *mut ggml_tensor,
1702 |         name: *const ::std::os::raw::c_char,
1703 |     ) -> *mut ggml_tensor;
1704 | }
1705 | extern "C" {
1706 |     pub fn ggml_format_name(
1707 |         tensor: *mut ggml_tensor,
1708 |         fmt: *const ::std::os::raw::c_char,
1709 |         ...
1710 |     ) -> *mut ggml_tensor;
1711 | }
1712 | extern "C" {
1713 |     pub fn ggml_dup(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1714 | }
1715 | extern "C" {
1716 |     pub fn ggml_dup_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1717 | }
1718 | extern "C" {
1719 |     pub fn ggml_add(
1720 |         ctx: *mut ggml_context,
1721 |         a: *mut ggml_tensor,
1722 |         b: *mut ggml_tensor,
1723 |     ) -> *mut ggml_tensor;
1724 | }
1725 | extern "C" {
1726 |     pub fn ggml_add_inplace(
1727 |         ctx: *mut ggml_context,
1728 |         a: *mut ggml_tensor,
1729 |         b: *mut ggml_tensor,
1730 |     ) -> *mut ggml_tensor;
1731 | }
1732 | extern "C" {
1733 |     pub fn ggml_add_cast(
1734 |         ctx: *mut ggml_context,
1735 |         a: *mut ggml_tensor,
1736 |         b: *mut ggml_tensor,
1737 |         type_: ggml_type,
1738 |     ) -> *mut ggml_tensor;
1739 | }
1740 | extern "C" {
1741 |     pub fn ggml_add1(
1742 |         ctx: *mut ggml_context,
1743 |         a: *mut ggml_tensor,
1744 |         b: *mut ggml_tensor,
1745 |     ) -> *mut ggml_tensor;
1746 | }
1747 | extern "C" {
1748 |     pub fn ggml_add1_inplace(
1749 |         ctx: *mut ggml_context,
1750 |         a: *mut ggml_tensor,
1751 |         b: *mut ggml_tensor,
1752 |     ) -> *mut ggml_tensor;
1753 | }
1754 | extern "C" {
1755 |     pub fn ggml_acc(
1756 |         ctx: *mut ggml_context,
1757 |         a: *mut ggml_tensor,
1758 |         b: *mut ggml_tensor,
1759 |         nb1: usize,
1760 |         nb2: usize,
1761 |         nb3: usize,
1762 |         offset: usize,
1763 |     ) -> *mut ggml_tensor;
1764 | }
1765 | extern "C" {
1766 |     pub fn ggml_acc_inplace(
1767 |         ctx: *mut ggml_context,
1768 |         a: *mut ggml_tensor,
1769 |         b: *mut ggml_tensor,
1770 |         nb1: usize,
1771 |         nb2: usize,
1772 |         nb3: usize,
1773 |         offset: usize,
1774 |     ) -> *mut ggml_tensor;
1775 | }
1776 | extern "C" {
1777 |     pub fn ggml_sub(
1778 |         ctx: *mut ggml_context,
1779 |         a: *mut ggml_tensor,
1780 |         b: *mut ggml_tensor,
1781 |     ) -> *mut ggml_tensor;
1782 | }
1783 | extern "C" {
1784 |     pub fn ggml_sub_inplace(
1785 |         ctx: *mut ggml_context,
1786 |         a: *mut ggml_tensor,
1787 |         b: *mut ggml_tensor,
1788 |     ) -> *mut ggml_tensor;
1789 | }
1790 | extern "C" {
1791 |     pub fn ggml_mul(
1792 |         ctx: *mut ggml_context,
1793 |         a: *mut ggml_tensor,
1794 |         b: *mut ggml_tensor,
1795 |     ) -> *mut ggml_tensor;
1796 | }
1797 | extern "C" {
1798 |     pub fn ggml_mul_inplace(
1799 |         ctx: *mut ggml_context,
1800 |         a: *mut ggml_tensor,
1801 |         b: *mut ggml_tensor,
1802 |     ) -> *mut ggml_tensor;
1803 | }
1804 | extern "C" {
1805 |     pub fn ggml_div(
1806 |         ctx: *mut ggml_context,
1807 |         a: *mut ggml_tensor,
1808 |         b: *mut ggml_tensor,
1809 |     ) -> *mut ggml_tensor;
1810 | }
1811 | extern "C" {
1812 |     pub fn ggml_div_inplace(
1813 |         ctx: *mut ggml_context,
1814 |         a: *mut ggml_tensor,
1815 |         b: *mut ggml_tensor,
1816 |     ) -> *mut ggml_tensor;
1817 | }
1818 | extern "C" {
1819 |     pub fn ggml_sqr(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1820 | }
1821 | extern "C" {
1822 |     pub fn ggml_sqr_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1823 | }
1824 | extern "C" {
1825 |     pub fn ggml_sqrt(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1826 | }
1827 | extern "C" {
1828 |     pub fn ggml_sqrt_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1829 | }
1830 | extern "C" {
1831 |     pub fn ggml_log(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1832 | }
1833 | extern "C" {
1834 |     pub fn ggml_log_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1835 | }
1836 | extern "C" {
1837 |     pub fn ggml_sin(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1838 | }
1839 | extern "C" {
1840 |     pub fn ggml_sin_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1841 | }
1842 | extern "C" {
1843 |     pub fn ggml_cos(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1844 | }
1845 | extern "C" {
1846 |     pub fn ggml_cos_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1847 | }
1848 | extern "C" {
1849 |     pub fn ggml_sum(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1850 | }
1851 | extern "C" {
1852 |     pub fn ggml_sum_rows(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1853 | }
1854 | extern "C" {
1855 |     pub fn ggml_mean(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1856 | }
1857 | extern "C" {
1858 |     pub fn ggml_argmax(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1859 | }
1860 | extern "C" {
1861 |     pub fn ggml_count_equal(
1862 |         ctx: *mut ggml_context,
1863 |         a: *mut ggml_tensor,
1864 |         b: *mut ggml_tensor,
1865 |     ) -> *mut ggml_tensor;
1866 | }
1867 | extern "C" {
1868 |     pub fn ggml_repeat(
1869 |         ctx: *mut ggml_context,
1870 |         a: *mut ggml_tensor,
1871 |         b: *mut ggml_tensor,
1872 |     ) -> *mut ggml_tensor;
1873 | }
1874 | extern "C" {
1875 |     pub fn ggml_repeat_back(
1876 |         ctx: *mut ggml_context,
1877 |         a: *mut ggml_tensor,
1878 |         b: *mut ggml_tensor,
1879 |     ) -> *mut ggml_tensor;
1880 | }
1881 | extern "C" {
1882 |     pub fn ggml_concat(
1883 |         ctx: *mut ggml_context,
1884 |         a: *mut ggml_tensor,
1885 |         b: *mut ggml_tensor,
1886 |         dim: ::std::os::raw::c_int,
1887 |     ) -> *mut ggml_tensor;
1888 | }
1889 | extern "C" {
1890 |     pub fn ggml_abs(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1891 | }
1892 | extern "C" {
1893 |     pub fn ggml_abs_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1894 | }
1895 | extern "C" {
1896 |     pub fn ggml_sgn(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1897 | }
1898 | extern "C" {
1899 |     pub fn ggml_sgn_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1900 | }
1901 | extern "C" {
1902 |     pub fn ggml_neg(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1903 | }
1904 | extern "C" {
1905 |     pub fn ggml_neg_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1906 | }
1907 | extern "C" {
1908 |     pub fn ggml_step(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1909 | }
1910 | extern "C" {
1911 |     pub fn ggml_step_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1912 | }
1913 | extern "C" {
1914 |     pub fn ggml_tanh(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1915 | }
1916 | extern "C" {
1917 |     pub fn ggml_tanh_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1918 | }
1919 | extern "C" {
1920 |     pub fn ggml_elu(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1921 | }
1922 | extern "C" {
1923 |     pub fn ggml_elu_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1924 | }
1925 | extern "C" {
1926 |     pub fn ggml_relu(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1927 | }
1928 | extern "C" {
1929 |     pub fn ggml_leaky_relu(
1930 |         ctx: *mut ggml_context,
1931 |         a: *mut ggml_tensor,
1932 |         negative_slope: f32,
1933 |         inplace: bool,
1934 |     ) -> *mut ggml_tensor;
1935 | }
1936 | extern "C" {
1937 |     pub fn ggml_relu_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1938 | }
1939 | extern "C" {
1940 |     pub fn ggml_sigmoid(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1941 | }
1942 | extern "C" {
1943 |     pub fn ggml_sigmoid_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1944 | }
1945 | extern "C" {
1946 |     pub fn ggml_gelu(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1947 | }
1948 | extern "C" {
1949 |     pub fn ggml_gelu_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1950 | }
1951 | extern "C" {
1952 |     pub fn ggml_gelu_quick(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1953 | }
1954 | extern "C" {
1955 |     pub fn ggml_gelu_quick_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor)
1956 |         -> *mut ggml_tensor;
1957 | }
1958 | extern "C" {
1959 |     pub fn ggml_silu(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1960 | }
1961 | extern "C" {
1962 |     pub fn ggml_silu_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1963 | }
1964 | extern "C" {
1965 |     pub fn ggml_silu_back(
1966 |         ctx: *mut ggml_context,
1967 |         a: *mut ggml_tensor,
1968 |         b: *mut ggml_tensor,
1969 |     ) -> *mut ggml_tensor;
1970 | }
1971 | extern "C" {
1972 |     pub fn ggml_hardswish(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1973 | }
1974 | extern "C" {
1975 |     pub fn ggml_hardsigmoid(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1976 | }
1977 | extern "C" {
1978 |     pub fn ggml_exp(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1979 | }
1980 | extern "C" {
1981 |     pub fn ggml_exp_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
1982 | }
1983 | extern "C" {
1984 |     pub fn ggml_norm(ctx: *mut ggml_context, a: *mut ggml_tensor, eps: f32) -> *mut ggml_tensor;
1985 | }
1986 | extern "C" {
1987 |     pub fn ggml_norm_inplace(
1988 |         ctx: *mut ggml_context,
1989 |         a: *mut ggml_tensor,
1990 |         eps: f32,
1991 |     ) -> *mut ggml_tensor;
1992 | }
1993 | extern "C" {
1994 |     pub fn ggml_rms_norm(ctx: *mut ggml_context, a: *mut ggml_tensor, eps: f32)
1995 |         -> *mut ggml_tensor;
1996 | }
1997 | extern "C" {
1998 |     pub fn ggml_rms_norm_inplace(
1999 |         ctx: *mut ggml_context,
2000 |         a: *mut ggml_tensor,
2001 |         eps: f32,
2002 |     ) -> *mut ggml_tensor;
2003 | }
2004 | extern "C" {
2005 |     pub fn ggml_group_norm(
2006 |         ctx: *mut ggml_context,
2007 |         a: *mut ggml_tensor,
2008 |         n_groups: ::std::os::raw::c_int,
2009 |         eps: f32,
2010 |     ) -> *mut ggml_tensor;
2011 | }
2012 | extern "C" {
2013 |     pub fn ggml_group_norm_inplace(
2014 |         ctx: *mut ggml_context,
2015 |         a: *mut ggml_tensor,
2016 |         n_groups: ::std::os::raw::c_int,
2017 |         eps: f32,
2018 |     ) -> *mut ggml_tensor;
2019 | }
2020 | extern "C" {
2021 |     pub fn ggml_rms_norm_back(
2022 |         ctx: *mut ggml_context,
2023 |         a: *mut ggml_tensor,
2024 |         b: *mut ggml_tensor,
2025 |         eps: f32,
2026 |     ) -> *mut ggml_tensor;
2027 | }
2028 | extern "C" {
2029 |     pub fn ggml_mul_mat(
2030 |         ctx: *mut ggml_context,
2031 |         a: *mut ggml_tensor,
2032 |         b: *mut ggml_tensor,
2033 |     ) -> *mut ggml_tensor;
2034 | }
2035 | extern "C" {
2036 |     pub fn ggml_mul_mat_set_prec(a: *mut ggml_tensor, prec: ggml_prec);
2037 | }
2038 | extern "C" {
2039 |     pub fn ggml_mul_mat_id(
2040 |         ctx: *mut ggml_context,
2041 |         as_: *mut ggml_tensor,
2042 |         b: *mut ggml_tensor,
2043 |         ids: *mut ggml_tensor,
2044 |     ) -> *mut ggml_tensor;
2045 | }
2046 | extern "C" {
2047 |     pub fn ggml_out_prod(
2048 |         ctx: *mut ggml_context,
2049 |         a: *mut ggml_tensor,
2050 |         b: *mut ggml_tensor,
2051 |     ) -> *mut ggml_tensor;
2052 | }
2053 | extern "C" {
2054 |     pub fn ggml_scale(ctx: *mut ggml_context, a: *mut ggml_tensor, s: f32) -> *mut ggml_tensor;
2055 | }
2056 | extern "C" {
2057 |     pub fn ggml_scale_inplace(
2058 |         ctx: *mut ggml_context,
2059 |         a: *mut ggml_tensor,
2060 |         s: f32,
2061 |     ) -> *mut ggml_tensor;
2062 | }
2063 | extern "C" {
2064 |     pub fn ggml_set(
2065 |         ctx: *mut ggml_context,
2066 |         a: *mut ggml_tensor,
2067 |         b: *mut ggml_tensor,
2068 |         nb1: usize,
2069 |         nb2: usize,
2070 |         nb3: usize,
2071 |         offset: usize,
2072 |     ) -> *mut ggml_tensor;
2073 | }
2074 | extern "C" {
2075 |     pub fn ggml_set_inplace(
2076 |         ctx: *mut ggml_context,
2077 |         a: *mut ggml_tensor,
2078 |         b: *mut ggml_tensor,
2079 |         nb1: usize,
2080 |         nb2: usize,
2081 |         nb3: usize,
2082 |         offset: usize,
2083 |     ) -> *mut ggml_tensor;
2084 | }
2085 | extern "C" {
2086 |     pub fn ggml_set_1d(
2087 |         ctx: *mut ggml_context,
2088 |         a: *mut ggml_tensor,
2089 |         b: *mut ggml_tensor,
2090 |         offset: usize,
2091 |     ) -> *mut ggml_tensor;
2092 | }
2093 | extern "C" {
2094 |     pub fn ggml_set_1d_inplace(
2095 |         ctx: *mut ggml_context,
2096 |         a: *mut ggml_tensor,
2097 |         b: *mut ggml_tensor,
2098 |         offset: usize,
2099 |     ) -> *mut ggml_tensor;
2100 | }
2101 | extern "C" {
2102 |     pub fn ggml_set_2d(
2103 |         ctx: *mut ggml_context,
2104 |         a: *mut ggml_tensor,
2105 |         b: *mut ggml_tensor,
2106 |         nb1: usize,
2107 |         offset: usize,
2108 |     ) -> *mut ggml_tensor;
2109 | }
2110 | extern "C" {
2111 |     pub fn ggml_set_2d_inplace(
2112 |         ctx: *mut ggml_context,
2113 |         a: *mut ggml_tensor,
2114 |         b: *mut ggml_tensor,
2115 |         nb1: usize,
2116 |         offset: usize,
2117 |     ) -> *mut ggml_tensor;
2118 | }
2119 | extern "C" {
2120 |     pub fn ggml_cpy(
2121 |         ctx: *mut ggml_context,
2122 |         a: *mut ggml_tensor,
2123 |         b: *mut ggml_tensor,
2124 |     ) -> *mut ggml_tensor;
2125 | }
2126 | extern "C" {
2127 |     pub fn ggml_cast(
2128 |         ctx: *mut ggml_context,
2129 |         a: *mut ggml_tensor,
2130 |         type_: ggml_type,
2131 |     ) -> *mut ggml_tensor;
2132 | }
2133 | extern "C" {
2134 |     pub fn ggml_cont(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
2135 | }
2136 | extern "C" {
2137 |     pub fn ggml_cont_1d(ctx: *mut ggml_context, a: *mut ggml_tensor, ne0: i64) -> *mut ggml_tensor;
2138 | }
2139 | extern "C" {
2140 |     pub fn ggml_cont_2d(
2141 |         ctx: *mut ggml_context,
2142 |         a: *mut ggml_tensor,
2143 |         ne0: i64,
2144 |         ne1: i64,
2145 |     ) -> *mut ggml_tensor;
2146 | }
2147 | extern "C" {
2148 |     pub fn ggml_cont_3d(
2149 |         ctx: *mut ggml_context,
2150 |         a: *mut ggml_tensor,
2151 |         ne0: i64,
2152 |         ne1: i64,
2153 |         ne2: i64,
2154 |     ) -> *mut ggml_tensor;
2155 | }
2156 | extern "C" {
2157 |     pub fn ggml_cont_4d(
2158 |         ctx: *mut ggml_context,
2159 |         a: *mut ggml_tensor,
2160 |         ne0: i64,
2161 |         ne1: i64,
2162 |         ne2: i64,
2163 |         ne3: i64,
2164 |     ) -> *mut ggml_tensor;
2165 | }
2166 | extern "C" {
2167 |     pub fn ggml_reshape(
2168 |         ctx: *mut ggml_context,
2169 |         a: *mut ggml_tensor,
2170 |         b: *mut ggml_tensor,
2171 |     ) -> *mut ggml_tensor;
2172 | }
2173 | extern "C" {
2174 |     pub fn ggml_reshape_1d(
2175 |         ctx: *mut ggml_context,
2176 |         a: *mut ggml_tensor,
2177 |         ne0: i64,
2178 |     ) -> *mut ggml_tensor;
2179 | }
2180 | extern "C" {
2181 |     pub fn ggml_reshape_2d(
2182 |         ctx: *mut ggml_context,
2183 |         a: *mut ggml_tensor,
2184 |         ne0: i64,
2185 |         ne1: i64,
2186 |     ) -> *mut ggml_tensor;
2187 | }
2188 | extern "C" {
2189 |     pub fn ggml_reshape_3d(
2190 |         ctx: *mut ggml_context,
2191 |         a: *mut ggml_tensor,
2192 |         ne0: i64,
2193 |         ne1: i64,
2194 |         ne2: i64,
2195 |     ) -> *mut ggml_tensor;
2196 | }
2197 | extern "C" {
2198 |     pub fn ggml_reshape_4d(
2199 |         ctx: *mut ggml_context,
2200 |         a: *mut ggml_tensor,
2201 |         ne0: i64,
2202 |         ne1: i64,
2203 |         ne2: i64,
2204 |         ne3: i64,
2205 |     ) -> *mut ggml_tensor;
2206 | }
2207 | extern "C" {
2208 |     pub fn ggml_view_1d(
2209 |         ctx: *mut ggml_context,
2210 |         a: *mut ggml_tensor,
2211 |         ne0: i64,
2212 |         offset: usize,
2213 |     ) -> *mut ggml_tensor;
2214 | }
2215 | extern "C" {
2216 |     pub fn ggml_view_2d(
2217 |         ctx: *mut ggml_context,
2218 |         a: *mut ggml_tensor,
2219 |         ne0: i64,
2220 |         ne1: i64,
2221 |         nb1: usize,
2222 |         offset: usize,
2223 |     ) -> *mut ggml_tensor;
2224 | }
2225 | extern "C" {
2226 |     pub fn ggml_view_3d(
2227 |         ctx: *mut ggml_context,
2228 |         a: *mut ggml_tensor,
2229 |         ne0: i64,
2230 |         ne1: i64,
2231 |         ne2: i64,
2232 |         nb1: usize,
2233 |         nb2: usize,
2234 |         offset: usize,
2235 |     ) -> *mut ggml_tensor;
2236 | }
2237 | extern "C" {
2238 |     pub fn ggml_view_4d(
2239 |         ctx: *mut ggml_context,
2240 |         a: *mut ggml_tensor,
2241 |         ne0: i64,
2242 |         ne1: i64,
2243 |         ne2: i64,
2244 |         ne3: i64,
2245 |         nb1: usize,
2246 |         nb2: usize,
2247 |         nb3: usize,
2248 |         offset: usize,
2249 |     ) -> *mut ggml_tensor;
2250 | }
2251 | extern "C" {
2252 |     pub fn ggml_permute(
2253 |         ctx: *mut ggml_context,
2254 |         a: *mut ggml_tensor,
2255 |         axis0: ::std::os::raw::c_int,
2256 |         axis1: ::std::os::raw::c_int,
2257 |         axis2: ::std::os::raw::c_int,
2258 |         axis3: ::std::os::raw::c_int,
2259 |     ) -> *mut ggml_tensor;
2260 | }
2261 | extern "C" {
2262 |     pub fn ggml_transpose(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
2263 | }
2264 | extern "C" {
2265 |     pub fn ggml_get_rows(
2266 |         ctx: *mut ggml_context,
2267 |         a: *mut ggml_tensor,
2268 |         b: *mut ggml_tensor,
2269 |     ) -> *mut ggml_tensor;
2270 | }
2271 | extern "C" {
2272 |     pub fn ggml_get_rows_back(
2273 |         ctx: *mut ggml_context,
2274 |         a: *mut ggml_tensor,
2275 |         b: *mut ggml_tensor,
2276 |         c: *mut ggml_tensor,
2277 |     ) -> *mut ggml_tensor;
2278 | }
2279 | extern "C" {
2280 |     pub fn ggml_diag(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
2281 | }
2282 | extern "C" {
2283 |     pub fn ggml_diag_mask_inf(
2284 |         ctx: *mut ggml_context,
2285 |         a: *mut ggml_tensor,
2286 |         n_past: ::std::os::raw::c_int,
2287 |     ) -> *mut ggml_tensor;
2288 | }
2289 | extern "C" {
2290 |     pub fn ggml_diag_mask_inf_inplace(
2291 |         ctx: *mut ggml_context,
2292 |         a: *mut ggml_tensor,
2293 |         n_past: ::std::os::raw::c_int,
2294 |     ) -> *mut ggml_tensor;
2295 | }
2296 | extern "C" {
2297 |     pub fn ggml_diag_mask_zero(
2298 |         ctx: *mut ggml_context,
2299 |         a: *mut ggml_tensor,
2300 |         n_past: ::std::os::raw::c_int,
2301 |     ) -> *mut ggml_tensor;
2302 | }
2303 | extern "C" {
2304 |     pub fn ggml_diag_mask_zero_inplace(
2305 |         ctx: *mut ggml_context,
2306 |         a: *mut ggml_tensor,
2307 |         n_past: ::std::os::raw::c_int,
2308 |     ) -> *mut ggml_tensor;
2309 | }
2310 | extern "C" {
2311 |     pub fn ggml_soft_max(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
2312 | }
2313 | extern "C" {
2314 |     pub fn ggml_soft_max_inplace(ctx: *mut ggml_context, a: *mut ggml_tensor) -> *mut ggml_tensor;
2315 | }
2316 | extern "C" {
2317 |     pub fn ggml_soft_max_ext(
2318 |         ctx: *mut ggml_context,
2319 |         a: *mut ggml_tensor,
2320 |         mask: *mut ggml_tensor,
2321 |         scale: f32,
2322 |         max_bias: f32,
2323 |     ) -> *mut ggml_tensor;
2324 | }
2325 | extern "C" {
2326 |     pub fn ggml_soft_max_back(
2327 |         ctx: *mut ggml_context,
2328 |         a: *mut ggml_tensor,
2329 |         b: *mut ggml_tensor,
2330 |     ) -> *mut ggml_tensor;
2331 | }
2332 | extern "C" {
2333 |     pub fn ggml_soft_max_back_inplace(
2334 |         ctx: *mut ggml_context,
2335 |         a: *mut ggml_tensor,
2336 |         b: *mut ggml_tensor,
2337 |     ) -> *mut ggml_tensor;
2338 | }
2339 | extern "C" {
2340 |     pub fn ggml_rope(
2341 |         ctx: *mut ggml_context,
2342 |         a: *mut ggml_tensor,
2343 |         b: *mut ggml_tensor,
2344 |         n_dims: ::std::os::raw::c_int,
2345 |         mode: ::std::os::raw::c_int,
2346 |     ) -> *mut ggml_tensor;
2347 | }
2348 | extern "C" {
2349 |     pub fn ggml_rope_inplace(
2350 |         ctx: *mut ggml_context,
2351 |         a: *mut ggml_tensor,
2352 |         b: *mut ggml_tensor,
2353 |         n_dims: ::std::os::raw::c_int,
2354 |         mode: ::std::os::raw::c_int,
2355 |     ) -> *mut ggml_tensor;
2356 | }
2357 | extern "C" {
2358 |     pub fn ggml_rope_ext(
2359 |         ctx: *mut ggml_context,
2360 |         a: *mut ggml_tensor,
2361 |         b: *mut ggml_tensor,
2362 |         c: *mut ggml_tensor,
2363 |         n_dims: ::std::os::raw::c_int,
2364 |         mode: ::std::os::raw::c_int,
2365 |         n_ctx_orig: ::std::os::raw::c_int,
2366 |         freq_base: f32,
2367 |         freq_scale: f32,
2368 |         ext_factor: f32,
2369 |         attn_factor: f32,
2370 |         beta_fast: f32,
2371 |         beta_slow: f32,
2372 |     ) -> *mut ggml_tensor;
2373 | }
2374 | extern "C" {
2375 |     pub fn ggml_rope_ext_inplace(
2376 |         ctx: *mut ggml_context,
2377 |         a: *mut ggml_tensor,
2378 |         b: *mut ggml_tensor,
2379 |         c: *mut ggml_tensor,
2380 |         n_dims: ::std::os::raw::c_int,
2381 |         mode: ::std::os::raw::c_int,
2382 |         n_ctx_orig: ::std::os::raw::c_int,
2383 |         freq_base: f32,
2384 |         freq_scale: f32,
2385 |         ext_factor: f32,
2386 |         attn_factor: f32,
2387 |         beta_fast: f32,
2388 |         beta_slow: f32,
2389 |     ) -> *mut ggml_tensor;
2390 | }
2391 | extern "C" {
2392 |     pub fn ggml_rope_custom(
2393 |         ctx: *mut ggml_context,
2394 |         a: *mut ggml_tensor,
2395 |         b: *mut ggml_tensor,
2396 |         n_dims: ::std::os::raw::c_int,
2397 |         mode: ::std::os::raw::c_int,
2398 |         n_ctx_orig: ::std::os::raw::c_int,
2399 |         freq_base: f32,
2400 |         freq_scale: f32,
2401 |         ext_factor: f32,
2402 |         attn_factor: f32,
2403 |         beta_fast: f32,
2404 |         beta_slow: f32,
2405 |     ) -> *mut ggml_tensor;
2406 | }
2407 | extern "C" {
2408 |     pub fn ggml_rope_custom_inplace(
2409 |         ctx: *mut ggml_context,
2410 |         a: *mut ggml_tensor,
2411 |         b: *mut ggml_tensor,
2412 |         n_dims: ::std::os::raw::c_int,
2413 |         mode: ::std::os::raw::c_int,
2414 |         n_ctx_orig: ::std::os::raw::c_int,
2415 |         freq_base: f32,
2416 |         freq_scale: f32,
2417 |         ext_factor: f32,
2418 |         attn_factor: f32,
2419 |         beta_fast: f32,
2420 |         beta_slow: f32,
2421 |     ) -> *mut ggml_tensor;
2422 | }
2423 | extern "C" {
2424 |     pub fn ggml_rope_yarn_corr_dims(
2425 |         n_dims: ::std::os::raw::c_int,
2426 |         n_ctx_orig: ::std::os::raw::c_int,
2427 |         freq_base: f32,
2428 |         beta_fast: f32,
2429 |         beta_slow: f32,
2430 |         dims: *mut f32,
2431 |     );
2432 | }
2433 | extern "C" {
2434 |     pub fn ggml_rope_back(
2435 |         ctx: *mut ggml_context,
2436 |         a: *mut ggml_tensor,
2437 |         b: *mut ggml_tensor,
2438 |         c: *mut ggml_tensor,
2439 |         n_dims: ::std::os::raw::c_int,
2440 |         mode: ::std::os::raw::c_int,
2441 |         n_ctx_orig: ::std::os::raw::c_int,
2442 |         freq_base: f32,
2443 |         freq_scale: f32,
2444 |         ext_factor: f32,
2445 |         attn_factor: f32,
2446 |         beta_fast: f32,
2447 |         beta_slow: f32,
2448 |     ) -> *mut ggml_tensor;
2449 | }
2450 | extern "C" {
2451 |     pub fn ggml_clamp(
2452 |         ctx: *mut ggml_context,
2453 |         a: *mut ggml_tensor,
2454 |         min: f32,
2455 |         max: f32,
2456 |     ) -> *mut ggml_tensor;
2457 | }
2458 | extern "C" {
2459 |     pub fn ggml_im2col(
2460 |         ctx: *mut ggml_context,
2461 |         a: *mut ggml_tensor,
2462 |         b: *mut ggml_tensor,
2463 |         s0: ::std::os::raw::c_int,
2464 |         s1: ::std::os::raw::c_int,
2465 |         p0: ::std::os::raw::c_int,
2466 |         p1: ::std::os::raw::c_int,
2467 |         d0: ::std::os::raw::c_int,
2468 |         d1: ::std::os::raw::c_int,
2469 |         is_2D: bool,
2470 |         dst_type: ggml_type,
2471 |     ) -> *mut ggml_tensor;
2472 | }
2473 | extern "C" {
2474 |     pub fn ggml_im2col_back(
2475 |         ctx: *mut ggml_context,
2476 |         a: *mut ggml_tensor,
2477 |         b: *mut ggml_tensor,
2478 |         ne: *mut i64,
2479 |         s0: ::std::os::raw::c_int,
2480 |         s1: ::std::os::raw::c_int,
2481 |         p0: ::std::os::raw::c_int,
2482 |         p1: ::std::os::raw::c_int,
2483 |         d0: ::std::os::raw::c_int,
2484 |         d1: ::std::os::raw::c_int,
2485 |         is_2D: bool,
2486 |     ) -> *mut ggml_tensor;
2487 | }
2488 | extern "C" {
2489 |     pub fn ggml_conv_depthwise_2d(
2490 |         ctx: *mut ggml_context,
2491 |         a: *mut ggml_tensor,
2492 |         b: *mut ggml_tensor,
2493 |         s0: ::std::os::raw::c_int,
2494 |         s1: ::std::os::raw::c_int,
2495 |         p0: ::std::os::raw::c_int,
2496 |         p1: ::std::os::raw::c_int,
2497 |         d0: ::std::os::raw::c_int,
2498 |         d1: ::std::os::raw::c_int,
2499 |     ) -> *mut ggml_tensor;
2500 | }
2501 | extern "C" {
2502 |     pub fn ggml_conv_1d(
2503 |         ctx: *mut ggml_context,
2504 |         a: *mut ggml_tensor,
2505 |         b: *mut ggml_tensor,
2506 |         s0: ::std::os::raw::c_int,
2507 |         p0: ::std::os::raw::c_int,
2508 |         d0: ::std::os::raw::c_int,
2509 |     ) -> *mut ggml_tensor;
2510 | }
2511 | extern "C" {
2512 |     pub fn ggml_conv_1d_ph(
2513 |         ctx: *mut ggml_context,
2514 |         a: *mut ggml_tensor,
2515 |         b: *mut ggml_tensor,
2516 |         s: ::std::os::raw::c_int,
2517 |         d: ::std::os::raw::c_int,
2518 |     ) -> *mut ggml_tensor;
2519 | }
2520 | extern "C" {
2521 |     pub fn ggml_conv_transpose_1d(
2522 |         ctx: *mut ggml_context,
2523 |         a: *mut ggml_tensor,
2524 |         b: *mut ggml_tensor,
2525 |         s0: ::std::os::raw::c_int,
2526 |         p0: ::std::os::raw::c_int,
2527 |         d0: ::std::os::raw::c_int,
2528 |     ) -> *mut ggml_tensor;
2529 | }
2530 | extern "C" {
2531 |     pub fn ggml_conv_2d(
2532 |         ctx: *mut ggml_context,
2533 |         a: *mut ggml_tensor,
2534 |         b: *mut ggml_tensor,
2535 |         s0: ::std::os::raw::c_int,
2536 |         s1: ::std::os::raw::c_int,
2537 |         p0: ::std::os::raw::c_int,
2538 |         p1: ::std::os::raw::c_int,
2539 |         d0: ::std::os::raw::c_int,
2540 |         d1: ::std::os::raw::c_int,
2541 |     ) -> *mut ggml_tensor;
2542 | }
2543 | extern "C" {
2544 |     pub fn ggml_conv_2d_sk_p0(
2545 |         ctx: *mut ggml_context,
2546 |         a: *mut ggml_tensor,
2547 |         b: *mut ggml_tensor,
2548 |     ) -> *mut ggml_tensor;
2549 | }
2550 | extern "C" {
2551 |     pub fn ggml_conv_2d_s1_ph(
2552 |         ctx: *mut ggml_context,
2553 |         a: *mut ggml_tensor,
2554 |         b: *mut ggml_tensor,
2555 |     ) -> *mut ggml_tensor;
2556 | }
2557 | extern "C" {
2558 |     pub fn ggml_conv_transpose_2d_p0(
2559 |         ctx: *mut ggml_context,
2560 |         a: *mut ggml_tensor,
2561 |         b: *mut ggml_tensor,
2562 |         stride: ::std::os::raw::c_int,
2563 |     ) -> *mut ggml_tensor;
2564 | }
2565 | pub const ggml_op_pool_GGML_OP_POOL_MAX: ggml_op_pool = 0;
2566 | pub const ggml_op_pool_GGML_OP_POOL_AVG: ggml_op_pool = 1;
2567 | pub const ggml_op_pool_GGML_OP_POOL_COUNT: ggml_op_pool = 2;
2568 | pub type ggml_op_pool = ::std::os::raw::c_uint;
2569 | extern "C" {
2570 |     pub fn ggml_pool_1d(
2571 |         ctx: *mut ggml_context,
2572 |         a: *mut ggml_tensor,
2573 |         op: ggml_op_pool,
2574 |         k0: ::std::os::raw::c_int,
2575 |         s0: ::std::os::raw::c_int,
2576 |         p0: ::std::os::raw::c_int,
2577 |     ) -> *mut ggml_tensor;
2578 | }
2579 | extern "C" {
2580 |     pub fn ggml_pool_2d(
2581 |         ctx: *mut ggml_context,
2582 |         a: *mut ggml_tensor,
2583 |         op: ggml_op_pool,
2584 |         k0: ::std::os::raw::c_int,
2585 |         k1: ::std::os::raw::c_int,
2586 |         s0: ::std::os::raw::c_int,
2587 |         s1: ::std::os::raw::c_int,
2588 |         p0: f32,
2589 |         p1: f32,
2590 |     ) -> *mut ggml_tensor;
2591 | }
2592 | extern "C" {
2593 |     pub fn ggml_pool_2d_back(
2594 |         ctx: *mut ggml_context,
2595 |         a: *mut ggml_tensor,
2596 |         af: *mut ggml_tensor,
2597 |         op: ggml_op_pool,
2598 |         k0: ::std::os::raw::c_int,
2599 |         k1: ::std::os::raw::c_int,
2600 |         s0: ::std::os::raw::c_int,
2601 |         s1: ::std::os::raw::c_int,
2602 |         p0: f32,
2603 |         p1: f32,
2604 |     ) -> *mut ggml_tensor;
2605 | }
2606 | extern "C" {
2607 |     pub fn ggml_upscale(
2608 |         ctx: *mut ggml_context,
2609 |         a: *mut ggml_tensor,
2610 |         scale_factor: ::std::os::raw::c_int,
2611 |     ) -> *mut ggml_tensor;
2612 | }
2613 | extern "C" {
2614 |     pub fn ggml_upscale_ext(
2615 |         ctx: *mut ggml_context,
2616 |         a: *mut ggml_tensor,
2617 |         ne0: ::std::os::raw::c_int,
2618 |         ne1: ::std::os::raw::c_int,
2619 |         ne2: ::std::os::raw::c_int,
2620 |         ne3: ::std::os::raw::c_int,
2621 |     ) -> *mut ggml_tensor;
2622 | }
2623 | extern "C" {
2624 |     pub fn ggml_pad(
2625 |         ctx: *mut ggml_context,
2626 |         a: *mut ggml_tensor,
2627 |         p0: ::std::os::raw::c_int,
2628 |         p1: ::std::os::raw::c_int,
2629 |         p2: ::std::os::raw::c_int,
2630 |         p3: ::std::os::raw::c_int,
2631 |     ) -> *mut ggml_tensor;
2632 | }
2633 | extern "C" {
2634 |     pub fn ggml_timestep_embedding(
2635 |         ctx: *mut ggml_context,
2636 |         timesteps: *mut ggml_tensor,
2637 |         dim: ::std::os::raw::c_int,
2638 |         max_period: ::std::os::raw::c_int,
2639 |     ) -> *mut ggml_tensor;
2640 | }
2641 | pub const ggml_sort_order_GGML_SORT_ORDER_ASC: ggml_sort_order = 0;
2642 | pub const ggml_sort_order_GGML_SORT_ORDER_DESC: ggml_sort_order = 1;
2643 | pub type ggml_sort_order = ::std::os::raw::c_uint;
2644 | extern "C" {
2645 |     pub fn ggml_argsort(
2646 |         ctx: *mut ggml_context,
2647 |         a: *mut ggml_tensor,
2648 |         order: ggml_sort_order,
2649 |     ) -> *mut ggml_tensor;
2650 | }
2651 | extern "C" {
2652 |     pub fn ggml_arange(
2653 |         ctx: *mut ggml_context,
2654 |         start: f32,
2655 |         stop: f32,
2656 |         step: f32,
2657 |     ) -> *mut ggml_tensor;
2658 | }
2659 | extern "C" {
2660 |     pub fn ggml_top_k(
2661 |         ctx: *mut ggml_context,
2662 |         a: *mut ggml_tensor,
2663 |         k: ::std::os::raw::c_int,
2664 |     ) -> *mut ggml_tensor;
2665 | }
2666 | extern "C" {
2667 |     pub fn ggml_flash_attn_ext(
2668 |         ctx: *mut ggml_context,
2669 |         q: *mut ggml_tensor,
2670 |         k: *mut ggml_tensor,
2671 |         v: *mut ggml_tensor,
2672 |         mask: *mut ggml_tensor,
2673 |         scale: f32,
2674 |         max_bias: f32,
2675 |         logit_softcap: f32,
2676 |     ) -> *mut ggml_tensor;
2677 | }
2678 | extern "C" {
2679 |     pub fn ggml_flash_attn_ext_set_prec(a: *mut ggml_tensor, prec: ggml_prec);
2680 | }
2681 | extern "C" {
2682 |     pub fn ggml_flash_attn_back(
2683 |         ctx: *mut ggml_context,
2684 |         q: *mut ggml_tensor,
2685 |         k: *mut ggml_tensor,
2686 |         v: *mut ggml_tensor,
2687 |         d: *mut ggml_tensor,
2688 |         masked: bool,
2689 |     ) -> *mut ggml_tensor;
2690 | }
2691 | extern "C" {
2692 |     pub fn ggml_ssm_conv(
2693 |         ctx: *mut ggml_context,
2694 |         sx: *mut ggml_tensor,
2695 |         c: *mut ggml_tensor,
2696 |     ) -> *mut ggml_tensor;
2697 | }
2698 | extern "C" {
2699 |     pub fn ggml_ssm_scan(
2700 |         ctx: *mut ggml_context,
2701 |         s: *mut ggml_tensor,
2702 |         x: *mut ggml_tensor,
2703 |         dt: *mut ggml_tensor,
2704 |         A: *mut ggml_tensor,
2705 |         B: *mut ggml_tensor,
2706 |         C: *mut ggml_tensor,
2707 |     ) -> *mut ggml_tensor;
2708 | }
2709 | extern "C" {
2710 |     pub fn ggml_win_part(
2711 |         ctx: *mut ggml_context,
2712 |         a: *mut ggml_tensor,
2713 |         w: ::std::os::raw::c_int,
2714 |     ) -> *mut ggml_tensor;
2715 | }
2716 | extern "C" {
2717 |     pub fn ggml_win_unpart(
2718 |         ctx: *mut ggml_context,
2719 |         a: *mut ggml_tensor,
2720 |         w0: ::std::os::raw::c_int,
2721 |         h0: ::std::os::raw::c_int,
2722 |         w: ::std::os::raw::c_int,
2723 |     ) -> *mut ggml_tensor;
2724 | }
2725 | extern "C" {
2726 |     pub fn ggml_unary(
2727 |         ctx: *mut ggml_context,
2728 |         a: *mut ggml_tensor,
2729 |         op: ggml_unary_op,
2730 |     ) -> *mut ggml_tensor;
2731 | }
2732 | extern "C" {
2733 |     pub fn ggml_unary_inplace(
2734 |         ctx: *mut ggml_context,
2735 |         a: *mut ggml_tensor,
2736 |         op: ggml_unary_op,
2737 |     ) -> *mut ggml_tensor;
2738 | }
2739 | extern "C" {
2740 |     pub fn ggml_get_rel_pos(
2741 |         ctx: *mut ggml_context,
2742 |         a: *mut ggml_tensor,
2743 |         qh: ::std::os::raw::c_int,
2744 |         kh: ::std::os::raw::c_int,
2745 |     ) -> *mut ggml_tensor;
2746 | }
2747 | extern "C" {
2748 |     pub fn ggml_add_rel_pos(
2749 |         ctx: *mut ggml_context,
2750 |         a: *mut ggml_tensor,
2751 |         pw: *mut ggml_tensor,
2752 |         ph: *mut ggml_tensor,
2753 |     ) -> *mut ggml_tensor;
2754 | }
2755 | extern "C" {
2756 |     pub fn ggml_add_rel_pos_inplace(
2757 |         ctx: *mut ggml_context,
2758 |         a: *mut ggml_tensor,
2759 |         pw: *mut ggml_tensor,
2760 |         ph: *mut ggml_tensor,
2761 |     ) -> *mut ggml_tensor;
2762 | }
2763 | extern "C" {
2764 |     pub fn ggml_rwkv_wkv(
2765 |         ctx: *mut ggml_context,
2766 |         k: *mut ggml_tensor,
2767 |         v: *mut ggml_tensor,
2768 |         r: *mut ggml_tensor,
2769 |         tf: *mut ggml_tensor,
2770 |         td: *mut ggml_tensor,
2771 |         state: *mut ggml_tensor,
2772 |     ) -> *mut ggml_tensor;
2773 | }
2774 | pub type ggml_unary_op_f32_t = ::std::option::Option<
2775 |     unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *mut f32, arg3: *const f32),
2776 | >;
2777 | pub type ggml_binary_op_f32_t = ::std::option::Option<
2778 |     unsafe extern "C" fn(
2779 |         arg1: ::std::os::raw::c_int,
2780 |         arg2: *mut f32,
2781 |         arg3: *const f32,
2782 |         arg4: *const f32,
2783 |     ),
2784 | >;
2785 | pub type ggml_custom1_op_f32_t =
2786 |     ::std::option::Option<unsafe extern "C" fn(arg1: *mut ggml_tensor, arg2: *const ggml_tensor)>;
2787 | pub type ggml_custom2_op_f32_t = ::std::option::Option<
2788 |     unsafe extern "C" fn(
2789 |         arg1: *mut ggml_tensor,
2790 |         arg2: *const ggml_tensor,
2791 |         arg3: *const ggml_tensor,
2792 |     ),
2793 | >;
2794 | pub type ggml_custom3_op_f32_t = ::std::option::Option<
2795 |     unsafe extern "C" fn(
2796 |         arg1: *mut ggml_tensor,
2797 |         arg2: *const ggml_tensor,
2798 |         arg3: *const ggml_tensor,
2799 |         arg4: *const ggml_tensor,
2800 |     ),
2801 | >;
2802 | extern "C" {
2803 |     pub fn ggml_map_unary_f32(
2804 |         ctx: *mut ggml_context,
2805 |         a: *mut ggml_tensor,
2806 |         fun: ggml_unary_op_f32_t,
2807 |     ) -> *mut ggml_tensor;
2808 | }
2809 | extern "C" {
2810 |     pub fn ggml_map_unary_inplace_f32(
2811 |         ctx: *mut ggml_context,
2812 |         a: *mut ggml_tensor,
2813 |         fun: ggml_unary_op_f32_t,
2814 |     ) -> *mut ggml_tensor;
2815 | }
2816 | extern "C" {
2817 |     pub fn ggml_map_binary_f32(
2818 |         ctx: *mut ggml_context,
2819 |         a: *mut ggml_tensor,
2820 |         b: *mut ggml_tensor,
2821 |         fun: ggml_binary_op_f32_t,
2822 |     ) -> *mut ggml_tensor;
2823 | }
2824 | extern "C" {
2825 |     pub fn ggml_map_binary_inplace_f32(
2826 |         ctx: *mut ggml_context,
2827 |         a: *mut ggml_tensor,
2828 |         b: *mut ggml_tensor,
2829 |         fun: ggml_binary_op_f32_t,
2830 |     ) -> *mut ggml_tensor;
2831 | }
2832 | extern "C" {
2833 |     pub fn ggml_map_custom1_f32(
2834 |         ctx: *mut ggml_context,
2835 |         a: *mut ggml_tensor,
2836 |         fun: ggml_custom1_op_f32_t,
2837 |     ) -> *mut ggml_tensor;
2838 | }
2839 | extern "C" {
2840 |     pub fn ggml_map_custom1_inplace_f32(
2841 |         ctx: *mut ggml_context,
2842 |         a: *mut ggml_tensor,
2843 |         fun: ggml_custom1_op_f32_t,
2844 |     ) -> *mut ggml_tensor;
2845 | }
2846 | extern "C" {
2847 |     pub fn ggml_map_custom2_f32(
2848 |         ctx: *mut ggml_context,
2849 |         a: *mut ggml_tensor,
2850 |         b: *mut ggml_tensor,
2851 |         fun: ggml_custom2_op_f32_t,
2852 |     ) -> *mut ggml_tensor;
2853 | }
2854 | extern "C" {
2855 |     pub fn ggml_map_custom2_inplace_f32(
2856 |         ctx: *mut ggml_context,
2857 |         a: *mut ggml_tensor,
2858 |         b: *mut ggml_tensor,
2859 |         fun: ggml_custom2_op_f32_t,
2860 |     ) -> *mut ggml_tensor;
2861 | }
2862 | extern "C" {
2863 |     pub fn ggml_map_custom3_f32(
2864 |         ctx: *mut ggml_context,
2865 |         a: *mut ggml_tensor,
2866 |         b: *mut ggml_tensor,
2867 |         c: *mut ggml_tensor,
2868 |         fun: ggml_custom3_op_f32_t,
2869 |     ) -> *mut ggml_tensor;
2870 | }
2871 | extern "C" {
2872 |     pub fn ggml_map_custom3_inplace_f32(
2873 |         ctx: *mut ggml_context,
2874 |         a: *mut ggml_tensor,
2875 |         b: *mut ggml_tensor,
2876 |         c: *mut ggml_tensor,
2877 |         fun: ggml_custom3_op_f32_t,
2878 |     ) -> *mut ggml_tensor;
2879 | }
2880 | pub type ggml_custom1_op_t = ::std::option::Option<
2881 |     unsafe extern "C" fn(
2882 |         dst: *mut ggml_tensor,
2883 |         a: *const ggml_tensor,
2884 |         ith: ::std::os::raw::c_int,
2885 |         nth: ::std::os::raw::c_int,
2886 |         userdata: *mut ::std::os::raw::c_void,
2887 |     ),
2888 | >;
2889 | pub type ggml_custom2_op_t = ::std::option::Option<
2890 |     unsafe extern "C" fn(
2891 |         dst: *mut ggml_tensor,
2892 |         a: *const ggml_tensor,
2893 |         b: *const ggml_tensor,
2894 |         ith: ::std::os::raw::c_int,
2895 |         nth: ::std::os::raw::c_int,
2896 |         userdata: *mut ::std::os::raw::c_void,
2897 |     ),
2898 | >;
2899 | pub type ggml_custom3_op_t = ::std::option::Option<
2900 |     unsafe extern "C" fn(
2901 |         dst: *mut ggml_tensor,
2902 |         a: *const ggml_tensor,
2903 |         b: *const ggml_tensor,
2904 |         c: *const ggml_tensor,
2905 |         ith: ::std::os::raw::c_int,
2906 |         nth: ::std::os::raw::c_int,
2907 |         userdata: *mut ::std::os::raw::c_void,
2908 |     ),
2909 | >;
2910 | extern "C" {
2911 |     pub fn ggml_map_custom1(
2912 |         ctx: *mut ggml_context,
2913 |         a: *mut ggml_tensor,
2914 |         fun: ggml_custom1_op_t,
2915 |         n_tasks: ::std::os::raw::c_int,
2916 |         userdata: *mut ::std::os::raw::c_void,
2917 |     ) -> *mut ggml_tensor;
2918 | }
2919 | extern "C" {
2920 |     pub fn ggml_map_custom1_inplace(
2921 |         ctx: *mut ggml_context,
2922 |         a: *mut ggml_tensor,
2923 |         fun: ggml_custom1_op_t,
2924 |         n_tasks: ::std::os::raw::c_int,
2925 |         userdata: *mut ::std::os::raw::c_void,
2926 |     ) -> *mut ggml_tensor;
2927 | }
2928 | extern "C" {
2929 |     pub fn ggml_map_custom2(
2930 |         ctx: *mut ggml_context,
2931 |         a: *mut ggml_tensor,
2932 |         b: *mut ggml_tensor,
2933 |         fun: ggml_custom2_op_t,
2934 |         n_tasks: ::std::os::raw::c_int,
2935 |         userdata: *mut ::std::os::raw::c_void,
2936 |     ) -> *mut ggml_tensor;
2937 | }
2938 | extern "C" {
2939 |     pub fn ggml_map_custom2_inplace(
2940 |         ctx: *mut ggml_context,
2941 |         a: *mut ggml_tensor,
2942 |         b: *mut ggml_tensor,
2943 |         fun: ggml_custom2_op_t,
2944 |         n_tasks: ::std::os::raw::c_int,
2945 |         userdata: *mut ::std::os::raw::c_void,
2946 |     ) -> *mut ggml_tensor;
2947 | }
2948 | extern "C" {
2949 |     pub fn ggml_map_custom3(
2950 |         ctx: *mut ggml_context,
2951 |         a: *mut ggml_tensor,
2952 |         b: *mut ggml_tensor,
2953 |         c: *mut ggml_tensor,
2954 |         fun: ggml_custom3_op_t,
2955 |         n_tasks: ::std::os::raw::c_int,
2956 |         userdata: *mut ::std::os::raw::c_void,
2957 |     ) -> *mut ggml_tensor;
2958 | }
2959 | extern "C" {
2960 |     pub fn ggml_map_custom3_inplace(
2961 |         ctx: *mut ggml_context,
2962 |         a: *mut ggml_tensor,
2963 |         b: *mut ggml_tensor,
2964 |         c: *mut ggml_tensor,
2965 |         fun: ggml_custom3_op_t,
2966 |         n_tasks: ::std::os::raw::c_int,
2967 |         userdata: *mut ::std::os::raw::c_void,
2968 |     ) -> *mut ggml_tensor;
2969 | }
2970 | extern "C" {
2971 |     pub fn ggml_cross_entropy_loss(
2972 |         ctx: *mut ggml_context,
2973 |         a: *mut ggml_tensor,
2974 |         b: *mut ggml_tensor,
2975 |     ) -> *mut ggml_tensor;
2976 | }
2977 | extern "C" {
2978 |     pub fn ggml_cross_entropy_loss_back(
2979 |         ctx: *mut ggml_context,
2980 |         a: *mut ggml_tensor,
2981 |         b: *mut ggml_tensor,
2982 |         c: *mut ggml_tensor,
2983 |     ) -> *mut ggml_tensor;
2984 | }
2985 | extern "C" {
2986 |     pub fn ggml_opt_step_adamw(
2987 |         ctx: *mut ggml_context,
2988 |         a: *mut ggml_tensor,
2989 |         grad: *mut ggml_tensor,
2990 |         alpha: f32,
2991 |         beta1: f32,
2992 |         beta2: f32,
2993 |         eps: f32,
2994 |         wd: f32,
2995 |     ) -> *mut ggml_tensor;
2996 | }
2997 | extern "C" {
2998 |     pub fn ggml_set_param(ctx: *mut ggml_context, tensor: *mut ggml_tensor);
2999 | }
3000 | extern "C" {
3001 |     pub fn ggml_set_loss(tensor: *mut ggml_tensor);
3002 | }
3003 | extern "C" {
3004 |     pub fn ggml_build_forward_expand(cgraph: *mut ggml_cgraph, tensor: *mut ggml_tensor);
3005 | }
3006 | extern "C" {
3007 |     pub fn ggml_build_backward_expand(
3008 |         ctx: *mut ggml_context,
3009 |         gf: *mut ggml_cgraph,
3010 |         gb: *mut ggml_cgraph,
3011 |         accumulate: bool,
3012 |     );
3013 | }
3014 | extern "C" {
3015 |     pub fn ggml_build_opt_adamw(
3016 |         ctx: *mut ggml_context,
3017 |         gf: *mut ggml_cgraph,
3018 |         gb: *mut ggml_cgraph,
3019 |         alpha: f32,
3020 |         beta1: f32,
3021 |         beta2: f32,
3022 |         eps: f32,
3023 |         wd: f32,
3024 |     );
3025 | }
3026 | extern "C" {
3027 |     pub fn ggml_new_graph(ctx: *mut ggml_context) -> *mut ggml_cgraph;
3028 | }
3029 | extern "C" {
3030 |     pub fn ggml_new_graph_custom(
3031 |         ctx: *mut ggml_context,
3032 |         size: usize,
3033 |         grads: bool,
3034 |     ) -> *mut ggml_cgraph;
3035 | }
3036 | extern "C" {
3037 |     pub fn ggml_graph_dup(ctx: *mut ggml_context, cgraph: *mut ggml_cgraph) -> *mut ggml_cgraph;
3038 | }
3039 | extern "C" {
3040 |     pub fn ggml_graph_cpy(src: *mut ggml_cgraph, dst: *mut ggml_cgraph);
3041 | }
3042 | extern "C" {
3043 |     pub fn ggml_graph_reset(cgraph: *mut ggml_cgraph);
3044 | }
3045 | extern "C" {
3046 |     pub fn ggml_graph_clear(cgraph: *mut ggml_cgraph);
3047 | }
3048 | extern "C" {
3049 |     pub fn ggml_graph_size(cgraph: *mut ggml_cgraph) -> ::std::os::raw::c_int;
3050 | }
3051 | extern "C" {
3052 |     pub fn ggml_graph_node(cgraph: *mut ggml_cgraph, i: ::std::os::raw::c_int) -> *mut ggml_tensor;
3053 | }
3054 | extern "C" {
3055 |     pub fn ggml_graph_nodes(cgraph: *mut ggml_cgraph) -> *mut *mut ggml_tensor;
3056 | }
3057 | extern "C" {
3058 |     pub fn ggml_graph_n_nodes(cgraph: *mut ggml_cgraph) -> ::std::os::raw::c_int;
3059 | }
3060 | extern "C" {
3061 |     pub fn ggml_graph_add_node(cgraph: *mut ggml_cgraph, tensor: *mut ggml_tensor);
3062 | }
3063 | extern "C" {
3064 |     pub fn ggml_graph_overhead() -> usize;
3065 | }
3066 | extern "C" {
3067 |     pub fn ggml_graph_overhead_custom(size: usize, grads: bool) -> usize;
3068 | }
3069 | extern "C" {
3070 |     pub fn ggml_threadpool_params_default(
3071 |         n_threads: ::std::os::raw::c_int,
3072 |     ) -> ggml_threadpool_params;
3073 | }
3074 | extern "C" {
3075 |     pub fn ggml_threadpool_params_init(
3076 |         p: *mut ggml_threadpool_params,
3077 |         n_threads: ::std::os::raw::c_int,
3078 |     );
3079 | }
3080 | extern "C" {
3081 |     pub fn ggml_threadpool_params_match(
3082 |         p0: *const ggml_threadpool_params,
3083 |         p1: *const ggml_threadpool_params,
3084 |     ) -> bool;
3085 | }
3086 | extern "C" {
3087 |     pub fn ggml_threadpool_new(params: *mut ggml_threadpool_params) -> *mut ggml_threadpool;
3088 | }
3089 | extern "C" {
3090 |     pub fn ggml_threadpool_free(threadpool: *mut ggml_threadpool);
3091 | }
3092 | extern "C" {
3093 |     pub fn ggml_threadpool_get_n_threads(threadpool: *mut ggml_threadpool)
3094 |         -> ::std::os::raw::c_int;
3095 | }
3096 | extern "C" {
3097 |     pub fn ggml_threadpool_pause(threadpool: *mut ggml_threadpool);
3098 | }
3099 | extern "C" {
3100 |     pub fn ggml_threadpool_resume(threadpool: *mut ggml_threadpool);
3101 | }
3102 | extern "C" {
3103 |     pub fn ggml_graph_plan(
3104 |         cgraph: *const ggml_cgraph,
3105 |         n_threads: ::std::os::raw::c_int,
3106 |         threadpool: *mut ggml_threadpool,
3107 |     ) -> ggml_cplan;
3108 | }
3109 | extern "C" {
3110 |     pub fn ggml_graph_compute(cgraph: *mut ggml_cgraph, cplan: *mut ggml_cplan) -> ggml_status;
3111 | }
3112 | extern "C" {
3113 |     pub fn ggml_graph_compute_with_ctx(
3114 |         ctx: *mut ggml_context,
3115 |         cgraph: *mut ggml_cgraph,
3116 |         n_threads: ::std::os::raw::c_int,
3117 |     ) -> ggml_status;
3118 | }
3119 | extern "C" {
3120 |     pub fn ggml_graph_get_tensor(
3121 |         cgraph: *mut ggml_cgraph,
3122 |         name: *const ::std::os::raw::c_char,
3123 |     ) -> *mut ggml_tensor;
3124 | }
3125 | extern "C" {
3126 |     pub fn ggml_graph_export(cgraph: *const ggml_cgraph, fname: *const ::std::os::raw::c_char);
3127 | }
3128 | extern "C" {
3129 |     pub fn ggml_graph_import(
3130 |         fname: *const ::std::os::raw::c_char,
3131 |         ctx_data: *mut *mut ggml_context,
3132 |         ctx_eval: *mut *mut ggml_context,
3133 |     ) -> *mut ggml_cgraph;
3134 | }
3135 | extern "C" {
3136 |     pub fn ggml_graph_print(cgraph: *const ggml_cgraph);
3137 | }
3138 | extern "C" {
3139 |     pub fn ggml_graph_dump_dot(
3140 |         gb: *const ggml_cgraph,
3141 |         gf: *const ggml_cgraph,
3142 |         filename: *const ::std::os::raw::c_char,
3143 |     );
3144 | }
3145 | extern "C" {
3146 |     pub fn ggml_build_backward_gradient_checkpointing(
3147 |         ctx: *mut ggml_context,
3148 |         gf: *mut ggml_cgraph,
3149 |         gb: *mut ggml_cgraph,
3150 |         gb_tmp: *mut ggml_cgraph,
3151 |         checkpoints: *mut *mut ggml_tensor,
3152 |         n_checkpoints: ::std::os::raw::c_int,
3153 |     );
3154 | }
3155 | pub const ggml_opt_type_GGML_OPT_TYPE_ADAM: ggml_opt_type = 0;
3156 | pub const ggml_opt_type_GGML_OPT_TYPE_LBFGS: ggml_opt_type = 1;
3157 | pub type ggml_opt_type = ::std::os::raw::c_uint;
3158 | pub const ggml_linesearch_GGML_LINESEARCH_DEFAULT: ggml_linesearch = 1;
3159 | pub const ggml_linesearch_GGML_LINESEARCH_BACKTRACKING_ARMIJO: ggml_linesearch = 0;
3160 | pub const ggml_linesearch_GGML_LINESEARCH_BACKTRACKING_WOLFE: ggml_linesearch = 1;
3161 | pub const ggml_linesearch_GGML_LINESEARCH_BACKTRACKING_STRONG_WOLFE: ggml_linesearch = 2;
3162 | pub type ggml_linesearch = ::std::os::raw::c_uint;
3163 | pub const ggml_opt_result_GGML_OPT_RESULT_OK: ggml_opt_result = 0;
3164 | pub const ggml_opt_result_GGML_OPT_RESULT_DID_NOT_CONVERGE: ggml_opt_result = 1;
3165 | pub const ggml_opt_result_GGML_OPT_RESULT_NO_CONTEXT: ggml_opt_result = 2;
3166 | pub const ggml_opt_result_GGML_OPT_RESULT_INVALID_WOLFE: ggml_opt_result = 3;
3167 | pub const ggml_opt_result_GGML_OPT_RESULT_FAIL: ggml_opt_result = 4;
3168 | pub const ggml_opt_result_GGML_OPT_RESULT_CANCEL: ggml_opt_result = 5;
3169 | pub const ggml_opt_result_GGML_LINESEARCH_FAIL: ggml_opt_result = -128;
3170 | pub const ggml_opt_result_GGML_LINESEARCH_MINIMUM_STEP: ggml_opt_result = -127;
3171 | pub const ggml_opt_result_GGML_LINESEARCH_MAXIMUM_STEP: ggml_opt_result = -126;
3172 | pub const ggml_opt_result_GGML_LINESEARCH_MAXIMUM_ITERATIONS: ggml_opt_result = -125;
3173 | pub const ggml_opt_result_GGML_LINESEARCH_INVALID_PARAMETERS: ggml_opt_result = -124;
3174 | pub type ggml_opt_result = ::std::os::raw::c_int;
3175 | pub type ggml_opt_callback = ::std::option::Option<
3176 |     unsafe extern "C" fn(
3177 |         data: *mut ::std::os::raw::c_void,
3178 |         accum_step: ::std::os::raw::c_int,
3179 |         sched: *mut f32,
3180 |         cancel: *mut bool,
3181 |     ),
3182 | >;
3183 | pub type ggml_log_callback = ::std::option::Option<
3184 |     unsafe extern "C" fn(
3185 |         level: ggml_log_level,
3186 |         text: *const ::std::os::raw::c_char,
3187 |         user_data: *mut ::std::os::raw::c_void,
3188 |     ),
3189 | >;
3190 | extern "C" {
3191 |     pub fn ggml_log_set(log_callback: ggml_log_callback, user_data: *mut ::std::os::raw::c_void);
3192 | }
3193 | #[repr(C)]
3194 | #[derive(Debug, Copy, Clone)]
3195 | pub struct ggml_opt_params {
3196 |     pub type_: ggml_opt_type,
3197 |     pub graph_size: usize,
3198 |     pub n_threads: ::std::os::raw::c_int,
3199 |     pub past: ::std::os::raw::c_int,
3200 |     pub delta: f32,
3201 |     pub max_no_improvement: ::std::os::raw::c_int,
3202 |     pub print_forward_graph: bool,
3203 |     pub print_backward_graph: bool,
3204 |     pub n_gradient_accumulation: ::std::os::raw::c_int,
3205 |     pub adam: ggml_opt_params__bindgen_ty_1,
3206 |     pub lbfgs: ggml_opt_params__bindgen_ty_2,
3207 | }
3208 | #[repr(C)]
3209 | #[derive(Debug, Copy, Clone)]
3210 | pub struct ggml_opt_params__bindgen_ty_1 {
3211 |     pub n_iter: ::std::os::raw::c_int,
3212 |     pub sched: f32,
3213 |     pub decay: f32,
3214 |     pub decay_min_ndim: ::std::os::raw::c_int,
3215 |     pub alpha: f32,
3216 |     pub beta1: f32,
3217 |     pub beta2: f32,
3218 |     pub eps: f32,
3219 |     pub eps_f: f32,
3220 |     pub eps_g: f32,
3221 |     pub gclip: f32,
3222 | }
3223 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3224 | const _: () = {
3225 |     ["Size of ggml_opt_params__bindgen_ty_1"]
3226 |         [::std::mem::size_of::<ggml_opt_params__bindgen_ty_1>() - 44usize];
3227 |     ["Alignment of ggml_opt_params__bindgen_ty_1"]
3228 |         [::std::mem::align_of::<ggml_opt_params__bindgen_ty_1>() - 4usize];
3229 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::n_iter"]
3230 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, n_iter) - 0usize];
3231 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::sched"]
3232 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, sched) - 4usize];
3233 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::decay"]
3234 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, decay) - 8usize];
3235 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::decay_min_ndim"]
3236 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, decay_min_ndim) - 12usize];
3237 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::alpha"]
3238 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, alpha) - 16usize];
3239 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::beta1"]
3240 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, beta1) - 20usize];
3241 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::beta2"]
3242 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, beta2) - 24usize];
3243 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::eps"]
3244 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, eps) - 28usize];
3245 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::eps_f"]
3246 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, eps_f) - 32usize];
3247 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::eps_g"]
3248 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, eps_g) - 36usize];
3249 |     ["Offset of field: ggml_opt_params__bindgen_ty_1::gclip"]
3250 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_1, gclip) - 40usize];
3251 | };
3252 | #[repr(C)]
3253 | #[derive(Debug, Copy, Clone)]
3254 | pub struct ggml_opt_params__bindgen_ty_2 {
3255 |     pub m: ::std::os::raw::c_int,
3256 |     pub n_iter: ::std::os::raw::c_int,
3257 |     pub max_linesearch: ::std::os::raw::c_int,
3258 |     pub eps: f32,
3259 |     pub ftol: f32,
3260 |     pub wolfe: f32,
3261 |     pub min_step: f32,
3262 |     pub max_step: f32,
3263 |     pub linesearch: ggml_linesearch,
3264 | }
3265 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3266 | const _: () = {
3267 |     ["Size of ggml_opt_params__bindgen_ty_2"]
3268 |         [::std::mem::size_of::<ggml_opt_params__bindgen_ty_2>() - 36usize];
3269 |     ["Alignment of ggml_opt_params__bindgen_ty_2"]
3270 |         [::std::mem::align_of::<ggml_opt_params__bindgen_ty_2>() - 4usize];
3271 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::m"]
3272 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, m) - 0usize];
3273 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::n_iter"]
3274 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, n_iter) - 4usize];
3275 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::max_linesearch"]
3276 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, max_linesearch) - 8usize];
3277 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::eps"]
3278 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, eps) - 12usize];
3279 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::ftol"]
3280 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, ftol) - 16usize];
3281 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::wolfe"]
3282 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, wolfe) - 20usize];
3283 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::min_step"]
3284 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, min_step) - 24usize];
3285 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::max_step"]
3286 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, max_step) - 28usize];
3287 |     ["Offset of field: ggml_opt_params__bindgen_ty_2::linesearch"]
3288 |         [::std::mem::offset_of!(ggml_opt_params__bindgen_ty_2, linesearch) - 32usize];
3289 | };
3290 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3291 | const _: () = {
3292 |     ["Size of ggml_opt_params"][::std::mem::size_of::<ggml_opt_params>() - 120usize];
3293 |     ["Alignment of ggml_opt_params"][::std::mem::align_of::<ggml_opt_params>() - 8usize];
3294 |     ["Offset of field: ggml_opt_params::type_"]
3295 |         [::std::mem::offset_of!(ggml_opt_params, type_) - 0usize];
3296 |     ["Offset of field: ggml_opt_params::graph_size"]
3297 |         [::std::mem::offset_of!(ggml_opt_params, graph_size) - 8usize];
3298 |     ["Offset of field: ggml_opt_params::n_threads"]
3299 |         [::std::mem::offset_of!(ggml_opt_params, n_threads) - 16usize];
3300 |     ["Offset of field: ggml_opt_params::past"]
3301 |         [::std::mem::offset_of!(ggml_opt_params, past) - 20usize];
3302 |     ["Offset of field: ggml_opt_params::delta"]
3303 |         [::std::mem::offset_of!(ggml_opt_params, delta) - 24usize];
3304 |     ["Offset of field: ggml_opt_params::max_no_improvement"]
3305 |         [::std::mem::offset_of!(ggml_opt_params, max_no_improvement) - 28usize];
3306 |     ["Offset of field: ggml_opt_params::print_forward_graph"]
3307 |         [::std::mem::offset_of!(ggml_opt_params, print_forward_graph) - 32usize];
3308 |     ["Offset of field: ggml_opt_params::print_backward_graph"]
3309 |         [::std::mem::offset_of!(ggml_opt_params, print_backward_graph) - 33usize];
3310 |     ["Offset of field: ggml_opt_params::n_gradient_accumulation"]
3311 |         [::std::mem::offset_of!(ggml_opt_params, n_gradient_accumulation) - 36usize];
3312 |     ["Offset of field: ggml_opt_params::adam"]
3313 |         [::std::mem::offset_of!(ggml_opt_params, adam) - 40usize];
3314 |     ["Offset of field: ggml_opt_params::lbfgs"]
3315 |         [::std::mem::offset_of!(ggml_opt_params, lbfgs) - 84usize];
3316 | };
3317 | #[repr(C)]
3318 | #[derive(Debug, Copy, Clone)]
3319 | pub struct ggml_opt_context {
3320 |     pub ctx: *mut ggml_context,
3321 |     pub params: ggml_opt_params,
3322 |     pub iter: ::std::os::raw::c_int,
3323 |     pub nx: i64,
3324 |     pub just_initialized: bool,
3325 |     pub loss_before: f32,
3326 |     pub loss_after: f32,
3327 |     pub adam: ggml_opt_context__bindgen_ty_1,
3328 |     pub lbfgs: ggml_opt_context__bindgen_ty_2,
3329 | }
3330 | #[repr(C)]
3331 | #[derive(Debug, Copy, Clone)]
3332 | pub struct ggml_opt_context__bindgen_ty_1 {
3333 |     pub g: *mut ggml_tensor,
3334 |     pub m: *mut ggml_tensor,
3335 |     pub v: *mut ggml_tensor,
3336 |     pub pf: *mut ggml_tensor,
3337 |     pub fx_best: f32,
3338 |     pub fx_prev: f32,
3339 |     pub n_no_improvement: ::std::os::raw::c_int,
3340 | }
3341 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3342 | const _: () = {
3343 |     ["Size of ggml_opt_context__bindgen_ty_1"]
3344 |         [::std::mem::size_of::<ggml_opt_context__bindgen_ty_1>() - 48usize];
3345 |     ["Alignment of ggml_opt_context__bindgen_ty_1"]
3346 |         [::std::mem::align_of::<ggml_opt_context__bindgen_ty_1>() - 8usize];
3347 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::g"]
3348 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, g) - 0usize];
3349 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::m"]
3350 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, m) - 8usize];
3351 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::v"]
3352 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, v) - 16usize];
3353 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::pf"]
3354 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, pf) - 24usize];
3355 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::fx_best"]
3356 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, fx_best) - 32usize];
3357 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::fx_prev"]
3358 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, fx_prev) - 36usize];
3359 |     ["Offset of field: ggml_opt_context__bindgen_ty_1::n_no_improvement"]
3360 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_1, n_no_improvement) - 40usize];
3361 | };
3362 | #[repr(C)]
3363 | #[derive(Debug, Copy, Clone)]
3364 | pub struct ggml_opt_context__bindgen_ty_2 {
3365 |     pub x: *mut ggml_tensor,
3366 |     pub xp: *mut ggml_tensor,
3367 |     pub g: *mut ggml_tensor,
3368 |     pub gp: *mut ggml_tensor,
3369 |     pub d: *mut ggml_tensor,
3370 |     pub pf: *mut ggml_tensor,
3371 |     pub lmal: *mut ggml_tensor,
3372 |     pub lmys: *mut ggml_tensor,
3373 |     pub lms: *mut ggml_tensor,
3374 |     pub lmy: *mut ggml_tensor,
3375 |     pub fx_best: f32,
3376 |     pub step: f32,
3377 |     pub j: ::std::os::raw::c_int,
3378 |     pub k: ::std::os::raw::c_int,
3379 |     pub end: ::std::os::raw::c_int,
3380 |     pub n_no_improvement: ::std::os::raw::c_int,
3381 | }
3382 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3383 | const _: () = {
3384 |     ["Size of ggml_opt_context__bindgen_ty_2"]
3385 |         [::std::mem::size_of::<ggml_opt_context__bindgen_ty_2>() - 104usize];
3386 |     ["Alignment of ggml_opt_context__bindgen_ty_2"]
3387 |         [::std::mem::align_of::<ggml_opt_context__bindgen_ty_2>() - 8usize];
3388 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::x"]
3389 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, x) - 0usize];
3390 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::xp"]
3391 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, xp) - 8usize];
3392 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::g"]
3393 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, g) - 16usize];
3394 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::gp"]
3395 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, gp) - 24usize];
3396 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::d"]
3397 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, d) - 32usize];
3398 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::pf"]
3399 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, pf) - 40usize];
3400 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::lmal"]
3401 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, lmal) - 48usize];
3402 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::lmys"]
3403 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, lmys) - 56usize];
3404 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::lms"]
3405 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, lms) - 64usize];
3406 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::lmy"]
3407 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, lmy) - 72usize];
3408 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::fx_best"]
3409 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, fx_best) - 80usize];
3410 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::step"]
3411 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, step) - 84usize];
3412 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::j"]
3413 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, j) - 88usize];
3414 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::k"]
3415 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, k) - 92usize];
3416 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::end"]
3417 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, end) - 96usize];
3418 |     ["Offset of field: ggml_opt_context__bindgen_ty_2::n_no_improvement"]
3419 |         [::std::mem::offset_of!(ggml_opt_context__bindgen_ty_2, n_no_improvement) - 100usize];
3420 | };
3421 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3422 | const _: () = {
3423 |     ["Size of ggml_opt_context"][::std::mem::size_of::<ggml_opt_context>() - 312usize];
3424 |     ["Alignment of ggml_opt_context"][::std::mem::align_of::<ggml_opt_context>() - 8usize];
3425 |     ["Offset of field: ggml_opt_context::ctx"]
3426 |         [::std::mem::offset_of!(ggml_opt_context, ctx) - 0usize];
3427 |     ["Offset of field: ggml_opt_context::params"]
3428 |         [::std::mem::offset_of!(ggml_opt_context, params) - 8usize];
3429 |     ["Offset of field: ggml_opt_context::iter"]
3430 |         [::std::mem::offset_of!(ggml_opt_context, iter) - 128usize];
3431 |     ["Offset of field: ggml_opt_context::nx"]
3432 |         [::std::mem::offset_of!(ggml_opt_context, nx) - 136usize];
3433 |     ["Offset of field: ggml_opt_context::just_initialized"]
3434 |         [::std::mem::offset_of!(ggml_opt_context, just_initialized) - 144usize];
3435 |     ["Offset of field: ggml_opt_context::loss_before"]
3436 |         [::std::mem::offset_of!(ggml_opt_context, loss_before) - 148usize];
3437 |     ["Offset of field: ggml_opt_context::loss_after"]
3438 |         [::std::mem::offset_of!(ggml_opt_context, loss_after) - 152usize];
3439 |     ["Offset of field: ggml_opt_context::adam"]
3440 |         [::std::mem::offset_of!(ggml_opt_context, adam) - 160usize];
3441 |     ["Offset of field: ggml_opt_context::lbfgs"]
3442 |         [::std::mem::offset_of!(ggml_opt_context, lbfgs) - 208usize];
3443 | };
3444 | extern "C" {
3445 |     pub fn ggml_opt_default_params(type_: ggml_opt_type) -> ggml_opt_params;
3446 | }
3447 | extern "C" {
3448 |     pub fn ggml_opt(
3449 |         ctx: *mut ggml_context,
3450 |         params: ggml_opt_params,
3451 |         f: *mut ggml_tensor,
3452 |     ) -> ggml_opt_result;
3453 | }
3454 | extern "C" {
3455 |     pub fn ggml_opt_init(
3456 |         ctx: *mut ggml_context,
3457 |         opt: *mut ggml_opt_context,
3458 |         params: ggml_opt_params,
3459 |         nx: i64,
3460 |     );
3461 | }
3462 | extern "C" {
3463 |     pub fn ggml_opt_resume(
3464 |         ctx: *mut ggml_context,
3465 |         opt: *mut ggml_opt_context,
3466 |         f: *mut ggml_tensor,
3467 |     ) -> ggml_opt_result;
3468 | }
3469 | extern "C" {
3470 |     pub fn ggml_opt_resume_g(
3471 |         ctx: *mut ggml_context,
3472 |         opt: *mut ggml_opt_context,
3473 |         f: *mut ggml_tensor,
3474 |         gf: *mut ggml_cgraph,
3475 |         gb: *mut ggml_cgraph,
3476 |         callback: ggml_opt_callback,
3477 |         callback_data: *mut ::std::os::raw::c_void,
3478 |     ) -> ggml_opt_result;
3479 | }
3480 | extern "C" {
3481 |     pub fn ggml_set_input(tensor: *mut ggml_tensor);
3482 | }
3483 | extern "C" {
3484 |     pub fn ggml_set_output(tensor: *mut ggml_tensor);
3485 | }
3486 | extern "C" {
3487 |     pub fn ggml_quantize_init(type_: ggml_type);
3488 | }
3489 | extern "C" {
3490 |     pub fn ggml_quantize_free();
3491 | }
3492 | extern "C" {
3493 |     pub fn ggml_quantize_requires_imatrix(type_: ggml_type) -> bool;
3494 | }
3495 | extern "C" {
3496 |     pub fn ggml_quantize_chunk(
3497 |         type_: ggml_type,
3498 |         src: *const f32,
3499 |         dst: *mut ::std::os::raw::c_void,
3500 |         start: i64,
3501 |         nrows: i64,
3502 |         n_per_row: i64,
3503 |         imatrix: *const f32,
3504 |     ) -> usize;
3505 | }
3506 | pub const gguf_type_GGUF_TYPE_UINT8: gguf_type = 0;
3507 | pub const gguf_type_GGUF_TYPE_INT8: gguf_type = 1;
3508 | pub const gguf_type_GGUF_TYPE_UINT16: gguf_type = 2;
3509 | pub const gguf_type_GGUF_TYPE_INT16: gguf_type = 3;
3510 | pub const gguf_type_GGUF_TYPE_UINT32: gguf_type = 4;
3511 | pub const gguf_type_GGUF_TYPE_INT32: gguf_type = 5;
3512 | pub const gguf_type_GGUF_TYPE_FLOAT32: gguf_type = 6;
3513 | pub const gguf_type_GGUF_TYPE_BOOL: gguf_type = 7;
3514 | pub const gguf_type_GGUF_TYPE_STRING: gguf_type = 8;
3515 | pub const gguf_type_GGUF_TYPE_ARRAY: gguf_type = 9;
3516 | pub const gguf_type_GGUF_TYPE_UINT64: gguf_type = 10;
3517 | pub const gguf_type_GGUF_TYPE_INT64: gguf_type = 11;
3518 | pub const gguf_type_GGUF_TYPE_FLOAT64: gguf_type = 12;
3519 | pub const gguf_type_GGUF_TYPE_COUNT: gguf_type = 13;
3520 | pub type gguf_type = ::std::os::raw::c_uint;
3521 | #[repr(C)]
3522 | #[derive(Debug, Copy, Clone)]
3523 | pub struct gguf_context {
3524 |     _unused: [u8; 0],
3525 | }
3526 | #[repr(C)]
3527 | #[derive(Debug, Copy, Clone)]
3528 | pub struct gguf_init_params {
3529 |     pub no_alloc: bool,
3530 |     pub ctx: *mut *mut ggml_context,
3531 | }
3532 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3533 | const _: () = {
3534 |     ["Size of gguf_init_params"][::std::mem::size_of::<gguf_init_params>() - 16usize];
3535 |     ["Alignment of gguf_init_params"][::std::mem::align_of::<gguf_init_params>() - 8usize];
3536 |     ["Offset of field: gguf_init_params::no_alloc"]
3537 |         [::std::mem::offset_of!(gguf_init_params, no_alloc) - 0usize];
3538 |     ["Offset of field: gguf_init_params::ctx"]
3539 |         [::std::mem::offset_of!(gguf_init_params, ctx) - 8usize];
3540 | };
3541 | extern "C" {
3542 |     pub fn gguf_init_empty() -> *mut gguf_context;
3543 | }
3544 | extern "C" {
3545 |     pub fn gguf_init_from_file(
3546 |         fname: *const ::std::os::raw::c_char,
3547 |         params: gguf_init_params,
3548 |     ) -> *mut gguf_context;
3549 | }
3550 | extern "C" {
3551 |     pub fn gguf_free(ctx: *mut gguf_context);
3552 | }
3553 | extern "C" {
3554 |     pub fn gguf_type_name(type_: gguf_type) -> *const ::std::os::raw::c_char;
3555 | }
3556 | extern "C" {
3557 |     pub fn gguf_get_version(ctx: *const gguf_context) -> ::std::os::raw::c_int;
3558 | }
3559 | extern "C" {
3560 |     pub fn gguf_get_alignment(ctx: *const gguf_context) -> usize;
3561 | }
3562 | extern "C" {
3563 |     pub fn gguf_get_data_offset(ctx: *const gguf_context) -> usize;
3564 | }
3565 | extern "C" {
3566 |     pub fn gguf_get_data(ctx: *const gguf_context) -> *mut ::std::os::raw::c_void;
3567 | }
3568 | extern "C" {
3569 |     pub fn gguf_get_n_kv(ctx: *const gguf_context) -> ::std::os::raw::c_int;
3570 | }
3571 | extern "C" {
3572 |     pub fn gguf_find_key(
3573 |         ctx: *const gguf_context,
3574 |         key: *const ::std::os::raw::c_char,
3575 |     ) -> ::std::os::raw::c_int;
3576 | }
3577 | extern "C" {
3578 |     pub fn gguf_get_key(
3579 |         ctx: *const gguf_context,
3580 |         key_id: ::std::os::raw::c_int,
3581 |     ) -> *const ::std::os::raw::c_char;
3582 | }
3583 | extern "C" {
3584 |     pub fn gguf_get_kv_type(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> gguf_type;
3585 | }
3586 | extern "C" {
3587 |     pub fn gguf_get_arr_type(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> gguf_type;
3588 | }
3589 | extern "C" {
3590 |     pub fn gguf_get_val_u8(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> u8;
3591 | }
3592 | extern "C" {
3593 |     pub fn gguf_get_val_i8(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> i8;
3594 | }
3595 | extern "C" {
3596 |     pub fn gguf_get_val_u16(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> u16;
3597 | }
3598 | extern "C" {
3599 |     pub fn gguf_get_val_i16(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> i16;
3600 | }
3601 | extern "C" {
3602 |     pub fn gguf_get_val_u32(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> u32;
3603 | }
3604 | extern "C" {
3605 |     pub fn gguf_get_val_i32(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> i32;
3606 | }
3607 | extern "C" {
3608 |     pub fn gguf_get_val_f32(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> f32;
3609 | }
3610 | extern "C" {
3611 |     pub fn gguf_get_val_u64(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> u64;
3612 | }
3613 | extern "C" {
3614 |     pub fn gguf_get_val_i64(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> i64;
3615 | }
3616 | extern "C" {
3617 |     pub fn gguf_get_val_f64(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> f64;
3618 | }
3619 | extern "C" {
3620 |     pub fn gguf_get_val_bool(ctx: *const gguf_context, key_id: ::std::os::raw::c_int) -> bool;
3621 | }
3622 | extern "C" {
3623 |     pub fn gguf_get_val_str(
3624 |         ctx: *const gguf_context,
3625 |         key_id: ::std::os::raw::c_int,
3626 |     ) -> *const ::std::os::raw::c_char;
3627 | }
3628 | extern "C" {
3629 |     pub fn gguf_get_val_data(
3630 |         ctx: *const gguf_context,
3631 |         key_id: ::std::os::raw::c_int,
3632 |     ) -> *const ::std::os::raw::c_void;
3633 | }
3634 | extern "C" {
3635 |     pub fn gguf_get_arr_n(
3636 |         ctx: *const gguf_context,
3637 |         key_id: ::std::os::raw::c_int,
3638 |     ) -> ::std::os::raw::c_int;
3639 | }
3640 | extern "C" {
3641 |     pub fn gguf_get_arr_data(
3642 |         ctx: *const gguf_context,
3643 |         key_id: ::std::os::raw::c_int,
3644 |     ) -> *const ::std::os::raw::c_void;
3645 | }
3646 | extern "C" {
3647 |     pub fn gguf_get_arr_str(
3648 |         ctx: *const gguf_context,
3649 |         key_id: ::std::os::raw::c_int,
3650 |         i: ::std::os::raw::c_int,
3651 |     ) -> *const ::std::os::raw::c_char;
3652 | }
3653 | extern "C" {
3654 |     pub fn gguf_get_n_tensors(ctx: *const gguf_context) -> ::std::os::raw::c_int;
3655 | }
3656 | extern "C" {
3657 |     pub fn gguf_find_tensor(
3658 |         ctx: *const gguf_context,
3659 |         name: *const ::std::os::raw::c_char,
3660 |     ) -> ::std::os::raw::c_int;
3661 | }
3662 | extern "C" {
3663 |     pub fn gguf_get_tensor_offset(ctx: *const gguf_context, i: ::std::os::raw::c_int) -> usize;
3664 | }
3665 | extern "C" {
3666 |     pub fn gguf_get_tensor_name(
3667 |         ctx: *const gguf_context,
3668 |         i: ::std::os::raw::c_int,
3669 |     ) -> *mut ::std::os::raw::c_char;
3670 | }
3671 | extern "C" {
3672 |     pub fn gguf_get_tensor_type(ctx: *const gguf_context, i: ::std::os::raw::c_int) -> ggml_type;
3673 | }
3674 | extern "C" {
3675 |     pub fn gguf_remove_key(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char);
3676 | }
3677 | extern "C" {
3678 |     pub fn gguf_set_val_u8(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: u8);
3679 | }
3680 | extern "C" {
3681 |     pub fn gguf_set_val_i8(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: i8);
3682 | }
3683 | extern "C" {
3684 |     pub fn gguf_set_val_u16(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: u16);
3685 | }
3686 | extern "C" {
3687 |     pub fn gguf_set_val_i16(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: i16);
3688 | }
3689 | extern "C" {
3690 |     pub fn gguf_set_val_u32(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: u32);
3691 | }
3692 | extern "C" {
3693 |     pub fn gguf_set_val_i32(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: i32);
3694 | }
3695 | extern "C" {
3696 |     pub fn gguf_set_val_f32(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: f32);
3697 | }
3698 | extern "C" {
3699 |     pub fn gguf_set_val_u64(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: u64);
3700 | }
3701 | extern "C" {
3702 |     pub fn gguf_set_val_i64(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: i64);
3703 | }
3704 | extern "C" {
3705 |     pub fn gguf_set_val_f64(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: f64);
3706 | }
3707 | extern "C" {
3708 |     pub fn gguf_set_val_bool(ctx: *mut gguf_context, key: *const ::std::os::raw::c_char, val: bool);
3709 | }
3710 | extern "C" {
3711 |     pub fn gguf_set_val_str(
3712 |         ctx: *mut gguf_context,
3713 |         key: *const ::std::os::raw::c_char,
3714 |         val: *const ::std::os::raw::c_char,
3715 |     );
3716 | }
3717 | extern "C" {
3718 |     pub fn gguf_set_arr_data(
3719 |         ctx: *mut gguf_context,
3720 |         key: *const ::std::os::raw::c_char,
3721 |         type_: gguf_type,
3722 |         data: *const ::std::os::raw::c_void,
3723 |         n: ::std::os::raw::c_int,
3724 |     );
3725 | }
3726 | extern "C" {
3727 |     pub fn gguf_set_arr_str(
3728 |         ctx: *mut gguf_context,
3729 |         key: *const ::std::os::raw::c_char,
3730 |         data: *mut *const ::std::os::raw::c_char,
3731 |         n: ::std::os::raw::c_int,
3732 |     );
3733 | }
3734 | extern "C" {
3735 |     pub fn gguf_set_kv(ctx: *mut gguf_context, src: *mut gguf_context);
3736 | }
3737 | extern "C" {
3738 |     pub fn gguf_add_tensor(ctx: *mut gguf_context, tensor: *const ggml_tensor);
3739 | }
3740 | extern "C" {
3741 |     pub fn gguf_set_tensor_type(
3742 |         ctx: *mut gguf_context,
3743 |         name: *const ::std::os::raw::c_char,
3744 |         type_: ggml_type,
3745 |     );
3746 | }
3747 | extern "C" {
3748 |     pub fn gguf_set_tensor_data(
3749 |         ctx: *mut gguf_context,
3750 |         name: *const ::std::os::raw::c_char,
3751 |         data: *const ::std::os::raw::c_void,
3752 |         size: usize,
3753 |     );
3754 | }
3755 | extern "C" {
3756 |     pub fn gguf_write_to_file(
3757 |         ctx: *const gguf_context,
3758 |         fname: *const ::std::os::raw::c_char,
3759 |         only_meta: bool,
3760 |     );
3761 | }
3762 | extern "C" {
3763 |     pub fn gguf_get_meta_size(ctx: *const gguf_context) -> usize;
3764 | }
3765 | extern "C" {
3766 |     pub fn gguf_get_meta_data(ctx: *const gguf_context, data: *mut ::std::os::raw::c_void);
3767 | }
3768 | extern "C" {
3769 |     pub fn ggml_cpu_has_avx() -> ::std::os::raw::c_int;
3770 | }
3771 | extern "C" {
3772 |     pub fn ggml_cpu_has_avx_vnni() -> ::std::os::raw::c_int;
3773 | }
3774 | extern "C" {
3775 |     pub fn ggml_cpu_has_avx2() -> ::std::os::raw::c_int;
3776 | }
3777 | extern "C" {
3778 |     pub fn ggml_cpu_has_avx512() -> ::std::os::raw::c_int;
3779 | }
3780 | extern "C" {
3781 |     pub fn ggml_cpu_has_avx512_vbmi() -> ::std::os::raw::c_int;
3782 | }
3783 | extern "C" {
3784 |     pub fn ggml_cpu_has_avx512_vnni() -> ::std::os::raw::c_int;
3785 | }
3786 | extern "C" {
3787 |     pub fn ggml_cpu_has_avx512_bf16() -> ::std::os::raw::c_int;
3788 | }
3789 | extern "C" {
3790 |     pub fn ggml_cpu_has_amx_int8() -> ::std::os::raw::c_int;
3791 | }
3792 | extern "C" {
3793 |     pub fn ggml_cpu_has_fma() -> ::std::os::raw::c_int;
3794 | }
3795 | extern "C" {
3796 |     pub fn ggml_cpu_has_neon() -> ::std::os::raw::c_int;
3797 | }
3798 | extern "C" {
3799 |     pub fn ggml_cpu_has_sve() -> ::std::os::raw::c_int;
3800 | }
3801 | extern "C" {
3802 |     pub fn ggml_cpu_has_arm_fma() -> ::std::os::raw::c_int;
3803 | }
3804 | extern "C" {
3805 |     pub fn ggml_cpu_has_metal() -> ::std::os::raw::c_int;
3806 | }
3807 | extern "C" {
3808 |     pub fn ggml_cpu_has_f16c() -> ::std::os::raw::c_int;
3809 | }
3810 | extern "C" {
3811 |     pub fn ggml_cpu_has_fp16_va() -> ::std::os::raw::c_int;
3812 | }
3813 | extern "C" {
3814 |     pub fn ggml_cpu_has_wasm_simd() -> ::std::os::raw::c_int;
3815 | }
3816 | extern "C" {
3817 |     pub fn ggml_cpu_has_blas() -> ::std::os::raw::c_int;
3818 | }
3819 | extern "C" {
3820 |     pub fn ggml_cpu_has_cuda() -> ::std::os::raw::c_int;
3821 | }
3822 | extern "C" {
3823 |     pub fn ggml_cpu_has_vulkan() -> ::std::os::raw::c_int;
3824 | }
3825 | extern "C" {
3826 |     pub fn ggml_cpu_has_kompute() -> ::std::os::raw::c_int;
3827 | }
3828 | extern "C" {
3829 |     pub fn ggml_cpu_has_gpublas() -> ::std::os::raw::c_int;
3830 | }
3831 | extern "C" {
3832 |     pub fn ggml_cpu_has_sse3() -> ::std::os::raw::c_int;
3833 | }
3834 | extern "C" {
3835 |     pub fn ggml_cpu_has_ssse3() -> ::std::os::raw::c_int;
3836 | }
3837 | extern "C" {
3838 |     pub fn ggml_cpu_has_riscv_v() -> ::std::os::raw::c_int;
3839 | }
3840 | extern "C" {
3841 |     pub fn ggml_cpu_has_sycl() -> ::std::os::raw::c_int;
3842 | }
3843 | extern "C" {
3844 |     pub fn ggml_cpu_has_rpc() -> ::std::os::raw::c_int;
3845 | }
3846 | extern "C" {
3847 |     pub fn ggml_cpu_has_vsx() -> ::std::os::raw::c_int;
3848 | }
3849 | extern "C" {
3850 |     pub fn ggml_cpu_has_matmul_int8() -> ::std::os::raw::c_int;
3851 | }
3852 | extern "C" {
3853 |     pub fn ggml_cpu_has_cann() -> ::std::os::raw::c_int;
3854 | }
3855 | extern "C" {
3856 |     pub fn ggml_cpu_has_llamafile() -> ::std::os::raw::c_int;
3857 | }
3858 | extern "C" {
3859 |     pub fn ggml_cpu_get_sve_cnt() -> ::std::os::raw::c_int;
3860 | }
3861 | pub type ggml_to_float_t = ::std::option::Option<
3862 |     unsafe extern "C" fn(x: *const ::std::os::raw::c_void, y: *mut f32, k: i64),
3863 | >;
3864 | pub type ggml_from_float_t = ::std::option::Option<
3865 |     unsafe extern "C" fn(x: *const f32, y: *mut ::std::os::raw::c_void, k: i64),
3866 | >;
3867 | pub type ggml_from_float_to_mat_t = ::std::option::Option<
3868 |     unsafe extern "C" fn(x: *const f32, y: *mut ::std::os::raw::c_void, nr: i64, k: i64, bs: i64),
3869 | >;
3870 | pub type ggml_vec_dot_t = ::std::option::Option<
3871 |     unsafe extern "C" fn(
3872 |         n: ::std::os::raw::c_int,
3873 |         s: *mut f32,
3874 |         bs: usize,
3875 |         x: *const ::std::os::raw::c_void,
3876 |         bx: usize,
3877 |         y: *const ::std::os::raw::c_void,
3878 |         by: usize,
3879 |         nrc: ::std::os::raw::c_int,
3880 |     ),
3881 | >;
3882 | pub type ggml_gemv_t = ::std::option::Option<
3883 |     unsafe extern "C" fn(
3884 |         n: ::std::os::raw::c_int,
3885 |         s: *mut f32,
3886 |         bs: usize,
3887 |         x: *const ::std::os::raw::c_void,
3888 |         y: *const ::std::os::raw::c_void,
3889 |         nr: ::std::os::raw::c_int,
3890 |         nc: ::std::os::raw::c_int,
3891 |     ),
3892 | >;
3893 | pub type ggml_gemm_t = ::std::option::Option<
3894 |     unsafe extern "C" fn(
3895 |         n: ::std::os::raw::c_int,
3896 |         s: *mut f32,
3897 |         bs: usize,
3898 |         x: *const ::std::os::raw::c_void,
3899 |         y: *const ::std::os::raw::c_void,
3900 |         nr: ::std::os::raw::c_int,
3901 |         nc: ::std::os::raw::c_int,
3902 |     ),
3903 | >;
3904 | #[repr(C)]
3905 | #[derive(Debug, Copy, Clone)]
3906 | pub struct ggml_type_traits {
3907 |     pub type_name: *const ::std::os::raw::c_char,
3908 |     pub blck_size: i64,
3909 |     pub blck_size_interleave: i64,
3910 |     pub type_size: usize,
3911 |     pub is_quantized: bool,
3912 |     pub to_float: ggml_to_float_t,
3913 |     pub from_float: ggml_from_float_t,
3914 |     pub from_float_ref: ggml_from_float_t,
3915 |     pub from_float_to_mat: ggml_from_float_to_mat_t,
3916 |     pub vec_dot: ggml_vec_dot_t,
3917 |     pub vec_dot_type: ggml_type,
3918 |     pub nrows: i64,
3919 |     pub ncols: i64,
3920 |     pub gemv: ggml_gemv_t,
3921 |     pub gemm: ggml_gemm_t,
3922 | }
3923 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
3924 | const _: () = {
3925 |     ["Size of ggml_type_traits"][::std::mem::size_of::<ggml_type_traits>() - 120usize];
3926 |     ["Alignment of ggml_type_traits"][::std::mem::align_of::<ggml_type_traits>() - 8usize];
3927 |     ["Offset of field: ggml_type_traits::type_name"]
3928 |         [::std::mem::offset_of!(ggml_type_traits, type_name) - 0usize];
3929 |     ["Offset of field: ggml_type_traits::blck_size"]
3930 |         [::std::mem::offset_of!(ggml_type_traits, blck_size) - 8usize];
3931 |     ["Offset of field: ggml_type_traits::blck_size_interleave"]
3932 |         [::std::mem::offset_of!(ggml_type_traits, blck_size_interleave) - 16usize];
3933 |     ["Offset of field: ggml_type_traits::type_size"]
3934 |         [::std::mem::offset_of!(ggml_type_traits, type_size) - 24usize];
3935 |     ["Offset of field: ggml_type_traits::is_quantized"]
3936 |         [::std::mem::offset_of!(ggml_type_traits, is_quantized) - 32usize];
3937 |     ["Offset of field: ggml_type_traits::to_float"]
3938 |         [::std::mem::offset_of!(ggml_type_traits, to_float) - 40usize];
3939 |     ["Offset of field: ggml_type_traits::from_float"]
3940 |         [::std::mem::offset_of!(ggml_type_traits, from_float) - 48usize];
3941 |     ["Offset of field: ggml_type_traits::from_float_ref"]
3942 |         [::std::mem::offset_of!(ggml_type_traits, from_float_ref) - 56usize];
3943 |     ["Offset of field: ggml_type_traits::from_float_to_mat"]
3944 |         [::std::mem::offset_of!(ggml_type_traits, from_float_to_mat) - 64usize];
3945 |     ["Offset of field: ggml_type_traits::vec_dot"]
3946 |         [::std::mem::offset_of!(ggml_type_traits, vec_dot) - 72usize];
3947 |     ["Offset of field: ggml_type_traits::vec_dot_type"]
3948 |         [::std::mem::offset_of!(ggml_type_traits, vec_dot_type) - 80usize];
3949 |     ["Offset of field: ggml_type_traits::nrows"]
3950 |         [::std::mem::offset_of!(ggml_type_traits, nrows) - 88usize];
3951 |     ["Offset of field: ggml_type_traits::ncols"]
3952 |         [::std::mem::offset_of!(ggml_type_traits, ncols) - 96usize];
3953 |     ["Offset of field: ggml_type_traits::gemv"]
3954 |         [::std::mem::offset_of!(ggml_type_traits, gemv) - 104usize];
3955 |     ["Offset of field: ggml_type_traits::gemm"]
3956 |         [::std::mem::offset_of!(ggml_type_traits, gemm) - 112usize];
3957 | };
3958 | extern "C" {
3959 |     pub fn ggml_get_type_traits(type_: ggml_type) -> *const ggml_type_traits;
3960 | }
3961 | #[repr(C)]
3962 | #[derive(Debug, Copy, Clone)]
3963 | pub struct whisper_context {
3964 |     _unused: [u8; 0],
3965 | }
3966 | #[repr(C)]
3967 | #[derive(Debug, Copy, Clone)]
3968 | pub struct whisper_state {
3969 |     _unused: [u8; 0],
3970 | }
3971 | pub type whisper_pos = i32;
3972 | pub type whisper_token = i32;
3973 | pub type whisper_seq_id = i32;
3974 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_NONE: whisper_alignment_heads_preset = 0;
3975 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_N_TOP_MOST: whisper_alignment_heads_preset =
3976 |     1;
3977 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_CUSTOM: whisper_alignment_heads_preset = 2;
3978 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_TINY_EN: whisper_alignment_heads_preset = 3;
3979 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_TINY: whisper_alignment_heads_preset = 4;
3980 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_BASE_EN: whisper_alignment_heads_preset = 5;
3981 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_BASE: whisper_alignment_heads_preset = 6;
3982 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_SMALL_EN: whisper_alignment_heads_preset =
3983 |     7;
3984 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_SMALL: whisper_alignment_heads_preset = 8;
3985 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_MEDIUM_EN: whisper_alignment_heads_preset =
3986 |     9;
3987 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_MEDIUM: whisper_alignment_heads_preset = 10;
3988 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V1: whisper_alignment_heads_preset =
3989 |     11;
3990 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V2: whisper_alignment_heads_preset =
3991 |     12;
3992 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V3: whisper_alignment_heads_preset =
3993 |     13;
3994 | pub const whisper_alignment_heads_preset_WHISPER_AHEADS_LARGE_V3_TURBO:
3995 |     whisper_alignment_heads_preset = 14;
3996 | pub type whisper_alignment_heads_preset = ::std::os::raw::c_uint;
3997 | #[repr(C)]
3998 | #[derive(Debug, Copy, Clone)]
3999 | pub struct whisper_ahead {
4000 |     pub n_text_layer: ::std::os::raw::c_int,
4001 |     pub n_head: ::std::os::raw::c_int,
4002 | }
4003 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4004 | const _: () = {
4005 |     ["Size of whisper_ahead"][::std::mem::size_of::<whisper_ahead>() - 8usize];
4006 |     ["Alignment of whisper_ahead"][::std::mem::align_of::<whisper_ahead>() - 4usize];
4007 |     ["Offset of field: whisper_ahead::n_text_layer"]
4008 |         [::std::mem::offset_of!(whisper_ahead, n_text_layer) - 0usize];
4009 |     ["Offset of field: whisper_ahead::n_head"]
4010 |         [::std::mem::offset_of!(whisper_ahead, n_head) - 4usize];
4011 | };
4012 | #[repr(C)]
4013 | #[derive(Debug, Copy, Clone)]
4014 | pub struct whisper_aheads {
4015 |     pub n_heads: usize,
4016 |     pub heads: *const whisper_ahead,
4017 | }
4018 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4019 | const _: () = {
4020 |     ["Size of whisper_aheads"][::std::mem::size_of::<whisper_aheads>() - 16usize];
4021 |     ["Alignment of whisper_aheads"][::std::mem::align_of::<whisper_aheads>() - 8usize];
4022 |     ["Offset of field: whisper_aheads::n_heads"]
4023 |         [::std::mem::offset_of!(whisper_aheads, n_heads) - 0usize];
4024 |     ["Offset of field: whisper_aheads::heads"]
4025 |         [::std::mem::offset_of!(whisper_aheads, heads) - 8usize];
4026 | };
4027 | #[repr(C)]
4028 | #[derive(Debug, Copy, Clone)]
4029 | pub struct whisper_context_params {
4030 |     pub use_gpu: bool,
4031 |     pub flash_attn: bool,
4032 |     pub gpu_device: ::std::os::raw::c_int,
4033 |     pub dtw_token_timestamps: bool,
4034 |     pub dtw_aheads_preset: whisper_alignment_heads_preset,
4035 |     pub dtw_n_top: ::std::os::raw::c_int,
4036 |     pub dtw_aheads: whisper_aheads,
4037 |     pub dtw_mem_size: usize,
4038 | }
4039 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4040 | const _: () = {
4041 |     ["Size of whisper_context_params"][::std::mem::size_of::<whisper_context_params>() - 48usize];
4042 |     ["Alignment of whisper_context_params"]
4043 |         [::std::mem::align_of::<whisper_context_params>() - 8usize];
4044 |     ["Offset of field: whisper_context_params::use_gpu"]
4045 |         [::std::mem::offset_of!(whisper_context_params, use_gpu) - 0usize];
4046 |     ["Offset of field: whisper_context_params::flash_attn"]
4047 |         [::std::mem::offset_of!(whisper_context_params, flash_attn) - 1usize];
4048 |     ["Offset of field: whisper_context_params::gpu_device"]
4049 |         [::std::mem::offset_of!(whisper_context_params, gpu_device) - 4usize];
4050 |     ["Offset of field: whisper_context_params::dtw_token_timestamps"]
4051 |         [::std::mem::offset_of!(whisper_context_params, dtw_token_timestamps) - 8usize];
4052 |     ["Offset of field: whisper_context_params::dtw_aheads_preset"]
4053 |         [::std::mem::offset_of!(whisper_context_params, dtw_aheads_preset) - 12usize];
4054 |     ["Offset of field: whisper_context_params::dtw_n_top"]
4055 |         [::std::mem::offset_of!(whisper_context_params, dtw_n_top) - 16usize];
4056 |     ["Offset of field: whisper_context_params::dtw_aheads"]
4057 |         [::std::mem::offset_of!(whisper_context_params, dtw_aheads) - 24usize];
4058 |     ["Offset of field: whisper_context_params::dtw_mem_size"]
4059 |         [::std::mem::offset_of!(whisper_context_params, dtw_mem_size) - 40usize];
4060 | };
4061 | #[repr(C)]
4062 | #[derive(Debug, Copy, Clone)]
4063 | pub struct whisper_token_data {
4064 |     pub id: whisper_token,
4065 |     pub tid: whisper_token,
4066 |     pub p: f32,
4067 |     pub plog: f32,
4068 |     pub pt: f32,
4069 |     pub ptsum: f32,
4070 |     pub t0: i64,
4071 |     pub t1: i64,
4072 |     pub t_dtw: i64,
4073 |     pub vlen: f32,
4074 | }
4075 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4076 | const _: () = {
4077 |     ["Size of whisper_token_data"][::std::mem::size_of::<whisper_token_data>() - 56usize];
4078 |     ["Alignment of whisper_token_data"][::std::mem::align_of::<whisper_token_data>() - 8usize];
4079 |     ["Offset of field: whisper_token_data::id"]
4080 |         [::std::mem::offset_of!(whisper_token_data, id) - 0usize];
4081 |     ["Offset of field: whisper_token_data::tid"]
4082 |         [::std::mem::offset_of!(whisper_token_data, tid) - 4usize];
4083 |     ["Offset of field: whisper_token_data::p"]
4084 |         [::std::mem::offset_of!(whisper_token_data, p) - 8usize];
4085 |     ["Offset of field: whisper_token_data::plog"]
4086 |         [::std::mem::offset_of!(whisper_token_data, plog) - 12usize];
4087 |     ["Offset of field: whisper_token_data::pt"]
4088 |         [::std::mem::offset_of!(whisper_token_data, pt) - 16usize];
4089 |     ["Offset of field: whisper_token_data::ptsum"]
4090 |         [::std::mem::offset_of!(whisper_token_data, ptsum) - 20usize];
4091 |     ["Offset of field: whisper_token_data::t0"]
4092 |         [::std::mem::offset_of!(whisper_token_data, t0) - 24usize];
4093 |     ["Offset of field: whisper_token_data::t1"]
4094 |         [::std::mem::offset_of!(whisper_token_data, t1) - 32usize];
4095 |     ["Offset of field: whisper_token_data::t_dtw"]
4096 |         [::std::mem::offset_of!(whisper_token_data, t_dtw) - 40usize];
4097 |     ["Offset of field: whisper_token_data::vlen"]
4098 |         [::std::mem::offset_of!(whisper_token_data, vlen) - 48usize];
4099 | };
4100 | #[repr(C)]
4101 | #[derive(Debug, Copy, Clone)]
4102 | pub struct whisper_model_loader {
4103 |     pub context: *mut ::std::os::raw::c_void,
4104 |     pub read: ::std::option::Option<
4105 |         unsafe extern "C" fn(
4106 |             ctx: *mut ::std::os::raw::c_void,
4107 |             output: *mut ::std::os::raw::c_void,
4108 |             read_size: usize,
4109 |         ) -> usize,
4110 |     >,
4111 |     pub eof: ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> bool>,
4112 |     pub close: ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>,
4113 | }
4114 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4115 | const _: () = {
4116 |     ["Size of whisper_model_loader"][::std::mem::size_of::<whisper_model_loader>() - 32usize];
4117 |     ["Alignment of whisper_model_loader"][::std::mem::align_of::<whisper_model_loader>() - 8usize];
4118 |     ["Offset of field: whisper_model_loader::context"]
4119 |         [::std::mem::offset_of!(whisper_model_loader, context) - 0usize];
4120 |     ["Offset of field: whisper_model_loader::read"]
4121 |         [::std::mem::offset_of!(whisper_model_loader, read) - 8usize];
4122 |     ["Offset of field: whisper_model_loader::eof"]
4123 |         [::std::mem::offset_of!(whisper_model_loader, eof) - 16usize];
4124 |     ["Offset of field: whisper_model_loader::close"]
4125 |         [::std::mem::offset_of!(whisper_model_loader, close) - 24usize];
4126 | };
4127 | pub const whisper_gretype_WHISPER_GRETYPE_END: whisper_gretype = 0;
4128 | pub const whisper_gretype_WHISPER_GRETYPE_ALT: whisper_gretype = 1;
4129 | pub const whisper_gretype_WHISPER_GRETYPE_RULE_REF: whisper_gretype = 2;
4130 | pub const whisper_gretype_WHISPER_GRETYPE_CHAR: whisper_gretype = 3;
4131 | pub const whisper_gretype_WHISPER_GRETYPE_CHAR_NOT: whisper_gretype = 4;
4132 | pub const whisper_gretype_WHISPER_GRETYPE_CHAR_RNG_UPPER: whisper_gretype = 5;
4133 | pub const whisper_gretype_WHISPER_GRETYPE_CHAR_ALT: whisper_gretype = 6;
4134 | pub type whisper_gretype = ::std::os::raw::c_uint;
4135 | #[repr(C)]
4136 | #[derive(Debug, Copy, Clone)]
4137 | pub struct whisper_grammar_element {
4138 |     pub type_: whisper_gretype,
4139 |     pub value: u32,
4140 | }
4141 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4142 | const _: () = {
4143 |     ["Size of whisper_grammar_element"][::std::mem::size_of::<whisper_grammar_element>() - 8usize];
4144 |     ["Alignment of whisper_grammar_element"]
4145 |         [::std::mem::align_of::<whisper_grammar_element>() - 4usize];
4146 |     ["Offset of field: whisper_grammar_element::type_"]
4147 |         [::std::mem::offset_of!(whisper_grammar_element, type_) - 0usize];
4148 |     ["Offset of field: whisper_grammar_element::value"]
4149 |         [::std::mem::offset_of!(whisper_grammar_element, value) - 4usize];
4150 | };
4151 | extern "C" {
4152 |     pub fn whisper_init_from_file_with_params(
4153 |         path_model: *const ::std::os::raw::c_char,
4154 |         params: whisper_context_params,
4155 |     ) -> *mut whisper_context;
4156 | }
4157 | extern "C" {
4158 |     pub fn whisper_init_from_buffer_with_params(
4159 |         buffer: *mut ::std::os::raw::c_void,
4160 |         buffer_size: usize,
4161 |         params: whisper_context_params,
4162 |     ) -> *mut whisper_context;
4163 | }
4164 | extern "C" {
4165 |     pub fn whisper_init_with_params(
4166 |         loader: *mut whisper_model_loader,
4167 |         params: whisper_context_params,
4168 |     ) -> *mut whisper_context;
4169 | }
4170 | extern "C" {
4171 |     pub fn whisper_init_from_file_with_params_no_state(
4172 |         path_model: *const ::std::os::raw::c_char,
4173 |         params: whisper_context_params,
4174 |     ) -> *mut whisper_context;
4175 | }
4176 | extern "C" {
4177 |     pub fn whisper_init_from_buffer_with_params_no_state(
4178 |         buffer: *mut ::std::os::raw::c_void,
4179 |         buffer_size: usize,
4180 |         params: whisper_context_params,
4181 |     ) -> *mut whisper_context;
4182 | }
4183 | extern "C" {
4184 |     pub fn whisper_init_with_params_no_state(
4185 |         loader: *mut whisper_model_loader,
4186 |         params: whisper_context_params,
4187 |     ) -> *mut whisper_context;
4188 | }
4189 | extern "C" {
4190 |     pub fn whisper_init_from_file(
4191 |         path_model: *const ::std::os::raw::c_char,
4192 |     ) -> *mut whisper_context;
4193 | }
4194 | extern "C" {
4195 |     pub fn whisper_init_from_buffer(
4196 |         buffer: *mut ::std::os::raw::c_void,
4197 |         buffer_size: usize,
4198 |     ) -> *mut whisper_context;
4199 | }
4200 | extern "C" {
4201 |     pub fn whisper_init(loader: *mut whisper_model_loader) -> *mut whisper_context;
4202 | }
4203 | extern "C" {
4204 |     pub fn whisper_init_from_file_no_state(
4205 |         path_model: *const ::std::os::raw::c_char,
4206 |     ) -> *mut whisper_context;
4207 | }
4208 | extern "C" {
4209 |     pub fn whisper_init_from_buffer_no_state(
4210 |         buffer: *mut ::std::os::raw::c_void,
4211 |         buffer_size: usize,
4212 |     ) -> *mut whisper_context;
4213 | }
4214 | extern "C" {
4215 |     pub fn whisper_init_no_state(loader: *mut whisper_model_loader) -> *mut whisper_context;
4216 | }
4217 | extern "C" {
4218 |     pub fn whisper_init_state(ctx: *mut whisper_context) -> *mut whisper_state;
4219 | }
4220 | extern "C" {
4221 |     pub fn whisper_ctx_init_openvino_encoder_with_state(
4222 |         ctx: *mut whisper_context,
4223 |         state: *mut whisper_state,
4224 |         model_path: *const ::std::os::raw::c_char,
4225 |         device: *const ::std::os::raw::c_char,
4226 |         cache_dir: *const ::std::os::raw::c_char,
4227 |     ) -> ::std::os::raw::c_int;
4228 | }
4229 | extern "C" {
4230 |     pub fn whisper_ctx_init_openvino_encoder(
4231 |         ctx: *mut whisper_context,
4232 |         model_path: *const ::std::os::raw::c_char,
4233 |         device: *const ::std::os::raw::c_char,
4234 |         cache_dir: *const ::std::os::raw::c_char,
4235 |     ) -> ::std::os::raw::c_int;
4236 | }
4237 | extern "C" {
4238 |     pub fn whisper_free(ctx: *mut whisper_context);
4239 | }
4240 | extern "C" {
4241 |     pub fn whisper_free_state(state: *mut whisper_state);
4242 | }
4243 | extern "C" {
4244 |     pub fn whisper_free_params(params: *mut whisper_full_params);
4245 | }
4246 | extern "C" {
4247 |     pub fn whisper_free_context_params(params: *mut whisper_context_params);
4248 | }
4249 | extern "C" {
4250 |     pub fn whisper_pcm_to_mel(
4251 |         ctx: *mut whisper_context,
4252 |         samples: *const f32,
4253 |         n_samples: ::std::os::raw::c_int,
4254 |         n_threads: ::std::os::raw::c_int,
4255 |     ) -> ::std::os::raw::c_int;
4256 | }
4257 | extern "C" {
4258 |     pub fn whisper_pcm_to_mel_with_state(
4259 |         ctx: *mut whisper_context,
4260 |         state: *mut whisper_state,
4261 |         samples: *const f32,
4262 |         n_samples: ::std::os::raw::c_int,
4263 |         n_threads: ::std::os::raw::c_int,
4264 |     ) -> ::std::os::raw::c_int;
4265 | }
4266 | extern "C" {
4267 |     pub fn whisper_set_mel(
4268 |         ctx: *mut whisper_context,
4269 |         data: *const f32,
4270 |         n_len: ::std::os::raw::c_int,
4271 |         n_mel: ::std::os::raw::c_int,
4272 |     ) -> ::std::os::raw::c_int;
4273 | }
4274 | extern "C" {
4275 |     pub fn whisper_set_mel_with_state(
4276 |         ctx: *mut whisper_context,
4277 |         state: *mut whisper_state,
4278 |         data: *const f32,
4279 |         n_len: ::std::os::raw::c_int,
4280 |         n_mel: ::std::os::raw::c_int,
4281 |     ) -> ::std::os::raw::c_int;
4282 | }
4283 | extern "C" {
4284 |     pub fn whisper_encode(
4285 |         ctx: *mut whisper_context,
4286 |         offset: ::std::os::raw::c_int,
4287 |         n_threads: ::std::os::raw::c_int,
4288 |     ) -> ::std::os::raw::c_int;
4289 | }
4290 | extern "C" {
4291 |     pub fn whisper_encode_with_state(
4292 |         ctx: *mut whisper_context,
4293 |         state: *mut whisper_state,
4294 |         offset: ::std::os::raw::c_int,
4295 |         n_threads: ::std::os::raw::c_int,
4296 |     ) -> ::std::os::raw::c_int;
4297 | }
4298 | extern "C" {
4299 |     pub fn whisper_decode(
4300 |         ctx: *mut whisper_context,
4301 |         tokens: *const whisper_token,
4302 |         n_tokens: ::std::os::raw::c_int,
4303 |         n_past: ::std::os::raw::c_int,
4304 |         n_threads: ::std::os::raw::c_int,
4305 |     ) -> ::std::os::raw::c_int;
4306 | }
4307 | extern "C" {
4308 |     pub fn whisper_decode_with_state(
4309 |         ctx: *mut whisper_context,
4310 |         state: *mut whisper_state,
4311 |         tokens: *const whisper_token,
4312 |         n_tokens: ::std::os::raw::c_int,
4313 |         n_past: ::std::os::raw::c_int,
4314 |         n_threads: ::std::os::raw::c_int,
4315 |     ) -> ::std::os::raw::c_int;
4316 | }
4317 | extern "C" {
4318 |     pub fn whisper_tokenize(
4319 |         ctx: *mut whisper_context,
4320 |         text: *const ::std::os::raw::c_char,
4321 |         tokens: *mut whisper_token,
4322 |         n_max_tokens: ::std::os::raw::c_int,
4323 |     ) -> ::std::os::raw::c_int;
4324 | }
4325 | extern "C" {
4326 |     pub fn whisper_token_count(
4327 |         ctx: *mut whisper_context,
4328 |         text: *const ::std::os::raw::c_char,
4329 |     ) -> ::std::os::raw::c_int;
4330 | }
4331 | extern "C" {
4332 |     pub fn whisper_lang_max_id() -> ::std::os::raw::c_int;
4333 | }
4334 | extern "C" {
4335 |     pub fn whisper_lang_id(lang: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
4336 | }
4337 | extern "C" {
4338 |     pub fn whisper_lang_str(id: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
4339 | }
4340 | extern "C" {
4341 |     pub fn whisper_lang_str_full(id: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
4342 | }
4343 | extern "C" {
4344 |     pub fn whisper_lang_auto_detect(
4345 |         ctx: *mut whisper_context,
4346 |         offset_ms: ::std::os::raw::c_int,
4347 |         n_threads: ::std::os::raw::c_int,
4348 |         lang_probs: *mut f32,
4349 |     ) -> ::std::os::raw::c_int;
4350 | }
4351 | extern "C" {
4352 |     pub fn whisper_lang_auto_detect_with_state(
4353 |         ctx: *mut whisper_context,
4354 |         state: *mut whisper_state,
4355 |         offset_ms: ::std::os::raw::c_int,
4356 |         n_threads: ::std::os::raw::c_int,
4357 |         lang_probs: *mut f32,
4358 |     ) -> ::std::os::raw::c_int;
4359 | }
4360 | extern "C" {
4361 |     pub fn whisper_n_len(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4362 | }
4363 | extern "C" {
4364 |     pub fn whisper_n_len_from_state(state: *mut whisper_state) -> ::std::os::raw::c_int;
4365 | }
4366 | extern "C" {
4367 |     pub fn whisper_n_vocab(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4368 | }
4369 | extern "C" {
4370 |     pub fn whisper_n_text_ctx(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4371 | }
4372 | extern "C" {
4373 |     pub fn whisper_n_audio_ctx(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4374 | }
4375 | extern "C" {
4376 |     pub fn whisper_is_multilingual(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4377 | }
4378 | extern "C" {
4379 |     pub fn whisper_model_n_vocab(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4380 | }
4381 | extern "C" {
4382 |     pub fn whisper_model_n_audio_ctx(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4383 | }
4384 | extern "C" {
4385 |     pub fn whisper_model_n_audio_state(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4386 | }
4387 | extern "C" {
4388 |     pub fn whisper_model_n_audio_head(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4389 | }
4390 | extern "C" {
4391 |     pub fn whisper_model_n_audio_layer(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4392 | }
4393 | extern "C" {
4394 |     pub fn whisper_model_n_text_ctx(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4395 | }
4396 | extern "C" {
4397 |     pub fn whisper_model_n_text_state(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4398 | }
4399 | extern "C" {
4400 |     pub fn whisper_model_n_text_head(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4401 | }
4402 | extern "C" {
4403 |     pub fn whisper_model_n_text_layer(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4404 | }
4405 | extern "C" {
4406 |     pub fn whisper_model_n_mels(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4407 | }
4408 | extern "C" {
4409 |     pub fn whisper_model_ftype(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4410 | }
4411 | extern "C" {
4412 |     pub fn whisper_model_type(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4413 | }
4414 | extern "C" {
4415 |     pub fn whisper_get_logits(ctx: *mut whisper_context) -> *mut f32;
4416 | }
4417 | extern "C" {
4418 |     pub fn whisper_get_logits_from_state(state: *mut whisper_state) -> *mut f32;
4419 | }
4420 | extern "C" {
4421 |     pub fn whisper_token_to_str(
4422 |         ctx: *mut whisper_context,
4423 |         token: whisper_token,
4424 |     ) -> *const ::std::os::raw::c_char;
4425 | }
4426 | extern "C" {
4427 |     pub fn whisper_model_type_readable(ctx: *mut whisper_context) -> *const ::std::os::raw::c_char;
4428 | }
4429 | extern "C" {
4430 |     pub fn whisper_token_eot(ctx: *mut whisper_context) -> whisper_token;
4431 | }
4432 | extern "C" {
4433 |     pub fn whisper_token_sot(ctx: *mut whisper_context) -> whisper_token;
4434 | }
4435 | extern "C" {
4436 |     pub fn whisper_token_solm(ctx: *mut whisper_context) -> whisper_token;
4437 | }
4438 | extern "C" {
4439 |     pub fn whisper_token_prev(ctx: *mut whisper_context) -> whisper_token;
4440 | }
4441 | extern "C" {
4442 |     pub fn whisper_token_nosp(ctx: *mut whisper_context) -> whisper_token;
4443 | }
4444 | extern "C" {
4445 |     pub fn whisper_token_not(ctx: *mut whisper_context) -> whisper_token;
4446 | }
4447 | extern "C" {
4448 |     pub fn whisper_token_beg(ctx: *mut whisper_context) -> whisper_token;
4449 | }
4450 | extern "C" {
4451 |     pub fn whisper_token_lang(
4452 |         ctx: *mut whisper_context,
4453 |         lang_id: ::std::os::raw::c_int,
4454 |     ) -> whisper_token;
4455 | }
4456 | extern "C" {
4457 |     pub fn whisper_token_translate(ctx: *mut whisper_context) -> whisper_token;
4458 | }
4459 | extern "C" {
4460 |     pub fn whisper_token_transcribe(ctx: *mut whisper_context) -> whisper_token;
4461 | }
4462 | extern "C" {
4463 |     pub fn whisper_print_timings(ctx: *mut whisper_context);
4464 | }
4465 | extern "C" {
4466 |     pub fn whisper_reset_timings(ctx: *mut whisper_context);
4467 | }
4468 | extern "C" {
4469 |     pub fn whisper_print_system_info() -> *const ::std::os::raw::c_char;
4470 | }
4471 | pub const whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY: whisper_sampling_strategy = 0;
4472 | pub const whisper_sampling_strategy_WHISPER_SAMPLING_BEAM_SEARCH: whisper_sampling_strategy = 1;
4473 | pub type whisper_sampling_strategy = ::std::os::raw::c_uint;
4474 | pub type whisper_new_segment_callback = ::std::option::Option<
4475 |     unsafe extern "C" fn(
4476 |         ctx: *mut whisper_context,
4477 |         state: *mut whisper_state,
4478 |         n_new: ::std::os::raw::c_int,
4479 |         user_data: *mut ::std::os::raw::c_void,
4480 |     ),
4481 | >;
4482 | pub type whisper_progress_callback = ::std::option::Option<
4483 |     unsafe extern "C" fn(
4484 |         ctx: *mut whisper_context,
4485 |         state: *mut whisper_state,
4486 |         progress: ::std::os::raw::c_int,
4487 |         user_data: *mut ::std::os::raw::c_void,
4488 |     ),
4489 | >;
4490 | pub type whisper_encoder_begin_callback = ::std::option::Option<
4491 |     unsafe extern "C" fn(
4492 |         ctx: *mut whisper_context,
4493 |         state: *mut whisper_state,
4494 |         user_data: *mut ::std::os::raw::c_void,
4495 |     ) -> bool,
4496 | >;
4497 | pub type whisper_logits_filter_callback = ::std::option::Option<
4498 |     unsafe extern "C" fn(
4499 |         ctx: *mut whisper_context,
4500 |         state: *mut whisper_state,
4501 |         tokens: *const whisper_token_data,
4502 |         n_tokens: ::std::os::raw::c_int,
4503 |         logits: *mut f32,
4504 |         user_data: *mut ::std::os::raw::c_void,
4505 |     ),
4506 | >;
4507 | #[repr(C)]
4508 | #[derive(Debug, Copy, Clone)]
4509 | pub struct whisper_full_params {
4510 |     pub strategy: whisper_sampling_strategy,
4511 |     pub n_threads: ::std::os::raw::c_int,
4512 |     pub n_max_text_ctx: ::std::os::raw::c_int,
4513 |     pub offset_ms: ::std::os::raw::c_int,
4514 |     pub duration_ms: ::std::os::raw::c_int,
4515 |     pub translate: bool,
4516 |     pub no_context: bool,
4517 |     pub no_timestamps: bool,
4518 |     pub single_segment: bool,
4519 |     pub print_special: bool,
4520 |     pub print_progress: bool,
4521 |     pub print_realtime: bool,
4522 |     pub print_timestamps: bool,
4523 |     pub token_timestamps: bool,
4524 |     pub thold_pt: f32,
4525 |     pub thold_ptsum: f32,
4526 |     pub max_len: ::std::os::raw::c_int,
4527 |     pub split_on_word: bool,
4528 |     pub max_tokens: ::std::os::raw::c_int,
4529 |     pub debug_mode: bool,
4530 |     pub audio_ctx: ::std::os::raw::c_int,
4531 |     pub tdrz_enable: bool,
4532 |     pub suppress_regex: *const ::std::os::raw::c_char,
4533 |     pub initial_prompt: *const ::std::os::raw::c_char,
4534 |     pub prompt_tokens: *const whisper_token,
4535 |     pub prompt_n_tokens: ::std::os::raw::c_int,
4536 |     pub language: *const ::std::os::raw::c_char,
4537 |     pub detect_language: bool,
4538 |     pub suppress_blank: bool,
4539 |     pub suppress_non_speech_tokens: bool,
4540 |     pub temperature: f32,
4541 |     pub max_initial_ts: f32,
4542 |     pub length_penalty: f32,
4543 |     pub temperature_inc: f32,
4544 |     pub entropy_thold: f32,
4545 |     pub logprob_thold: f32,
4546 |     pub no_speech_thold: f32,
4547 |     pub greedy: whisper_full_params__bindgen_ty_1,
4548 |     pub beam_search: whisper_full_params__bindgen_ty_2,
4549 |     pub new_segment_callback: whisper_new_segment_callback,
4550 |     pub new_segment_callback_user_data: *mut ::std::os::raw::c_void,
4551 |     pub progress_callback: whisper_progress_callback,
4552 |     pub progress_callback_user_data: *mut ::std::os::raw::c_void,
4553 |     pub encoder_begin_callback: whisper_encoder_begin_callback,
4554 |     pub encoder_begin_callback_user_data: *mut ::std::os::raw::c_void,
4555 |     pub abort_callback: ggml_abort_callback,
4556 |     pub abort_callback_user_data: *mut ::std::os::raw::c_void,
4557 |     pub logits_filter_callback: whisper_logits_filter_callback,
4558 |     pub logits_filter_callback_user_data: *mut ::std::os::raw::c_void,
4559 |     pub grammar_rules: *mut *const whisper_grammar_element,
4560 |     pub n_grammar_rules: usize,
4561 |     pub i_start_rule: usize,
4562 |     pub grammar_penalty: f32,
4563 | }
4564 | #[repr(C)]
4565 | #[derive(Debug, Copy, Clone)]
4566 | pub struct whisper_full_params__bindgen_ty_1 {
4567 |     pub best_of: ::std::os::raw::c_int,
4568 | }
4569 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4570 | const _: () = {
4571 |     ["Size of whisper_full_params__bindgen_ty_1"]
4572 |         [::std::mem::size_of::<whisper_full_params__bindgen_ty_1>() - 4usize];
4573 |     ["Alignment of whisper_full_params__bindgen_ty_1"]
4574 |         [::std::mem::align_of::<whisper_full_params__bindgen_ty_1>() - 4usize];
4575 |     ["Offset of field: whisper_full_params__bindgen_ty_1::best_of"]
4576 |         [::std::mem::offset_of!(whisper_full_params__bindgen_ty_1, best_of) - 0usize];
4577 | };
4578 | #[repr(C)]
4579 | #[derive(Debug, Copy, Clone)]
4580 | pub struct whisper_full_params__bindgen_ty_2 {
4581 |     pub beam_size: ::std::os::raw::c_int,
4582 |     pub patience: f32,
4583 | }
4584 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4585 | const _: () = {
4586 |     ["Size of whisper_full_params__bindgen_ty_2"]
4587 |         [::std::mem::size_of::<whisper_full_params__bindgen_ty_2>() - 8usize];
4588 |     ["Alignment of whisper_full_params__bindgen_ty_2"]
4589 |         [::std::mem::align_of::<whisper_full_params__bindgen_ty_2>() - 4usize];
4590 |     ["Offset of field: whisper_full_params__bindgen_ty_2::beam_size"]
4591 |         [::std::mem::offset_of!(whisper_full_params__bindgen_ty_2, beam_size) - 0usize];
4592 |     ["Offset of field: whisper_full_params__bindgen_ty_2::patience"]
4593 |         [::std::mem::offset_of!(whisper_full_params__bindgen_ty_2, patience) - 4usize];
4594 | };
4595 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4596 | const _: () = {
4597 |     ["Size of whisper_full_params"][::std::mem::size_of::<whisper_full_params>() - 264usize];
4598 |     ["Alignment of whisper_full_params"][::std::mem::align_of::<whisper_full_params>() - 8usize];
4599 |     ["Offset of field: whisper_full_params::strategy"]
4600 |         [::std::mem::offset_of!(whisper_full_params, strategy) - 0usize];
4601 |     ["Offset of field: whisper_full_params::n_threads"]
4602 |         [::std::mem::offset_of!(whisper_full_params, n_threads) - 4usize];
4603 |     ["Offset of field: whisper_full_params::n_max_text_ctx"]
4604 |         [::std::mem::offset_of!(whisper_full_params, n_max_text_ctx) - 8usize];
4605 |     ["Offset of field: whisper_full_params::offset_ms"]
4606 |         [::std::mem::offset_of!(whisper_full_params, offset_ms) - 12usize];
4607 |     ["Offset of field: whisper_full_params::duration_ms"]
4608 |         [::std::mem::offset_of!(whisper_full_params, duration_ms) - 16usize];
4609 |     ["Offset of field: whisper_full_params::translate"]
4610 |         [::std::mem::offset_of!(whisper_full_params, translate) - 20usize];
4611 |     ["Offset of field: whisper_full_params::no_context"]
4612 |         [::std::mem::offset_of!(whisper_full_params, no_context) - 21usize];
4613 |     ["Offset of field: whisper_full_params::no_timestamps"]
4614 |         [::std::mem::offset_of!(whisper_full_params, no_timestamps) - 22usize];
4615 |     ["Offset of field: whisper_full_params::single_segment"]
4616 |         [::std::mem::offset_of!(whisper_full_params, single_segment) - 23usize];
4617 |     ["Offset of field: whisper_full_params::print_special"]
4618 |         [::std::mem::offset_of!(whisper_full_params, print_special) - 24usize];
4619 |     ["Offset of field: whisper_full_params::print_progress"]
4620 |         [::std::mem::offset_of!(whisper_full_params, print_progress) - 25usize];
4621 |     ["Offset of field: whisper_full_params::print_realtime"]
4622 |         [::std::mem::offset_of!(whisper_full_params, print_realtime) - 26usize];
4623 |     ["Offset of field: whisper_full_params::print_timestamps"]
4624 |         [::std::mem::offset_of!(whisper_full_params, print_timestamps) - 27usize];
4625 |     ["Offset of field: whisper_full_params::token_timestamps"]
4626 |         [::std::mem::offset_of!(whisper_full_params, token_timestamps) - 28usize];
4627 |     ["Offset of field: whisper_full_params::thold_pt"]
4628 |         [::std::mem::offset_of!(whisper_full_params, thold_pt) - 32usize];
4629 |     ["Offset of field: whisper_full_params::thold_ptsum"]
4630 |         [::std::mem::offset_of!(whisper_full_params, thold_ptsum) - 36usize];
4631 |     ["Offset of field: whisper_full_params::max_len"]
4632 |         [::std::mem::offset_of!(whisper_full_params, max_len) - 40usize];
4633 |     ["Offset of field: whisper_full_params::split_on_word"]
4634 |         [::std::mem::offset_of!(whisper_full_params, split_on_word) - 44usize];
4635 |     ["Offset of field: whisper_full_params::max_tokens"]
4636 |         [::std::mem::offset_of!(whisper_full_params, max_tokens) - 48usize];
4637 |     ["Offset of field: whisper_full_params::debug_mode"]
4638 |         [::std::mem::offset_of!(whisper_full_params, debug_mode) - 52usize];
4639 |     ["Offset of field: whisper_full_params::audio_ctx"]
4640 |         [::std::mem::offset_of!(whisper_full_params, audio_ctx) - 56usize];
4641 |     ["Offset of field: whisper_full_params::tdrz_enable"]
4642 |         [::std::mem::offset_of!(whisper_full_params, tdrz_enable) - 60usize];
4643 |     ["Offset of field: whisper_full_params::suppress_regex"]
4644 |         [::std::mem::offset_of!(whisper_full_params, suppress_regex) - 64usize];
4645 |     ["Offset of field: whisper_full_params::initial_prompt"]
4646 |         [::std::mem::offset_of!(whisper_full_params, initial_prompt) - 72usize];
4647 |     ["Offset of field: whisper_full_params::prompt_tokens"]
4648 |         [::std::mem::offset_of!(whisper_full_params, prompt_tokens) - 80usize];
4649 |     ["Offset of field: whisper_full_params::prompt_n_tokens"]
4650 |         [::std::mem::offset_of!(whisper_full_params, prompt_n_tokens) - 88usize];
4651 |     ["Offset of field: whisper_full_params::language"]
4652 |         [::std::mem::offset_of!(whisper_full_params, language) - 96usize];
4653 |     ["Offset of field: whisper_full_params::detect_language"]
4654 |         [::std::mem::offset_of!(whisper_full_params, detect_language) - 104usize];
4655 |     ["Offset of field: whisper_full_params::suppress_blank"]
4656 |         [::std::mem::offset_of!(whisper_full_params, suppress_blank) - 105usize];
4657 |     ["Offset of field: whisper_full_params::suppress_non_speech_tokens"]
4658 |         [::std::mem::offset_of!(whisper_full_params, suppress_non_speech_tokens) - 106usize];
4659 |     ["Offset of field: whisper_full_params::temperature"]
4660 |         [::std::mem::offset_of!(whisper_full_params, temperature) - 108usize];
4661 |     ["Offset of field: whisper_full_params::max_initial_ts"]
4662 |         [::std::mem::offset_of!(whisper_full_params, max_initial_ts) - 112usize];
4663 |     ["Offset of field: whisper_full_params::length_penalty"]
4664 |         [::std::mem::offset_of!(whisper_full_params, length_penalty) - 116usize];
4665 |     ["Offset of field: whisper_full_params::temperature_inc"]
4666 |         [::std::mem::offset_of!(whisper_full_params, temperature_inc) - 120usize];
4667 |     ["Offset of field: whisper_full_params::entropy_thold"]
4668 |         [::std::mem::offset_of!(whisper_full_params, entropy_thold) - 124usize];
4669 |     ["Offset of field: whisper_full_params::logprob_thold"]
4670 |         [::std::mem::offset_of!(whisper_full_params, logprob_thold) - 128usize];
4671 |     ["Offset of field: whisper_full_params::no_speech_thold"]
4672 |         [::std::mem::offset_of!(whisper_full_params, no_speech_thold) - 132usize];
4673 |     ["Offset of field: whisper_full_params::greedy"]
4674 |         [::std::mem::offset_of!(whisper_full_params, greedy) - 136usize];
4675 |     ["Offset of field: whisper_full_params::beam_search"]
4676 |         [::std::mem::offset_of!(whisper_full_params, beam_search) - 140usize];
4677 |     ["Offset of field: whisper_full_params::new_segment_callback"]
4678 |         [::std::mem::offset_of!(whisper_full_params, new_segment_callback) - 152usize];
4679 |     ["Offset of field: whisper_full_params::new_segment_callback_user_data"]
4680 |         [::std::mem::offset_of!(whisper_full_params, new_segment_callback_user_data) - 160usize];
4681 |     ["Offset of field: whisper_full_params::progress_callback"]
4682 |         [::std::mem::offset_of!(whisper_full_params, progress_callback) - 168usize];
4683 |     ["Offset of field: whisper_full_params::progress_callback_user_data"]
4684 |         [::std::mem::offset_of!(whisper_full_params, progress_callback_user_data) - 176usize];
4685 |     ["Offset of field: whisper_full_params::encoder_begin_callback"]
4686 |         [::std::mem::offset_of!(whisper_full_params, encoder_begin_callback) - 184usize];
4687 |     ["Offset of field: whisper_full_params::encoder_begin_callback_user_data"]
4688 |         [::std::mem::offset_of!(whisper_full_params, encoder_begin_callback_user_data) - 192usize];
4689 |     ["Offset of field: whisper_full_params::abort_callback"]
4690 |         [::std::mem::offset_of!(whisper_full_params, abort_callback) - 200usize];
4691 |     ["Offset of field: whisper_full_params::abort_callback_user_data"]
4692 |         [::std::mem::offset_of!(whisper_full_params, abort_callback_user_data) - 208usize];
4693 |     ["Offset of field: whisper_full_params::logits_filter_callback"]
4694 |         [::std::mem::offset_of!(whisper_full_params, logits_filter_callback) - 216usize];
4695 |     ["Offset of field: whisper_full_params::logits_filter_callback_user_data"]
4696 |         [::std::mem::offset_of!(whisper_full_params, logits_filter_callback_user_data) - 224usize];
4697 |     ["Offset of field: whisper_full_params::grammar_rules"]
4698 |         [::std::mem::offset_of!(whisper_full_params, grammar_rules) - 232usize];
4699 |     ["Offset of field: whisper_full_params::n_grammar_rules"]
4700 |         [::std::mem::offset_of!(whisper_full_params, n_grammar_rules) - 240usize];
4701 |     ["Offset of field: whisper_full_params::i_start_rule"]
4702 |         [::std::mem::offset_of!(whisper_full_params, i_start_rule) - 248usize];
4703 |     ["Offset of field: whisper_full_params::grammar_penalty"]
4704 |         [::std::mem::offset_of!(whisper_full_params, grammar_penalty) - 256usize];
4705 | };
4706 | extern "C" {
4707 |     pub fn whisper_context_default_params_by_ref() -> *mut whisper_context_params;
4708 | }
4709 | extern "C" {
4710 |     pub fn whisper_context_default_params() -> whisper_context_params;
4711 | }
4712 | extern "C" {
4713 |     pub fn whisper_full_default_params_by_ref(
4714 |         strategy: whisper_sampling_strategy,
4715 |     ) -> *mut whisper_full_params;
4716 | }
4717 | extern "C" {
4718 |     pub fn whisper_full_default_params(strategy: whisper_sampling_strategy) -> whisper_full_params;
4719 | }
4720 | extern "C" {
4721 |     pub fn whisper_full(
4722 |         ctx: *mut whisper_context,
4723 |         params: whisper_full_params,
4724 |         samples: *const f32,
4725 |         n_samples: ::std::os::raw::c_int,
4726 |     ) -> ::std::os::raw::c_int;
4727 | }
4728 | extern "C" {
4729 |     pub fn whisper_full_with_state(
4730 |         ctx: *mut whisper_context,
4731 |         state: *mut whisper_state,
4732 |         params: whisper_full_params,
4733 |         samples: *const f32,
4734 |         n_samples: ::std::os::raw::c_int,
4735 |     ) -> ::std::os::raw::c_int;
4736 | }
4737 | extern "C" {
4738 |     pub fn whisper_full_parallel(
4739 |         ctx: *mut whisper_context,
4740 |         params: whisper_full_params,
4741 |         samples: *const f32,
4742 |         n_samples: ::std::os::raw::c_int,
4743 |         n_processors: ::std::os::raw::c_int,
4744 |     ) -> ::std::os::raw::c_int;
4745 | }
4746 | extern "C" {
4747 |     pub fn whisper_full_n_segments(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4748 | }
4749 | extern "C" {
4750 |     pub fn whisper_full_n_segments_from_state(state: *mut whisper_state) -> ::std::os::raw::c_int;
4751 | }
4752 | extern "C" {
4753 |     pub fn whisper_full_lang_id(ctx: *mut whisper_context) -> ::std::os::raw::c_int;
4754 | }
4755 | extern "C" {
4756 |     pub fn whisper_full_lang_id_from_state(state: *mut whisper_state) -> ::std::os::raw::c_int;
4757 | }
4758 | extern "C" {
4759 |     pub fn whisper_full_get_segment_t0(
4760 |         ctx: *mut whisper_context,
4761 |         i_segment: ::std::os::raw::c_int,
4762 |     ) -> i64;
4763 | }
4764 | extern "C" {
4765 |     pub fn whisper_full_get_segment_t0_from_state(
4766 |         state: *mut whisper_state,
4767 |         i_segment: ::std::os::raw::c_int,
4768 |     ) -> i64;
4769 | }
4770 | extern "C" {
4771 |     pub fn whisper_full_get_segment_t1(
4772 |         ctx: *mut whisper_context,
4773 |         i_segment: ::std::os::raw::c_int,
4774 |     ) -> i64;
4775 | }
4776 | extern "C" {
4777 |     pub fn whisper_full_get_segment_t1_from_state(
4778 |         state: *mut whisper_state,
4779 |         i_segment: ::std::os::raw::c_int,
4780 |     ) -> i64;
4781 | }
4782 | extern "C" {
4783 |     pub fn whisper_full_get_segment_speaker_turn_next(
4784 |         ctx: *mut whisper_context,
4785 |         i_segment: ::std::os::raw::c_int,
4786 |     ) -> bool;
4787 | }
4788 | extern "C" {
4789 |     pub fn whisper_full_get_segment_speaker_turn_next_from_state(
4790 |         state: *mut whisper_state,
4791 |         i_segment: ::std::os::raw::c_int,
4792 |     ) -> bool;
4793 | }
4794 | extern "C" {
4795 |     pub fn whisper_full_get_segment_text(
4796 |         ctx: *mut whisper_context,
4797 |         i_segment: ::std::os::raw::c_int,
4798 |     ) -> *const ::std::os::raw::c_char;
4799 | }
4800 | extern "C" {
4801 |     pub fn whisper_full_get_segment_text_from_state(
4802 |         state: *mut whisper_state,
4803 |         i_segment: ::std::os::raw::c_int,
4804 |     ) -> *const ::std::os::raw::c_char;
4805 | }
4806 | extern "C" {
4807 |     pub fn whisper_full_n_tokens(
4808 |         ctx: *mut whisper_context,
4809 |         i_segment: ::std::os::raw::c_int,
4810 |     ) -> ::std::os::raw::c_int;
4811 | }
4812 | extern "C" {
4813 |     pub fn whisper_full_n_tokens_from_state(
4814 |         state: *mut whisper_state,
4815 |         i_segment: ::std::os::raw::c_int,
4816 |     ) -> ::std::os::raw::c_int;
4817 | }
4818 | extern "C" {
4819 |     pub fn whisper_full_get_token_text(
4820 |         ctx: *mut whisper_context,
4821 |         i_segment: ::std::os::raw::c_int,
4822 |         i_token: ::std::os::raw::c_int,
4823 |     ) -> *const ::std::os::raw::c_char;
4824 | }
4825 | extern "C" {
4826 |     pub fn whisper_full_get_token_text_from_state(
4827 |         ctx: *mut whisper_context,
4828 |         state: *mut whisper_state,
4829 |         i_segment: ::std::os::raw::c_int,
4830 |         i_token: ::std::os::raw::c_int,
4831 |     ) -> *const ::std::os::raw::c_char;
4832 | }
4833 | extern "C" {
4834 |     pub fn whisper_full_get_token_id(
4835 |         ctx: *mut whisper_context,
4836 |         i_segment: ::std::os::raw::c_int,
4837 |         i_token: ::std::os::raw::c_int,
4838 |     ) -> whisper_token;
4839 | }
4840 | extern "C" {
4841 |     pub fn whisper_full_get_token_id_from_state(
4842 |         state: *mut whisper_state,
4843 |         i_segment: ::std::os::raw::c_int,
4844 |         i_token: ::std::os::raw::c_int,
4845 |     ) -> whisper_token;
4846 | }
4847 | extern "C" {
4848 |     pub fn whisper_full_get_token_data(
4849 |         ctx: *mut whisper_context,
4850 |         i_segment: ::std::os::raw::c_int,
4851 |         i_token: ::std::os::raw::c_int,
4852 |     ) -> whisper_token_data;
4853 | }
4854 | extern "C" {
4855 |     pub fn whisper_full_get_token_data_from_state(
4856 |         state: *mut whisper_state,
4857 |         i_segment: ::std::os::raw::c_int,
4858 |         i_token: ::std::os::raw::c_int,
4859 |     ) -> whisper_token_data;
4860 | }
4861 | extern "C" {
4862 |     pub fn whisper_full_get_token_p(
4863 |         ctx: *mut whisper_context,
4864 |         i_segment: ::std::os::raw::c_int,
4865 |         i_token: ::std::os::raw::c_int,
4866 |     ) -> f32;
4867 | }
4868 | extern "C" {
4869 |     pub fn whisper_full_get_token_p_from_state(
4870 |         state: *mut whisper_state,
4871 |         i_segment: ::std::os::raw::c_int,
4872 |         i_token: ::std::os::raw::c_int,
4873 |     ) -> f32;
4874 | }
4875 | extern "C" {
4876 |     pub fn whisper_bench_memcpy(n_threads: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
4877 | }
4878 | extern "C" {
4879 |     pub fn whisper_bench_memcpy_str(
4880 |         n_threads: ::std::os::raw::c_int,
4881 |     ) -> *const ::std::os::raw::c_char;
4882 | }
4883 | extern "C" {
4884 |     pub fn whisper_bench_ggml_mul_mat(n_threads: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
4885 | }
4886 | extern "C" {
4887 |     pub fn whisper_bench_ggml_mul_mat_str(
4888 |         n_threads: ::std::os::raw::c_int,
4889 |     ) -> *const ::std::os::raw::c_char;
4890 | }
4891 | extern "C" {
4892 |     pub fn whisper_log_set(log_callback: ggml_log_callback, user_data: *mut ::std::os::raw::c_void);
4893 | }
4894 | pub type __builtin_va_list = [__va_list_tag; 1usize];
4895 | #[repr(C)]
4896 | #[derive(Debug, Copy, Clone)]
4897 | pub struct __va_list_tag {
4898 |     pub gp_offset: ::std::os::raw::c_uint,
4899 |     pub fp_offset: ::std::os::raw::c_uint,
4900 |     pub overflow_arg_area: *mut ::std::os::raw::c_void,
4901 |     pub reg_save_area: *mut ::std::os::raw::c_void,
4902 | }
4903 | #[allow(clippy::unnecessary_operation, clippy::identity_op)]
4904 | const _: () = {
4905 |     ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
4906 |     ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
4907 |     ["Offset of field: __va_list_tag::gp_offset"]
4908 |         [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
4909 |     ["Offset of field: __va_list_tag::fp_offset"]
4910 |         [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
4911 |     ["Offset of field: __va_list_tag::overflow_arg_area"]
4912 |         [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
4913 |     ["Offset of field: __va_list_tag::reg_save_area"]
4914 |         [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
4915 | };
4916 | #[repr(C)]
4917 | #[derive(Debug, Copy, Clone)]
4918 | pub struct ggml_backend_buffer {
4919 |     pub _address: u8,
4920 | }
4921 | 


--------------------------------------------------------------------------------
/sys/src/lib.rs:
--------------------------------------------------------------------------------
1 | #![allow(non_upper_case_globals)]
2 | #![allow(non_camel_case_types)]
3 | #![allow(non_snake_case)]
4 | 
5 | include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
6 | 


--------------------------------------------------------------------------------
/sys/wrapper.h:
--------------------------------------------------------------------------------
1 | #include <include/whisper.h>
2 | #include <ggml/include/ggml.h>
3 | 


--------------------------------------------------------------------------------