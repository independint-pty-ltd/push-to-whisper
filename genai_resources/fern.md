├── .github
    └── workflows
    │   └── rust.yml
├── .gitignore
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.toml
├── LICENSE
├── README.md
├── examples
    ├── cmd-program.rs
    ├── colored.rs
    ├── date-based-file-log.rs
    ├── meta-logging.rs
    ├── pretty-colored-screenshot.png
    ├── pretty-colored.rs
    ├── syslog.rs
    ├── syslog3.rs
    ├── syslog4.rs
    └── syslog7.rs
├── src
    ├── builders.rs
    ├── colors.rs
    ├── errors.rs
    ├── lib.rs
    ├── log_impl.rs
    ├── meta.rs
    └── syslog.rs
└── tests
    ├── channel_logging.rs
    ├── enabled_is_deep_check.rs
    ├── file_logging.rs
    ├── global_logging.rs
    ├── meta_logging.rs
    ├── panic_logging.rs
    ├── reopen_logging.rs
    ├── support.rs
    └── write_logging.rs


/.github/workflows/rust.yml:
--------------------------------------------------------------------------------
 1 | name: tests
 2 | 
 3 | on:
 4 |   push:
 5 |   pull_request:
 6 | 
 7 | env:
 8 |   CARGO_TERM_COLOR: always
 9 | 
10 | jobs:
11 |   test:
12 |     name: Run Tests
13 |     runs-on: ${{ matrix.os }}
14 |     strategy:
15 |       matrix:
16 |         rust:
17 |           - stable
18 |           - beta
19 |         os:
20 |           - ubuntu-latest
21 |           - windows-latest
22 |         toolchain:
23 |           - stable
24 |           - 1.70.0
25 |     steps:
26 |       - uses: actions/checkout@v4
27 |       - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
28 |       - run: cargo build --all-targets
29 |       - run: cargo build --all-targets --no-default-features
30 |       - run: cargo build --all-targets --all-features
31 |       - run: cargo test
32 |       - run: cargo test --no-default-features
33 |       - run: cargo test --features=colored
34 |       - run: cargo test --features=syslog-3
35 |       - run: cargo test --features=syslog-4
36 |       - run: cargo test --features=syslog-6
37 |       - run: cargo test --features=syslog-7
38 |       - run: cargo test --features=reopen-03
39 |       - run: cargo test --features=reopen-1
40 |       - run: cargo test --features=meta-logging-in-format
41 |       - run: cargo test --all-features
42 |       - run: cargo run --example cmd-program
43 |       - run: cargo run --example cmd-program -- --verbose
44 |       - run: cargo run --example colored --features colored
45 |       - run: cargo run --example pretty-colored --features colored
46 |       - run: cargo run --example date-based-file-log --features date-based
47 |       # we don't exactly have a good test suite for DateBased right now, so let's at least do this:
48 |       - run: cargo run --example date-based-file-log --features date-based,meta-logging-in-format
49 |       - run: cargo doc --all-features
50 |         env:
51 |           RUSTDOCFLAGS: -D warnings
52 |   linux:
53 |     name: Linux Examples
54 |     runs-on: ubuntu-latest
55 |     strategy:
56 |       matrix:
57 |         rust:
58 |           - stable
59 |           - beta
60 |         toolchain:
61 |           - stable
62 |           - 1.70.0
63 |     steps:
64 |       - uses: actions/checkout@v4
65 |       - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
66 |       - run: cargo run --example syslog3 --features syslog-3
67 |       - run: cargo run --example syslog4 --features syslog-4
68 |       - run: cargo run --example syslog --features syslog-6
69 |       - run: cargo run --example syslog7 --features syslog-7
70 |   msrv:
71 |     name: MSRV Compat
72 |     runs-on: ${{ matrix.os }}
73 |     strategy:
74 |       matrix:
75 |         os:
76 |           - ubuntu-latest
77 |           - windows-latest
78 |     steps:
79 |       - uses: actions/checkout@v4
80 |       - run: rustup update 1.60.0 && rustup default 1.60.0
81 |       - run: cargo build
82 |   optional_lints:
83 |     name: Optional Lints
84 |     runs-on: ubuntu-latest
85 |     steps:
86 |       - uses: actions/checkout@v4
87 |       - run: rustup update stable && rustup default stable
88 |       - run: cargo fmt --check
89 |       - run: cargo clippy --all-features --all-targets -- -D warnings
90 | 


--------------------------------------------------------------------------------
/.gitignore:
--------------------------------------------------------------------------------
 1 | # Compiled source #
 2 | ###################
 3 | target
 4 | 
 5 | # Build files #
 6 | ###############
 7 | /Cargo.lock
 8 | 
 9 | # Packages #
10 | ############
11 | *.7z
12 | *.dmg
13 | *.gz
14 | *.iso
15 | *.jar
16 | *.rar
17 | *.tar
18 | *.zip
19 | 
20 | # OS generated files #
21 | ######################
22 | .DS_Store
23 | ehthumbs.db
24 | Icon?
25 | Thumbs.db
26 | 
27 | # Project files #
28 | #################
29 | .classpath
30 | .externalToolBuilders
31 | .idea
32 | .project
33 | .settings
34 | build
35 | dist
36 | nbproject
37 | atlassian-ide-plugin.xml
38 | build.xml
39 | nb-configuration.xml
40 | *.iml
41 | *.ipr
42 | *.iws
43 | *.sublime-project
44 | *.sublime-workspace
45 | .vscode
46 | 


--------------------------------------------------------------------------------
/CHANGELOG.md:
--------------------------------------------------------------------------------
  1 | Unreleased
  2 | ==========
  3 | 
  4 | 
  5 | 0.7.1 (2024-12-15)
  6 | ==================
  7 | 
  8 | - Add syslog v7 support (thanks [@BlackDex]!)
  9 | 
 10 | 0.7.0 (2024-10-20)
 11 | ==================
 12 | 
 13 | - Upgrade `colored` to version 2. This is a breaking change due to
 14 |   `colored` being exposed in the public API of `fern`. (thanks [@faern] for
 15 |   doing the boilerplate here!)
 16 | - Remove most of the unsoundness warning, and update it to reflect fern 0.7.0
 17 |   fixing the issue.
 18 | 
 19 | 0.6.2 (2023-03-23)
 20 | ==================
 21 | 
 22 | - Add security warning for "colored" feature when using a global allocator
 23 |   to doc homepage and README.md. See
 24 |   [fern-0.6.2 README.md](https://github.com/daboross/fern/blob/fern-0.6.2/README.md)
 25 | - Change examples to use `env_logger`-style formatting
 26 | - Improve main documentation example explanation
 27 | - Misc. style improvements (clippy lint fixes which have no functional change)
 28 | 
 29 | 0.6.1 (2022-04-15)
 30 | ==================
 31 | 
 32 | - Document reopen feature requiring a feature flag
 33 |   (thanks [@Palladinium]!)
 34 | - Fix typo in colors documentation (thanks [@sourcefrog]!)
 35 | - Add support for reopen v1 under reopen-1 feature (thanks [@vorner]!)
 36 | - Add syslog v6 support under syslog-6 feature (thanks folk at [@EasyPost]!)
 37 | - Fix README badges
 38 | 
 39 | 0.6.0 (2020-03-09)
 40 | ==================
 41 | 
 42 | - Move date-based file logger under its own feature (thanks [@dekellum]!)
 43 | 
 44 | 0.5.9 (2019-10-23)
 45 | ==================
 46 | 
 47 | - Add a rotating date-based file logger (thanks [@tanujitghosh]!)
 48 | - Add a file logger which supports reopening files on SIGHUP via the
 49 |   [reopen] crate (thanks [@itkovian]!)
 50 | - Ensure Windows console colors are enabled whenever a ColoredLevelConfig
 51 |   is created on Windows (thanks [@Songtronix]!)
 52 | - Change minimum rust version from 1.16 to 1.32, as it had already
 53 |   been effectively changed by a patch update in one of our
 54 |   dependencies (cfg-if)
 55 | - Update crate to Rust 2018 edition (thanks [@tymcauley]!)
 56 | 
 57 | 0.5.8 (2019-03-25)
 58 | ==================
 59 | 
 60 | - Change `syslog` feature to no longer re-export anything on Windows.
 61 |   Previously, using `syslog` on windows would simply fail to compile.
 62 |   (thanks [@17dec]!)
 63 | - Fix `log_enabled!` macro only checking dispatch at surface and not
 64 |   at sub-levels. Actually logging still only does a shallow check, but
 65 |   now `log_enabled!()` should be actually accurate.
 66 | 
 67 | 0.5.7 (2018-11-11)
 68 | ==================
 69 | 
 70 | - Fix colored log level display to honor formatting flags such as "{:>5}"
 71 |   (thanks [@ExpHP]!)
 72 | 
 73 | 0.5.6 (2018-06-19)
 74 | ==================
 75 | 
 76 | - Add another fuller example for colored logging (thanks [@digitalatigid]!)
 77 | - Add support for syslog version 4.0.0 under feature flag `syslog-4`.
 78 |   - Does not remove syslog-3 support
 79 |   - Includes support for RFC5424 formatting, but requires manually
 80 |     transforming the log record into the key/value pairs syslog
 81 |     expects.
 82 | - Add shorthand for calling an arbitrary function as a logging backend
 83 | 
 84 | 0.5.5 (2018-03-25)
 85 | ==================
 86 | 
 87 | - Add a log handler for logging into an arbitrary `Write` object. (thanks [@vorner]!)
 88 | 
 89 | 0.5.4 (2018-02-17)
 90 | ==================
 91 | 
 92 | - Add a log handler which panics on all messages. This can be used in
 93 |   test configurations to turn warning or error messages into hard
 94 |   errors.
 95 | - meta: add test coverage reporting via tarpaulin and coveralls
 96 | 
 97 | 0.5.3 (2018-02-04)
 98 | ==================
 99 | 
100 | - Add support for `Display::fmt` implementations which call the global
101 |   logger via a 'meta-logging-in-format' flag. (thanks [@jakunar]!)
102 |   - This is disabled by default, see 'meta' module for more info.
103 | 
104 | 0.5.2 (2018-01-02)
105 | ==================
106 | 
107 | - Re-add compatibility for rust versions 1.16.0+, and add CI testing
108 |   with rustc 1.16.0 to ensure this is kept in the future.
109 | - Add some more general documentation updates and clarity increases.
110 | - Add a CHANGELOG.md which mirrors git tag releases.
111 | - Update documentation links to point to docs.rs rather than custom
112 |   hosted documentation.
113 | - Fix ColoredLevelConfig::default being an inherent method rather than
114 |   an implementation of the Default trait.
115 | - Add direct support for the syslog crate under the "syslog-3"
116 |   feature flag.
117 |   - Add a module worth of documentation for using fern with syslog.
118 | 
119 | 0.5.1 (2017-12-26)
120 | ==================
121 | 
122 | - Re-add support for colored log levels with the 'colored' feature
123 |   - Support for this was accidentally dropped in 0.5.0.
124 | - Fix the ability to run tests on windows, and refactor integration
125 |   tests for developer clarity
126 | - Update documentation for clarity
127 | 
128 | Short list of changes in 0.5.0:
129 | - Updated from log 0.3 to log 0.4. Both are interoperable, but using
130 |   log 0.4 provides a much cleaner log interface for fern to use
131 |   internally
132 | - Removed fern::FernLog.
133 | - Renamed fern::color::ColoredLogLevelConfig to ColoredLevelConfig
134 | - Greatly simplified testing
135 | 
136 | 0.5.0 (2017-12-25)
137 | ==================
138 | 
139 | - Update from log 0.3 to log 0.4. Both log versions are interoperable,
140 |   but line numbers from libraries using 0.4 won't show up in binaries
141 |   with recievers using log 0.4.
142 |   - To clarify: both fern 0.4 and 0.5 will work perfectly with all
143 |     libraries, but line numbers will always be 0 if you use fern 0.4
144 |      and log 0.4.
145 | - Remove fern::FernLog. With log 0.4, log records can be constructed
146 |   directly, and fern loggers can now recieve just a record rather than
147 |   a record and the formatted display string.
148 | - Notable changes in the log crate: log::LogLevel is renamed to
149 |   log::Level, and log::LogLevelFilter is renamed to log::LevelFilter.
150 | - fern::color::ColoredLogLevelConfig has been renamed to
151 |   fern::color::ColoredLevelConfig to match log crate renamings.
152 | - fern tests have been greatly simplified with the new support for
153 |   creating log records manually. it's now possible to just run
154 |   "cargo test" and test all of fern's functionality.
155 | 
156 | 0.4.4 (2017-12-22)
157 | ==================
158 | 
159 | - Add support for coloring log levels in Unix terminals using the
160 |   'colored' crate (thanks [@nihiluis]!)
161 |   - This is enabled via the 'colored' feature, and adds a fern::color
162 |     module.
163 | 
164 | 0.4.3 (2017-09-20)
165 | ==================
166 | 
167 | - Add support for sending to an std::sync::mpsc::Sender as a log output
168 |   (thanks [@gingerDevilish]!)
169 | 
170 | 0.4.2 (2017-08-20)
171 | ==================
172 | 
173 | - Documentation hotfix after a premature release of version 0.4.1
174 | 
175 | 0.4.1 (2017-08-20)
176 | ==================
177 | 
178 | - Lots of documentation tweaks and reworking
179 | - Add CONTRIBUTING file and update README to invite new contributors
180 | - Improve example application to be more realistic
181 | - A few small internal improvements, mostly code cleanup here
182 | 
183 | 0.4.0 (2017-05-09)
184 | ==================
185 | 
186 | - Rework API surface to be builder-based for simpler configuration
187 |   - Rename DispatchConfig to Dispatch, OutputConfig to Output and
188 |     FernLogger to FernLog
189 | 
190 | - Rework inner log structure for more efficiency
191 |   - Different outputs are now stored in an `enum` rather than every
192 |     sublogger being a Box<FernLog> with dynamic dispatch
193 |   - Remove LogError; handle errors within individual loggers now - and
194 |     only within loggers which actually need it
195 |   - Remove unnecessary wrapping of streams with an Arc (now just uses
196 |     Mutex for File streams)
197 |   - Remove unnecessary wrapping of Stdout and Stderr streams with a
198 |     Mutex, when they are already synchronized
199 |   - Pass around just &fmt::Arguments + &log::LogRecord instead of
200 |     passing each individual LogRecord part
201 | 
202 | - Move opening of files and stdout/stderr from configuration
203 |   "building" to configuring
204 |   - Instead of taking OpenOptions, log configuration now just takes an
205 |     already-opened std::io::File object
206 |   - fern::InitError is now a convenience API, and is never returned
207 |     from any fern APIs
208 | 
209 | - Redo formatting to work without allocation - formatting closures now
210 |   finish with a callback rather than returning a value
211 | - Update examples to use `chrono` instead of the `time` crate
212 |   - This removes another extra allocation - chrono can format time
213 |     directly to a writer, without allocating intermediate the result
214 |     to a String
215 | 
216 | - Add much more documentation: almost every builder method has a full
217 |   example, and all features should be thoroughly explained
218 | - Add appveyor and travis-ci badges to README and Cargo.toml
219 | 
220 | 0.3.5 (2015-05-06)
221 | ==================
222 | 
223 | - Build changes to .travis.yml
224 | - Add html_root_url doc attribute
225 | - Add file_with_line_sep and file_with_options_and_line_sep
226 |   configuration construction options to allow specifying a line
227 |   separator other than the default '\n'
228 | 
229 | 0.3.4 (2015-04-16)
230 | ==================
231 | 
232 | - Update for rustc version e9080ec39 (1.0.0-beta.2)
233 |   - Update to use no_test to ignore doc tests, rather than ignore
234 |   - Remove all stability attributes on public types
235 |   - Add rust version matrix for testing on travis, to test on beta as
236 |     well as nightly builds
237 | 
238 | 0.3.3 (2015-04-03)
239 | ==================
240 | 
241 | - Update for rustc version 9854143cb (1.0.0-beta)
242 |   - Derive Clone for all types deriving Copy
243 |   - Update docs a bit for that switch to `time` crate
244 | - Switch to time crate instead of chrono for tests, as chrono hasn't
245 |   updated for rustc 1.0.0-beta yet.
246 | - Instead of implementing a sudo-time crate as a workaround for
247 |   https://github.com/rust-lang/cargo/issues/1474, just disable the doc
248 |   test, and copy the code to a separate file in tests/
249 | 
250 | 0.3.2 (2015-04-03)
251 | ==================
252 | 
253 | - Update to rustc version 2e3b0c051
254 |   - Add a workaround for
255 |     https://github.com/rust-lang/cargo/issues/1474 in doc tests
256 |   - Implement From for errors instead of FromError
257 |   - Remove now unrequired feature gate
258 | - Implement error::Error for error types
259 | 
260 | 0.3.1 (2015-03-26)
261 | ==================
262 | 
263 | - Updates to rustc version 27901849e
264 | 
265 | 0.3.0 (2015-03-25)
266 | ==================
267 | 
268 | - Updates to rustc version 123a754cb
269 | - Updates to log version 0.3.0
270 | - Reworks fern::OutputConfig to be a struct with functions to
271 |   construct configurations, rather than an enum with variants for each
272 |   configuration.
273 |   - This is a breaking change, as all constructors on
274 |     fern::OutputConfig have been renamed from UpperCase to lower_case.
275 |   - This also now allows fern::OutputConfig to be constructed with
276 |     anything which implements `AsRef<path::Path>`.
277 |     - For example, `fern::OutputConfig::file("some-file.log")` works,
278 |       without having to construct a Path or PathBuf manually.
279 | 
280 | 0.2.1 (2015-03-19)
281 | ==================
282 | 
283 | - Updates to rustc version 3e4be02b8
284 | - Updates documentation
285 | 
286 | 0.2.0 (2015-03-08)
287 | ==================
288 | 
289 | This version reworks the public API in order to turn fern into a
290 | backend to the `log` crate.
291 | 
292 | API Changes:
293 | - Remove the `local` module, as the `log` crate now handles storing a
294 |   global logger.
295 | - fern::Logger *must* now be Sync + Send
296 | - BoxedLogger and ArcLogger typedefs are removed, due to writing `+
297 |   Sync + Send` no longer being required
298 |  - Now everything just uses Box<Logger>
299 | - Level is removed, in favor of using log::LogLevel and
300 |   log::LogLevelFilter
301 | - LoggerConfig is renamed into DispatchConfig
302 | - Rename `Error` to `LogError`
303 |   - Implement `fmt::Display` for `LogError`
304 | - A new `Formatter` type is added for formatting closures. It also now
305 |   takes a &log::LogLocation parameters as well.
306 | - OutputConfig::Parent is renamed into OutputConfig::Child, this seems
307 |   to make more sense, given that you can have any number of children
308 | - Logger::log() now takes (&str, &log::LogLevel, &log::LogLocation)
309 |   instead of (&fern::Level, &str)
310 | - Add an `IntoLog` trait which DispatchConfig and OutputConfig
311 |   implement (instead of having `into_logger()` on each of them.
312 |   - Add an `into_log()` method to the IntoLog trait that turns a log
313 |     configuration into a `log::Log` (as apposed to `fern::Logger`)
314 |   - Rename `IntoLog.into_logger()` to `IntoLog.into_fern_logger()` in
315 |     order to differentiate from the `into_log()` method.
316 | - Add a `fern::init_global_logger()` method which sets the global
317 |   `log` crate logger from a log configuration
318 | - Add an `InitError` error which is used by `init_global_logger()` for
319 |   either an IO error or `log::SetLoggerError`
320 | - Update everything to use the new io and path modules
321 | - Add a `FileOptions` option to `OutputConfig` which allows for
322 |   specifying an `OpenOptions` for opening the log file with
323 | 
324 | Additional Changes:
325 | - The docs have been rewritten to be up to date with all the above
326 |   changes
327 |   - The code snippets in the docs are now all tested! This is instead
328 |     of having `no_test` and not having fully workable code examples.
329 | - There is a new `tests/lib.rs` file which contains tests for
330 |   initializing fern and log filtering with different log levels.
331 | 
332 | 0.1.12 (2015-02-21)
333 | ===================
334 | 
335 | - Fixes compile warnings and errors for rustc version 522d09dfe
336 |   (thanks [@gareins]!)
337 |  - Adds static life bound
338 |  - Switches to using old_path feature instead of path feature
339 | 
340 | 0.1.11 (2015-02-13)
341 | ===================
342 | 
343 | - Fixes small documentation error
344 | - Fixes compile errors in rustc version 3ef8ff1f8
345 | 
346 | 0.1.10 (2015-02-03)
347 | ===================
348 | 
349 | - Finishes updating to rustc version eaf4c5c78
350 |   - Last version compiled, but had many warnings
351 |   - Move all #[experimental] features to #[unstable]
352 |   - Add #![feature(io, core)]
353 |   - Remove unrequired .iter() call
354 | 
355 | 0.1.9 (2015-02-03)
356 | ==================
357 | 
358 | - Updates to rustc version eaf4c5c78
359 |   - Changes all usages of std::io to std::old_io
360 | 
361 | 0.1.8 (2015-01-27)
362 | ==================
363 | 
364 | - Updates to rustc version 458a6a2f6
365 | 
366 | 0.1.7 (2015-01-09)
367 | ==================
368 | 
369 | - Update to latest rustc (44a287e6e)
370 | 
371 | 0.1.6 (2015-01-07)
372 | ==================
373 | 
374 | This update mainly just cleans stuff up and updates for the latest
375 | rustc (ea6f65c5f)
376 | 
377 | - Update to using f.write_str(... instead of write!(f, "{}", ...) for
378 |   simplicity
379 | - Update to use (closure)(...) instead of closure.call((...)) because
380 |   directly calling works now
381 | - Remove #![feature()] attributes for unboxed_closures and
382 |   old_orphan_check, as they are no longer required.
383 | 
384 | 0.1.5 (2015-01-05)
385 | ==================
386 | 
387 | - Updates for the latest rustc version, ad9e75938.
388 | - Fixes all lines which go past the 99 character line limit.
389 | 
390 | 
391 | 0.1.4 (2015-01-01)
392 | ==================
393 | 
394 | This version is *not* backwards compatible. The change was necessary
395 | for the latest rust update however, so only a minor version increment
396 | was added.
397 | 
398 | - Changes from using IoResult<()> to Result<(), fern::Error> for
399 |   return types from logging operations.
400 | - Updates for latest rustc
401 | 
402 | 0.1.3 (2014-12-27)
403 | ==================
404 | 
405 | - Adds a new public module, local, which stores a thread-local logger.
406 | - Adds a new logger 'NullLogger', which does nothing with logged
407 |   mesages.
408 | - Fixes WriterLogger to append to opened files instead of overwriting.
409 | - Adds a ton more documentation
410 | 
411 | 0.1.2 (2014-12-24)
412 | ==================
413 | 
414 | - Adds type aliases BoxedLogger and ArcLogger, which resolve to
415 |   `Box<Logger + Sync + Send>` and
416 |   `sync::Arc<Box<Logger + Sync + Send>>` respectively.
417 | 
418 | 0.1.1 (2014-12-22)
419 | ==================
420 | 
421 | - Adds a workaround for a bug introduced a compiler update.
422 | 
423 | 0.1.0 (2014-12-19)
424 | ==================
425 | 
426 | First release, version 0.1.0.
427 | 
428 | 
429 | [reopen]: https://github.com/vorner/reopen
430 | [@gareins]: https://github.com/gareins
431 | [@gingerDevilish]: https://github.com/gingerDevilish
432 | [@nihiluis]: https://github.com/nihiluis
433 | [@jakunar]: https://github.com/jakunar
434 | [@vorner]: https://github.com/vorner
435 | [@digitalatigid]: https://github.com/digitalatigid
436 | [@ExpHP]: https://github.com/ExpHP
437 | [@17dec]: https://github.com/17dec
438 | [@tanujitghosh]: https://github.com/tanujitghosh
439 | [@itkovian]: https://github.com/itkovian
440 | [@tymcauley]: https://github.com/tymcauley
441 | [@Songtronix]: https://github.com/Songtronix
442 | [@dekellum]: https://github.com/dekellum
443 | [@Palladinium]: https://github.com/Palladinium
444 | [@sourcefrog]: https://github.com/sourcefrog
445 | [@autarch]: https://github.com/autarch
446 | [@vorner]: https://github.com/vorner
447 | [@EasyPost]: https://github.com/EasyPost
448 | [@faern]: https://github.com/faern
449 | [@BlackDex]: https://github.com/BlackDex
450 | 


--------------------------------------------------------------------------------
/CONTRIBUTING.md:
--------------------------------------------------------------------------------
 1 | # Contributing
 2 | 
 3 | 
 4 | ## Overview (mirrored in README)
 5 | 
 6 | There's one thing I need right now, more than anything else: input on what fern does well, and what it should keep
 7 | doing well. See [Project Direction](#project-direction).
 8 | 
 9 | Besides that, I'm open to PRs! I'll probably review promptly, and I'm always open to being nudged if I don't.
10 | 
11 | For small PRs, I'll mark anything I need changed in a review, and work with you on that.
12 | 
13 | For larger PRs, I reserve the right to pull in your commits as they are, then fix things I want to be different myself.
14 | In a workplace, I'd try to never do this - but this is a hobby project for me, and I'd rather be overly particular about
15 | fern's implementation than be reasonable.
16 | 
17 | This is a change from my previous policy.
18 | 
19 | ## Code of Conduct.
20 | 
21 | All interactions are expected to follow [the Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html).
22 | 
23 | ## `fern` project structure
24 | 
25 | Fern attempts to be an idiomatic rust library and to maintain a sane structure. All source code is located in `src/`, and tests are in `tests/`.
26 | 
27 | The source is split into four modules:
28 | - `lib.rs` contains top-level traits, module documentation, and helper functions
29 | - `builders.rs` contains all the configuration code
30 | - `errors.rs` contains error handling for finishing configuration
31 | - and `log_impl.rs` contains the implementation for `log::Log` which is created to run for the actual logging.
32 | 
33 | Hopefully these modules are fairly separated, and it's clear when you'll need to work on multiple sections. Adding a new log implementation, for instance, will need to touch `builders.rs` for configuration, and `log_impl.rs` for the implementation - both pieces of code will connect via `builders::Dispatch::into_dispatch`, but besides that, things should be fairly separate.
34 | 
35 | ## Pull requests
36 | 
37 | Pull requests are _the_ way to change code using git. If you aren't familiar with them in general, GitHub has some [excellent documentation](https://help.github.com/articles/about-pull-requests/).
38 | 
39 | There aren't many hard guidelines in this repository on how specifically to format your request. Main points:
40 | 
41 | - Please include a descriptive title for your pull request, and elaborate on what's changed in the description.
42 | - Feel free to open a PR before the feature is completely ready, and commit directly to the PR branch.
43 | - Please include at least a short description in each commit, and more of one in the "main" feature commit. Doesn't
44 |   have to be much, but someone reading the history should easily tell what's different now from before.
45 | - Use `cargo fmt` to format your code.
46 | 
47 | ## Testing
48 | 
49 | To run build everything and run all tests, use:
50 | 
51 | ```sh
52 | cargo build --all-features --all-targets
53 | cargo test --all-features
54 | ```
55 | 
56 | ## Mentoring
57 | 
58 | Contributing to a project can be daunting.
59 | 
60 | Email me at daboross @ daboross.net with any questions!
61 | 


--------------------------------------------------------------------------------
/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "fern"
 3 | # Remember to update html_root_url in src/lib.rs with each version.
 4 | version = "0.7.1"
 5 | authors = ["David Ross <daboross@daboross.net>"]
 6 | description = "Simple, efficient logging"
 7 | edition = "2021"
 8 | # when updating this, also update toolchain in .github/workflows/rust.yml
 9 | rust-version = "1.60"
10 | 
11 | documentation = "https://docs.rs/fern/"
12 | repository = "https://github.com/daboross/fern"
13 | readme = "README.md"
14 | 
15 | license = "MIT"
16 | keywords = ["log", "logging", "logger"]
17 | categories = ["development-tools::debugging"]
18 | 
19 | include = ["Cargo.toml", "src/**/*", "tests/**/*", "examples/**/*", "LICENSE", "README.md", "CONTRIBUTING.md", "CHANGELOG.md"]
20 | 
21 | [dependencies]
22 | log = { version = "0.4", features = ["std"] }
23 | colored = { version = "2.1.0", optional = true }
24 | chrono = { version = "0.4", default-features = false, features = ["std", "clock"], optional = true }
25 | 
26 | [target."cfg(not(windows))".dependencies]
27 | syslog3 = { version = "3", package = "syslog", optional = true }
28 | syslog4 = { version = "4", package = "syslog", optional = true }
29 | syslog6 = { version = "6", package = "syslog", optional = true }
30 | syslog7 = { version = "7", package = "syslog", optional = true }
31 | reopen1 = { version = "~1", package = "reopen", features = ["signals"], optional = true }
32 | reopen03 = { version = "^0.3", package = "reopen", optional = true }
33 | libc = { version = "0.2.58", optional = true }
34 | 
35 | [features]
36 | syslog-3 = ["syslog3"]
37 | syslog-4 = ["syslog4"]
38 | syslog-6 = ["syslog6"]
39 | syslog-7 = ["syslog7"]
40 | reopen-03 = ["reopen03", "libc"]
41 | reopen-1 = ["reopen1", "libc"]
42 | meta-logging-in-format = []
43 | date-based = ["chrono"]
44 | 
45 | [dev-dependencies]
46 | tempfile = "3"
47 | clap = "2.22"
48 | humantime = "2.1.0"
49 | 
50 | [[example]]
51 | name = "cmd-program"
52 | 
53 | [[example]]
54 | name = "date-based-file-log"
55 | required-features = ["date-based"]
56 | 
57 | [[example]]
58 | name = "colored"
59 | required-features = ["colored"]
60 | 
61 | [[example]]
62 | name = "pretty-colored"
63 | required-features = ["colored"]
64 | 
65 | [[example]]
66 | name = "syslog7"
67 | required-features = ["syslog-7"]
68 | 
69 | [[example]]
70 | name = "syslog"
71 | required-features = ["syslog-6"]
72 | 
73 | [[example]]
74 | name = "syslog4"
75 | required-features = ["syslog-4"]
76 | 
77 | [[example]]
78 | name = "syslog3"
79 | required-features = ["syslog-3"]
80 | 
81 | [[example]]
82 | name = "meta-logging"
83 | 
84 | [package.metadata.docs.rs]
85 | all-features = true
86 | 


--------------------------------------------------------------------------------
/LICENSE:
--------------------------------------------------------------------------------
1 | Copyright (c) 2014-2017 David Ross
2 | 
3 | Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
4 | 
5 | The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
6 | 
7 | THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
8 | 


--------------------------------------------------------------------------------
/README.md:
--------------------------------------------------------------------------------
 1 | fern
 2 | ====
 3 | [![crates.io version badge][cratesio-badge]][fern-crate]
 4 | [![Build Status][test-status-badge]][test-status-link]
 5 | 
 6 | - [documentation][fern-docs]
 7 | - [crates.io page][fern-crate]
 8 | - [example program][fern-example]
 9 | 
10 | Simple, efficient logging for [Rust].
11 | 
12 | Logging configuration is recursively branched: formatting, filters, and output can be applied at each
13 | `fern::Dispatch`, applying to increasingly specific kinds of logging.
14 | 
15 | ```rust
16 | // Configure logger at runtime
17 | fern::Dispatch::new()
18 |     // Perform allocation-free log formatting
19 |     .format(|out, message, record| {
20 |         out.finish(format_args!(
21 |             "[{} {} {}] {}",
22 |             humantime::format_rfc3339(std::time::SystemTime::now()),
23 |             record.level(),
24 |             record.target(),
25 |             message
26 |         ))
27 |     })
28 |     // Add blanket level filter -
29 |     .level(log::LevelFilter::Debug)
30 |     // - and per-module overrides
31 |     .level_for("hyper", log::LevelFilter::Info)
32 |     // Output to stdout, files, and other Dispatch configurations
33 |     .chain(std::io::stdout())
34 |     .chain(fern::log_file("output.log")?)
35 |     // Apply globally
36 |     .apply()?;
37 | 
38 | // and log using log crate macros!
39 | log::info!("hello, world!");
40 | ```
41 | 
42 | Examples of all features at the [api docs][fern-docs]. See fern in use with this [example command line program][fern-example].
43 | 
44 | ## Project Direction
45 | 
46 | I've posted a GitHub Discussion talking about the future of fern: https://github.com/daboross/fern/discussions/147
47 | 
48 | If you've ever used fern, or you do today, I'd love input!
49 | 
50 | ## fern 0.4.4, 0.5.\*, 0.6.\* security warning - `colored` crate + custom global allocator
51 | 
52 | One of our downstream dependencies, [atty](https://docs.rs/atty/), through
53 | [colored](https://docs.rs/colored/), has an unsoundness issue:
54 | <https://rustsec.org/advisories/RUSTSEC-2021-0145.html>.
55 | 
56 | This shows up in one situation: if you're using `colored` 0.1.0 and a custom global allocator.
57 | 
58 | Upgrade to `fern` 0.7.0 to fix.
59 | 
60 | ### Contributing
61 | 
62 | There's one thing I need right now, more than anything else: input on what fern does well, and what it should keep
63 | doing well. See [Project Direction](#project-direction).
64 | 
65 | Besides that, I'm open to PRs! I'll probably review promptly, and I'm always open to being nudged if I don't.
66 | 
67 | For small PRs, I'll mark anything I need changed in a review, and work with you on that.
68 | 
69 | For larger PRs, I reserve the right to pull in your commits as they are, then fix things I want to be different myself.
70 | In a workplace, I'd try to never do this - but this is a hobby project for me, and I'd rather be overly particular about
71 | fern's implementation than be reasonable.
72 | 
73 | This is a change from my previous policy.
74 | 
75 | See [CONTRIBUTING](./CONTRIBUTING.md) for technical information on contributing.
76 | 
77 | [Rust]: https://www.rust-lang.org/
78 | [test-status-badge]: https://github.com/daboross/fern/workflows/tests/badge.svg?branch=main&event=push
79 | [test-status-link]: https://github.com/daboross/fern/actions/workflows/rust.yml
80 | [issue-resolution-badge]: http://isitmaintained.com/badge/resolution/daboross/fern.svg
81 | [isitmaintained-link]: http://isitmaintained.com/project/daboross/fern
82 | [cratesio-badge]: https://img.shields.io/crates/v/fern.svg
83 | [fern-docs]: https://docs.rs/fern/
84 | [fern-crate]: https://crates.io/crates/fern
85 | [fern-example]: https://github.com/daboross/fern/tree/main/examples/cmd-program.rs
86 | [log]: https://github.com/rust-lang/log
87 | 


--------------------------------------------------------------------------------
/examples/cmd-program.rs:
--------------------------------------------------------------------------------
  1 | use std::{io, time::SystemTime};
  2 | 
  3 | use log::{debug, info, trace, warn};
  4 | 
  5 | fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
  6 |     let mut base_config = fern::Dispatch::new();
  7 | 
  8 |     base_config = match verbosity {
  9 |         0 => {
 10 |             // Let's say we depend on something which whose "info" level messages are too
 11 |             // verbose to include in end-user output. If we don't need them,
 12 |             // let's not include them.
 13 |             base_config
 14 |                 .level(log::LevelFilter::Info)
 15 |                 .level_for("overly-verbose-target", log::LevelFilter::Warn)
 16 |         }
 17 |         1 => base_config
 18 |             .level(log::LevelFilter::Debug)
 19 |             .level_for("overly-verbose-target", log::LevelFilter::Info),
 20 |         2 => base_config.level(log::LevelFilter::Debug),
 21 |         _3_or_more => base_config.level(log::LevelFilter::Trace),
 22 |     };
 23 | 
 24 |     // Separate file config so we can include year, month and day in file logs
 25 |     let file_config = fern::Dispatch::new()
 26 |         .format(|out, message, record| {
 27 |             out.finish(format_args!(
 28 |                 "[{} {} {}] {}",
 29 |                 humantime::format_rfc3339_seconds(SystemTime::now()),
 30 |                 record.level(),
 31 |                 record.target(),
 32 |                 message
 33 |             ))
 34 |         })
 35 |         .chain(fern::log_file("program.log")?);
 36 | 
 37 |     let stdout_config = fern::Dispatch::new()
 38 |         .format(|out, message, record| {
 39 |             // special format for debug messages coming from our own crate.
 40 |             if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
 41 |                 out.finish(format_args!(
 42 |                     "DEBUG @ {}: {}",
 43 |                     humantime::format_rfc3339_seconds(SystemTime::now()),
 44 |                     message
 45 |                 ))
 46 |             } else {
 47 |                 out.finish(format_args!(
 48 |                     "[{} {} {}] {}",
 49 |                     humantime::format_rfc3339_seconds(SystemTime::now()),
 50 |                     record.level(),
 51 |                     record.target(),
 52 |                     message
 53 |                 ))
 54 |             }
 55 |         })
 56 |         .chain(io::stdout());
 57 | 
 58 |     base_config
 59 |         .chain(file_config)
 60 |         .chain(stdout_config)
 61 |         .apply()?;
 62 | 
 63 |     Ok(())
 64 | }
 65 | 
 66 | fn main() {
 67 |     let cmd_arguments = clap::App::new("cmd-program")
 68 |         .arg(
 69 |             clap::Arg::with_name("verbose")
 70 |                 .short("v")
 71 |                 .long("verbose")
 72 |                 .multiple(true)
 73 |                 .help("Increases logging verbosity each use for up to 3 times"),
 74 |         )
 75 |         .get_matches();
 76 | 
 77 |     let verbosity: u64 = cmd_arguments.occurrences_of("verbose");
 78 | 
 79 |     setup_logging(verbosity).expect("failed to initialize logging.");
 80 | 
 81 |     info!("MyProgram v0.0.1 starting up!");
 82 | 
 83 |     debug!("DEBUG output enabled.");
 84 |     trace!("TRACE output enabled.");
 85 | 
 86 |     // Emulate a library we're using which has tons of debugging on the 'info'
 87 |     // level.
 88 |     info!(target: "overly-verbose-target", "hey, another library here, we're starting.");
 89 | 
 90 |     for i in 0..5 {
 91 |         info!("executing section: {}", i);
 92 | 
 93 |         debug!("section {} 1/4 complete.", i);
 94 | 
 95 |         info!(target: "overly-verbose-target", "completed operation.");
 96 | 
 97 |         debug!("section {} 1/2 complete.", i);
 98 | 
 99 |         info!(target: "overly-verbose-target", "completed operation.");
100 | 
101 |         info!(target: "overly-verbose-target", "completed operation.");
102 | 
103 |         debug!("section {} 3/4 complete.", i);
104 | 
105 |         info!("section {} completed!", i);
106 |     }
107 | 
108 |     warn!(target: "overly-verbose-target", "AHHH something's on fire.");
109 | 
110 |     info!("MyProgram operation completed, shutting down.");
111 | }
112 | 


--------------------------------------------------------------------------------
/examples/colored.rs:
--------------------------------------------------------------------------------
 1 | use std::time::SystemTime;
 2 | 
 3 | use fern::colors::{Color, ColoredLevelConfig};
 4 | use log::{debug, error, warn};
 5 | 
 6 | fn main() {
 7 |     let colors = ColoredLevelConfig::new().debug(Color::Magenta);
 8 | 
 9 |     fern::Dispatch::new()
10 |         .chain(std::io::stdout())
11 |         .format(move |out, message, record| {
12 |             out.finish(format_args!(
13 |                 "[{} {} {}] {}",
14 |                 humantime::format_rfc3339_seconds(SystemTime::now()),
15 |                 // This will color the log level only, not the whole line. Just a touch.
16 |                 colors.color(record.level()),
17 |                 record.target(),
18 |                 message
19 |             ))
20 |         })
21 |         .apply()
22 |         .unwrap();
23 | 
24 |     error!("hi");
25 |     debug!("sup");
26 |     warn!("oh");
27 | }
28 | 


--------------------------------------------------------------------------------
/examples/date-based-file-log.rs:
--------------------------------------------------------------------------------
 1 | use log::{debug, info, warn};
 2 | 
 3 | fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
 4 |     fern::Dispatch::new()
 5 |         .level(log::LevelFilter::Debug)
 6 |         .chain(fern::DateBased::new("program.log.", "%Y-%m-%d"))
 7 |         .apply()?;
 8 | 
 9 |     Ok(())
10 | }
11 | 
12 | fn main() {
13 |     setup_logging().expect("failed to initialize logging.");
14 | 
15 |     for i in 0..5 {
16 |         info!("executing section: {}", i);
17 | 
18 |         debug!("section {} 1/4 complete.", i);
19 | 
20 |         debug!("section {} 1/2 complete.", i);
21 | 
22 |         debug!("section {} 3/4 complete.", i);
23 | 
24 |         info!("section {} completed!", i);
25 |     }
26 | 
27 |     warn!("AHHH something's on fire.");
28 | }
29 | 


--------------------------------------------------------------------------------
/examples/meta-logging.rs:
--------------------------------------------------------------------------------
 1 | //! This is an example to test the "meta-logging-in-format" fern cargo features.
 2 | //!
 3 | //! The example will hang if the feature is disabled, and will produce cohesive
 4 | //! logs if it's enabled.
 5 | use std::fmt;
 6 | 
 7 | use log::{debug, info};
 8 | 
 9 | fn main() {
10 |     fern::Dispatch::new()
11 |         .chain(std::io::stdout())
12 |         .chain(std::io::stderr())
13 |         .chain(fern::log_file("hello.txt").unwrap())
14 |         .format(move |out, message, record| {
15 |             out.finish(format_args!("[{}] {}", record.level(), message))
16 |         })
17 |         .apply()
18 |         .unwrap();
19 | 
20 |     // in order to actually trigger the situation that deadlocks, we need a custom
21 |     // Display implementation which performs logging:
22 |     struct Thing<'a>(&'a str);
23 | 
24 |     impl fmt::Display for Thing<'_> {
25 |         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
26 |             debug!("formatting Thing wrapping ({})", self.0);
27 |             f.write_str(self.0)
28 |         }
29 |     }
30 | 
31 |     info!("I'm logging {}!", Thing("aha"));
32 | }
33 | 


--------------------------------------------------------------------------------
/examples/pretty-colored-screenshot.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/daboross/fern/f24e11e687deb1a26f68dc3fe3662d1904496bb4/examples/pretty-colored-screenshot.png


--------------------------------------------------------------------------------
/examples/pretty-colored.rs:
--------------------------------------------------------------------------------
 1 | //! This example shows how to configure fern to output really nicely colored
 2 | //! logs
 3 | //! - when the log level is error, the whole line is red
 4 | //! - when the log level is warn, the whole line is yellow
 5 | //! - when the log level is info, the level name is green and the rest of the
 6 | //!   line is white
 7 | //! - when the log level is debug, the whole line is white
 8 | //! - when the log level is trace, the whole line is gray ("bright black")
 9 | use std::time::SystemTime;
10 | 
11 | use fern::colors::{Color, ColoredLevelConfig};
12 | use log::{debug, error, info, trace, warn};
13 | 
14 | fn main() {
15 |     set_up_logging();
16 |     // let's simulate some logging
17 |     info!("starting simulation!");
18 |     for i in 0..26 {
19 |         trace!("loading: {}%, very verbose debbuging information", 4 * i);
20 |         if 5 == i {
21 |             debug!("this is taking so long... boooring!");
22 |         } else if 10 == i {
23 |             debug!("still alive! yay!");
24 |         } else if 13 == i {
25 |             info!("halfway there!");
26 |         } else if 16 == i {
27 |             debug!("*scratches nose*");
28 |             warn!("nose is itching, continuing anyways");
29 |         } else if 20 == i {
30 |             debug!("uh oh");
31 |             warn!(">nose itching intensifies");
32 |             error!("HATCHOOO!");
33 |             debug!("encountered minor problem, trying to recover");
34 |             info!("gesundheit");
35 |             debug!("recovered from minor problem, continuing");
36 |         } else if 25 == i {
37 |             info!("successfully loaded nothing");
38 |             info!("have a good time!");
39 |         }
40 |     }
41 | }
42 | 
43 | // ===================== Logging Set Up =====================
44 | fn set_up_logging() {
45 |     // configure colors for the whole line
46 |     let colors_line = ColoredLevelConfig::new()
47 |         .error(Color::Red)
48 |         .warn(Color::Yellow)
49 |         // we actually don't need to specify the color for debug and info, they are white by default
50 |         .info(Color::White)
51 |         .debug(Color::White)
52 |         // depending on the terminals color scheme, this is the same as the background color
53 |         .trace(Color::BrightBlack);
54 | 
55 |     // configure colors for the name of the level.
56 |     // since almost all of them are the same as the color for the whole line, we
57 |     // just clone `colors_line` and overwrite our changes
58 |     let colors_level = colors_line.info(Color::Green);
59 |     // here we set up our fern Dispatch
60 |     fern::Dispatch::new()
61 |         .format(move |out, message, record| {
62 |             out.finish(format_args!(
63 |                 "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
64 |                 color_line = format_args!(
65 |                     "\x1B[{}m",
66 |                     colors_line.get_color(&record.level()).to_fg_str()
67 |                 ),
68 |                 date = humantime::format_rfc3339_seconds(SystemTime::now()),
69 |                 target = record.target(),
70 |                 level = colors_level.color(record.level()),
71 |                 message = message,
72 |             ));
73 |         })
74 |         // set the default log level. to filter out verbose log messages from dependencies, set
75 |         // this to Warn and overwrite the log level for your crate.
76 |         .level(log::LevelFilter::Warn)
77 |         // change log levels for individual modules. Note: This looks for the record's target
78 |         // field which defaults to the module path but can be overwritten with the `target`
79 |         // parameter:
80 |         // `info!(target="special_target", "This log message is about special_target");`
81 |         .level_for("pretty_colored", log::LevelFilter::Trace)
82 |         // output to stdout
83 |         .chain(std::io::stdout())
84 |         .apply()
85 |         .unwrap();
86 | 
87 |     debug!("finished setting up logging! yay!");
88 | }
89 | 


--------------------------------------------------------------------------------
/examples/syslog.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(not(windows))]
 2 | // This is necessary because `fern` depends on both version 3 and 4.
 3 | use syslog6 as syslog;
 4 | 
 5 | #[cfg(not(windows))]
 6 | use log::{debug, info, warn};
 7 | 
 8 | #[cfg(not(windows))]
 9 | fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
10 |     let syslog_fmt = syslog::Formatter3164 {
11 |         facility: syslog::Facility::LOG_USER,
12 |         hostname: None,
13 |         process: "fern-syslog-example".into(),
14 |         pid: 0,
15 |     };
16 |     fern::Dispatch::new()
17 |         // by default only accept warning messages so as not to spam
18 |         .level(log::LevelFilter::Warn)
19 |         // but accept Info if we explicitly mention it
20 |         .level_for("explicit-syslog", log::LevelFilter::Info)
21 |         .chain(syslog::unix(syslog_fmt)?)
22 |         .apply()?;
23 | 
24 |     Ok(())
25 | }
26 | 
27 | #[cfg(not(windows))]
28 | fn main() {
29 |     setup_logging().expect("failed to initialize logging.");
30 | 
31 |     // None of this will be shown in the syslog:
32 |     for i in 0..5 {
33 |         info!("executing section: {}", i);
34 | 
35 |         debug!("section {} 1/4 complete.", i);
36 | 
37 |         debug!("section {} 1/2 complete.", i);
38 | 
39 |         debug!("section {} 3/4 complete.", i);
40 | 
41 |         info!("section {} completed!", i);
42 |     }
43 | 
44 |     // these two *will* show.
45 | 
46 |     info!(target: "explicit-syslog", "hello to the syslog! this is rust.");
47 | 
48 |     warn!("AHHH something's on fire.");
49 | }
50 | 
51 | #[cfg(windows)]
52 | fn main() {
53 |     panic!("this example does not work on Windows.");
54 | }
55 | 


--------------------------------------------------------------------------------
/examples/syslog3.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(not(windows))]
 2 | // This is necessary because `fern` depends on both version 3 and 4.
 3 | use syslog3 as syslog;
 4 | 
 5 | #[cfg(not(windows))]
 6 | use log::{debug, info, warn};
 7 | 
 8 | #[cfg(not(windows))]
 9 | fn setup_logging() -> Result<(), fern::InitError> {
10 |     fern::Dispatch::new()
11 |         // by default only accept warning messages so as not to spam
12 |         .level(log::LevelFilter::Warn)
13 |         // but accept Info if we explicitly mention it
14 |         .level_for("explicit-syslog", log::LevelFilter::Info)
15 |         .chain(syslog::unix(syslog::Facility::LOG_USER)?)
16 |         .apply()?;
17 | 
18 |     Ok(())
19 | }
20 | 
21 | #[cfg(not(windows))]
22 | fn main() {
23 |     setup_logging().expect("failed to initialize logging.");
24 | 
25 |     // None of this will be shown in the syslog:
26 |     for i in 0..5 {
27 |         info!("executing section: {}", i);
28 | 
29 |         debug!("section {} 1/4 complete.", i);
30 | 
31 |         debug!("section {} 1/2 complete.", i);
32 | 
33 |         debug!("section {} 3/4 complete.", i);
34 | 
35 |         info!("section {} completed!", i);
36 |     }
37 | 
38 |     // these two *will* show.
39 | 
40 |     info!(target: "explicit-syslog", "hello to the syslog! this is rust.");
41 | 
42 |     warn!("AHHH something's on fire.");
43 | }
44 | 
45 | #[cfg(windows)]
46 | fn main() {
47 |     panic!("this example does not work on Windows.");
48 | }
49 | 


--------------------------------------------------------------------------------
/examples/syslog4.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(not(windows))]
 2 | // This is necessary because `fern` depends on both version 3 and 4.
 3 | use syslog4 as syslog;
 4 | 
 5 | #[cfg(not(windows))]
 6 | use log::{debug, info, warn};
 7 | 
 8 | #[cfg(not(windows))]
 9 | fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
10 |     let syslog_fmt = syslog::Formatter3164 {
11 |         facility: syslog::Facility::LOG_USER,
12 |         hostname: None,
13 |         process: "fern-syslog-example".into(),
14 |         pid: 0,
15 |     };
16 |     fern::Dispatch::new()
17 |         // by default only accept warning messages so as not to spam
18 |         .level(log::LevelFilter::Warn)
19 |         // but accept Info if we explicitly mention it
20 |         .level_for("explicit-syslog", log::LevelFilter::Info)
21 |         .chain(syslog::unix(syslog_fmt)?)
22 |         .apply()?;
23 | 
24 |     Ok(())
25 | }
26 | 
27 | #[cfg(not(windows))]
28 | fn main() {
29 |     setup_logging().expect("failed to initialize logging.");
30 | 
31 |     // None of this will be shown in the syslog:
32 |     for i in 0..5 {
33 |         info!("executing section: {}", i);
34 | 
35 |         debug!("section {} 1/4 complete.", i);
36 | 
37 |         debug!("section {} 1/2 complete.", i);
38 | 
39 |         debug!("section {} 3/4 complete.", i);
40 | 
41 |         info!("section {} completed!", i);
42 |     }
43 | 
44 |     // these two *will* show.
45 | 
46 |     info!(target: "explicit-syslog", "hello to the syslog! this is rust.");
47 | 
48 |     warn!("AHHH something's on fire.");
49 | }
50 | 
51 | #[cfg(windows)]
52 | fn main() {
53 |     panic!("this example does not work on Windows.");
54 | }
55 | 


--------------------------------------------------------------------------------
/examples/syslog7.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(not(windows))]
 2 | // This is necessary because `fern` depends on both version 3, 4 and 6
 3 | use syslog7 as syslog;
 4 | 
 5 | #[cfg(not(windows))]
 6 | use log::{debug, info, warn};
 7 | 
 8 | #[cfg(not(windows))]
 9 | fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
10 |     let syslog_fmt = syslog::Formatter3164 {
11 |         facility: syslog::Facility::LOG_USER,
12 |         hostname: None,
13 |         process: "fern-syslog-example".into(),
14 |         pid: 0,
15 |     };
16 |     fern::Dispatch::new()
17 |         // by default only accept warning messages so as not to spam
18 |         .level(log::LevelFilter::Warn)
19 |         // but accept Info if we explicitly mention it
20 |         .level_for("explicit-syslog", log::LevelFilter::Info)
21 |         .chain(syslog::unix(syslog_fmt)?)
22 |         .apply()?;
23 | 
24 |     Ok(())
25 | }
26 | 
27 | #[cfg(not(windows))]
28 | fn main() {
29 |     setup_logging().expect("failed to initialize logging.");
30 | 
31 |     // None of this will be shown in the syslog:
32 |     for i in 0..5 {
33 |         info!("executing section: {}", i);
34 | 
35 |         debug!("section {} 1/4 complete.", i);
36 | 
37 |         debug!("section {} 1/2 complete.", i);
38 | 
39 |         debug!("section {} 3/4 complete.", i);
40 | 
41 |         info!("section {} completed!", i);
42 |     }
43 | 
44 |     // these two *will* show.
45 | 
46 |     info!(target: "explicit-syslog", "hello to the syslog! this is rust.");
47 | 
48 |     warn!("AHHH something's on fire.");
49 | }
50 | 
51 | #[cfg(windows)]
52 | fn main() {
53 |     panic!("this example does not work on Windows.");
54 | }
55 | 


--------------------------------------------------------------------------------
/src/builders.rs:
--------------------------------------------------------------------------------
   1 | use std::{
   2 |     borrow::Cow,
   3 |     cmp, fmt, fs, io,
   4 |     io::Write,
   5 |     sync::{mpsc::Sender, Arc, Mutex},
   6 | };
   7 | 
   8 | #[cfg(feature = "date-based")]
   9 | use std::path::{Path, PathBuf};
  10 | 
  11 | #[cfg(all(not(windows), any(feature = "syslog-4", feature = "syslog-6")))]
  12 | use std::collections::HashMap;
  13 | 
  14 | #[cfg(all(not(windows), feature = "syslog-7"))]
  15 | use std::collections::BTreeMap;
  16 | 
  17 | use log::Log;
  18 | 
  19 | use crate::{log_impl, Filter, FormatCallback, Formatter};
  20 | 
  21 | #[cfg(feature = "date-based")]
  22 | use crate::log_impl::DateBasedState;
  23 | 
  24 | #[cfg(all(not(windows), feature = "syslog-4"))]
  25 | use crate::{Syslog4Rfc3164Logger, Syslog4Rfc5424Logger, Syslog4TransformFn};
  26 | 
  27 | #[cfg(all(not(windows), feature = "syslog-6"))]
  28 | use crate::{Syslog6Rfc3164Logger, Syslog6Rfc5424Logger, Syslog6TransformFn};
  29 | 
  30 | #[cfg(all(not(windows), feature = "syslog-7"))]
  31 | use crate::{Syslog7Rfc3164Logger, Syslog7Rfc5424Logger, Syslog7TransformFn};
  32 | 
  33 | /// The base dispatch logger.
  34 | ///
  35 | /// This allows for formatting log records, limiting what records can be passed
  36 | /// through, and then dispatching records to other dispatch loggers or output
  37 | /// loggers.
  38 | ///
  39 | /// Note that all methods are position-insensitive.
  40 | /// `Dispatch::new().format(a).chain(b)` produces the exact same result
  41 | /// as `Dispatch::new().chain(b).format(a)`. Given this, it is preferred to put
  42 | /// 'format' and other modifiers before 'chain' for the sake of clarity.
  43 | ///
  44 | /// Example usage demonstrating all features:
  45 | ///
  46 | /// ```no_run
  47 | /// # // no_run because this creates log files.
  48 | /// use std::{fs, io};
  49 | ///
  50 | /// # fn setup_logger() -> Result<(), fern::InitError> {
  51 | /// fern::Dispatch::new()
  52 | ///     .format(|out, message, record| {
  53 | ///         out.finish(format_args!(
  54 | ///             "[{} {}] {}",
  55 | ///             record.level(),
  56 | ///             record.target(),
  57 | ///             message,
  58 | ///         ))
  59 | ///     })
  60 | ///     .chain(
  61 | ///         fern::Dispatch::new()
  62 | ///             // by default only accept warn messages
  63 | ///             .level(log::LevelFilter::Warn)
  64 | ///             // accept info messages from the current crate too
  65 | ///             .level_for("my_crate", log::LevelFilter::Info)
  66 | ///             // `io::Stdout`, `io::Stderr` and `io::File` can be directly passed in.
  67 | ///             .chain(io::stdout()),
  68 | ///     )
  69 | ///     .chain(
  70 | ///         fern::Dispatch::new()
  71 | ///             // output all messages
  72 | ///             .level(log::LevelFilter::Trace)
  73 | ///             // except for hyper, in that case only show info messages
  74 | ///             .level_for("hyper", log::LevelFilter::Info)
  75 | ///             // `log_file(x)` equates to
  76 | ///             // `OpenOptions::new().write(true).append(true).create(true).open(x)`
  77 | ///             .chain(fern::log_file("persistent-log.log")?)
  78 | ///             .chain(
  79 | ///                 fs::OpenOptions::new()
  80 | ///                     .write(true)
  81 | ///                     .create(true)
  82 | ///                     .truncate(true)
  83 | ///                     .create(true)
  84 | ///                     .open("/tmp/temp.log")?,
  85 | ///             ),
  86 | ///     )
  87 | ///     .chain(
  88 | ///         fern::Dispatch::new()
  89 | ///             .level(log::LevelFilter::Error)
  90 | ///             .filter(|_meta_data| {
  91 | ///                 // as an example, randomly reject half of the messages
  92 | ///                 # /*
  93 | ///                 rand::random()
  94 | ///                 # */
  95 | ///                 # true
  96 | ///             })
  97 | ///             .chain(io::stderr()),
  98 | ///     )
  99 | ///     // and finally, set as the global logger!
 100 | ///     .apply()?;
 101 | /// # Ok(())
 102 | /// # }
 103 | /// #
 104 | /// # fn main() { setup_logger().expect("failed to set up logger") }
 105 | /// ```
 106 | #[must_use = "this is only a logger configuration and must be consumed with into_log() or apply()"]
 107 | pub struct Dispatch {
 108 |     format: Option<Box<Formatter>>,
 109 |     children: Vec<OutputInner>,
 110 |     default_level: log::LevelFilter,
 111 |     levels: Vec<(Cow<'static, str>, log::LevelFilter)>,
 112 |     filters: Vec<Box<Filter>>,
 113 | }
 114 | 
 115 | /// Logger which is usable as an output for multiple other loggers.
 116 | ///
 117 | /// This struct contains a built logger stored in an [`Arc`], and can be
 118 | /// safely cloned.
 119 | ///
 120 | /// See [`Dispatch::into_shared`].
 121 | ///
 122 | /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
 123 | /// [`Dispatch::into_shared`]: struct.Dispatch.html#method.into_shared
 124 | #[derive(Clone)]
 125 | pub struct SharedDispatch {
 126 |     inner: Arc<log_impl::Dispatch>,
 127 |     min_level: log::LevelFilter,
 128 | }
 129 | 
 130 | impl Dispatch {
 131 |     /// Creates a dispatch, which will initially do nothing.
 132 |     #[inline]
 133 |     pub fn new() -> Self {
 134 |         Dispatch {
 135 |             format: None,
 136 |             children: Vec::new(),
 137 |             default_level: log::LevelFilter::Trace,
 138 |             levels: Vec::new(),
 139 |             filters: Vec::new(),
 140 |         }
 141 |     }
 142 | 
 143 |     /// Sets the formatter of this dispatch. The closure should accept a
 144 |     /// callback, a message and a log record, and write the resulting
 145 |     /// format to the writer.
 146 |     ///
 147 |     /// The log record is passed for completeness, but the `args()` method of
 148 |     /// the record should be ignored, and the [`fmt::Arguments`] given
 149 |     /// should be used instead. `record.args()` may be used to retrieve the
 150 |     /// _original_ log message, but in order to allow for true log
 151 |     /// chaining, formatters should use the given message instead whenever
 152 |     /// including the message in the output.
 153 |     ///
 154 |     /// To avoid all allocation of intermediate results, the formatter is
 155 |     /// "completed" by calling a callback, which then calls the rest of the
 156 |     /// logging chain with the new formatted message. The callback object keeps
 157 |     /// track of if it was called or not via a stack boolean as well, so if
 158 |     /// you don't use `out.finish` the log message will continue down
 159 |     /// the logger chain unformatted.
 160 |     ///
 161 |     /// [`fmt::Arguments`]: https://doc.rust-lang.org/std/fmt/struct.Arguments.html
 162 |     ///
 163 |     /// Example usage:
 164 |     ///
 165 |     /// ```
 166 |     /// fern::Dispatch::new().format(|out, message, record| {
 167 |     ///     out.finish(format_args!(
 168 |     ///         "[{} {}] {}",
 169 |     ///         record.level(),
 170 |     ///         record.target(),
 171 |     ///         message
 172 |     ///     ))
 173 |     /// })
 174 |     ///     # .into_log();
 175 |     /// ```
 176 |     #[inline]
 177 |     pub fn format<F>(mut self, formatter: F) -> Self
 178 |     where
 179 |         F: Fn(FormatCallback, &fmt::Arguments, &log::Record) + Sync + Send + 'static,
 180 |     {
 181 |         self.format = Some(Box::new(formatter));
 182 |         self
 183 |     }
 184 | 
 185 |     /// Adds a child to this dispatch.
 186 |     ///
 187 |     /// All log records which pass all filters will be formatted and then sent
 188 |     /// to all child loggers in sequence.
 189 |     ///
 190 |     /// Note: If the child logger is also a Dispatch, and cannot accept any log
 191 |     /// records, it will be dropped. This only happens if the child either
 192 |     /// has no children itself, or has a minimum log level of
 193 |     /// [`LevelFilter::Off`].
 194 |     ///
 195 |     /// [`LevelFilter::Off`]: https://docs.rs/log/0.4/log/enum.LevelFilter.html#variant.Off
 196 |     ///
 197 |     /// Example usage:
 198 |     ///
 199 |     /// ```
 200 |     /// fern::Dispatch::new().chain(fern::Dispatch::new().chain(std::io::stdout()))
 201 |     ///     # .into_log();
 202 |     /// ```
 203 |     #[inline]
 204 |     pub fn chain<T: Into<Output>>(mut self, logger: T) -> Self {
 205 |         self.children.push(logger.into().0);
 206 |         self
 207 |     }
 208 | 
 209 |     /// Sets the overarching level filter for this logger. All messages not
 210 |     /// already filtered by something set by [`Dispatch::level_for`] will
 211 |     /// be affected.
 212 |     ///
 213 |     /// All messages filtered will be discarded if less severe than the given
 214 |     /// level.
 215 |     ///
 216 |     /// Default level is [`LevelFilter::Trace`].
 217 |     ///
 218 |     /// [`Dispatch::level_for`]: #method.level_for
 219 |     /// [`LevelFilter::Trace`]: https://docs.rs/log/0.4/log/enum.LevelFilter.html#variant.Trace
 220 |     ///
 221 |     /// Example usage:
 222 |     ///
 223 |     /// ```
 224 |     /// # fn main() {
 225 |     /// fern::Dispatch::new().level(log::LevelFilter::Info)
 226 |     ///     # .into_log();
 227 |     /// # }
 228 |     /// ```
 229 |     #[inline]
 230 |     pub fn level(mut self, level: log::LevelFilter) -> Self {
 231 |         self.default_level = level;
 232 |         self
 233 |     }
 234 | 
 235 |     /// Sets a per-target log level filter. Default target for log messages is
 236 |     /// `crate_name::module_name` or
 237 |     /// `crate_name` for logs in the crate root. Targets can also be set with
 238 |     /// `info!(target: "target-name", ...)`.
 239 |     ///
 240 |     /// For each log record fern will first try to match the most specific
 241 |     /// level_for, and then progressively more general ones until either a
 242 |     /// matching level is found, or the default level is used.
 243 |     ///
 244 |     /// For example, a log for the target `hyper::http::h1` will first test a
 245 |     /// level_for for `hyper::http::h1`, then for `hyper::http`, then for
 246 |     /// `hyper`, then use the default level.
 247 |     ///
 248 |     /// Examples:
 249 |     ///
 250 |     /// A program wants to include a lot of debugging output, but the library
 251 |     /// "hyper" is known to work well, so debug output from it should be
 252 |     /// excluded:
 253 |     ///
 254 |     /// ```
 255 |     /// # fn main() {
 256 |     /// fern::Dispatch::new()
 257 |     ///     .level(log::LevelFilter::Trace)
 258 |     ///     .level_for("hyper", log::LevelFilter::Info)
 259 |     ///     # .into_log();
 260 |     /// # }
 261 |     /// ```
 262 |     ///
 263 |     /// A program has a ton of debug output per-module, but there is so much
 264 |     /// that debugging more than one module at a time is not very useful.
 265 |     /// The command line accepts a list of modules to debug, while keeping the
 266 |     /// rest of the program at info level:
 267 |     ///
 268 |     /// ```
 269 |     /// fn setup_logging<T, I>(verbose_modules: T) -> Result<(), fern::InitError>
 270 |     /// where
 271 |     ///     I: AsRef<str>,
 272 |     ///     T: IntoIterator<Item = I>,
 273 |     /// {
 274 |     ///     let mut config = fern::Dispatch::new().level(log::LevelFilter::Info);
 275 |     ///
 276 |     ///     for module_name in verbose_modules {
 277 |     ///         config = config.level_for(
 278 |     ///             format!("my_crate_name::{}", module_name.as_ref()),
 279 |     ///             log::LevelFilter::Debug,
 280 |     ///         );
 281 |     ///     }
 282 |     ///
 283 |     ///     config.chain(std::io::stdout()).apply()?;
 284 |     ///
 285 |     ///     Ok(())
 286 |     /// }
 287 |     /// #
 288 |     /// # // we're ok with apply() failing.
 289 |     /// # fn main() { let _ = setup_logging(&["hi"]); }
 290 |     /// ```
 291 |     #[inline]
 292 |     pub fn level_for<T: Into<Cow<'static, str>>>(
 293 |         mut self,
 294 |         module: T,
 295 |         level: log::LevelFilter,
 296 |     ) -> Self {
 297 |         let module = module.into();
 298 | 
 299 |         if let Some((index, _)) = self
 300 |             .levels
 301 |             .iter()
 302 |             .enumerate()
 303 |             .find(|(_, (name, _))| *name == module)
 304 |         {
 305 |             self.levels.remove(index);
 306 |         }
 307 | 
 308 |         self.levels.push((module, level));
 309 |         self
 310 |     }
 311 | 
 312 |     /// Adds a custom filter which can reject messages passing through this
 313 |     /// logger.
 314 |     ///
 315 |     /// The logger will continue to process log records only if all filters
 316 |     /// return `true`.
 317 |     ///
 318 |     /// [`Dispatch::level`] and [`Dispatch::level_for`] are preferred if
 319 |     /// applicable.
 320 |     ///
 321 |     /// [`Dispatch::level`]: #method.level
 322 |     /// [`Dispatch::level_for`]: #method.level_for
 323 |     ///
 324 |     /// Example usage:
 325 |     ///
 326 |     /// This sends error level messages to stderr and others to stdout.
 327 |     ///
 328 |     /// ```
 329 |     /// # fn main() {
 330 |     /// fern::Dispatch::new()
 331 |     ///     .level(log::LevelFilter::Info)
 332 |     ///     .chain(
 333 |     ///         fern::Dispatch::new()
 334 |     ///             .filter(|metadata| {
 335 |     ///                 // Reject messages with the `Error` log level.
 336 |     ///                 metadata.level() != log::LevelFilter::Error
 337 |     ///             })
 338 |     ///             .chain(std::io::stderr()),
 339 |     ///     )
 340 |     ///     .chain(
 341 |     ///         fern::Dispatch::new()
 342 |     ///             .level(log::LevelFilter::Error)
 343 |     ///             .chain(std::io::stdout()),
 344 |     ///     )
 345 |     ///     # .into_log();
 346 |     /// # }
 347 |     #[inline]
 348 |     pub fn filter<F>(mut self, filter: F) -> Self
 349 |     where
 350 |         F: Fn(&log::Metadata) -> bool + Send + Sync + 'static,
 351 |     {
 352 |         self.filters.push(Box::new(filter));
 353 |         self
 354 |     }
 355 | 
 356 |     /// Builds this dispatch and stores it in a clonable structure containing
 357 |     /// an [`Arc`].
 358 |     ///
 359 |     /// Once "shared", the dispatch can be used as an output for multiple other
 360 |     /// dispatch loggers.
 361 |     ///
 362 |     /// Example usage:
 363 |     ///
 364 |     /// This separates info and warn messages, sending info to stdout + a log
 365 |     /// file, and warn to stderr + the same log file. Shared is used so the
 366 |     /// program only opens "file.log" once.
 367 |     ///
 368 |     /// ```no_run
 369 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
 370 |     ///
 371 |     /// let file_out = fern::Dispatch::new()
 372 |     ///     .chain(fern::log_file("file.log")?)
 373 |     ///     .into_shared();
 374 |     ///
 375 |     /// let info_out = fern::Dispatch::new()
 376 |     ///     .level(log::LevelFilter::Debug)
 377 |     ///     .filter(|metadata|
 378 |     ///         // keep only info and debug (reject warn and error)
 379 |     ///         metadata.level() <= log::Level::Info)
 380 |     ///     .chain(std::io::stdout())
 381 |     ///     .chain(file_out.clone());
 382 |     ///
 383 |     /// let warn_out = fern::Dispatch::new()
 384 |     ///     .level(log::LevelFilter::Warn)
 385 |     ///     .chain(std::io::stderr())
 386 |     ///     .chain(file_out);
 387 |     ///
 388 |     /// fern::Dispatch::new()
 389 |     ///     .chain(info_out)
 390 |     ///     .chain(warn_out)
 391 |     ///     .apply();
 392 |     ///
 393 |     /// # Ok(())
 394 |     /// # }
 395 |     /// #
 396 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
 397 |     /// ```
 398 |     ///
 399 |     /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
 400 |     pub fn into_shared(self) -> SharedDispatch {
 401 |         let (min_level, dispatch) = self.into_dispatch();
 402 | 
 403 |         SharedDispatch {
 404 |             inner: Arc::new(dispatch),
 405 |             min_level,
 406 |         }
 407 |     }
 408 | 
 409 |     /// Builds this into the actual logger implementation.
 410 |     ///
 411 |     /// This could probably be refactored, but having everything in one place
 412 |     /// is also nice.
 413 |     fn into_dispatch(self) -> (log::LevelFilter, log_impl::Dispatch) {
 414 |         let Dispatch {
 415 |             format,
 416 |             children,
 417 |             default_level,
 418 |             levels,
 419 |             mut filters,
 420 |         } = self;
 421 | 
 422 |         let mut max_child_level = log::LevelFilter::Off;
 423 | 
 424 |         let output = children
 425 |             .into_iter()
 426 |             .flat_map(|child| match child {
 427 |                 OutputInner::Stdout { stream, line_sep } => {
 428 |                     max_child_level = log::LevelFilter::Trace;
 429 |                     Some(log_impl::Output::Stdout(log_impl::Stdout {
 430 |                         stream,
 431 |                         line_sep,
 432 |                     }))
 433 |                 }
 434 |                 OutputInner::Stderr { stream, line_sep } => {
 435 |                     max_child_level = log::LevelFilter::Trace;
 436 |                     Some(log_impl::Output::Stderr(log_impl::Stderr {
 437 |                         stream,
 438 |                         line_sep,
 439 |                     }))
 440 |                 }
 441 |                 OutputInner::File { stream, line_sep } => {
 442 |                     max_child_level = log::LevelFilter::Trace;
 443 |                     Some(log_impl::Output::File(log_impl::File {
 444 |                         stream: Mutex::new(io::BufWriter::new(stream)),
 445 |                         line_sep,
 446 |                     }))
 447 |                 }
 448 |                 OutputInner::Writer { stream, line_sep } => {
 449 |                     max_child_level = log::LevelFilter::Trace;
 450 |                     Some(log_impl::Output::Writer(log_impl::Writer {
 451 |                         stream: Mutex::new(stream),
 452 |                         line_sep,
 453 |                     }))
 454 |                 }
 455 |                 #[cfg(all(not(windows), feature = "reopen-03"))]
 456 |                 OutputInner::Reopen { stream, line_sep } => {
 457 |                     max_child_level = log::LevelFilter::Trace;
 458 |                     Some(log_impl::Output::Reopen(log_impl::Reopen {
 459 |                         stream: Mutex::new(stream),
 460 |                         line_sep,
 461 |                     }))
 462 |                 }
 463 |                 #[cfg(all(not(windows), feature = "reopen-1"))]
 464 |                 OutputInner::Reopen1 { stream, line_sep } => {
 465 |                     max_child_level = log::LevelFilter::Trace;
 466 |                     Some(log_impl::Output::Reopen1(log_impl::Reopen1 {
 467 |                         stream: Mutex::new(stream),
 468 |                         line_sep,
 469 |                     }))
 470 |                 }
 471 |                 OutputInner::Sender { stream, line_sep } => {
 472 |                     max_child_level = log::LevelFilter::Trace;
 473 |                     Some(log_impl::Output::Sender(log_impl::Sender {
 474 |                         stream: Mutex::new(stream),
 475 |                         line_sep,
 476 |                     }))
 477 |                 }
 478 |                 #[cfg(all(not(windows), feature = "syslog-3"))]
 479 |                 OutputInner::Syslog3(log) => {
 480 |                     max_child_level = log::LevelFilter::Trace;
 481 |                     Some(log_impl::Output::Syslog3(log_impl::Syslog3 { inner: log }))
 482 |                 }
 483 |                 #[cfg(all(not(windows), feature = "syslog-4"))]
 484 |                 OutputInner::Syslog4Rfc3164(logger) => {
 485 |                     max_child_level = log::LevelFilter::Trace;
 486 |                     Some(log_impl::Output::Syslog4Rfc3164(log_impl::Syslog4Rfc3164 {
 487 |                         inner: Mutex::new(logger),
 488 |                     }))
 489 |                 }
 490 |                 #[cfg(all(not(windows), feature = "syslog-4"))]
 491 |                 OutputInner::Syslog4Rfc5424 { logger, transform } => {
 492 |                     max_child_level = log::LevelFilter::Trace;
 493 |                     Some(log_impl::Output::Syslog4Rfc5424(log_impl::Syslog4Rfc5424 {
 494 |                         inner: Mutex::new(logger),
 495 |                         transform,
 496 |                     }))
 497 |                 }
 498 |                 #[cfg(all(not(windows), feature = "syslog-6"))]
 499 |                 OutputInner::Syslog6Rfc3164(logger) => {
 500 |                     max_child_level = log::LevelFilter::Trace;
 501 |                     Some(log_impl::Output::Syslog6Rfc3164(log_impl::Syslog6Rfc3164 {
 502 |                         inner: Mutex::new(logger),
 503 |                     }))
 504 |                 }
 505 |                 #[cfg(all(not(windows), feature = "syslog-6"))]
 506 |                 OutputInner::Syslog6Rfc5424 { logger, transform } => {
 507 |                     max_child_level = log::LevelFilter::Trace;
 508 |                     Some(log_impl::Output::Syslog6Rfc5424(log_impl::Syslog6Rfc5424 {
 509 |                         inner: Mutex::new(logger),
 510 |                         transform,
 511 |                     }))
 512 |                 }
 513 |                 #[cfg(all(not(windows), feature = "syslog-7"))]
 514 |                 OutputInner::Syslog7Rfc3164(logger) => {
 515 |                     max_child_level = log::LevelFilter::Trace;
 516 |                     Some(log_impl::Output::Syslog7Rfc3164(log_impl::Syslog7Rfc3164 {
 517 |                         inner: Mutex::new(logger),
 518 |                     }))
 519 |                 }
 520 |                 #[cfg(all(not(windows), feature = "syslog-7"))]
 521 |                 OutputInner::Syslog7Rfc5424 { logger, transform } => {
 522 |                     max_child_level = log::LevelFilter::Trace;
 523 |                     Some(log_impl::Output::Syslog7Rfc5424(log_impl::Syslog7Rfc5424 {
 524 |                         inner: Mutex::new(logger),
 525 |                         transform,
 526 |                     }))
 527 |                 }
 528 |                 OutputInner::Panic => {
 529 |                     max_child_level = log::LevelFilter::Trace;
 530 |                     Some(log_impl::Output::Panic(log_impl::Panic))
 531 |                 }
 532 |                 OutputInner::Dispatch(child_dispatch) => {
 533 |                     let (child_level, child) = child_dispatch.into_dispatch();
 534 |                     if child_level > log::LevelFilter::Off {
 535 |                         max_child_level = cmp::max(max_child_level, child_level);
 536 |                         Some(log_impl::Output::Dispatch(child))
 537 |                     } else {
 538 |                         None
 539 |                     }
 540 |                 }
 541 |                 OutputInner::SharedDispatch(child_dispatch) => {
 542 |                     let SharedDispatch {
 543 |                         inner: child,
 544 |                         min_level: child_level,
 545 |                     } = child_dispatch;
 546 | 
 547 |                     if child_level > log::LevelFilter::Off {
 548 |                         max_child_level = cmp::max(max_child_level, child_level);
 549 |                         Some(log_impl::Output::SharedDispatch(child))
 550 |                     } else {
 551 |                         None
 552 |                     }
 553 |                 }
 554 |                 OutputInner::OtherBoxed(child_log) => {
 555 |                     max_child_level = log::LevelFilter::Trace;
 556 |                     Some(log_impl::Output::OtherBoxed(child_log))
 557 |                 }
 558 |                 OutputInner::OtherStatic(child_log) => {
 559 |                     max_child_level = log::LevelFilter::Trace;
 560 |                     Some(log_impl::Output::OtherStatic(child_log))
 561 |                 }
 562 |                 #[cfg(feature = "date-based")]
 563 |                 OutputInner::DateBased { config } => {
 564 |                     max_child_level = log::LevelFilter::Trace;
 565 | 
 566 |                     let config = log_impl::DateBasedConfig::new(
 567 |                         config.line_sep,
 568 |                         config.file_prefix,
 569 |                         config.file_suffix,
 570 |                         if config.utc_time {
 571 |                             log_impl::ConfiguredTimezone::Utc
 572 |                         } else {
 573 |                             log_impl::ConfiguredTimezone::Local
 574 |                         },
 575 |                     );
 576 | 
 577 |                     let computed_suffix = config.compute_current_suffix();
 578 | 
 579 |                     // ignore errors - we'll just retry later.
 580 |                     let initial_file = config.open_current_log_file(&computed_suffix).ok();
 581 | 
 582 |                     Some(log_impl::Output::DateBased(log_impl::DateBased {
 583 |                         config,
 584 |                         state: Mutex::new(DateBasedState::new(computed_suffix, initial_file)),
 585 |                     }))
 586 |                 }
 587 |             })
 588 |             .collect();
 589 | 
 590 |         let min_level = levels
 591 |             .iter()
 592 |             .map(|t| t.1)
 593 |             .max()
 594 |             .map_or(default_level, |lvl| cmp::max(lvl, default_level));
 595 |         let real_min = cmp::min(min_level, max_child_level);
 596 | 
 597 |         filters.shrink_to_fit();
 598 | 
 599 |         let dispatch = log_impl::Dispatch {
 600 |             output,
 601 |             default_level,
 602 |             levels: levels.into(),
 603 |             format,
 604 |             filters,
 605 |         };
 606 | 
 607 |         (real_min, dispatch)
 608 |     }
 609 | 
 610 |     /// Builds this logger into a `Box<dyn log::Log>` and calculates the minimum
 611 |     /// log level needed to have any effect.
 612 |     ///
 613 |     /// While this method is exposed publicly, [`Dispatch::apply`] is typically
 614 |     /// used instead.
 615 |     ///
 616 |     /// The returned LevelFilter is a calculation for all level filters of this
 617 |     /// logger and child loggers, and is the minimum log level needed to
 618 |     /// for a record to have any chance of passing through this logger.
 619 |     ///
 620 |     /// [`Dispatch::apply`]: #method.apply
 621 |     ///
 622 |     /// Example usage:
 623 |     ///
 624 |     /// ```
 625 |     /// # fn main() {
 626 |     /// let (min_level, log) = fern::Dispatch::new()
 627 |     ///     .level(log::LevelFilter::Info)
 628 |     ///     .chain(std::io::stdout())
 629 |     ///     .into_log();
 630 |     ///
 631 |     /// assert_eq!(min_level, log::LevelFilter::Info);
 632 |     /// # }
 633 |     /// ```
 634 |     pub fn into_log(self) -> (log::LevelFilter, Box<dyn log::Log>) {
 635 |         let (level, logger) = self.into_dispatch();
 636 |         if level == log::LevelFilter::Off {
 637 |             (level, Box::new(log_impl::Null))
 638 |         } else {
 639 |             (level, Box::new(logger))
 640 |         }
 641 |     }
 642 | 
 643 |     /// Builds this logger and instantiates it as the global [`log`] logger.
 644 |     ///
 645 |     /// # Errors:
 646 |     ///
 647 |     /// This function will return an error if a global logger has already been
 648 |     /// set to a previous logger.
 649 |     ///
 650 |     /// [`log`]: https://github.com/rust-lang-nursery/log
 651 |     pub fn apply(self) -> Result<(), log::SetLoggerError> {
 652 |         let (max_level, log) = self.into_log();
 653 | 
 654 |         log::set_boxed_logger(log)?;
 655 |         log::set_max_level(max_level);
 656 | 
 657 |         Ok(())
 658 |     }
 659 | }
 660 | 
 661 | /// This enum contains various outputs that you can send messages to.
 662 | enum OutputInner {
 663 |     /// Prints all messages to stdout with `line_sep` separator.
 664 |     Stdout {
 665 |         stream: io::Stdout,
 666 |         line_sep: Cow<'static, str>,
 667 |     },
 668 |     /// Prints all messages to stderr with `line_sep` separator.
 669 |     Stderr {
 670 |         stream: io::Stderr,
 671 |         line_sep: Cow<'static, str>,
 672 |     },
 673 |     /// Writes all messages to file with `line_sep` separator.
 674 |     File {
 675 |         stream: fs::File,
 676 |         line_sep: Cow<'static, str>,
 677 |     },
 678 |     /// Writes all messages to the writer with `line_sep` separator.
 679 |     Writer {
 680 |         stream: Box<dyn Write + Send>,
 681 |         line_sep: Cow<'static, str>,
 682 |     },
 683 |     /// Writes all messages to the reopen::Reopen file with `line_sep`
 684 |     /// separator.
 685 |     #[cfg(all(not(windows), feature = "reopen-03"))]
 686 |     Reopen {
 687 |         stream: reopen03::Reopen<fs::File>,
 688 |         line_sep: Cow<'static, str>,
 689 |     },
 690 |     /// Writes all messages to the reopen::Reopen file with `line_sep`
 691 |     /// separator.
 692 |     #[cfg(all(not(windows), feature = "reopen-1"))]
 693 |     Reopen1 {
 694 |         stream: reopen1::Reopen<fs::File>,
 695 |         line_sep: Cow<'static, str>,
 696 |     },
 697 |     /// Writes all messages to mpst::Sender with `line_sep` separator.
 698 |     Sender {
 699 |         stream: Sender<String>,
 700 |         line_sep: Cow<'static, str>,
 701 |     },
 702 |     /// Passes all messages to other dispatch.
 703 |     Dispatch(Dispatch),
 704 |     /// Passes all messages to other dispatch that's shared.
 705 |     SharedDispatch(SharedDispatch),
 706 |     /// Passes all messages to other logger.
 707 |     OtherBoxed(Box<dyn Log>),
 708 |     /// Passes all messages to other logger.
 709 |     OtherStatic(&'static dyn Log),
 710 |     /// Passes all messages to the syslog.
 711 |     #[cfg(all(not(windows), feature = "syslog-3"))]
 712 |     Syslog3(syslog3::Logger),
 713 |     /// Passes all messages to the syslog.
 714 |     #[cfg(all(not(windows), feature = "syslog-4"))]
 715 |     Syslog4Rfc3164(Syslog4Rfc3164Logger),
 716 |     /// Sends all messages through the transform then passes to the syslog.
 717 |     #[cfg(all(not(windows), feature = "syslog-4"))]
 718 |     Syslog4Rfc5424 {
 719 |         logger: Syslog4Rfc5424Logger,
 720 |         transform: Box<Syslog4TransformFn>,
 721 |     },
 722 |     #[cfg(all(not(windows), feature = "syslog-6"))]
 723 |     Syslog6Rfc3164(Syslog6Rfc3164Logger),
 724 |     /// Sends all messages through the transform then passes to the syslog.
 725 |     #[cfg(all(not(windows), feature = "syslog-6"))]
 726 |     Syslog6Rfc5424 {
 727 |         logger: Syslog6Rfc5424Logger,
 728 |         transform: Box<Syslog6TransformFn>,
 729 |     },
 730 |     #[cfg(all(not(windows), feature = "syslog-7"))]
 731 |     Syslog7Rfc3164(Syslog7Rfc3164Logger),
 732 |     /// Sends all messages through the transform then passes to the syslog.
 733 |     #[cfg(all(not(windows), feature = "syslog-7"))]
 734 |     Syslog7Rfc5424 {
 735 |         logger: Syslog7Rfc5424Logger,
 736 |         transform: Box<Syslog7TransformFn>,
 737 |     },
 738 |     /// Panics with messages text for all messages.
 739 |     Panic,
 740 |     /// File logger with custom date and timestamp suffix in file name.
 741 |     #[cfg(feature = "date-based")]
 742 |     DateBased { config: DateBased },
 743 | }
 744 | 
 745 | /// Logger which will panic whenever anything is logged. The panic
 746 | /// will be exactly the message of the log.
 747 | ///
 748 | /// `Panic` is useful primarily as a secondary logger, filtered by warning or
 749 | /// error.
 750 | ///
 751 | /// # Examples
 752 | ///
 753 | /// This configuration will output all messages to stdout and panic if an Error
 754 | /// message is sent.
 755 | ///
 756 | /// ```
 757 | /// fern::Dispatch::new()
 758 | ///     // format, etc.
 759 | ///     .chain(std::io::stdout())
 760 | ///     .chain(
 761 | ///         fern::Dispatch::new()
 762 | ///             .level(log::LevelFilter::Error)
 763 | ///             .chain(fern::Panic),
 764 | ///     )
 765 | ///     # /*
 766 | ///     .apply()?;
 767 | ///     # */ .into_log();
 768 | /// ```
 769 | ///
 770 | /// This sets up a "panic on warn+" logger, and ignores errors so it can be
 771 | /// called multiple times.
 772 | ///
 773 | /// This might be useful in test setup, for example, to disallow warn-level
 774 | /// messages.
 775 | ///
 776 | /// ```no_run
 777 | /// fn setup_panic_logging() {
 778 | ///     fern::Dispatch::new()
 779 | ///         .level(log::LevelFilter::Warn)
 780 | ///         .chain(fern::Panic)
 781 | ///         .apply()
 782 | ///         // ignore errors from setting up logging twice
 783 | ///         .ok();
 784 | /// }
 785 | /// ```
 786 | pub struct Panic;
 787 | 
 788 | /// Configuration for a logger output.
 789 | pub struct Output(OutputInner);
 790 | 
 791 | impl From<Dispatch> for Output {
 792 |     /// Creates an output logger forwarding all messages to the dispatch.
 793 |     fn from(log: Dispatch) -> Self {
 794 |         Output(OutputInner::Dispatch(log))
 795 |     }
 796 | }
 797 | 
 798 | impl From<SharedDispatch> for Output {
 799 |     /// Creates an output logger forwarding all messages to the dispatch.
 800 |     fn from(log: SharedDispatch) -> Self {
 801 |         Output(OutputInner::SharedDispatch(log))
 802 |     }
 803 | }
 804 | 
 805 | impl From<Box<dyn Log>> for Output {
 806 |     /// Creates an output logger forwarding all messages to the custom logger.
 807 |     fn from(log: Box<dyn Log>) -> Self {
 808 |         Output(OutputInner::OtherBoxed(log))
 809 |     }
 810 | }
 811 | 
 812 | impl From<&'static dyn Log> for Output {
 813 |     /// Creates an output logger forwarding all messages to the custom logger.
 814 |     fn from(log: &'static dyn Log) -> Self {
 815 |         Output(OutputInner::OtherStatic(log))
 816 |     }
 817 | }
 818 | 
 819 | impl From<fs::File> for Output {
 820 |     /// Creates an output logger which writes all messages to the file with
 821 |     /// `\n` as the separator.
 822 |     ///
 823 |     /// File writes are buffered and flushed once per log record.
 824 |     fn from(file: fs::File) -> Self {
 825 |         Output(OutputInner::File {
 826 |             stream: file,
 827 |             line_sep: "\n".into(),
 828 |         })
 829 |     }
 830 | }
 831 | 
 832 | impl From<Box<dyn Write + Send>> for Output {
 833 |     /// Creates an output logger which writes all messages to the writer with
 834 |     /// `\n` as the separator.
 835 |     ///
 836 |     /// This does no buffering and it is up to the writer to do buffering as
 837 |     /// needed (eg. wrap it in `BufWriter`). However, flush is called after
 838 |     /// each log record.
 839 |     fn from(writer: Box<dyn Write + Send>) -> Self {
 840 |         Output(OutputInner::Writer {
 841 |             stream: writer,
 842 |             line_sep: "\n".into(),
 843 |         })
 844 |     }
 845 | }
 846 | 
 847 | #[cfg(all(not(windows), feature = "reopen-03"))]
 848 | impl From<reopen03::Reopen<fs::File>> for Output {
 849 |     /// Creates an output logger which writes all messages to the file contained
 850 |     /// in the Reopen struct, using `\n` as the separator.
 851 |     fn from(reopen: reopen03::Reopen<fs::File>) -> Self {
 852 |         Output(OutputInner::Reopen {
 853 |             stream: reopen,
 854 |             line_sep: "\n".into(),
 855 |         })
 856 |     }
 857 | }
 858 | 
 859 | #[cfg(all(not(windows), feature = "reopen-1"))]
 860 | impl From<reopen1::Reopen<fs::File>> for Output {
 861 |     /// Creates an output logger which writes all messages to the file contained
 862 |     /// in the Reopen struct, using `\n` as the separator.
 863 |     fn from(reopen: reopen1::Reopen<fs::File>) -> Self {
 864 |         Output(OutputInner::Reopen1 {
 865 |             stream: reopen,
 866 |             line_sep: "\n".into(),
 867 |         })
 868 |     }
 869 | }
 870 | 
 871 | impl From<io::Stdout> for Output {
 872 |     /// Creates an output logger which writes all messages to stdout with the
 873 |     /// given handle and `\n` as the separator.
 874 |     fn from(stream: io::Stdout) -> Self {
 875 |         Output(OutputInner::Stdout {
 876 |             stream,
 877 |             line_sep: "\n".into(),
 878 |         })
 879 |     }
 880 | }
 881 | 
 882 | impl From<io::Stderr> for Output {
 883 |     /// Creates an output logger which writes all messages to stderr with the
 884 |     /// given handle and `\n` as the separator.
 885 |     fn from(stream: io::Stderr) -> Self {
 886 |         Output(OutputInner::Stderr {
 887 |             stream,
 888 |             line_sep: "\n".into(),
 889 |         })
 890 |     }
 891 | }
 892 | 
 893 | impl From<Sender<String>> for Output {
 894 |     /// Creates an output logger which writes all messages to the given
 895 |     /// mpsc::Sender with  '\n' as the separator.
 896 |     ///
 897 |     /// All messages sent to the mpsc channel are suffixed with '\n'.
 898 |     fn from(stream: Sender<String>) -> Self {
 899 |         Output(OutputInner::Sender {
 900 |             stream,
 901 |             line_sep: "\n".into(),
 902 |         })
 903 |     }
 904 | }
 905 | 
 906 | #[cfg(all(not(windows), feature = "syslog-3"))]
 907 | impl From<syslog3::Logger> for Output {
 908 |     /// Creates an output logger which writes all messages to the given syslog
 909 |     /// output.
 910 |     ///
 911 |     /// Log levels are translated trace => debug, debug => debug, info =>
 912 |     /// informational, warn => warning, and error => error.
 913 |     ///
 914 |     /// This requires the `"syslog-3"` feature.
 915 |     fn from(log: syslog3::Logger) -> Self {
 916 |         Output(OutputInner::Syslog3(log))
 917 |     }
 918 | }
 919 | 
 920 | #[cfg(all(not(windows), feature = "syslog-3"))]
 921 | impl From<Box<syslog3::Logger>> for Output {
 922 |     /// Creates an output logger which writes all messages to the given syslog
 923 |     /// output.
 924 |     ///
 925 |     /// Log levels are translated trace => debug, debug => debug, info =>
 926 |     /// informational, warn => warning, and error => error.
 927 |     ///
 928 |     /// Note that while this takes a `Box<Logger>` for convenience (syslog
 929 |     /// methods return `Box`es), it will be immediately unboxed upon storage
 930 |     /// in the configuration structure. This will create a configuration
 931 |     /// identical to that created by passing a raw `syslog::Logger`.
 932 |     ///
 933 |     /// This requires the `"syslog-3"` feature.
 934 |     fn from(log: Box<syslog3::Logger>) -> Self {
 935 |         Output(OutputInner::Syslog3(*log))
 936 |     }
 937 | }
 938 | 
 939 | #[cfg(all(not(windows), feature = "syslog-4"))]
 940 | impl From<Syslog4Rfc3164Logger> for Output {
 941 |     /// Creates an output logger which writes all messages to the given syslog.
 942 |     ///
 943 |     /// Log levels are translated trace => debug, debug => debug, info =>
 944 |     /// informational, warn => warning, and error => error.
 945 |     ///
 946 |     /// Note that due to <https://github.com/Geal/rust-syslog/issues/41>,
 947 |     /// logging to this backend requires one allocation per log call.
 948 |     ///
 949 |     /// This is for RFC 3164 loggers. To use an RFC 5424 logger, use the
 950 |     /// [`Output::syslog_5424`] helper method.
 951 |     ///
 952 |     /// This requires the `"syslog-4"` feature.
 953 |     fn from(log: Syslog4Rfc3164Logger) -> Self {
 954 |         Output(OutputInner::Syslog4Rfc3164(log))
 955 |     }
 956 | }
 957 | 
 958 | #[cfg(all(not(windows), feature = "syslog-6"))]
 959 | impl From<Syslog6Rfc3164Logger> for Output {
 960 |     /// Creates an output logger which writes all messages to the given syslog.
 961 |     ///
 962 |     /// Log levels are translated trace => debug, debug => debug, info =>
 963 |     /// informational, warn => warning, and error => error.
 964 |     ///
 965 |     /// Note that due to <https://github.com/Geal/rust-syslog/issues/41>,
 966 |     /// logging to this backend requires one allocation per log call.
 967 |     ///
 968 |     /// This is for RFC 3164 loggers. To use an RFC 5424 logger, use the
 969 |     /// [`Output::syslog_5424`] helper method.
 970 |     ///
 971 |     /// This requires the `"syslog-6"` feature.
 972 |     fn from(log: Syslog6Rfc3164Logger) -> Self {
 973 |         Output(OutputInner::Syslog6Rfc3164(log))
 974 |     }
 975 | }
 976 | 
 977 | #[cfg(all(not(windows), feature = "syslog-7"))]
 978 | impl From<Syslog7Rfc3164Logger> for Output {
 979 |     /// Creates an output logger which writes all messages to the given syslog.
 980 |     ///
 981 |     /// Log levels are translated trace => debug, debug => debug, info =>
 982 |     /// informational, warn => warning, and error => error.
 983 |     ///
 984 |     /// Note that due to <https://github.com/Geal/rust-syslog/issues/41>,
 985 |     /// logging to this backend requires one allocation per log call.
 986 |     ///
 987 |     /// This is for RFC 3164 loggers. To use an RFC 5424 logger, use the
 988 |     /// [`Output::syslog_5424`] helper method.
 989 |     ///
 990 |     /// This requires the `"syslog-7"` feature.
 991 |     fn from(log: Syslog7Rfc3164Logger) -> Self {
 992 |         Output(OutputInner::Syslog7Rfc3164(log))
 993 |     }
 994 | }
 995 | 
 996 | impl From<Panic> for Output {
 997 |     /// Creates an output logger which will panic with message text for all
 998 |     /// messages.
 999 |     fn from(_: Panic) -> Self {
1000 |         Output(OutputInner::Panic)
1001 |     }
1002 | }
1003 | 
1004 | impl Output {
1005 |     /// Returns a file logger using a custom separator.
1006 |     ///
1007 |     /// If the default separator of `\n` is acceptable, an [`fs::File`]
1008 |     /// instance can be passed into [`Dispatch::chain`] directly.
1009 |     ///
1010 |     /// ```no_run
1011 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1012 |     /// fern::Dispatch::new().chain(std::fs::File::create("log")?)
1013 |     ///     # .into_log();
1014 |     /// # Ok(())
1015 |     /// # }
1016 |     /// #
1017 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1018 |     /// ```
1019 |     ///
1020 |     /// ```no_run
1021 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1022 |     /// fern::Dispatch::new().chain(fern::log_file("log")?)
1023 |     ///     # .into_log();
1024 |     /// # Ok(())
1025 |     /// # }
1026 |     /// #
1027 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1028 |     /// ```
1029 |     ///
1030 |     /// Example usage (using [`fern::log_file`]):
1031 |     ///
1032 |     /// ```no_run
1033 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1034 |     /// fern::Dispatch::new().chain(fern::Output::file(fern::log_file("log")?, "\r\n"))
1035 |     ///     # .into_log();
1036 |     /// # Ok(())
1037 |     /// # }
1038 |     /// #
1039 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1040 |     /// ```
1041 |     ///
1042 |     /// [`fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
1043 |     /// [`Dispatch::chain`]: struct.Dispatch.html#method.chain
1044 |     /// [`fern::log_file`]: fn.log_file.html
1045 |     pub fn file<T: Into<Cow<'static, str>>>(file: fs::File, line_sep: T) -> Self {
1046 |         Output(OutputInner::File {
1047 |             stream: file,
1048 |             line_sep: line_sep.into(),
1049 |         })
1050 |     }
1051 | 
1052 |     /// Returns a logger using arbitrary write object and custom separator.
1053 |     ///
1054 |     /// If the default separator of `\n` is acceptable, an `Box<dyn Write + Send>`
1055 |     /// instance can be passed into [`Dispatch::chain`] directly.
1056 |     ///
1057 |     /// ```no_run
1058 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1059 |     /// // Anything implementing 'Write' works.
1060 |     /// let mut writer = std::io::Cursor::new(Vec::<u8>::new());
1061 |     ///
1062 |     /// fern::Dispatch::new()
1063 |     ///     // as long as we explicitly cast into a type-erased Box
1064 |     ///     .chain(Box::new(writer) as Box<dyn std::io::Write + Send>)
1065 |     ///     # .into_log();
1066 |     /// #     Ok(())
1067 |     /// # }
1068 |     /// #
1069 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1070 |     /// ```
1071 |     ///
1072 |     /// Example usage:
1073 |     ///
1074 |     /// ```no_run
1075 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1076 |     /// let writer = Box::new(std::io::Cursor::new(Vec::<u8>::new()));
1077 |     ///
1078 |     /// fern::Dispatch::new().chain(fern::Output::writer(writer, "\r\n"))
1079 |     ///     # .into_log();
1080 |     /// #     Ok(())
1081 |     /// # }
1082 |     /// #
1083 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1084 |     /// ```
1085 |     ///
1086 |     /// [`Dispatch::chain`]: struct.Dispatch.html#method.chain
1087 |     pub fn writer<T: Into<Cow<'static, str>>>(writer: Box<dyn Write + Send>, line_sep: T) -> Self {
1088 |         Output(OutputInner::Writer {
1089 |             stream: writer,
1090 |             line_sep: line_sep.into(),
1091 |         })
1092 |     }
1093 | 
1094 |     /// Returns a reopenable logger, i.e., handling SIGHUP.
1095 |     ///
1096 |     /// If the default separator of `\n` is acceptable, a `Reopen`
1097 |     /// instance can be passed into [`Dispatch::chain`] directly.
1098 |     ///
1099 |     /// This function is not available on Windows, and it requires the `reopen-03`
1100 |     /// feature to be enabled.
1101 |     ///
1102 |     /// ```no_run
1103 |     /// use std::fs::OpenOptions;
1104 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1105 |     /// let reopenable = reopen03::Reopen::new(Box::new(|| {
1106 |     ///     OpenOptions::new()
1107 |     ///         .create(true)
1108 |     ///         .write(true)
1109 |     ///         .append(true)
1110 |     ///         .open("/tmp/output.log")
1111 |     /// }))
1112 |     /// .unwrap();
1113 |     ///
1114 |     /// fern::Dispatch::new().chain(fern::Output::reopen(reopenable, "\n"))
1115 |     ///     # .into_log();
1116 |     /// #     Ok(())
1117 |     /// # }
1118 |     /// #
1119 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1120 |     /// ```
1121 |     /// [`Dispatch::chain`]: struct.Dispatch.html#method.chain
1122 |     #[cfg(all(not(windows), feature = "reopen-03"))]
1123 |     pub fn reopen<T: Into<Cow<'static, str>>>(
1124 |         reopen: reopen03::Reopen<fs::File>,
1125 |         line_sep: T,
1126 |     ) -> Self {
1127 |         Output(OutputInner::Reopen {
1128 |             stream: reopen,
1129 |             line_sep: line_sep.into(),
1130 |         })
1131 |     }
1132 | 
1133 |     /// Returns a reopenable logger, i.e., handling SIGHUP.
1134 |     ///
1135 |     /// If the default separator of `\n` is acceptable, a `Reopen`
1136 |     /// instance can be passed into [`Dispatch::chain`] directly.
1137 |     ///
1138 |     /// This function is not available on Windows, and it requires the `reopen-03`
1139 |     /// feature to be enabled.
1140 |     ///
1141 |     /// ```no_run
1142 |     /// use std::fs::OpenOptions;
1143 |     /// # fn setup_logger() -> Result<(), fern::InitError> {
1144 |     /// let reopenable = reopen1::Reopen::new(Box::new(|| {
1145 |     ///     OpenOptions::new()
1146 |     ///         .create(true)
1147 |     ///         .write(true)
1148 |     ///         .append(true)
1149 |     ///         .open("/tmp/output.log")
1150 |     /// }))
1151 |     /// .unwrap();
1152 |     ///
1153 |     /// fern::Dispatch::new().chain(fern::Output::reopen1(reopenable, "\n"))
1154 |     ///     # .into_log();
1155 |     /// #     Ok(())
1156 |     /// # }
1157 |     /// #
1158 |     /// # fn main() { setup_logger().expect("failed to set up logger"); }
1159 |     /// ```
1160 |     /// [`Dispatch::chain`]: struct.Dispatch.html#method.chain
1161 |     #[cfg(all(not(windows), feature = "reopen-1"))]
1162 |     pub fn reopen1<T: Into<Cow<'static, str>>>(
1163 |         reopen: reopen1::Reopen<fs::File>,
1164 |         line_sep: T,
1165 |     ) -> Self {
1166 |         Output(OutputInner::Reopen1 {
1167 |             stream: reopen,
1168 |             line_sep: line_sep.into(),
1169 |         })
1170 |     }
1171 | 
1172 |     /// Returns an stdout logger using a custom separator.
1173 |     ///
1174 |     /// If the default separator of `\n` is acceptable, an `io::Stdout`
1175 |     /// instance can be passed into `Dispatch::chain()` directly.
1176 |     ///
1177 |     /// ```
1178 |     /// fern::Dispatch::new().chain(std::io::stdout())
1179 |     ///     # .into_log();
1180 |     /// ```
1181 |     ///
1182 |     /// Example usage:
1183 |     ///
1184 |     /// ```
1185 |     /// fern::Dispatch::new()
1186 |     ///     // some unix tools use null bytes as message terminators so
1187 |     ///     // newlines in messages can be treated differently.
1188 |     ///     .chain(fern::Output::stdout("\0"))
1189 |     ///     # .into_log();
1190 |     /// ```
1191 |     pub fn stdout<T: Into<Cow<'static, str>>>(line_sep: T) -> Self {
1192 |         Output(OutputInner::Stdout {
1193 |             stream: io::stdout(),
1194 |             line_sep: line_sep.into(),
1195 |         })
1196 |     }
1197 | 
1198 |     /// Returns an stderr logger using a custom separator.
1199 |     ///
1200 |     /// If the default separator of `\n` is acceptable, an `io::Stderr`
1201 |     /// instance can be passed into `Dispatch::chain()` directly.
1202 |     ///
1203 |     /// ```
1204 |     /// fern::Dispatch::new().chain(std::io::stderr())
1205 |     ///     # .into_log();
1206 |     /// ```
1207 |     ///
1208 |     /// Example usage:
1209 |     ///
1210 |     /// ```
1211 |     /// fern::Dispatch::new().chain(fern::Output::stderr("\n\n\n"))
1212 |     ///     # .into_log();
1213 |     /// ```
1214 |     pub fn stderr<T: Into<Cow<'static, str>>>(line_sep: T) -> Self {
1215 |         Output(OutputInner::Stderr {
1216 |             stream: io::stderr(),
1217 |             line_sep: line_sep.into(),
1218 |         })
1219 |     }
1220 | 
1221 |     /// Returns a mpsc::Sender logger using a custom separator.
1222 |     ///
1223 |     /// If the default separator of `\n` is acceptable, an
1224 |     /// `mpsc::Sender<String>` instance can be passed into `Dispatch::
1225 |     /// chain()` directly.
1226 |     ///
1227 |     /// Each log message will be suffixed with the separator, then sent as a
1228 |     /// single String to the given sender.
1229 |     ///
1230 |     /// ```
1231 |     /// use std::sync::mpsc::channel;
1232 |     ///
1233 |     /// let (tx, rx) = channel();
1234 |     /// fern::Dispatch::new().chain(tx)
1235 |     ///     # .into_log();
1236 |     /// ```
1237 |     pub fn sender<T: Into<Cow<'static, str>>>(sender: Sender<String>, line_sep: T) -> Self {
1238 |         Output(OutputInner::Sender {
1239 |             stream: sender,
1240 |             line_sep: line_sep.into(),
1241 |         })
1242 |     }
1243 | 
1244 |     /// Returns a logger which logs into an RFC5424 syslog.
1245 |     ///
1246 |     /// This method takes an additional transform method to turn the log data
1247 |     /// into RFC5424 data.
1248 |     ///
1249 |     /// I've honestly got no clue what the expected keys and values are for
1250 |     /// this kind of logging, so I'm just going to link [the rfc] instead.
1251 |     ///
1252 |     /// If you're an expert on syslog logging and would like to contribute
1253 |     /// an example to put here, it would be gladly accepted!
1254 |     ///
1255 |     /// This requires the `"syslog-4"` feature.
1256 |     ///
1257 |     /// [the rfc]: https://tools.ietf.org/html/rfc5424
1258 |     #[cfg(all(not(windows), feature = "syslog-4"))]
1259 |     pub fn syslog_5424<F>(logger: Syslog4Rfc5424Logger, transform: F) -> Self
1260 |     where
1261 |         F: Fn(&log::Record) -> (i32, HashMap<String, HashMap<String, String>>, String)
1262 |             + Sync
1263 |             + Send
1264 |             + 'static,
1265 |     {
1266 |         Output(OutputInner::Syslog4Rfc5424 {
1267 |             logger,
1268 |             transform: Box::new(transform),
1269 |         })
1270 |     }
1271 | 
1272 |     /// Returns a logger which logs into an RFC5424 syslog (using syslog version 6)
1273 |     ///
1274 |     /// This method takes an additional transform method to turn the log data
1275 |     /// into RFC5424 data.
1276 |     ///
1277 |     /// I've honestly got no clue what the expected keys and values are for
1278 |     /// this kind of logging, so I'm just going to link [the rfc] instead.
1279 |     ///
1280 |     /// If you're an expert on syslog logging and would like to contribute
1281 |     /// an example to put here, it would be gladly accepted!
1282 |     ///
1283 |     /// This requires the `"syslog-6"` feature.
1284 |     ///
1285 |     /// [the rfc]: https://tools.ietf.org/html/rfc5424
1286 |     #[cfg(all(not(windows), feature = "syslog-6"))]
1287 |     pub fn syslog6_5424<F>(logger: Syslog6Rfc5424Logger, transform: F) -> Self
1288 |     where
1289 |         F: Fn(&log::Record) -> (u32, HashMap<String, HashMap<String, String>>, String)
1290 |             + Sync
1291 |             + Send
1292 |             + 'static,
1293 |     {
1294 |         Output(OutputInner::Syslog6Rfc5424 {
1295 |             logger,
1296 |             transform: Box::new(transform),
1297 |         })
1298 |     }
1299 | 
1300 |     /// Returns a logger which logs into an RFC5424 syslog (using syslog version 6)
1301 |     ///
1302 |     /// This method takes an additional transform method to turn the log data
1303 |     /// into RFC5424 data.
1304 |     ///
1305 |     /// I've honestly got no clue what the expected keys and values are for
1306 |     /// this kind of logging, so I'm just going to link [the rfc] instead.
1307 |     ///
1308 |     /// If you're an expert on syslog logging and would like to contribute
1309 |     /// an example to put here, it would be gladly accepted!
1310 |     ///
1311 |     /// This requires the `"syslog-7"` feature.
1312 |     ///
1313 |     /// [the rfc]: https://tools.ietf.org/html/rfc5424
1314 |     #[cfg(all(not(windows), feature = "syslog-7"))]
1315 |     pub fn syslog7_5424<F>(logger: Syslog7Rfc5424Logger, transform: F) -> Self
1316 |     where
1317 |         F: Fn(&log::Record) -> (u32, BTreeMap<String, BTreeMap<String, String>>, String)
1318 |             + Sync
1319 |             + Send
1320 |             + 'static,
1321 |     {
1322 |         Output(OutputInner::Syslog7Rfc5424 {
1323 |             logger,
1324 |             transform: Box::new(transform),
1325 |         })
1326 |     }
1327 | 
1328 |     /// Returns a logger which simply calls the given function with each
1329 |     /// message.
1330 |     ///
1331 |     /// The function will be called inline in the thread the log occurs on.
1332 |     ///
1333 |     /// Example usage:
1334 |     ///
1335 |     /// ```
1336 |     /// fern::Dispatch::new().chain(fern::Output::call(|record| {
1337 |     ///     // this is mundane, but you can do anything here.
1338 |     ///     println!("{}", record.args());
1339 |     /// }))
1340 |     ///     # .into_log();
1341 |     /// ```
1342 |     pub fn call<F>(func: F) -> Self
1343 |     where
1344 |         F: Fn(&log::Record) + Sync + Send + 'static,
1345 |     {
1346 |         struct CallShim<F>(F);
1347 |         impl<F> log::Log for CallShim<F>
1348 |         where
1349 |             F: Fn(&log::Record) + Sync + Send + 'static,
1350 |         {
1351 |             fn enabled(&self, _: &log::Metadata) -> bool {
1352 |                 true
1353 |             }
1354 |             fn log(&self, record: &log::Record) {
1355 |                 (self.0)(record)
1356 |             }
1357 |             fn flush(&self) {}
1358 |         }
1359 | 
1360 |         Self::from(Box::new(CallShim(func)) as Box<dyn log::Log>)
1361 |     }
1362 | }
1363 | 
1364 | impl Default for Dispatch {
1365 |     /// Returns a logger configuration that does nothing with log records.
1366 |     ///
1367 |     /// Equivalent to [`Dispatch::new`].
1368 |     ///
1369 |     /// [`Dispatch::new`]: #method.new
1370 |     fn default() -> Self {
1371 |         Self::new()
1372 |     }
1373 | }
1374 | 
1375 | impl fmt::Debug for Dispatch {
1376 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
1377 |         struct LevelsDebug<'a>(&'a [(Cow<'static, str>, log::LevelFilter)]);
1378 |         impl fmt::Debug for LevelsDebug<'_> {
1379 |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
1380 |                 f.debug_map()
1381 |                     .entries(self.0.iter().map(|t| (t.0.as_ref(), t.1)))
1382 |                     .finish()
1383 |             }
1384 |         }
1385 |         struct FiltersDebug<'a>(&'a [Box<Filter>]);
1386 |         impl fmt::Debug for FiltersDebug<'_> {
1387 |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
1388 |                 f.debug_list()
1389 |                     .entries(self.0.iter().map(|_| "<filter closure>"))
1390 |                     .finish()
1391 |             }
1392 |         }
1393 |         f.debug_struct("Dispatch")
1394 |             .field(
1395 |                 "format",
1396 |                 &self.format.as_ref().map(|_| "<formatter closure>"),
1397 |             )
1398 |             .field("children", &self.children)
1399 |             .field("default_level", &self.default_level)
1400 |             .field("levels", &LevelsDebug(&self.levels))
1401 |             .field("filters", &FiltersDebug(&self.filters))
1402 |             .finish()
1403 |     }
1404 | }
1405 | 
1406 | impl fmt::Debug for OutputInner {
1407 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
1408 |         match *self {
1409 |             OutputInner::Stdout {
1410 |                 ref stream,
1411 |                 ref line_sep,
1412 |             } => f
1413 |                 .debug_struct("Output::Stdout")
1414 |                 .field("stream", stream)
1415 |                 .field("line_sep", line_sep)
1416 |                 .finish(),
1417 |             OutputInner::Stderr {
1418 |                 ref stream,
1419 |                 ref line_sep,
1420 |             } => f
1421 |                 .debug_struct("Output::Stderr")
1422 |                 .field("stream", stream)
1423 |                 .field("line_sep", line_sep)
1424 |                 .finish(),
1425 |             OutputInner::File {
1426 |                 ref stream,
1427 |                 ref line_sep,
1428 |             } => f
1429 |                 .debug_struct("Output::File")
1430 |                 .field("stream", stream)
1431 |                 .field("line_sep", line_sep)
1432 |                 .finish(),
1433 |             OutputInner::Writer { ref line_sep, .. } => f
1434 |                 .debug_struct("Output::Writer")
1435 |                 .field("stream", &"<unknown writer>")
1436 |                 .field("line_sep", line_sep)
1437 |                 .finish(),
1438 |             #[cfg(all(not(windows), feature = "reopen-03"))]
1439 |             OutputInner::Reopen { ref line_sep, .. } => f
1440 |                 .debug_struct("Output::Reopen")
1441 |                 .field("stream", &"<unknown reopen file>")
1442 |                 .field("line_sep", line_sep)
1443 |                 .finish(),
1444 |             #[cfg(all(not(windows), feature = "reopen-1"))]
1445 |             OutputInner::Reopen1 {
1446 |                 ref line_sep,
1447 |                 ref stream,
1448 |             } => f
1449 |                 .debug_struct("Output::Reopen1")
1450 |                 .field("stream", stream)
1451 |                 .field("line_sep", line_sep)
1452 |                 .finish(),
1453 |             OutputInner::Sender {
1454 |                 ref stream,
1455 |                 ref line_sep,
1456 |             } => f
1457 |                 .debug_struct("Output::Sender")
1458 |                 .field("stream", stream)
1459 |                 .field("line_sep", line_sep)
1460 |                 .finish(),
1461 |             #[cfg(all(not(windows), feature = "syslog-3"))]
1462 |             OutputInner::Syslog3(_) => f
1463 |                 .debug_tuple("Output::Syslog3")
1464 |                 .field(&"<unprintable syslog::Logger>")
1465 |                 .finish(),
1466 |             #[cfg(all(not(windows), feature = "syslog-4"))]
1467 |             OutputInner::Syslog4Rfc3164 { .. } => f
1468 |                 .debug_tuple("Output::Syslog4Rfc3164")
1469 |                 .field(&"<unprintable syslog::Logger>")
1470 |                 .finish(),
1471 |             #[cfg(all(not(windows), feature = "syslog-4"))]
1472 |             OutputInner::Syslog4Rfc5424 { .. } => f
1473 |                 .debug_tuple("Output::Syslog4Rfc5424")
1474 |                 .field(&"<unprintable syslog::Logger>")
1475 |                 .finish(),
1476 |             #[cfg(all(not(windows), feature = "syslog-6"))]
1477 |             OutputInner::Syslog6Rfc3164 { .. } => f
1478 |                 .debug_tuple("Output::Syslog6Rfc3164")
1479 |                 .field(&"<unprintable syslog::Logger>")
1480 |                 .finish(),
1481 |             #[cfg(all(not(windows), feature = "syslog-6"))]
1482 |             OutputInner::Syslog6Rfc5424 { .. } => f
1483 |                 .debug_tuple("Output::Syslog6Rfc5424")
1484 |                 .field(&"<unprintable syslog::Logger>")
1485 |                 .finish(),
1486 |             #[cfg(all(not(windows), feature = "syslog-7"))]
1487 |             OutputInner::Syslog7Rfc3164 { .. } => f
1488 |                 .debug_tuple("Output::Syslog7Rfc3164")
1489 |                 .field(&"<unprintable syslog::Logger>")
1490 |                 .finish(),
1491 |             #[cfg(all(not(windows), feature = "syslog-7"))]
1492 |             OutputInner::Syslog7Rfc5424 { .. } => f
1493 |                 .debug_tuple("Output::Syslog7Rfc5424")
1494 |                 .field(&"<unprintable syslog::Logger>")
1495 |                 .finish(),
1496 |             OutputInner::Dispatch(ref dispatch) => {
1497 |                 f.debug_tuple("Output::Dispatch").field(dispatch).finish()
1498 |             }
1499 |             OutputInner::SharedDispatch(_) => f
1500 |                 .debug_tuple("Output::SharedDispatch")
1501 |                 .field(&"<built Dispatch logger>")
1502 |                 .finish(),
1503 |             OutputInner::OtherBoxed { .. } => f
1504 |                 .debug_tuple("Output::OtherBoxed")
1505 |                 .field(&"<boxed logger>")
1506 |                 .finish(),
1507 |             OutputInner::OtherStatic { .. } => f
1508 |                 .debug_tuple("Output::OtherStatic")
1509 |                 .field(&"<boxed logger>")
1510 |                 .finish(),
1511 |             OutputInner::Panic => f.debug_tuple("Output::Panic").finish(),
1512 |             #[cfg(feature = "date-based")]
1513 |             OutputInner::DateBased { ref config } => f
1514 |                 .debug_struct("Output::DateBased")
1515 |                 .field("config", config)
1516 |                 .finish(),
1517 |         }
1518 |     }
1519 | }
1520 | 
1521 | impl fmt::Debug for Output {
1522 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
1523 |         self.0.fmt(f)
1524 |     }
1525 | }
1526 | 
1527 | /// This is used to generate log file suffixed based on date, hour, and minute.
1528 | ///
1529 | /// The log file will be rotated automatically when the date changes.
1530 | #[derive(Debug)]
1531 | #[cfg(feature = "date-based")]
1532 | pub struct DateBased {
1533 |     file_prefix: PathBuf,
1534 |     file_suffix: Cow<'static, str>,
1535 |     line_sep: Cow<'static, str>,
1536 |     utc_time: bool,
1537 | }
1538 | 
1539 | #[cfg(feature = "date-based")]
1540 | impl DateBased {
1541 |     /// Create new date-based file logger with the given file prefix and
1542 |     /// strftime-based suffix pattern.
1543 |     ///
1544 |     /// On initialization, fern will create a file with the suffix formatted
1545 |     /// with the current time (either utc or local, see below). Each time a
1546 |     /// record is logged, the format is checked against the current time, and if
1547 |     /// the time has changed, the old file is closed and a new one opened.
1548 |     ///
1549 |     /// `file_suffix` will be interpreted as an `strftime` format. See
1550 |     /// [`chrono::format::strftime`] for more information.
1551 |     ///
1552 |     /// `file_prefix` may be a full file path, and will be prepended to the
1553 |     /// suffix to create the final file.
1554 |     ///
1555 |     /// Note that no separator will be placed in between `file_name` and
1556 |     /// `file_suffix_pattern`. So if you call `DateBased::new("hello",
1557 |     /// "%Y")`, the result will be a filepath `hello2019`.
1558 |     ///
1559 |     /// By default, this will use local time. For UTC time instead, use the
1560 |     /// [`.utc_time()`][DateBased::utc_time] method after creating.
1561 |     ///
1562 |     /// By default, this will use `\n` as a line separator. For a custom
1563 |     /// separator, use the [`.line_sep`][DateBased::line_sep] method
1564 |     /// after creating.
1565 |     ///
1566 |     /// # Examples
1567 |     ///
1568 |     /// Containing the date (year, month and day):
1569 |     ///
1570 |     /// ```
1571 |     /// // logs/2019-10-23-my-program.log
1572 |     /// let log = fern::DateBased::new("logs/", "%Y-%m-%d-my-program.log");
1573 |     ///
1574 |     /// // program.log.23102019
1575 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y");
1576 |     /// ```
1577 |     ///
1578 |     /// Containing the hour:
1579 |     ///
1580 |     /// ```
1581 |     /// // logs/2019-10-23 13 my-program.log
1582 |     /// let log = fern::DateBased::new("logs/", "%Y-%m-%d %H my-program.log");
1583 |     ///
1584 |     /// // program.log.2310201913
1585 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y%H");
1586 |     /// ```
1587 |     ///
1588 |     /// Containing the minute:
1589 |     ///
1590 |     /// ```
1591 |     /// // logs/2019-10-23 13 my-program.log
1592 |     /// let log = fern::DateBased::new("logs/", "%Y-%m-%d %H my-program.log");
1593 |     ///
1594 |     /// // program.log.2310201913
1595 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y%H");
1596 |     /// ```
1597 |     ///
1598 |     /// UNIX time, or seconds since 00:00 Jan 1st 1970:
1599 |     ///
1600 |     /// ```
1601 |     /// // logs/1571822854-my-program.log
1602 |     /// let log = fern::DateBased::new("logs/", "%s-my-program.log");
1603 |     ///
1604 |     /// // program.log.1571822854
1605 |     /// let log = fern::DateBased::new("my-program.log.", "%s");
1606 |     /// ```
1607 |     ///
1608 |     /// Hourly, using UTC time:
1609 |     ///
1610 |     /// ```
1611 |     /// // logs/2019-10-23 23 my-program.log
1612 |     /// let log = fern::DateBased::new("logs/", "%Y-%m-%d %H my-program.log").utc_time();
1613 |     ///
1614 |     /// // program.log.2310201923
1615 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y%H").utc_time();
1616 |     /// ```
1617 |     ///
1618 |     /// [`chrono::format::strftime`]: https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html
1619 |     pub fn new<T, U>(file_prefix: T, file_suffix: U) -> Self
1620 |     where
1621 |         T: AsRef<Path>,
1622 |         U: Into<Cow<'static, str>>,
1623 |     {
1624 |         DateBased {
1625 |             utc_time: false,
1626 |             file_prefix: file_prefix.as_ref().to_owned(),
1627 |             file_suffix: file_suffix.into(),
1628 |             line_sep: "\n".into(),
1629 |         }
1630 |     }
1631 | 
1632 |     /// Changes the line separator this logger will use.
1633 |     ///
1634 |     /// The default line separator is `\n`.
1635 |     ///
1636 |     /// # Examples
1637 |     ///
1638 |     /// Using a windows line separator:
1639 |     ///
1640 |     /// ```
1641 |     /// let log = fern::DateBased::new("logs", "%s.log").line_sep("\r\n");
1642 |     /// ```
1643 |     pub fn line_sep<T>(mut self, line_sep: T) -> Self
1644 |     where
1645 |         T: Into<Cow<'static, str>>,
1646 |     {
1647 |         self.line_sep = line_sep.into();
1648 |         self
1649 |     }
1650 | 
1651 |     /// Orients this log file suffix formatting to use UTC time.
1652 |     ///
1653 |     /// The default is local time.
1654 |     ///
1655 |     /// # Examples
1656 |     ///
1657 |     /// This will use UTC time to determine the date:
1658 |     ///
1659 |     /// ```
1660 |     /// // program.log.2310201923
1661 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y%H").utc_time();
1662 |     /// ```
1663 |     pub fn utc_time(mut self) -> Self {
1664 |         self.utc_time = true;
1665 |         self
1666 |     }
1667 | 
1668 |     /// Orients this log file suffix formatting to use local time.
1669 |     ///
1670 |     /// This is the default option.
1671 |     ///
1672 |     /// # Examples
1673 |     ///
1674 |     /// This log file will use local time - the latter method call overrides the
1675 |     /// former.
1676 |     ///
1677 |     /// ```
1678 |     /// // program.log.2310201923
1679 |     /// let log = fern::DateBased::new("my-program.log.", "%d%m%Y%H")
1680 |     ///     .utc_time()
1681 |     ///     .local_time();
1682 |     /// ```
1683 |     pub fn local_time(mut self) -> Self {
1684 |         self.utc_time = false;
1685 |         self
1686 |     }
1687 | }
1688 | 
1689 | #[cfg(feature = "date-based")]
1690 | impl From<DateBased> for Output {
1691 |     /// Create an output logger which defers to the given date-based logger. Use
1692 |     /// configuration methods on [DateBased] to set line separator and filename.
1693 |     fn from(config: DateBased) -> Self {
1694 |         Output(OutputInner::DateBased { config })
1695 |     }
1696 | }
1697 | 


--------------------------------------------------------------------------------
/src/colors.rs:
--------------------------------------------------------------------------------
  1 | //! Support for ANSI terminal colors via the colored crate.
  2 | //!
  3 | //! To enable support for colors, add the `"colored"` feature in your
  4 | //! `Cargo.toml`:
  5 | //!
  6 | //! ```toml
  7 | //! [dependencies]
  8 | //! fern = { version = "0.7", features = ["colored"] }
  9 | //! ```
 10 | //!
 11 | //! ---
 12 | //!
 13 | //! Colors are mainly supported via coloring the log level itself, but it's
 14 | //! also possible to color each entire log line based off of the log level.
 15 | //!
 16 | //! First, here's an example which colors the log levels themselves ("INFO" /
 17 | //! "WARN" text will have color, but won't color the rest of the line).
 18 | //! [`ColoredLevelConfig`] lets us configure the colors per-level, but also has
 19 | //! sane defaults.
 20 | //!
 21 | //! ```
 22 | //! use fern::colors::{Color, ColoredLevelConfig};
 23 | //!
 24 | //! let mut colors = ColoredLevelConfig::new()
 25 | //!     // use builder methods
 26 | //!     .info(Color::Green);
 27 | //! // or access raw fields
 28 | //! colors.warn = Color::Magenta;
 29 | //! ```
 30 | //!
 31 | //! It can then be used within any regular fern formatting closure:
 32 | //!
 33 | //! ```
 34 | //! # let colors = fern::colors::ColoredLevelConfig::new();
 35 | //! fern::Dispatch::new()
 36 | //!     // ...
 37 | //!     .format(move |out, message, record| {
 38 | //!         out.finish(format_args!(
 39 | //!             "[{}] {}",
 40 | //!             // just use 'colors.color(..)' instead of the level
 41 | //!             // itself to insert ANSI colors.
 42 | //!             colors.color(record.level()),
 43 | //!             message,
 44 | //!         ))
 45 | //!     })
 46 | //!     # .into_log();
 47 | //! ```
 48 | //!
 49 | //! ---
 50 | //!
 51 | //! Coloring levels is nice, but the alternative is good too. For an example of an
 52 | //! application coloring each entire log line with the right color, see
 53 | //! [examples/pretty-colored.rs][ex].
 54 | //!
 55 | //! [`ColoredLevelConfig`]: struct.ColoredLevelConfig.html
 56 | //! [ex]: https://github.com/daboross/fern/blob/fern-0.7.0/examples/pretty-colored.rs
 57 | use std::fmt;
 58 | 
 59 | pub use colored::Color;
 60 | use log::Level;
 61 | 
 62 | /// Extension crate allowing the use of `.colored` on Levels.
 63 | trait ColoredLogLevel {
 64 |     /// Colors this log level with the given color.
 65 |     fn colored(&self, color: Color) -> WithFgColor<Level>;
 66 | }
 67 | 
 68 | /// Opaque structure which represents some text data and a color to display it
 69 | /// with.
 70 | ///
 71 | /// This implements [`fmt::Display`] to displaying the inner text (usually a
 72 | /// log level) with ANSI color markers before to set the color and after to
 73 | /// reset the color.
 74 | ///
 75 | /// `WithFgColor` instances can be created and displayed without any allocation.
 76 | // this is necessary in order to avoid using colored::ColorString, which has a
 77 | // Display implementation involving many allocations, and would involve two
 78 | // more string allocations even to create it.
 79 | //
 80 | // [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
 81 | pub struct WithFgColor<T>
 82 | where
 83 |     T: fmt::Display,
 84 | {
 85 |     text: T,
 86 |     color: Color,
 87 | }
 88 | 
 89 | impl<T> fmt::Display for WithFgColor<T>
 90 | where
 91 |     T: fmt::Display,
 92 | {
 93 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 94 |         write!(f, "\x1B[{}m", self.color.to_fg_str())?;
 95 |         fmt::Display::fmt(&self.text, f)?;
 96 |         write!(f, "\x1B[0m")?;
 97 |         Ok(())
 98 |     }
 99 | }
100 | 
101 | /// Configuration specifying colors a log level can be colored as.
102 | ///
103 | /// Example usage setting custom 'info' and 'debug' colors:
104 | ///
105 | /// ```
106 | /// use fern::colors::{Color, ColoredLevelConfig};
107 | ///
108 | /// let colors = ColoredLevelConfig::new()
109 | ///     .info(Color::Green)
110 | ///     .debug(Color::Magenta);
111 | ///
112 | /// fern::Dispatch::new()
113 | ///     .format(move |out, message, record| {
114 | ///         out.finish(format_args!(
115 | ///             "[{}] {}",
116 | ///             colors.color(record.level()),
117 | ///             message
118 | ///         ))
119 | ///     })
120 | ///     .chain(std::io::stdout())
121 | /// # /*
122 | ///     .apply()?;
123 | /// # */
124 | /// #   .into_log();
125 | /// ```
126 | #[derive(Copy, Clone)]
127 | #[must_use = "builder methods take config by value and thus must be reassigned to variable"]
128 | pub struct ColoredLevelConfig {
129 |     /// The color to color logs with the [`Error`] level.
130 |     ///
131 |     /// [`Error`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Error
132 |     pub error: Color,
133 |     /// The color to color logs with the [`Warn`] level.
134 |     ///
135 |     /// [`Warn`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Warn
136 |     pub warn: Color,
137 |     /// The color to color logs with the [`Info`] level.
138 |     ///
139 |     /// [`Info`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Info
140 |     pub info: Color,
141 |     /// The color to color logs with the [`Debug`] level.
142 |     ///
143 |     /// [`Debug`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Debug
144 |     pub debug: Color,
145 |     /// The color to color logs with the [`Trace`] level.
146 |     ///
147 |     /// [`Trace`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Trace
148 |     pub trace: Color,
149 | }
150 | 
151 | impl ColoredLevelConfig {
152 |     /// Creates a new ColoredLevelConfig with the default colors.
153 |     ///
154 |     /// This matches the behavior of [`ColoredLevelConfig::default`].
155 |     ///
156 |     /// [`ColoredLevelConfig::default`]: #method.default
157 |     #[inline]
158 |     pub fn new() -> Self {
159 |         #[cfg(windows)]
160 |         {
161 |             let _ = colored::control::set_virtual_terminal(true);
162 |         }
163 |         Self::default()
164 |     }
165 | 
166 |     /// Overrides the [`Error`] level color with the given color.
167 |     ///
168 |     /// The default color is [`Color::Red`].
169 |     ///
170 |     /// [`Error`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Error
171 |     /// [`Color::Red`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.Red
172 |     pub fn error(mut self, error: Color) -> Self {
173 |         self.error = error;
174 |         self
175 |     }
176 | 
177 |     /// Overrides the [`Warn`] level color with the given color.
178 |     ///
179 |     /// The default color is [`Color::Yellow`].
180 |     ///
181 |     /// [`Warn`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Warn
182 |     /// [`Color::Yellow`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.Yellow
183 |     pub fn warn(mut self, warn: Color) -> Self {
184 |         self.warn = warn;
185 |         self
186 |     }
187 | 
188 |     /// Overrides the [`Info`] level color with the given color.
189 |     ///
190 |     /// The default color is [`Color::White`].
191 |     ///
192 |     /// [`Info`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Info
193 |     /// [`Color::White`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.White
194 |     pub fn info(mut self, info: Color) -> Self {
195 |         self.info = info;
196 |         self
197 |     }
198 | 
199 |     /// Overrides the [`Debug`] level color with the given color.
200 |     ///
201 |     /// The default color is [`Color::White`].
202 |     ///
203 |     /// [`Debug`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Debug
204 |     /// [`Color::White`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.White
205 |     pub fn debug(mut self, debug: Color) -> Self {
206 |         self.debug = debug;
207 |         self
208 |     }
209 | 
210 |     /// Overrides the [`Trace`] level color with the given color.
211 |     ///
212 |     /// The default color is [`Color::White`].
213 |     ///
214 |     /// [`Trace`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Trace
215 |     /// [`Color::White`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.White
216 |     pub fn trace(mut self, trace: Color) -> Self {
217 |         self.trace = trace;
218 |         self
219 |     }
220 | 
221 |     /// Colors the given log level with the color in this configuration
222 |     /// corresponding to it's level.
223 |     ///
224 |     /// The structure returned is opaque, but will print the Level surrounded
225 |     /// by ANSI color codes when displayed. This will work correctly for
226 |     /// UNIX terminals, but due to a lack of support from the [`colored`]
227 |     /// crate, this will not function in Windows.
228 |     ///
229 |     /// [`colored`]: https://github.com/mackwic/colored
230 |     pub fn color(&self, level: Level) -> WithFgColor<Level> {
231 |         level.colored(self.get_color(&level))
232 |     }
233 | 
234 |     /// Retrieves the color that a log level should be colored as.
235 |     pub fn get_color(&self, level: &Level) -> Color {
236 |         match *level {
237 |             Level::Error => self.error,
238 |             Level::Warn => self.warn,
239 |             Level::Info => self.info,
240 |             Level::Debug => self.debug,
241 |             Level::Trace => self.trace,
242 |         }
243 |     }
244 | }
245 | 
246 | impl Default for ColoredLevelConfig {
247 |     /// Retrieves the default configuration. This has:
248 |     ///
249 |     /// - [`Error`] as [`Color::Red`]
250 |     /// - [`Warn`] as [`Color::Yellow`]
251 |     /// - [`Info`] as [`Color::White`]
252 |     /// - [`Debug`] as [`Color::White`]
253 |     /// - [`Trace`] as [`Color::White`]
254 |     ///
255 |     /// [`Error`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Error
256 |     /// [`Warn`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Warn
257 |     /// [`Info`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Info
258 |     /// [`Debug`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Debug
259 |     /// [`Trace`]: https://docs.rs/log/0.4/log/enum.Level.html#variant.Trace
260 |     /// [`Color::White`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.White
261 |     /// [`Color::Yellow`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.Yellow
262 |     /// [`Color::Red`]: https://docs.rs/colored/1/colored/enum.Color.html#variant.Red
263 |     fn default() -> Self {
264 |         ColoredLevelConfig {
265 |             error: Color::Red,
266 |             warn: Color::Yellow,
267 |             debug: Color::White,
268 |             info: Color::White,
269 |             trace: Color::White,
270 |         }
271 |     }
272 | }
273 | 
274 | impl ColoredLogLevel for Level {
275 |     fn colored(&self, color: Color) -> WithFgColor<Level> {
276 |         WithFgColor { text: *self, color }
277 |     }
278 | }
279 | 
280 | #[cfg(test)]
281 | #[cfg(not(windows))]
282 | mod test {
283 |     use colored::{Color::*, Colorize};
284 | 
285 |     use super::WithFgColor;
286 | 
287 |     #[test]
288 |     fn fg_color_matches_colored_behavior() {
289 |         for &color in &[
290 |             Black,
291 |             Red,
292 |             Green,
293 |             Yellow,
294 |             Blue,
295 |             Magenta,
296 |             Cyan,
297 |             White,
298 |             BrightBlack,
299 |             BrightRed,
300 |             BrightGreen,
301 |             BrightYellow,
302 |             BrightBlue,
303 |             BrightMagenta,
304 |             BrightCyan,
305 |             BrightWhite,
306 |         ] {
307 |             colored::control::SHOULD_COLORIZE.set_override(true);
308 |             assert_eq!(
309 |                 format!("{}", "test".color(color)),
310 |                 format!(
311 |                     "{}",
312 |                     WithFgColor {
313 |                         text: "test",
314 |                         color,
315 |                     }
316 |                 )
317 |             );
318 |         }
319 |     }
320 | 
321 |     #[test]
322 |     fn fg_color_respects_formatting_flags() {
323 |         let s = format!(
324 |             "{:^8}",
325 |             WithFgColor {
326 |                 text: "test",
327 |                 color: Yellow,
328 |             }
329 |         );
330 |         assert!(s.contains("  test  "));
331 |         assert!(!s.contains("   test  "));
332 |         assert!(!s.contains("  test   "));
333 |     }
334 | }
335 | 


--------------------------------------------------------------------------------
/src/errors.rs:
--------------------------------------------------------------------------------
 1 | use std::{error, fmt, io};
 2 | 
 3 | /// Convenience error combining possible errors which could occur while
 4 | /// initializing logging.
 5 | ///
 6 | /// Fern does not use this error natively, but functions which set up fern and
 7 | /// open log files will often need to return both [`io::Error`] and
 8 | /// [`SetLoggerError`]. This error is for that purpose.
 9 | ///
10 | /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
11 | /// [`SetLoggerError`]: ../log/struct.SetLoggerError.html
12 | #[derive(Debug)]
13 | pub enum InitError {
14 |     /// IO error.
15 |     Io(io::Error),
16 |     /// The log crate's global logger was already initialized when trying to
17 |     /// initialize a logger.
18 |     SetLoggerError(log::SetLoggerError),
19 | }
20 | 
21 | impl From<io::Error> for InitError {
22 |     fn from(error: io::Error) -> InitError {
23 |         InitError::Io(error)
24 |     }
25 | }
26 | 
27 | impl From<log::SetLoggerError> for InitError {
28 |     fn from(error: log::SetLoggerError) -> InitError {
29 |         InitError::SetLoggerError(error)
30 |     }
31 | }
32 | 
33 | impl fmt::Display for InitError {
34 |     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
35 |         match *self {
36 |             InitError::Io(ref e) => write!(f, "IO Error initializing logger: {}", e),
37 |             InitError::SetLoggerError(ref e) => write!(f, "logging initialization failed: {}", e),
38 |         }
39 |     }
40 | }
41 | 
42 | impl error::Error for InitError {
43 |     fn description(&self) -> &str {
44 |         match *self {
45 |             InitError::Io(..) => "IO error while initializing logging",
46 |             InitError::SetLoggerError(..) => {
47 |                 "logging system already initialized with different logger"
48 |             }
49 |         }
50 |     }
51 | 
52 |     fn cause(&self) -> Option<&dyn error::Error> {
53 |         match *self {
54 |             InitError::Io(ref e) => Some(e),
55 |             InitError::SetLoggerError(ref e) => Some(e),
56 |         }
57 |     }
58 | }
59 | 


--------------------------------------------------------------------------------
/src/lib.rs:
--------------------------------------------------------------------------------
  1 | #![deny(missing_docs)]
  2 | #![doc(html_root_url = "https://docs.rs/fern/0.7.1")]
  3 | //! Efficient, configurable logging in Rust.
  4 | //!
  5 | //! # fern 0.4.4, 0.5.\*, 0.6.\* security warning - `colored` feature + global allocator
  6 | //!
  7 | //! One of our downstream dependencies, [atty](https://docs.rs/atty/), through
  8 | //! [colored](https://docs.rs/colored/), has an unsoundness issue:
  9 | //! <https://rustsec.org/advisories/RUSTSEC-2021-0145.html>.
 10 | //!
 11 | //! This shows up in one situation: if you're using `colored` 0.1.0 (the crate, or our
 12 | //! feature), and a custom global allocator.
 13 | //!
 14 | //! Upgrade to `fern` 0.7.0, and `colored` 0.2.0 if you depend on it directly, to fix this issue.
 15 | //!
 16 | //! # Depending on fern
 17 | //!
 18 | //! Ensure you require both fern and log in your project's `Cargo.toml`:
 19 | //!
 20 | //! ```toml
 21 | //! [dependencies]
 22 | //! log = "0.4"
 23 | //! fern = "0.7"
 24 | //! ```
 25 | //!
 26 | //! # Example setup
 27 | //!
 28 | //! With fern, all logger configuration is done via builder-like methods on
 29 | //! instances of the [`Dispatch`] structure.
 30 | //!
 31 | //! Here's an example logger which formats messages, and sends everything Debug
 32 | //! and above to both stdout and an output.log file:
 33 | //!
 34 | //! ```no_run
 35 | //! use log::{debug, error, info, trace, warn};
 36 | //! use std::time::SystemTime;
 37 | //!
 38 | //! fn setup_logger() -> Result<(), fern::InitError> {
 39 | //!     fern::Dispatch::new()
 40 | //!         .format(|out, message, record| {
 41 | //!             out.finish(format_args!(
 42 | //!                 "[{} {} {}] {}",
 43 | //!                 humantime::format_rfc3339_seconds(SystemTime::now()),
 44 | //!                 record.level(),
 45 | //!                 record.target(),
 46 | //!                 message
 47 | //!             ))
 48 | //!         })
 49 | //!         .level(log::LevelFilter::Debug)
 50 | //!         .chain(std::io::stdout())
 51 | //!         .chain(fern::log_file("output.log")?)
 52 | //!         .apply()?;
 53 | //!     Ok(())
 54 | //! }
 55 | //!
 56 | //! fn main() -> Result<(), Box<dyn std::error::Error>> {
 57 | //!     setup_logger()?;
 58 | //!
 59 | //!     info!("Hello, world!");
 60 | //!     warn!("Warning!");
 61 | //!     debug!("Now exiting.");
 62 | //!
 63 | //!     Ok(())
 64 | //! }
 65 | //! ```
 66 | //!
 67 | //! Let's unwrap this:
 68 | //!
 69 | //!
 70 | //! ```
 71 | //! fern::Dispatch::new()
 72 | //! # ;
 73 | //! ```
 74 | //!
 75 | //! [`Dispatch::new`] creates an empty configuration.
 76 | //!
 77 | //! ```
 78 | //! # fern::Dispatch::new()
 79 | //! .format(|out, message, record| {
 80 | //!     out.finish(format_args!(
 81 | //!         "..."
 82 | //!     ))
 83 | //! })
 84 | //! # ;
 85 | //! ```
 86 | //!
 87 | //! This incantation sets the `Dispatch` format! The closure taking in
 88 | //! `out, message, record` will be called once for each message going through
 89 | //! the dispatch, and the formatted log message will be used for any downstream
 90 | //! consumers.
 91 | //!
 92 | //! Do any work you want in this closure, and then call `out.finish` at the end.
 93 | //! The callback-style result passing with `out.finish(format_args!())` lets us
 94 | //! format without any intermediate string allocation.
 95 | //!
 96 | //! [`format_args!`] has the same format as [`println!`], just returning a
 97 | //! not-yet-written result we can use internally.
 98 | //!
 99 | //! ```
100 | //! std::time::SystemTime::now()
101 | //! # ;
102 | //! ```
103 | //!
104 | //! [`std::time::SystemTime::now`] retrieves the current time.
105 | //!
106 | //! ```
107 | //! humantime::format_rfc3339_seconds(
108 | //!     // ...
109 | //!     # std::time::SystemTime::now()
110 | //! )
111 | //! # ;
112 | //! ```
113 | //!
114 | //! [`humantime::format_rfc3339_seconds`] formats the current time into an
115 | //! RFC3339 timestamp, with second-precision.
116 | //!
117 | //! RFC3339 looks like `2018-02-14T00:28:07Z`, always using UTC, ignoring system
118 | //! timezone.
119 | //!
120 | //! `humantime` is a nice light dependency, but only offers this one format.
121 | //! For more custom time formatting, I recommend
122 | //! [`jiff`](https://docs.rs/jiff/).
123 | //!
124 | //! Now, back to the [`Dispatch`] methods:
125 | //!
126 | //! ```
127 | //! # fern::Dispatch::new()
128 | //! .level(log::LevelFilter::Debug)
129 | //! # ;
130 | //! ```
131 | //!
132 | //! Sets the minimum logging level for all modules, if not overwritten with
133 | //! [`Dispatch::level_for`], to [`Level::Debug`][log::Level::Debug].
134 | //!
135 | //! ```
136 | //! # fern::Dispatch::new()
137 | //! .chain(std::io::stdout())
138 | //! # ;
139 | //! ```
140 | //!
141 | //! Adds a child to the logger. With this, all messages which pass the filters
142 | //! will be sent to stdout.
143 | //!
144 | //! [`Dispatch::chain`] accepts [`Stdout`], [`Stderr`], [`File`] and other
145 | //! [`Dispatch`] instances.
146 | //!
147 | //! ```no_run
148 | //! # fern::Dispatch::new()
149 | //! .chain(fern::log_file("output.log")?)
150 | //! # ; <Result<(), Box<dyn std::error::Error>>>::Ok(())
151 | //! ```
152 | //!
153 | //! Adds a second child sending messages to the file "output.log".
154 | //!
155 | //! See [`log_file`].
156 | //!
157 | //! ```
158 | //! # fern::Dispatch::new()
159 | //! // ...
160 | //! .apply()
161 | //! # ;
162 | //! ```
163 | //!
164 | //! Consumes the configuration and instantiates it as the current runtime global
165 | //! logger.
166 | //!
167 | //! This will fail if and only if `.apply()` or equivalent form another crate
168 | //! has already been used this runtime.
169 | //!
170 | //! Since the binary crate is the only one ever setting up logging, and it's
171 | //! usually done near the start of `main`, the [`Dispatch::apply`] result can be
172 | //! reasonably unwrapped: it's a bug if any crate is calling this method more
173 | //! than once.
174 | //!
175 | //! ---
176 | //!
177 | //! The final output will look like:
178 | //!
179 | //! ```text
180 | //! [2023-03-18T20:12:50Z INFO cmd_program] Hello, world!
181 | //! [2023-03-18T20:12:50Z WARN cmd_program] Warning!
182 | //! [2023-03-18T20:12:50Z DEBUG cmd_program] Now exiting.
183 | //! ```
184 | //!
185 | //! # Logging
186 | //!
187 | //! Once the logger has been set, it will pick up all logging calls from your
188 | //! crate and all libraries you depend on.
189 | //!
190 | //! ```rust
191 | //! # use log::{debug, error, info, trace, warn};
192 | //!
193 | //! # fn setup_logger() -> Result<(), fern::InitError> {
194 | //! fern::Dispatch::new()
195 | //!     // ...
196 | //!     .apply()?;
197 | //! # Ok(())
198 | //! # }
199 | //!
200 | //! # fn main() {
201 | //! # setup_logger().ok(); // we're ok with this not succeeding.
202 | //! trace!("Trace message");
203 | //! debug!("Debug message");
204 | //! info!("Info message");
205 | //! warn!("Warning message");
206 | //! error!("Error message");
207 | //! # }
208 | //! ```
209 | //!
210 | //! # More
211 | //!
212 | //! The [`Dispatch`] documentation has example usages of each method, and the
213 | //! [full example program] might be useful for using fern in a larger
214 | //! application context.
215 | //!
216 | //! See the [colors] module for examples using ANSI terminal coloring.
217 | //!
218 | //! See the [syslog] module for examples outputting to the unix syslog, or the
219 | //! [syslog full example program] for a more realistic sample.
220 | //!
221 | //! See the [meta] module for information on getting logging-within-logging
222 | //! working correctly.
223 | //!
224 | //! [`Stdout`]: std::io::Stdout
225 | //! [`Stderr`]: std::io::Stderr
226 | //! [`File`]: std::fs::File
227 | //! [full example program]: https://github.com/daboross/fern/tree/fern-0.7.0/examples/cmd-program.rs
228 | //! [syslog full example program]: https://github.com/daboross/fern/tree/fern-0.7.0/examples/syslog.rs
229 | //! [`humantime::format_rfc3339_seconds`]: https://docs.rs/humantime/2/humantime/fn.format_rfc3339_seconds.html
230 | use std::{
231 |     convert::AsRef,
232 |     fmt,
233 |     fs::{File, OpenOptions},
234 |     io,
235 |     path::Path,
236 | };
237 | 
238 | #[cfg(all(not(windows), any(feature = "syslog-4", feature = "syslog-6")))]
239 | use std::collections::HashMap;
240 | 
241 | #[cfg(all(not(windows), feature = "syslog-7"))]
242 | use std::collections::BTreeMap;
243 | 
244 | pub use crate::{
245 |     builders::{Dispatch, Output, Panic},
246 |     errors::InitError,
247 |     log_impl::FormatCallback,
248 | };
249 | 
250 | mod builders;
251 | mod errors;
252 | mod log_impl;
253 | 
254 | #[cfg(feature = "colored")]
255 | pub mod colors;
256 | #[cfg(all(
257 |     feature = "syslog-3",
258 |     feature = "syslog-4",
259 |     // disable on windows when running doctests, as the code itself only runs on
260 |     // linux. enable on windows otherwise because it's a documentation-only
261 |     // module.
262 |     any(not(windows), not(doctest))
263 | ))]
264 | pub mod syslog;
265 | 
266 | pub mod meta;
267 | 
268 | /// A type alias for a log formatter.
269 | ///
270 | /// As of fern `0.5`, the passed `fmt::Arguments` will always be the same as
271 | /// the given `log::Record`'s `.args()`.
272 | pub type Formatter = dyn Fn(FormatCallback, &fmt::Arguments, &log::Record) + Sync + Send + 'static;
273 | 
274 | /// A type alias for a log filter. Returning true means the record should
275 | /// succeed - false means it should fail.
276 | pub type Filter = dyn Fn(&log::Metadata) -> bool + Send + Sync + 'static;
277 | 
278 | #[cfg(feature = "date-based")]
279 | pub use crate::builders::DateBased;
280 | 
281 | #[cfg(all(not(windows), feature = "syslog-4"))]
282 | type Syslog4Rfc3164Logger = syslog4::Logger<syslog4::LoggerBackend, String, syslog4::Formatter3164>;
283 | 
284 | #[cfg(all(not(windows), feature = "syslog-4"))]
285 | type Syslog4Rfc5424Logger = syslog4::Logger<
286 |     syslog4::LoggerBackend,
287 |     (i32, HashMap<String, HashMap<String, String>>, String),
288 |     syslog4::Formatter5424,
289 | >;
290 | 
291 | #[cfg(all(not(windows), feature = "syslog-6"))]
292 | type Syslog6Rfc3164Logger = syslog6::Logger<syslog6::LoggerBackend, syslog6::Formatter3164>;
293 | 
294 | #[cfg(all(not(windows), feature = "syslog-6"))]
295 | type Syslog6Rfc5424Logger = syslog6::Logger<syslog6::LoggerBackend, syslog6::Formatter5424>;
296 | 
297 | #[cfg(all(not(windows), feature = "syslog-7"))]
298 | type Syslog7Rfc3164Logger = syslog7::Logger<syslog7::LoggerBackend, syslog7::Formatter3164>;
299 | 
300 | #[cfg(all(not(windows), feature = "syslog-7"))]
301 | type Syslog7Rfc5424Logger = syslog7::Logger<syslog7::LoggerBackend, syslog7::Formatter5424>;
302 | 
303 | #[cfg(all(not(windows), feature = "syslog-4"))]
304 | type Syslog4TransformFn =
305 |     dyn Fn(&log::Record) -> (i32, HashMap<String, HashMap<String, String>>, String) + Send + Sync;
306 | 
307 | #[cfg(all(not(windows), feature = "syslog-6"))]
308 | type Syslog6TransformFn =
309 |     dyn Fn(&log::Record) -> (u32, HashMap<String, HashMap<String, String>>, String) + Send + Sync;
310 | 
311 | #[cfg(all(not(windows), feature = "syslog-7"))]
312 | type Syslog7TransformFn =
313 |     dyn Fn(&log::Record) -> (u32, BTreeMap<String, BTreeMap<String, String>>, String) + Send + Sync;
314 | 
315 | /// Convenience method for opening a log file with common options.
316 | ///
317 | /// Equivalent to:
318 | ///
319 | /// ```no_run
320 | /// std::fs::OpenOptions::new()
321 | ///     .write(true)
322 | ///     .create(true)
323 | ///     .append(true)
324 | ///     .open("filename")
325 | /// # ;
326 | /// ```
327 | ///
328 | /// See [`OpenOptions`] for more information.
329 | ///
330 | /// [`OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
331 | #[inline]
332 | pub fn log_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
333 |     OpenOptions::new().create(true).append(true).open(path)
334 | }
335 | 
336 | /// Convenience method for opening a re-openable log file with common options.
337 | ///
338 | /// The file opening is equivalent to:
339 | ///
340 | /// ```no_run
341 | /// std::fs::OpenOptions::new()
342 | ///     .write(true)
343 | ///     .create(true)
344 | ///     .append(true)
345 | ///     .open("filename")
346 | /// # ;
347 | /// ```
348 | ///
349 | /// See [`OpenOptions`] for more information.
350 | ///
351 | /// [`OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
352 | ///
353 | /// This function is not available on Windows, and it requires the `reopen-03`
354 | /// feature to be enabled.
355 | #[cfg(all(not(windows), feature = "reopen-03"))]
356 | #[inline]
357 | pub fn log_reopen(path: &Path, signal: Option<libc::c_int>) -> io::Result<reopen03::Reopen<File>> {
358 |     let p = path.to_owned();
359 |     let r = reopen03::Reopen::new(Box::new(move || log_file(&p)))?;
360 | 
361 |     if let Some(s) = signal {
362 |         r.handle().register_signal(s)?;
363 |     }
364 |     Ok(r)
365 | }
366 | 
367 | /// Convenience method for opening a re-openable log file with common options.
368 | ///
369 | /// The file opening is equivalent to:
370 | ///
371 | /// ```no_run
372 | /// std::fs::OpenOptions::new()
373 | ///     .write(true)
374 | ///     .create(true)
375 | ///     .append(true)
376 | ///     .open("filename")
377 | /// # ;
378 | /// ```
379 | ///
380 | /// See [`OpenOptions`] for more information.
381 | ///
382 | /// [`OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
383 | ///
384 | /// This function requires the `reopen-1` feature to be enabled.
385 | #[cfg(all(not(windows), feature = "reopen-1"))]
386 | #[inline]
387 | pub fn log_reopen1<S: IntoIterator<Item = libc::c_int>>(
388 |     path: &Path,
389 |     signals: S,
390 | ) -> io::Result<reopen1::Reopen<File>> {
391 |     let p = path.to_owned();
392 |     let r = reopen1::Reopen::new(Box::new(move || log_file(&p)))?;
393 | 
394 |     for s in signals {
395 |         r.handle().register_signal(s)?;
396 |     }
397 |     Ok(r)
398 | }
399 | 


--------------------------------------------------------------------------------
/src/log_impl.rs:
--------------------------------------------------------------------------------
   1 | use std::{
   2 |     borrow::Cow,
   3 |     collections::HashMap,
   4 |     fmt, fs,
   5 |     io::{self, BufWriter, Write},
   6 |     sync::{mpsc, Arc, Mutex},
   7 | };
   8 | 
   9 | #[cfg(feature = "date-based")]
  10 | use std::{
  11 |     ffi::OsString,
  12 |     fs::OpenOptions,
  13 |     path::{Path, PathBuf},
  14 | };
  15 | 
  16 | use log::{self, Log};
  17 | 
  18 | use crate::{Filter, Formatter};
  19 | 
  20 | #[cfg(all(not(windows), feature = "syslog-4"))]
  21 | use crate::{Syslog4Rfc3164Logger, Syslog4Rfc5424Logger, Syslog4TransformFn};
  22 | #[cfg(all(not(windows), feature = "syslog-6"))]
  23 | use crate::{Syslog6Rfc3164Logger, Syslog6Rfc5424Logger, Syslog6TransformFn};
  24 | #[cfg(all(not(windows), feature = "syslog-7"))]
  25 | use crate::{Syslog7Rfc3164Logger, Syslog7Rfc5424Logger, Syslog7TransformFn};
  26 | 
  27 | pub enum LevelConfiguration {
  28 |     JustDefault,
  29 |     Minimal(Vec<(Cow<'static, str>, log::LevelFilter)>),
  30 |     Many(HashMap<Cow<'static, str>, log::LevelFilter>),
  31 | }
  32 | 
  33 | pub struct Dispatch {
  34 |     pub output: Vec<Output>,
  35 |     pub default_level: log::LevelFilter,
  36 |     pub levels: LevelConfiguration,
  37 |     pub format: Option<Box<Formatter>>,
  38 |     pub filters: Vec<Box<Filter>>,
  39 | }
  40 | 
  41 | /// Callback struct for use within a formatter closure
  42 | ///
  43 | /// Callbacks are used for formatting in order to allow usage of
  44 | /// [`std::fmt`]-based formatting without the allocation of the formatted
  45 | /// result which would be required to return it.
  46 | ///
  47 | /// Example usage:
  48 | ///
  49 | /// ```
  50 | /// fern::Dispatch::new().format(|callback: fern::FormatCallback, message, record| {
  51 | ///     callback.finish(format_args!("[{}] {}", record.level(), message))
  52 | /// })
  53 | /// # ;
  54 | /// ```
  55 | ///
  56 | /// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
  57 | #[must_use = "format callback must be used for log to process correctly"]
  58 | pub struct FormatCallback<'a>(InnerFormatCallback<'a>);
  59 | 
  60 | struct InnerFormatCallback<'a>(&'a mut bool, &'a Dispatch, &'a log::Record<'a>);
  61 | 
  62 | pub enum Output {
  63 |     Stdout(Stdout),
  64 |     Stderr(Stderr),
  65 |     File(File),
  66 |     Sender(Sender),
  67 |     #[cfg(all(not(windows), feature = "syslog-3"))]
  68 |     Syslog3(Syslog3),
  69 |     #[cfg(all(not(windows), feature = "syslog-4"))]
  70 |     Syslog4Rfc3164(Syslog4Rfc3164),
  71 |     #[cfg(all(not(windows), feature = "syslog-4"))]
  72 |     Syslog4Rfc5424(Syslog4Rfc5424),
  73 |     #[cfg(all(not(windows), feature = "syslog-6"))]
  74 |     Syslog6Rfc3164(Syslog6Rfc3164),
  75 |     #[cfg(all(not(windows), feature = "syslog-6"))]
  76 |     Syslog6Rfc5424(Syslog6Rfc5424),
  77 |     #[cfg(all(not(windows), feature = "syslog-7"))]
  78 |     Syslog7Rfc3164(Syslog7Rfc3164),
  79 |     #[cfg(all(not(windows), feature = "syslog-7"))]
  80 |     Syslog7Rfc5424(Syslog7Rfc5424),
  81 |     Dispatch(Dispatch),
  82 |     SharedDispatch(Arc<Dispatch>),
  83 |     OtherBoxed(Box<dyn Log>),
  84 |     OtherStatic(&'static dyn Log),
  85 |     Panic(Panic),
  86 |     Writer(Writer),
  87 |     #[cfg(feature = "date-based")]
  88 |     DateBased(DateBased),
  89 |     #[cfg(all(not(windows), feature = "reopen-03"))]
  90 |     Reopen(Reopen),
  91 |     #[cfg(all(not(windows), feature = "reopen-1"))]
  92 |     Reopen1(Reopen1),
  93 | }
  94 | 
  95 | pub struct Stdout {
  96 |     pub stream: io::Stdout,
  97 |     pub line_sep: Cow<'static, str>,
  98 | }
  99 | 
 100 | pub struct Stderr {
 101 |     pub stream: io::Stderr,
 102 |     pub line_sep: Cow<'static, str>,
 103 | }
 104 | 
 105 | pub struct File {
 106 |     pub stream: Mutex<BufWriter<fs::File>>,
 107 |     pub line_sep: Cow<'static, str>,
 108 | }
 109 | 
 110 | pub struct Sender {
 111 |     pub stream: Mutex<mpsc::Sender<String>>,
 112 |     pub line_sep: Cow<'static, str>,
 113 | }
 114 | 
 115 | pub struct Writer {
 116 |     pub stream: Mutex<Box<dyn Write + Send>>,
 117 |     pub line_sep: Cow<'static, str>,
 118 | }
 119 | 
 120 | #[cfg(all(not(windows), feature = "reopen-03"))]
 121 | pub struct Reopen {
 122 |     pub stream: Mutex<reopen03::Reopen<fs::File>>,
 123 |     pub line_sep: Cow<'static, str>,
 124 | }
 125 | 
 126 | #[cfg(all(not(windows), feature = "reopen-1"))]
 127 | pub struct Reopen1 {
 128 |     pub stream: Mutex<reopen1::Reopen<fs::File>>,
 129 |     pub line_sep: Cow<'static, str>,
 130 | }
 131 | 
 132 | #[cfg(all(not(windows), feature = "syslog-3"))]
 133 | pub struct Syslog3 {
 134 |     pub inner: syslog3::Logger,
 135 | }
 136 | 
 137 | #[cfg(all(not(windows), feature = "syslog-4"))]
 138 | pub struct Syslog4Rfc3164 {
 139 |     pub inner: Mutex<Syslog4Rfc3164Logger>,
 140 | }
 141 | 
 142 | #[cfg(all(not(windows), feature = "syslog-4"))]
 143 | pub struct Syslog4Rfc5424 {
 144 |     pub inner: Mutex<Syslog4Rfc5424Logger>,
 145 |     pub transform: Box<Syslog4TransformFn>,
 146 | }
 147 | 
 148 | #[cfg(all(not(windows), feature = "syslog-6"))]
 149 | pub struct Syslog6Rfc3164 {
 150 |     pub inner: Mutex<Syslog6Rfc3164Logger>,
 151 | }
 152 | 
 153 | #[cfg(all(not(windows), feature = "syslog-6"))]
 154 | pub struct Syslog6Rfc5424 {
 155 |     pub inner: Mutex<Syslog6Rfc5424Logger>,
 156 |     pub transform: Box<Syslog6TransformFn>,
 157 | }
 158 | 
 159 | #[cfg(all(not(windows), feature = "syslog-7"))]
 160 | pub struct Syslog7Rfc3164 {
 161 |     pub inner: Mutex<Syslog7Rfc3164Logger>,
 162 | }
 163 | 
 164 | #[cfg(all(not(windows), feature = "syslog-7"))]
 165 | pub struct Syslog7Rfc5424 {
 166 |     pub inner: Mutex<Syslog7Rfc5424Logger>,
 167 |     pub transform: Box<Syslog7TransformFn>,
 168 | }
 169 | 
 170 | pub struct Panic;
 171 | 
 172 | pub struct Null;
 173 | 
 174 | /// File logger with a dynamic time-based name.
 175 | #[derive(Debug)]
 176 | #[cfg(feature = "date-based")]
 177 | pub struct DateBased {
 178 |     pub config: DateBasedConfig,
 179 |     pub state: Mutex<DateBasedState>,
 180 | }
 181 | 
 182 | #[derive(Debug)]
 183 | #[cfg(feature = "date-based")]
 184 | pub enum ConfiguredTimezone {
 185 |     Local,
 186 |     Utc,
 187 | }
 188 | 
 189 | #[derive(Debug)]
 190 | #[cfg(feature = "date-based")]
 191 | pub struct DateBasedConfig {
 192 |     pub line_sep: Cow<'static, str>,
 193 |     /// This is a Path not an str so it can hold invalid UTF8 paths correctly.
 194 |     pub file_prefix: PathBuf,
 195 |     pub file_suffix: Cow<'static, str>,
 196 |     pub timezone: ConfiguredTimezone,
 197 | }
 198 | 
 199 | #[derive(Debug)]
 200 | #[cfg(feature = "date-based")]
 201 | pub struct DateBasedState {
 202 |     pub current_suffix: String,
 203 |     pub file_stream: Option<BufWriter<fs::File>>,
 204 | }
 205 | 
 206 | #[cfg(feature = "date-based")]
 207 | impl DateBasedState {
 208 |     pub fn new(current_suffix: String, file_stream: Option<fs::File>) -> Self {
 209 |         DateBasedState {
 210 |             current_suffix,
 211 |             file_stream: file_stream.map(BufWriter::new),
 212 |         }
 213 |     }
 214 | 
 215 |     pub fn replace_file(&mut self, new_suffix: String, new_file: Option<fs::File>) {
 216 |         if let Some(mut old) = self.file_stream.take() {
 217 |             let _ = old.flush();
 218 |         }
 219 |         self.current_suffix = new_suffix;
 220 |         self.file_stream = new_file.map(BufWriter::new)
 221 |     }
 222 | }
 223 | 
 224 | #[cfg(feature = "date-based")]
 225 | impl DateBasedConfig {
 226 |     pub fn new(
 227 |         line_sep: Cow<'static, str>,
 228 |         file_prefix: PathBuf,
 229 |         file_suffix: Cow<'static, str>,
 230 |         timezone: ConfiguredTimezone,
 231 |     ) -> Self {
 232 |         DateBasedConfig {
 233 |             line_sep,
 234 |             file_prefix,
 235 |             file_suffix,
 236 |             timezone,
 237 |         }
 238 |     }
 239 | 
 240 |     pub fn compute_current_suffix(&self) -> String {
 241 |         match self.timezone {
 242 |             ConfiguredTimezone::Utc => chrono::Utc::now().format(&self.file_suffix).to_string(),
 243 |             ConfiguredTimezone::Local => chrono::Local::now().format(&self.file_suffix).to_string(),
 244 |         }
 245 |     }
 246 | 
 247 |     pub fn compute_file_path(&self, suffix: &str) -> PathBuf {
 248 |         let mut path = OsString::from(&*self.file_prefix);
 249 |         // use the OsString::push method, not PathBuf::push which would add a path
 250 |         // separator
 251 |         path.push(suffix);
 252 |         path.into()
 253 |     }
 254 | 
 255 |     pub fn open_log_file(path: &Path) -> io::Result<fs::File> {
 256 |         OpenOptions::new().create(true).append(true).open(path)
 257 |     }
 258 | 
 259 |     pub fn open_current_log_file(&self, suffix: &str) -> io::Result<fs::File> {
 260 |         Self::open_log_file(&self.compute_file_path(suffix))
 261 |     }
 262 | }
 263 | 
 264 | impl From<Vec<(Cow<'static, str>, log::LevelFilter)>> for LevelConfiguration {
 265 |     fn from(mut levels: Vec<(Cow<'static, str>, log::LevelFilter)>) -> Self {
 266 |         // Benchmarked separately: https://gist.github.com/daboross/976978d8200caf86e02acb6805961195
 267 |         // Use Vec if there are fewer than 15 items, HashMap if there are more than 15.
 268 |         match levels.len() {
 269 |             0 => LevelConfiguration::JustDefault,
 270 |             x if x > 15 => LevelConfiguration::Many(levels.into_iter().collect()),
 271 |             _ => {
 272 |                 levels.shrink_to_fit();
 273 |                 LevelConfiguration::Minimal(levels)
 274 |             }
 275 |         }
 276 |     }
 277 | }
 278 | 
 279 | impl LevelConfiguration {
 280 |     // inline since we use it literally once.
 281 |     #[inline]
 282 |     fn find_module(&self, module: &str) -> Option<log::LevelFilter> {
 283 |         match *self {
 284 |             LevelConfiguration::JustDefault => None,
 285 |             _ => {
 286 |                 if let Some(level) = self.find_exact(module) {
 287 |                     return Some(level);
 288 |                 }
 289 | 
 290 |                 // The manual for loop here lets us just iterate over the module string once
 291 |                 // while still finding each sub-module. For the module string
 292 |                 // "hyper::http::h1", this loop will test first "hyper::http"
 293 |                 // then "hyper".
 294 |                 let mut last_char_colon = false;
 295 | 
 296 |                 for (index, ch) in module.char_indices().rev() {
 297 |                     if last_char_colon {
 298 |                         last_char_colon = false;
 299 |                         if ch == ':' {
 300 |                             let sub_module = &module[0..index];
 301 | 
 302 |                             if let Some(level) = self.find_exact(sub_module) {
 303 |                                 return Some(level);
 304 |                             }
 305 |                         }
 306 |                     } else if ch == ':' {
 307 |                         last_char_colon = true;
 308 |                     }
 309 |                 }
 310 | 
 311 |                 None
 312 |             }
 313 |         }
 314 |     }
 315 | 
 316 |     fn find_exact(&self, module: &str) -> Option<log::LevelFilter> {
 317 |         match *self {
 318 |             LevelConfiguration::JustDefault => None,
 319 |             LevelConfiguration::Minimal(ref levels) => levels
 320 |                 .iter()
 321 |                 .find(|(test_module, _)| test_module == module)
 322 |                 .map(|(_, level)| *level),
 323 |             LevelConfiguration::Many(ref levels) => levels.get(module).cloned(),
 324 |         }
 325 |     }
 326 | }
 327 | 
 328 | impl Log for Output {
 329 |     fn enabled(&self, metadata: &log::Metadata) -> bool {
 330 |         match *self {
 331 |             Output::Stdout(ref s) => s.enabled(metadata),
 332 |             Output::Stderr(ref s) => s.enabled(metadata),
 333 |             Output::File(ref s) => s.enabled(metadata),
 334 |             Output::Sender(ref s) => s.enabled(metadata),
 335 |             Output::Dispatch(ref s) => s.enabled(metadata),
 336 |             Output::SharedDispatch(ref s) => s.enabled(metadata),
 337 |             Output::OtherBoxed(ref s) => s.enabled(metadata),
 338 |             Output::OtherStatic(ref s) => s.enabled(metadata),
 339 |             #[cfg(all(not(windows), feature = "syslog-3"))]
 340 |             Output::Syslog3(ref s) => s.enabled(metadata),
 341 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 342 |             Output::Syslog4Rfc3164(ref s) => s.enabled(metadata),
 343 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 344 |             Output::Syslog4Rfc5424(ref s) => s.enabled(metadata),
 345 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 346 |             Output::Syslog6Rfc3164(ref s) => s.enabled(metadata),
 347 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 348 |             Output::Syslog6Rfc5424(ref s) => s.enabled(metadata),
 349 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 350 |             Output::Syslog7Rfc3164(ref s) => s.enabled(metadata),
 351 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 352 |             Output::Syslog7Rfc5424(ref s) => s.enabled(metadata),
 353 |             Output::Panic(ref s) => s.enabled(metadata),
 354 |             Output::Writer(ref s) => s.enabled(metadata),
 355 |             #[cfg(feature = "date-based")]
 356 |             Output::DateBased(ref s) => s.enabled(metadata),
 357 |             #[cfg(all(not(windows), feature = "reopen-03"))]
 358 |             Output::Reopen(ref s) => s.enabled(metadata),
 359 |             #[cfg(all(not(windows), feature = "reopen-1"))]
 360 |             Output::Reopen1(ref s) => s.enabled(metadata),
 361 |         }
 362 |     }
 363 | 
 364 |     fn log(&self, record: &log::Record) {
 365 |         match *self {
 366 |             Output::Stdout(ref s) => s.log(record),
 367 |             Output::Stderr(ref s) => s.log(record),
 368 |             Output::File(ref s) => s.log(record),
 369 |             Output::Sender(ref s) => s.log(record),
 370 |             Output::Dispatch(ref s) => s.log(record),
 371 |             Output::SharedDispatch(ref s) => s.log(record),
 372 |             Output::OtherBoxed(ref s) => s.log(record),
 373 |             Output::OtherStatic(ref s) => s.log(record),
 374 |             #[cfg(all(not(windows), feature = "syslog-3"))]
 375 |             Output::Syslog3(ref s) => s.log(record),
 376 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 377 |             Output::Syslog4Rfc3164(ref s) => s.log(record),
 378 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 379 |             Output::Syslog4Rfc5424(ref s) => s.log(record),
 380 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 381 |             Output::Syslog6Rfc3164(ref s) => s.log(record),
 382 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 383 |             Output::Syslog6Rfc5424(ref s) => s.log(record),
 384 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 385 |             Output::Syslog7Rfc3164(ref s) => s.log(record),
 386 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 387 |             Output::Syslog7Rfc5424(ref s) => s.log(record),
 388 |             Output::Panic(ref s) => s.log(record),
 389 |             Output::Writer(ref s) => s.log(record),
 390 |             #[cfg(feature = "date-based")]
 391 |             Output::DateBased(ref s) => s.log(record),
 392 |             #[cfg(all(not(windows), feature = "reopen-03"))]
 393 |             Output::Reopen(ref s) => s.log(record),
 394 |             #[cfg(all(not(windows), feature = "reopen-1"))]
 395 |             Output::Reopen1(ref s) => s.log(record),
 396 |         }
 397 |     }
 398 | 
 399 |     fn flush(&self) {
 400 |         match *self {
 401 |             Output::Stdout(ref s) => s.flush(),
 402 |             Output::Stderr(ref s) => s.flush(),
 403 |             Output::File(ref s) => s.flush(),
 404 |             Output::Sender(ref s) => s.flush(),
 405 |             Output::Dispatch(ref s) => s.flush(),
 406 |             Output::SharedDispatch(ref s) => s.flush(),
 407 |             Output::OtherBoxed(ref s) => s.flush(),
 408 |             Output::OtherStatic(ref s) => s.flush(),
 409 |             #[cfg(all(not(windows), feature = "syslog-3"))]
 410 |             Output::Syslog3(ref s) => s.flush(),
 411 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 412 |             Output::Syslog4Rfc3164(ref s) => s.flush(),
 413 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 414 |             Output::Syslog4Rfc5424(ref s) => s.flush(),
 415 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 416 |             Output::Syslog6Rfc3164(ref s) => s.flush(),
 417 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 418 |             Output::Syslog6Rfc5424(ref s) => s.flush(),
 419 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 420 |             Output::Syslog7Rfc3164(ref s) => s.flush(),
 421 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 422 |             Output::Syslog7Rfc5424(ref s) => s.flush(),
 423 |             Output::Panic(ref s) => s.flush(),
 424 |             Output::Writer(ref s) => s.flush(),
 425 |             #[cfg(feature = "date-based")]
 426 |             Output::DateBased(ref s) => s.flush(),
 427 |             #[cfg(all(not(windows), feature = "reopen-03"))]
 428 |             Output::Reopen(ref s) => s.flush(),
 429 |             #[cfg(all(not(windows), feature = "reopen-1"))]
 430 |             Output::Reopen1(ref s) => s.flush(),
 431 |         }
 432 |     }
 433 | }
 434 | 
 435 | impl Log for Null {
 436 |     fn enabled(&self, _: &log::Metadata) -> bool {
 437 |         false
 438 |     }
 439 | 
 440 |     fn log(&self, _: &log::Record) {}
 441 | 
 442 |     fn flush(&self) {}
 443 | }
 444 | 
 445 | impl Log for Dispatch {
 446 |     fn enabled(&self, metadata: &log::Metadata) -> bool {
 447 |         self.deep_enabled(metadata)
 448 |     }
 449 | 
 450 |     fn log(&self, record: &log::Record) {
 451 |         if self.shallow_enabled(record.metadata()) {
 452 |             match self.format {
 453 |                 Some(ref format) => {
 454 |                     // flag to ensure the log message is completed even if the formatter doesn't
 455 |                     // complete the callback.
 456 |                     let mut callback_called_flag = false;
 457 | 
 458 |                     (format)(
 459 |                         FormatCallback(InnerFormatCallback(
 460 |                             &mut callback_called_flag,
 461 |                             self,
 462 |                             record,
 463 |                         )),
 464 |                         record.args(),
 465 |                         record,
 466 |                     );
 467 | 
 468 |                     if !callback_called_flag {
 469 |                         self.finish_logging(record);
 470 |                     }
 471 |                 }
 472 |                 None => {
 473 |                     self.finish_logging(record);
 474 |                 }
 475 |             }
 476 |         }
 477 |     }
 478 | 
 479 |     fn flush(&self) {
 480 |         for log in &self.output {
 481 |             log.flush();
 482 |         }
 483 |     }
 484 | }
 485 | 
 486 | impl Dispatch {
 487 |     fn finish_logging(&self, record: &log::Record) {
 488 |         for log in &self.output {
 489 |             log.log(record);
 490 |         }
 491 |     }
 492 | 
 493 |     /// Check whether this log's filters prevent the given log from happening.
 494 |     fn shallow_enabled(&self, metadata: &log::Metadata) -> bool {
 495 |         metadata.level()
 496 |             <= self
 497 |                 .levels
 498 |                 .find_module(metadata.target())
 499 |                 .unwrap_or(self.default_level)
 500 |             && self.filters.iter().all(|f| f(metadata))
 501 |     }
 502 | 
 503 |     /// Check whether a log with the given metadata would eventually end up
 504 |     /// outputting something.
 505 |     ///
 506 |     /// This is recursive, and checks children.
 507 |     fn deep_enabled(&self, metadata: &log::Metadata) -> bool {
 508 |         self.shallow_enabled(metadata) && self.output.iter().any(|l| l.enabled(metadata))
 509 |     }
 510 | }
 511 | 
 512 | impl FormatCallback<'_> {
 513 |     /// Complete the formatting call that this FormatCallback was created for.
 514 |     ///
 515 |     /// This will call the rest of the logging chain using the given formatted
 516 |     /// message as the new payload message.
 517 |     ///
 518 |     /// Example usage:
 519 |     ///
 520 |     /// ```
 521 |     /// # fern::Dispatch::new()
 522 |     /// # .format(|callback: fern::FormatCallback, message, record| {
 523 |     /// callback.finish(format_args!("[{}] {}", record.level(), message))
 524 |     /// # })
 525 |     /// # .into_log();
 526 |     /// ```
 527 |     ///
 528 |     /// See [`format_args!`].
 529 |     ///
 530 |     /// [`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
 531 |     pub fn finish(self, formatted_message: fmt::Arguments) {
 532 |         let FormatCallback(InnerFormatCallback(callback_called_flag, dispatch, record)) = self;
 533 | 
 534 |         // let the dispatch know that we did in fact get called.
 535 |         *callback_called_flag = true;
 536 | 
 537 |         // NOTE: This needs to be updated whenever new things are added to
 538 |         // `log::Record`.
 539 |         let new_record = log::RecordBuilder::new()
 540 |             .args(formatted_message)
 541 |             .metadata(record.metadata().clone())
 542 |             .level(record.level())
 543 |             .target(record.target())
 544 |             .module_path(record.module_path())
 545 |             .file(record.file())
 546 |             .line(record.line())
 547 |             .build();
 548 | 
 549 |         dispatch.finish_logging(&new_record);
 550 |     }
 551 | }
 552 | 
 553 | // No need to write this twice (used for Stdout and Stderr structs)
 554 | macro_rules! std_log_impl {
 555 |     ($ident:ident) => {
 556 |         impl Log for $ident {
 557 |             fn enabled(&self, _: &log::Metadata) -> bool {
 558 |                 true
 559 |             }
 560 | 
 561 |             fn log(&self, record: &log::Record) {
 562 |                 fallback_on_error(record, |record| {
 563 |                     if cfg!(feature = "meta-logging-in-format") {
 564 |                         // Formatting first prevents deadlocks when the process of formatting
 565 |                         // itself is logged. note: this is only ever needed if some
 566 |                         // Debug, Display, or other formatting trait itself is
 567 |                         // logging things too.
 568 |                         let msg = format!("{}{}", record.args(), self.line_sep);
 569 | 
 570 |                         write!(self.stream.lock(), "{}", msg)?;
 571 |                     } else {
 572 |                         write!(self.stream.lock(), "{}{}", record.args(), self.line_sep)?;
 573 |                     }
 574 | 
 575 |                     Ok(())
 576 |                 });
 577 |             }
 578 | 
 579 |             fn flush(&self) {
 580 |                 let _ = self.stream.lock().flush();
 581 |             }
 582 |         }
 583 |     };
 584 | }
 585 | 
 586 | std_log_impl!(Stdout);
 587 | std_log_impl!(Stderr);
 588 | 
 589 | macro_rules! writer_log_impl {
 590 |     ($ident:ident) => {
 591 |         impl Log for $ident {
 592 |             fn enabled(&self, _: &log::Metadata) -> bool {
 593 |                 true
 594 |             }
 595 | 
 596 |             fn log(&self, record: &log::Record) {
 597 |                 fallback_on_error(record, |record| {
 598 |                     if cfg!(feature = "meta-logging-in-format") {
 599 |                         // Formatting first prevents deadlocks on file-logging,
 600 |                         // when the process of formatting itself is logged.
 601 |                         // note: this is only ever needed if some Debug, Display, or other
 602 |                         // formatting trait itself is logging.
 603 |                         let msg = format!("{}{}", record.args(), self.line_sep);
 604 | 
 605 |                         let mut writer = self.stream.lock().unwrap_or_else(|e| e.into_inner());
 606 | 
 607 |                         write!(writer, "{}", msg)?;
 608 | 
 609 |                         writer.flush()?;
 610 |                     } else {
 611 |                         let mut writer = self.stream.lock().unwrap_or_else(|e| e.into_inner());
 612 | 
 613 |                         write!(writer, "{}{}", record.args(), self.line_sep)?;
 614 | 
 615 |                         writer.flush()?;
 616 |                     }
 617 |                     Ok(())
 618 |                 });
 619 |             }
 620 | 
 621 |             fn flush(&self) {
 622 |                 let _ = self
 623 |                     .stream
 624 |                     .lock()
 625 |                     .unwrap_or_else(|e| e.into_inner())
 626 |                     .flush();
 627 |             }
 628 |         }
 629 |     };
 630 | }
 631 | 
 632 | writer_log_impl!(File);
 633 | writer_log_impl!(Writer);
 634 | 
 635 | #[cfg(all(not(windows), feature = "reopen-03"))]
 636 | writer_log_impl!(Reopen);
 637 | 
 638 | #[cfg(all(not(windows), feature = "reopen-1"))]
 639 | writer_log_impl!(Reopen1);
 640 | 
 641 | impl Log for Sender {
 642 |     fn enabled(&self, _: &log::Metadata) -> bool {
 643 |         true
 644 |     }
 645 | 
 646 |     fn log(&self, record: &log::Record) {
 647 |         fallback_on_error(record, |record| {
 648 |             let msg = format!("{}{}", record.args(), self.line_sep);
 649 |             self.stream
 650 |                 .lock()
 651 |                 .unwrap_or_else(|e| e.into_inner())
 652 |                 .send(msg)?;
 653 |             Ok(())
 654 |         });
 655 |     }
 656 | 
 657 |     fn flush(&self) {}
 658 | }
 659 | 
 660 | #[cfg(all(
 661 |     not(windows),
 662 |     any(
 663 |         feature = "syslog-3",
 664 |         feature = "syslog-4",
 665 |         feature = "syslog-6",
 666 |         feature = "syslog-7"
 667 |     )
 668 | ))]
 669 | macro_rules! send_syslog {
 670 |     ($logger:expr, $level:expr, $message:expr) => {
 671 |         use log::Level;
 672 |         match $level {
 673 |             Level::Error => $logger.err($message)?,
 674 |             Level::Warn => $logger.warning($message)?,
 675 |             Level::Info => $logger.info($message)?,
 676 |             Level::Debug | Level::Trace => $logger.debug($message)?,
 677 |         }
 678 |     };
 679 | }
 680 | 
 681 | #[cfg(all(not(windows), feature = "syslog-3"))]
 682 | impl Log for Syslog3 {
 683 |     fn enabled(&self, _: &log::Metadata) -> bool {
 684 |         true
 685 |     }
 686 | 
 687 |     fn log(&self, record: &log::Record) {
 688 |         fallback_on_error(record, |record| {
 689 |             let message = record.args();
 690 |             send_syslog!(self.inner, record.level(), message);
 691 | 
 692 |             Ok(())
 693 |         });
 694 |     }
 695 |     fn flush(&self) {}
 696 | }
 697 | 
 698 | #[cfg(all(not(windows), feature = "syslog-4"))]
 699 | impl Log for Syslog4Rfc3164 {
 700 |     fn enabled(&self, _: &log::Metadata) -> bool {
 701 |         true
 702 |     }
 703 | 
 704 |     fn log(&self, record: &log::Record) {
 705 |         fallback_on_error(record, |record| {
 706 |             let message = record.args().to_string();
 707 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 708 |             send_syslog!(log, record.level(), message);
 709 | 
 710 |             Ok(())
 711 |         });
 712 |     }
 713 |     fn flush(&self) {}
 714 | }
 715 | 
 716 | #[cfg(all(not(windows), feature = "syslog-4"))]
 717 | impl Log for Syslog4Rfc5424 {
 718 |     fn enabled(&self, _: &log::Metadata) -> bool {
 719 |         true
 720 |     }
 721 | 
 722 |     fn log(&self, record: &log::Record) {
 723 |         fallback_on_error(record, |record| {
 724 |             let transformed = (self.transform)(record);
 725 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 726 |             send_syslog!(log, record.level(), transformed);
 727 | 
 728 |             Ok(())
 729 |         });
 730 |     }
 731 |     fn flush(&self) {}
 732 | }
 733 | 
 734 | #[cfg(all(not(windows), feature = "syslog-6"))]
 735 | impl Log for Syslog6Rfc3164 {
 736 |     fn enabled(&self, _: &log::Metadata) -> bool {
 737 |         true
 738 |     }
 739 | 
 740 |     fn log(&self, record: &log::Record) {
 741 |         fallback_on_error(record, |record| {
 742 |             let message = record.args().to_string();
 743 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 744 |             send_syslog!(log, record.level(), message);
 745 | 
 746 |             Ok(())
 747 |         });
 748 |     }
 749 |     fn flush(&self) {}
 750 | }
 751 | 
 752 | #[cfg(all(not(windows), feature = "syslog-6"))]
 753 | impl Log for Syslog6Rfc5424 {
 754 |     fn enabled(&self, _: &log::Metadata) -> bool {
 755 |         true
 756 |     }
 757 | 
 758 |     fn log(&self, record: &log::Record) {
 759 |         fallback_on_error(record, |record| {
 760 |             let transformed = (self.transform)(record);
 761 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 762 |             send_syslog!(log, record.level(), transformed);
 763 | 
 764 |             Ok(())
 765 |         });
 766 |     }
 767 |     fn flush(&self) {}
 768 | }
 769 | 
 770 | #[cfg(all(not(windows), feature = "syslog-7"))]
 771 | impl Log for Syslog7Rfc3164 {
 772 |     fn enabled(&self, _: &log::Metadata) -> bool {
 773 |         true
 774 |     }
 775 | 
 776 |     fn log(&self, record: &log::Record) {
 777 |         fallback_on_error(record, |record| {
 778 |             let message = record.args().to_string();
 779 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 780 |             send_syslog!(log, record.level(), message);
 781 | 
 782 |             Ok(())
 783 |         });
 784 |     }
 785 |     fn flush(&self) {}
 786 | }
 787 | 
 788 | #[cfg(all(not(windows), feature = "syslog-7"))]
 789 | impl Log for Syslog7Rfc5424 {
 790 |     fn enabled(&self, _: &log::Metadata) -> bool {
 791 |         true
 792 |     }
 793 | 
 794 |     fn log(&self, record: &log::Record) {
 795 |         fallback_on_error(record, |record| {
 796 |             let transformed = (self.transform)(record);
 797 |             let mut log = self.inner.lock().unwrap_or_else(|e| e.into_inner());
 798 |             send_syslog!(log, record.level(), transformed);
 799 | 
 800 |             Ok(())
 801 |         });
 802 |     }
 803 |     fn flush(&self) {}
 804 | }
 805 | 
 806 | impl Log for Panic {
 807 |     fn enabled(&self, _: &log::Metadata) -> bool {
 808 |         true
 809 |     }
 810 | 
 811 |     fn log(&self, record: &log::Record) {
 812 |         panic!("{}", record.args());
 813 |     }
 814 | 
 815 |     fn flush(&self) {}
 816 | }
 817 | 
 818 | #[cfg(feature = "date-based")]
 819 | impl Log for DateBased {
 820 |     fn enabled(&self, _: &log::Metadata) -> bool {
 821 |         true
 822 |     }
 823 | 
 824 |     fn log(&self, record: &log::Record) {
 825 |         fallback_on_error(record, |record| {
 826 |             // Formatting first prevents deadlocks on file-logging,
 827 |             // when the process of formatting itself is logged.
 828 |             // note: this is only ever needed if some Debug, Display, or other
 829 |             // formatting trait itself is logging.
 830 |             #[cfg(feature = "meta-logging-in-format")]
 831 |             let msg = format!("{}{}", record.args(), self.config.line_sep);
 832 | 
 833 |             let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
 834 | 
 835 |             // check if log needs to be rotated
 836 |             let new_suffix = self.config.compute_current_suffix();
 837 |             if state.file_stream.is_none() || state.current_suffix != new_suffix {
 838 |                 let file_open_result = self.config.open_current_log_file(&new_suffix);
 839 |                 match file_open_result {
 840 |                     Ok(file) => {
 841 |                         state.replace_file(new_suffix, Some(file));
 842 |                     }
 843 |                     Err(e) => {
 844 |                         state.replace_file(new_suffix, None);
 845 |                         return Err(e.into());
 846 |                     }
 847 |                 }
 848 |             }
 849 | 
 850 |             // either just initialized writer above, or already errored out.
 851 |             let writer = state.file_stream.as_mut().unwrap();
 852 | 
 853 |             #[cfg(feature = "meta-logging-in-format")]
 854 |             write!(writer, "{}", msg)?;
 855 |             #[cfg(not(feature = "meta-logging-in-format"))]
 856 |             write!(writer, "{}{}", record.args(), self.config.line_sep)?;
 857 | 
 858 |             writer.flush()?;
 859 | 
 860 |             Ok(())
 861 |         });
 862 |     }
 863 | 
 864 |     fn flush(&self) {
 865 |         let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
 866 | 
 867 |         if let Some(stream) = &mut state.file_stream {
 868 |             let _ = stream.flush();
 869 |         }
 870 |     }
 871 | }
 872 | 
 873 | #[inline(always)]
 874 | fn fallback_on_error<F>(record: &log::Record, log_func: F)
 875 | where
 876 |     F: FnOnce(&log::Record) -> Result<(), LogError>,
 877 | {
 878 |     if let Err(error) = log_func(record) {
 879 |         backup_logging(record, &error)
 880 |     }
 881 | }
 882 | 
 883 | fn backup_logging(record: &log::Record, error: &LogError) {
 884 |     let second = write!(
 885 |         io::stderr(),
 886 |         "Error performing logging.\
 887 |          \n\tattempted to log: {}\
 888 |          \n\trecord: {:?}\
 889 |          \n\tlogging error: {}",
 890 |         record.args(),
 891 |         record,
 892 |         error
 893 |     );
 894 | 
 895 |     if let Err(second_error) = second {
 896 |         panic!(
 897 |             "Error performing stderr logging after error occurred during regular logging.\
 898 |              \n\tattempted to log: {}\
 899 |              \n\trecord: {:?}\
 900 |              \n\tfirst logging error: {}\
 901 |              \n\tstderr error: {}",
 902 |             record.args(),
 903 |             record,
 904 |             error,
 905 |             second_error,
 906 |         );
 907 |     }
 908 | }
 909 | 
 910 | #[derive(Debug)]
 911 | enum LogError {
 912 |     Io(io::Error),
 913 |     Send(mpsc::SendError<String>),
 914 |     #[cfg(all(not(windows), feature = "syslog-4"))]
 915 |     Syslog4(syslog4::Error),
 916 |     #[cfg(all(not(windows), feature = "syslog-6"))]
 917 |     Syslog6(syslog6::Error),
 918 |     #[cfg(all(not(windows), feature = "syslog-7"))]
 919 |     Syslog7(syslog7::Error),
 920 | }
 921 | 
 922 | impl fmt::Display for LogError {
 923 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 924 |         match *self {
 925 |             LogError::Io(ref e) => write!(f, "{}", e),
 926 |             LogError::Send(ref e) => write!(f, "{}", e),
 927 |             #[cfg(all(not(windows), feature = "syslog-4"))]
 928 |             LogError::Syslog4(ref e) => write!(f, "{}", e),
 929 |             #[cfg(all(not(windows), feature = "syslog-6"))]
 930 |             LogError::Syslog6(ref e) => write!(f, "{}", e),
 931 |             #[cfg(all(not(windows), feature = "syslog-7"))]
 932 |             LogError::Syslog7(ref e) => write!(f, "{}", e),
 933 |         }
 934 |     }
 935 | }
 936 | 
 937 | impl From<io::Error> for LogError {
 938 |     fn from(error: io::Error) -> Self {
 939 |         LogError::Io(error)
 940 |     }
 941 | }
 942 | 
 943 | impl From<mpsc::SendError<String>> for LogError {
 944 |     fn from(error: mpsc::SendError<String>) -> Self {
 945 |         LogError::Send(error)
 946 |     }
 947 | }
 948 | 
 949 | #[cfg(all(not(windows), feature = "syslog-4"))]
 950 | impl From<syslog4::Error> for LogError {
 951 |     fn from(error: syslog4::Error) -> Self {
 952 |         LogError::Syslog4(error)
 953 |     }
 954 | }
 955 | 
 956 | #[cfg(all(not(windows), feature = "syslog-6"))]
 957 | impl From<syslog6::Error> for LogError {
 958 |     fn from(error: syslog6::Error) -> Self {
 959 |         LogError::Syslog6(error)
 960 |     }
 961 | }
 962 | 
 963 | #[cfg(all(not(windows), feature = "syslog-7"))]
 964 | impl From<syslog7::Error> for LogError {
 965 |     fn from(error: syslog7::Error) -> Self {
 966 |         LogError::Syslog7(error)
 967 |     }
 968 | }
 969 | 
 970 | #[cfg(test)]
 971 | mod test {
 972 |     use super::LevelConfiguration;
 973 |     use log::LevelFilter::*;
 974 | 
 975 |     #[test]
 976 |     fn test_level_config_find_exact_minimal() {
 977 |         let config = LevelConfiguration::Minimal(
 978 |             vec![("mod1", Info), ("mod2", Debug), ("mod3", Off)]
 979 |                 .into_iter()
 980 |                 .map(|(k, v)| (k.into(), v))
 981 |                 .collect(),
 982 |         );
 983 | 
 984 |         assert_eq!(config.find_exact("mod1"), Some(Info));
 985 |         assert_eq!(config.find_exact("mod2"), Some(Debug));
 986 |         assert_eq!(config.find_exact("mod3"), Some(Off));
 987 |     }
 988 | 
 989 |     #[test]
 990 |     fn test_level_config_find_exact_many() {
 991 |         let config = LevelConfiguration::Many(
 992 |             vec![("mod1", Info), ("mod2", Debug), ("mod3", Off)]
 993 |                 .into_iter()
 994 |                 .map(|(k, v)| (k.into(), v))
 995 |                 .collect(),
 996 |         );
 997 | 
 998 |         assert_eq!(config.find_exact("mod1"), Some(Info));
 999 |         assert_eq!(config.find_exact("mod2"), Some(Debug));
1000 |         assert_eq!(config.find_exact("mod3"), Some(Off));
1001 |     }
1002 | 
1003 |     #[test]
1004 |     fn test_level_config_simple_hierarchy() {
1005 |         let config = LevelConfiguration::Minimal(
1006 |             vec![("mod1", Info), ("mod2::sub_mod", Debug), ("mod3", Off)]
1007 |                 .into_iter()
1008 |                 .map(|(k, v)| (k.into(), v))
1009 |                 .collect(),
1010 |         );
1011 | 
1012 |         assert_eq!(config.find_module("mod1::sub_mod"), Some(Info));
1013 |         assert_eq!(config.find_module("mod2::sub_mod::sub_mod_2"), Some(Debug));
1014 |         assert_eq!(config.find_module("mod3::sub_mod::sub_mod_2"), Some(Off));
1015 |     }
1016 | 
1017 |     #[test]
1018 |     fn test_level_config_hierarchy_correct() {
1019 |         let config = LevelConfiguration::Minimal(
1020 |             vec![
1021 |                 ("root", Trace),
1022 |                 ("root::sub1", Debug),
1023 |                 ("root::sub2", Info),
1024 |                 // should work with all insertion orders
1025 |                 ("root::sub2::sub2.3::sub2.4", Error),
1026 |                 ("root::sub2::sub2.3", Warn),
1027 |                 ("root::sub3", Off),
1028 |             ]
1029 |             .into_iter()
1030 |             .map(|(k, v)| (k.into(), v))
1031 |             .collect(),
1032 |         );
1033 | 
1034 |         assert_eq!(config.find_module("root"), Some(Trace));
1035 |         assert_eq!(config.find_module("root::other_module"), Some(Trace));
1036 | 
1037 |         // We want to ensure that it does pick up most specific level before trying
1038 |         // anything more general.
1039 |         assert_eq!(config.find_module("root::sub1"), Some(Debug));
1040 |         assert_eq!(config.find_module("root::sub1::other_module"), Some(Debug));
1041 | 
1042 |         assert_eq!(config.find_module("root::sub2"), Some(Info));
1043 |         assert_eq!(config.find_module("root::sub2::other"), Some(Info));
1044 | 
1045 |         assert_eq!(config.find_module("root::sub2::sub2.3"), Some(Warn));
1046 |         assert_eq!(
1047 |             config.find_module("root::sub2::sub2.3::sub2.4"),
1048 |             Some(Error)
1049 |         );
1050 | 
1051 |         assert_eq!(config.find_module("root::sub3"), Some(Off));
1052 |         assert_eq!(
1053 |             config.find_module("root::sub3::any::children::of::sub3"),
1054 |             Some(Off)
1055 |         );
1056 |     }
1057 | 
1058 |     #[test]
1059 |     fn test_level_config_similar_names_are_not_same() {
1060 |         let config = LevelConfiguration::Minimal(
1061 |             vec![("root", Trace), ("rootay", Info)]
1062 |                 .into_iter()
1063 |                 .map(|(k, v)| (k.into(), v))
1064 |                 .collect(),
1065 |         );
1066 | 
1067 |         assert_eq!(config.find_module("root"), Some(Trace));
1068 |         assert_eq!(config.find_module("root::sub"), Some(Trace));
1069 |         assert_eq!(config.find_module("rooty"), None);
1070 |         assert_eq!(config.find_module("rooty::sub"), None);
1071 |         assert_eq!(config.find_module("rootay"), Some(Info));
1072 |         assert_eq!(config.find_module("rootay::sub"), Some(Info));
1073 |     }
1074 | 
1075 |     #[test]
1076 |     fn test_level_config_single_colon_is_not_double_colon() {
1077 |         let config = LevelConfiguration::Minimal(
1078 |             vec![
1079 |                 ("root", Trace),
1080 |                 ("root::su", Debug),
1081 |                 ("root::su:b2", Info),
1082 |                 ("root::sub2", Warn),
1083 |             ]
1084 |             .into_iter()
1085 |             .map(|(k, v)| (k.into(), v))
1086 |             .collect(),
1087 |         );
1088 | 
1089 |         assert_eq!(config.find_module("root"), Some(Trace));
1090 | 
1091 |         assert_eq!(config.find_module("root::su"), Some(Debug));
1092 |         assert_eq!(config.find_module("root::su::b2"), Some(Debug));
1093 | 
1094 |         assert_eq!(config.find_module("root::su:b2"), Some(Info));
1095 |         assert_eq!(config.find_module("root::su:b2::b3"), Some(Info));
1096 | 
1097 |         assert_eq!(config.find_module("root::sub2"), Some(Warn));
1098 |         assert_eq!(config.find_module("root::sub2::b3"), Some(Warn));
1099 |     }
1100 | 
1101 |     #[test]
1102 |     fn test_level_config_all_chars() {
1103 |         let config = LevelConfiguration::Minimal(
1104 |             vec![("♲", Trace), ("☸", Debug), ("♲::☸", Info), ("♲::\t", Debug)]
1105 |                 .into_iter()
1106 |                 .map(|(k, v)| (k.into(), v))
1107 |                 .collect(),
1108 |         );
1109 | 
1110 |         assert_eq!(config.find_module("♲"), Some(Trace));
1111 |         assert_eq!(config.find_module("♲::other"), Some(Trace));
1112 | 
1113 |         assert_eq!(config.find_module("☸"), Some(Debug));
1114 |         assert_eq!(config.find_module("☸::any"), Some(Debug));
1115 | 
1116 |         assert_eq!(config.find_module("♲::☸"), Some(Info));
1117 |         assert_eq!(config.find_module("♲☸"), None);
1118 | 
1119 |         assert_eq!(config.find_module("♲::\t"), Some(Debug));
1120 |         assert_eq!(config.find_module("♲::\t::\n\n::\t"), Some(Debug));
1121 |         assert_eq!(config.find_module("♲::\t\t"), Some(Trace));
1122 |     }
1123 | }
1124 | 


--------------------------------------------------------------------------------
/src/meta.rs:
--------------------------------------------------------------------------------
 1 | /*!
 2 | Fern supports logging most things by default, except for one kind of struct: structs which make log
 3 | calls to the global logger from within their `Display` or `Debug` implementations.
 4 | 
 5 | Here's an example of such a structure:
 6 | 
 7 | ```
 8 | # use log::debug;
 9 | # use std::fmt;
10 | #
11 | struct Thing<'a>(&'a str);
12 | 
13 | impl<'a> fmt::Display for Thing<'a> {
14 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
15 |         debug!("just displayed a Thing wrapping {}", self.0);
16 |         f.write_str(self.0)
17 |     }
18 | }
19 | 
20 | # fn main() {}
21 | ```
22 | 
23 | This structure, and this structure alone, will cause some problems when logging in fern. There are
24 | mitigations, but since it's a fairly niche use case, they are disabled by default.
25 | 
26 | The problems are, depending on which backend you use:
27 | 
28 | - stdout/stderr: logging will 'stutter', with the logs output inside the `Display` implementation
29 |   cutting other log lines down the center
30 | - file: thread will deadlock, and all future logs will also deadlock
31 | 
32 | There are two mitigations you can make, both completely fix this error.
33 | 
34 | The simplest mitigation to this is to enable the `meta-logging-in-format` feature of `fern`. The
35 | disadvantage is that this means fern makes an additional allocation per log call per affected
36 | backend. Not a huge cost, but enough to mean it's disabled by default. To enable this, use the
37 | following in your `Cargo.toml`:
38 | 
39 | ```toml
40 | [dependencies]
41 | # ...
42 | fern = { version = "0.7", features = ["meta-logging-in-format"] }
43 | ```
44 | 
45 | The second mitigation is one you can make inside a formatting closure. This means extra code
46 | complexity, but it also means you can enable it per-logger: the fix isn't global. This fix is also
47 | redundant if you've already enable the above feature. To add the second mitigation, replacec
48 | `format_args!()` with `format!()` as displayed below:
49 | 
50 | ```
51 | fern::Dispatch::new()
52 |     # /*
53 |     ...
54 |     # */
55 |     // instead of doing this:
56 |     .format(move |out, message, record| {
57 |         out.finish(format_args!("[{}] {}", record.level(), message))
58 |     })
59 |     // do this:
60 |     .format(move |out, message, record| {
61 |         let formatted = format!("[{}] {}", record.level(), message);
62 | 
63 |         out.finish(format_args!("{}", formatted))
64 |     })
65 | # ;
66 | ```
67 | 
68 | This second mitigation works by forcing the `Display` implementation to run before any text has
69 | started to log to the backend. There's an additional allocation per log, but it no longer deadlocks!
70 | 
71 | This mitigation also has the advantage of ensuring there's only one call to `Display::fmt`. If youc
72 | use `meta-logging-in-format` and have multiple backends, `Display::fmt` will still be called once
73 | per backend. With this, it will only be called once.
74 | 
75 | ------
76 | 
77 | If you've never experienced this problem, there's no need to fix it - `Display::fmt` and
78 | `Debug::fmt` are normally implemented as "pure" functions with no side effects.
79 | */
80 | 


--------------------------------------------------------------------------------
/src/syslog.rs:
--------------------------------------------------------------------------------
  1 | /*!
  2 | Example usage of `fern` with the `syslog` crate.
  3 | 
  4 | Be sure to depend on `syslog` and the `syslog` feature in `Cargo.toml`:
  5 | 
  6 | ```toml
  7 | [dependencies]
  8 | fern = { version = "0.7", features = ["syslog-6"] }]
  9 | syslog = "6"
 10 | ```
 11 | 
 12 | To use `syslog`, simply create the log you want, and pass it into `Dispatch::chain`:
 13 | 
 14 | ```no_run
 15 | # use syslog6 as syslog;
 16 | # fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
 17 | let formatter = syslog::Formatter3164 {
 18 |     facility: syslog::Facility::LOG_USER,
 19 |     hostname: None,
 20 |     process: "hello-world".to_owned(),
 21 |     pid: 0,
 22 | };
 23 | 
 24 | fern::Dispatch::new()
 25 |     .chain(syslog::unix(formatter)?)
 26 |     .apply()?;
 27 | # Ok(())
 28 | # }
 29 | # fn main() { setup_logging().ok(); }
 30 | ```
 31 | 
 32 | ---
 33 | 
 34 | ## Alternate syslog versions
 35 | 
 36 | If you're using syslog=4.0.0 exactly, one line "ok" will be printed to stdout on log configuration.
 37 | This is [a bug in syslog](https://github.com/Geal/rust-syslog/issues/39), and there is nothing we
 38 | can change in fern to fix that.
 39 | 
 40 | One way to avoid this is to use a different version of `syslog`, `fern` also supports. To pin syslog3,
 41 | use the `syslog-3` feature and depend on `syslog = "3"` instead.
 42 | 
 43 | ```toml
 44 | [dependencies]
 45 | fern = { version = "0.7", features = ["syslog-3"] }]
 46 | syslog = "3"
 47 | ```
 48 | 
 49 | The setup is very similar, except with less configuration to start the syslog logger:
 50 | 
 51 | ```rust
 52 | # use syslog3 as syslog;
 53 | # fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
 54 | fern::Dispatch::new()
 55 |     .chain(syslog::unix(syslog::Facility::LOG_USER)?)
 56 |     .apply()?;
 57 | # Ok(())
 58 | # }
 59 | # fn main() { setup_logging().ok(); }
 60 | ```
 61 | 
 62 | The rest of this document applies to all syslog versions, but the examples will be using
 63 | syslog 6 as it is the latest version.
 64 | 
 65 | ---
 66 | 
 67 | One thing with `syslog` is that you don't generally want to apply any log formatting. The system
 68 | logger will handle that for you.
 69 | 
 70 | However, you probably will want to format messages you also send to stdout! Fortunately, selective
 71 | configuration is easy with fern:
 72 | 
 73 | ```no_run
 74 | # use syslog6 as syslog;
 75 | # fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
 76 | let syslog_formatter = syslog::Formatter3164 {
 77 |     facility: syslog::Facility::LOG_USER,
 78 |     hostname: None,
 79 |     process: "hello-world".to_owned(),
 80 |     pid: 0,
 81 | };
 82 | 
 83 | // top level config
 84 | fern::Dispatch::new()
 85 |     .chain(
 86 |         // console config
 87 |         fern::Dispatch::new()
 88 |             .level(log::LevelFilter::Debug)
 89 |             .format(move |out, message, record| {
 90 |                 out.finish(format_args!(
 91 |                     "[{}] {}",
 92 |                     record.level(),
 93 |                     message,
 94 |                 ))
 95 |             })
 96 |             .chain(std::io::stdout())
 97 |     )
 98 |     .chain(
 99 |         // syslog config
100 |         fern::Dispatch::new()
101 |             .level(log::LevelFilter::Info)
102 |             .chain(syslog::unix(syslog_formatter)?)
103 |     )
104 |     .apply()?;
105 | # Ok(())
106 | # }
107 | # fn main() { setup_logging().ok(); }
108 | ```
109 | 
110 | With this, all info and above messages will be sent to the syslog with no formatting, and
111 | the messages sent to the console will still look nice as usual.
112 | 
113 | ---
114 | 
115 | One last pattern you might want to know: creating a log target which must be explicitly mentioned
116 | in order to work.
117 | 
118 | ```no_run
119 | # use syslog6 as syslog;
120 | # fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
121 | # let formatter = syslog::Formatter3164 {
122 | #     facility: syslog::Facility::LOG_USER,
123 | #     hostname: None,
124 | #     process: "hello-world".to_owned(),
125 | #     pid: 0,
126 | # };
127 | fern::Dispatch::new()
128 |     // by default only accept warning messages from libraries so we don't spam
129 |     .level(log::LevelFilter::Warn)
130 |     // but accept Info and Debug if we explicitly mention syslog
131 |     .level_for("explicit-syslog", log::LevelFilter::Debug)
132 |     .chain(syslog::unix(formatter)?)
133 |     .apply()?;
134 | # Ok(())
135 | # }
136 | # fn main() { setup_logging().ok(); }
137 | ```
138 | 
139 | With this configuration, only warning messages will get through by default. If we do want to
140 | send info or debug messages, we can do so explicitly:
141 | 
142 | ```no_run
143 | # use log::{debug, info, warn};
144 | # fn main() {
145 | debug!("this won't get through");
146 | // especially useful if this is from library you depend on.
147 | info!("neither will this");
148 | warn!("this will!");
149 | 
150 | info!(target: "explicit-syslog", "this will also show up!");
151 | # }
152 | ```
153 | */
154 | 


--------------------------------------------------------------------------------
/tests/channel_logging.rs:
--------------------------------------------------------------------------------
 1 | //! Tests!
 2 | use log::Level::*;
 3 | 
 4 | mod support;
 5 | 
 6 | use support::manual_log;
 7 | 
 8 | #[test]
 9 | fn test_channel_logging() {
10 |     use std::sync::mpsc;
11 |     // Create the channel
12 |     let (send, recv) = mpsc::channel();
13 | 
14 |     let (_max_level, logger) = fern::Dispatch::new().chain(send).into_log();
15 | 
16 |     let l = &*logger;
17 |     manual_log(l, Info, "message1");
18 |     manual_log(l, Info, "message2");
19 | 
20 |     logger.flush();
21 | 
22 |     assert_eq!(recv.recv().unwrap(), "message1\n");
23 |     assert_eq!(recv.recv().unwrap(), "message2\n");
24 | }
25 | 


--------------------------------------------------------------------------------
/tests/enabled_is_deep_check.rs:
--------------------------------------------------------------------------------
 1 | //! See https://github.com/daboross/fern/issues/38
 2 | use log::log_enabled;
 3 | 
 4 | #[test]
 5 | fn ensure_enabled_is_a_deep_check() {
 6 |     let dummy = fern::Dispatch::new()
 7 |         .level(log::LevelFilter::Warn)
 8 |         .chain(std::io::stdout());
 9 | 
10 |     let stdout = fern::Dispatch::new()
11 |         .level(log::LevelFilter::Info)
12 |         .level_for("abc", log::LevelFilter::Debug)
13 |         .chain(std::io::stdout());
14 | 
15 |     fern::Dispatch::new()
16 |         .chain(stdout)
17 |         .chain(dummy)
18 |         .apply()
19 |         .unwrap();
20 | 
21 |     assert!(!log_enabled!(log::Level::Debug));
22 | }
23 | 


--------------------------------------------------------------------------------
/tests/file_logging.rs:
--------------------------------------------------------------------------------
  1 | //! Tests!
  2 | use std::{fs, io, io::prelude::*};
  3 | 
  4 | use log::Level::*;
  5 | 
  6 | mod support;
  7 | 
  8 | use support::manual_log;
  9 | 
 10 | #[test]
 11 | fn test_basic_logging_file_logging() {
 12 |     // Create a temporary directory to put a log file into for testing
 13 |     let temp_log_dir = tempfile::tempdir().expect("Failed to set up temporary directory");
 14 |     let log_file = temp_log_dir.path().join("test.log");
 15 | 
 16 |     {
 17 |         // Create a basic logger configuration
 18 |         let (_max_level, logger) = fern::Dispatch::new()
 19 |             .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
 20 |             .level(log::LevelFilter::Info)
 21 |             .chain(io::stdout())
 22 |             .chain(fern::log_file(log_file).expect("Failed to open log file"))
 23 |             .into_log();
 24 | 
 25 |         let l = &*logger;
 26 |         manual_log(l, Trace, "SHOULD NOT DISPLAY");
 27 |         manual_log(l, Debug, "SHOULD NOT DISPLAY");
 28 |         manual_log(l, Info, "Test information message");
 29 |         manual_log(l, Warn, "Test warning message");
 30 |         manual_log(l, Error, "Test error message");
 31 | 
 32 |         // ensure all File objects are dropped and OS buffers are flushed.
 33 |         log::logger().flush();
 34 | 
 35 |         {
 36 |             let result = {
 37 |                 let mut log_read = fs::File::open(temp_log_dir.path().join("test.log")).unwrap();
 38 |                 let mut buf = String::new();
 39 |                 log_read.read_to_string(&mut buf).unwrap();
 40 |                 buf
 41 |             };
 42 |             assert!(
 43 |                 !result.contains("SHOULD NOT DISPLAY"),
 44 |                 "expected result not including \"SHOULD_NOT_DISPLAY\", found:\n```\n{}\n```\n",
 45 |                 result
 46 |             );
 47 |             assert!(
 48 |                 result.contains("[INFO] Test information message"),
 49 |                 "expected result including \"[INFO] Test information message\", found:\n```\n{}\n```\n",
 50 |                 result
 51 |             );
 52 |             assert!(
 53 |                 result.contains("[WARN] Test warning message"),
 54 |                 "expected result including \"[WARN] Test warning message\", found:\n```\n{}\n```\n",
 55 |                 result
 56 |             );
 57 |             assert!(
 58 |                 result.contains("[ERROR] Test error message"),
 59 |                 "expected result to not include \"[ERROR] Test error message\", found:\n```\n{}\n```\n",
 60 |                 result
 61 |             );
 62 |         }
 63 |     } // ensure logger is dropped before temp dir
 64 | 
 65 |     temp_log_dir
 66 |         .close()
 67 |         .expect("Failed to clean up temporary directory");
 68 | }
 69 | 
 70 | #[test]
 71 | fn test_custom_line_separators() {
 72 |     // Create a temporary directory to put a log file into for testing
 73 |     let temp_log_dir = tempfile::tempdir().expect("Failed to set up temporary directory");
 74 |     let log_file = temp_log_dir.path().join("test_custom_line_sep.log");
 75 | 
 76 |     {
 77 |         // Create a basic logger configuration
 78 |         let (_max_level, logger) = fern::Dispatch::new()
 79 |             // default format is just the message if not specified
 80 |             // default log level is 'trace' if not specified (logs all messages)
 81 |             // output to the log file with the "\r\n" line separator.
 82 |             .chain(fern::Output::file(
 83 |                 fern::log_file(log_file).expect("Failed to open log file"),
 84 |                 "\r\n",
 85 |             ))
 86 |             .into_log();
 87 | 
 88 |         let l = &*logger;
 89 |         manual_log(l, Info, "message1");
 90 |         manual_log(l, Info, "message2");
 91 | 
 92 |         // ensure all File objects are dropped and OS buffers are flushed.
 93 |         logger.flush();
 94 | 
 95 |         {
 96 |             let result = {
 97 |                 let mut log_read =
 98 |                     fs::File::open(temp_log_dir.path().join("test_custom_line_sep.log")).unwrap();
 99 |                 let mut buf = String::new();
100 |                 log_read.read_to_string(&mut buf).unwrap();
101 |                 buf
102 |             };
103 |             assert_eq!(&result, "message1\r\nmessage2\r\n");
104 |         }
105 |     } // ensure logger is dropped before temp dir
106 | 
107 |     temp_log_dir
108 |         .close()
109 |         .expect("Failed to clean up temporary directory");
110 | }
111 | 


--------------------------------------------------------------------------------
/tests/global_logging.rs:
--------------------------------------------------------------------------------
 1 | //! Tests!
 2 | use std::sync::{Arc, Mutex};
 3 | 
 4 | use log::{debug, error, info, trace, warn};
 5 | 
 6 | /// Custom logger built to verify our exact test case.
 7 | struct LogVerify {
 8 |     info: bool,
 9 |     warn: bool,
10 |     error: bool,
11 | }
12 | 
13 | impl LogVerify {
14 |     fn new() -> Self {
15 |         LogVerify {
16 |             info: false,
17 |             warn: false,
18 |             error: false,
19 |         }
20 |     }
21 |     fn log(&mut self, record: &log::Record) {
22 |         let formatted_message = format!("{}", record.args());
23 |         match &*formatted_message {
24 |             "[INFO] Test information message" => {
25 |                 assert!(!self.info, "expected only one info message");
26 |                 self.info = true;
27 |             }
28 |             "[WARN] Test warning message" => {
29 |                 assert!(!self.warn, "expected only one warn message");
30 |                 self.warn = true;
31 |             }
32 |             "[ERROR] Test error message" => {
33 |                 assert!(!self.error, "expected only one error message");
34 |                 self.error = true;
35 |             }
36 |             other => panic!("unexpected message: '{}'", other),
37 |         }
38 |     }
39 | }
40 | /// Wrapper for our verification which acts as the actual logger.
41 | #[derive(Clone)]
42 | struct LogVerifyWrapper(Arc<Mutex<LogVerify>>);
43 | 
44 | impl LogVerifyWrapper {
45 |     fn new() -> Self {
46 |         LogVerifyWrapper(Arc::new(Mutex::new(LogVerify::new())))
47 |     }
48 | 
49 |     fn cloned_boxed_logger(&self) -> Box<dyn log::Log> {
50 |         Box::new(self.clone())
51 |     }
52 | }
53 | 
54 | impl log::Log for LogVerifyWrapper {
55 |     fn enabled(&self, _: &log::Metadata) -> bool {
56 |         true
57 |     }
58 |     fn log(&self, record: &log::Record) {
59 |         self.0.lock().unwrap().log(record);
60 |     }
61 |     fn flush(&self) {}
62 | }
63 | 
64 | #[test]
65 | fn test_global_logger() {
66 |     let verify = LogVerifyWrapper::new();
67 | 
68 |     // Create a basic logger configuration
69 |     fern::Dispatch::new()
70 |         .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
71 |         // Only log messages Info and above
72 |         .level(log::LevelFilter::Info)
73 |         // Output to our verification logger for verification
74 |         .chain(verify.cloned_boxed_logger())
75 |         .apply()
76 |         .expect("Failed to initialize logger: global logger already set!");
77 | 
78 |     trace!("SHOULD NOT DISPLAY");
79 |     debug!("SHOULD NOT DISPLAY");
80 |     info!("Test information message");
81 |     warn!("Test warning message");
82 |     error!("Test error message");
83 | 
84 |     // ensure all buffers are flushed.
85 |     log::logger().flush();
86 | 
87 |     let verify_acquired = verify.0.lock().unwrap();
88 |     assert!(verify_acquired.info, "expected info message to be received");
89 |     assert!(verify_acquired.warn, "expected warn message to be received");
90 |     assert!(
91 |         verify_acquired.error,
92 |         "expected error message to be received"
93 |     );
94 | }
95 | 


--------------------------------------------------------------------------------
/tests/meta_logging.rs:
--------------------------------------------------------------------------------
 1 | //! This provides testing of the 'meta-logging' feature, which allows for
 2 | //! deadlock-free logging within logging formatters.
 3 | //!
 4 | //! These tests *will* deadlock if the feature is not enabled, so they're
 5 | //! disabled by default.
 6 | #![cfg(feature = "meta-logging-in-format")]
 7 | use std::{fmt, fs, io, io::prelude::*};
 8 | 
 9 | use log::{Level::*, Log};
10 | 
11 | mod support;
12 | 
13 | use support::manual_log;
14 | 
15 | // in order to actually trigger the situation that deadlocks, we need a custom
16 | // Display implementation which performs logging:
17 | struct VerboseDisplayThing<'a> {
18 |     log_copy: &'a dyn Log,
19 |     msg: &'a str,
20 | }
21 | 
22 | impl fmt::Display for VerboseDisplayThing<'_> {
23 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
24 |         manual_log(
25 |             self.log_copy,
26 |             Debug,
27 |             format_args!(
28 |                 "VerboseDisplayThing is being displayed! [contents: {}]",
29 |                 self.msg
30 |             ),
31 |         );
32 |         f.write_str(self.msg)
33 |     }
34 | }
35 | 
36 | #[test]
37 | fn file_deadlock() {
38 |     // Create a temporary directory to put a log file into for testing
39 |     let temp_log_dir = tempfile::tempdir().expect("Failed to set up temporary directory");
40 |     let log_file = temp_log_dir.path().join("test.log");
41 | 
42 |     {
43 |         let (_max_level, logger) = fern::Dispatch::new()
44 |             .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
45 |             .chain(io::stdout())
46 |             .chain(fern::log_file(log_file).expect("Failed to open log file"))
47 |             .into_log();
48 | 
49 |         let l = &*logger;
50 | 
51 |         manual_log(
52 |             l,
53 |             Info,
54 |             format_args!(
55 |                 "Hello, world! {}",
56 |                 VerboseDisplayThing {
57 |                     log_copy: l,
58 |                     msg: "it's verbose!",
59 |                 }
60 |             ),
61 |         );
62 | 
63 |         // ensure all File objects are dropped and OS buffers are flushed.
64 |         log::logger().flush();
65 | 
66 |         {
67 |             let contents = {
68 |                 let mut log_read = fs::File::open(temp_log_dir.path().join("test.log")).unwrap();
69 |                 let mut buf = String::new();
70 |                 log_read.read_to_string(&mut buf).unwrap();
71 |                 buf
72 |             };
73 |             assert_eq!(
74 |                 contents,
75 |                 // double logs because we're logging to stdout & the file
76 |                 "[DEBUG] VerboseDisplayThing is being displayed! [contents: it's verbose!]\
77 |                  \n[DEBUG] VerboseDisplayThing is being displayed! [contents: it's verbose!]\
78 |                  \n[INFO] Hello, world! it's verbose!\n"
79 |             );
80 |         }
81 |     } // ensure logger is dropped before temp dir
82 | 
83 |     temp_log_dir
84 |         .close()
85 |         .expect("Failed to clean up temporary directory");
86 | }
87 | 


--------------------------------------------------------------------------------
/tests/panic_logging.rs:
--------------------------------------------------------------------------------
 1 | //! Test the functionality of panicking on error+ log messages.
 2 | use log::Level::*;
 3 | 
 4 | mod support;
 5 | 
 6 | use support::manual_log;
 7 | 
 8 | #[test]
 9 | #[should_panic(expected = "special panic message here")]
10 | fn test_panic_panics() {
11 |     let (_max_level, logger) = fern::Dispatch::new().chain(fern::Panic).into_log();
12 | 
13 |     let l = &*logger;
14 | 
15 |     manual_log(l, Info, "special panic message here");
16 | }
17 | 
18 | fn warn_and_higher_panics_config() -> Box<dyn log::Log> {
19 |     let (_max_level, logger) = fern::Dispatch::new()
20 |         .chain(
21 |             fern::Dispatch::new()
22 |                 .level(log::LevelFilter::Warn)
23 |                 .chain(fern::Panic),
24 |         )
25 |         .chain(std::io::stdout())
26 |         .into_log();
27 |     logger
28 | }
29 | 
30 | #[test]
31 | fn double_chained_with_panics_no_info_panic() {
32 |     let l = &*warn_and_higher_panics_config();
33 | 
34 |     manual_log(l, Info, "this should not panic");
35 | }
36 | 
37 | #[test]
38 | #[should_panic(expected = "this should panic")]
39 | fn double_chained_with_panics_yes_error_panic() {
40 |     let l = &*warn_and_higher_panics_config();
41 | 
42 |     manual_log(l, Error, "this should panic");
43 | }
44 | 


--------------------------------------------------------------------------------
/tests/reopen_logging.rs:
--------------------------------------------------------------------------------
  1 | //! Tests!
  2 | #![cfg(all(not(windows), feature = "reopen-1"))]
  3 | use std::{fs, io, io::prelude::*};
  4 | 
  5 | use log::Level::*;
  6 | 
  7 | mod support;
  8 | 
  9 | use support::manual_log;
 10 | 
 11 | #[test]
 12 | fn test_basic_logging_reopen_logging() {
 13 |     // Create a temporary directory to put a log file into for testing
 14 |     let temp_log_dir = tempfile::tempdir().expect("Failed to set up temporary directory");
 15 |     let log_file = temp_log_dir.path().join("test.log");
 16 | 
 17 |     {
 18 |         // Create a basic logger configuration
 19 |         let (_max_level, logger) = fern::Dispatch::new()
 20 |             .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
 21 |             .level(log::LevelFilter::Info)
 22 |             .chain(io::stdout())
 23 |             .chain(fern::log_reopen1(&log_file, None).expect("Failed to open log file"))
 24 |             .into_log();
 25 | 
 26 |         let l = &*logger;
 27 |         manual_log(l, Trace, "SHOULD NOT DISPLAY");
 28 |         manual_log(l, Debug, "SHOULD NOT DISPLAY");
 29 |         manual_log(l, Info, "Test information message");
 30 |         manual_log(l, Warn, "Test warning message");
 31 |         manual_log(l, Error, "Test error message");
 32 | 
 33 |         // ensure all File objects are dropped and OS buffers are flushed.
 34 |         log::logger().flush();
 35 | 
 36 |         {
 37 |             let result = {
 38 |                 let mut log_read = fs::File::open(temp_log_dir.path().join("test.log")).unwrap();
 39 |                 let mut buf = String::new();
 40 |                 log_read.read_to_string(&mut buf).unwrap();
 41 |                 buf
 42 |             };
 43 |             assert!(
 44 |                 !result.contains("SHOULD NOT DISPLAY"),
 45 |                 "expected result not including \"SHOULD_NOT_DISPLAY\", found:\n```\n{}\n```\n",
 46 |                 result
 47 |             );
 48 |             assert!(
 49 |                 result.contains("[INFO] Test information message"),
 50 |                 "expected result including \"[INFO] Test information message\", found:\n```\n{}\n```\n",
 51 |                 result
 52 |             );
 53 |             assert!(
 54 |                 result.contains("[WARN] Test warning message"),
 55 |                 "expected result including \"[WARN] Test warning message\", found:\n```\n{}\n```\n",
 56 |                 result
 57 |             );
 58 |             assert!(
 59 |                 result.contains("[ERROR] Test error message"),
 60 |                 "expected result to not include \"[ERROR] Test error message\", found:\n```\n{}\n```\n",
 61 |                 result
 62 |             );
 63 |         }
 64 |     } // ensure logger is dropped before temp dir
 65 | 
 66 |     temp_log_dir
 67 |         .close()
 68 |         .expect("Failed to clean up temporary directory");
 69 | }
 70 | 
 71 | #[test]
 72 | fn test_custom_line_separators() {
 73 |     // Create a temporary directory to put a log file into for testing
 74 |     let temp_log_dir = tempfile::tempdir().expect("Failed to set up temporary directory");
 75 |     let log_file = temp_log_dir.path().join("test_custom_line_sep.log");
 76 | 
 77 |     {
 78 |         // Create a basic logger configuration
 79 |         let (_max_level, logger) = fern::Dispatch::new()
 80 |             // default format is just the message if not specified
 81 |             // default log level is 'trace' if not specified (logs all messages)
 82 |             // output to the log file with the "\r\n" line separator.
 83 |             .chain(fern::Output::reopen1(
 84 |                 fern::log_reopen1(&log_file, None).expect("Failed to open log file"),
 85 |                 "\r\n",
 86 |             ))
 87 |             .into_log();
 88 | 
 89 |         let l = &*logger;
 90 |         manual_log(l, Info, "message1");
 91 |         manual_log(l, Info, "message2");
 92 | 
 93 |         // ensure all File objects are dropped and OS buffers are flushed.
 94 |         logger.flush();
 95 | 
 96 |         {
 97 |             let result = {
 98 |                 let mut log_read =
 99 |                     fs::File::open(temp_log_dir.path().join("test_custom_line_sep.log")).unwrap();
100 |                 let mut buf = String::new();
101 |                 log_read.read_to_string(&mut buf).unwrap();
102 |                 buf
103 |             };
104 |             assert_eq!(&result, "message1\r\nmessage2\r\n");
105 |         }
106 |     } // ensure logger is dropped before temp dir
107 | 
108 |     temp_log_dir
109 |         .close()
110 |         .expect("Failed to clean up temporary directory");
111 | }
112 | 


--------------------------------------------------------------------------------
/tests/support.rs:
--------------------------------------------------------------------------------
 1 | //! Support module for tests
 2 | use std::fmt;
 3 | 
 4 | /// Utility to manually enter a log message into a logger. All extra metadata
 5 | /// (target, line number, etc) will be blank.
 6 | pub fn manual_log<T, U>(logger: &T, level: log::Level, message: U)
 7 | where
 8 |     T: log::Log + ?Sized,
 9 |     U: fmt::Display,
10 | {
11 |     logger.log(
12 |         &log::RecordBuilder::new()
13 |             .args(format_args!("{}", message))
14 |             .level(level)
15 |             .build(),
16 |     );
17 | }
18 | 


--------------------------------------------------------------------------------
/tests/write_logging.rs:
--------------------------------------------------------------------------------
 1 | //! Tests for the raw write logging functionality.
 2 | use std::{
 3 |     io,
 4 |     sync::{
 5 |         atomic::{AtomicBool, Ordering},
 6 |         Arc,
 7 |     },
 8 | };
 9 | 
10 | use log::Level::*;
11 | 
12 | mod support;
13 | 
14 | use support::manual_log;
15 | 
16 | #[test]
17 | fn test_raw_write_logging() {
18 |     struct TestWriter {
19 |         buf: Vec<u8>,
20 |         flag: Arc<AtomicBool>,
21 |     }
22 | 
23 |     impl io::Write for TestWriter {
24 |         fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
25 |             self.buf.write(buf)
26 |         }
27 | 
28 |         fn flush(&mut self) -> io::Result<()> {
29 |             self.buf.flush()?;
30 | 
31 |             let expected = b"[INFO] Test information message\n";
32 | 
33 |             if self.buf == expected {
34 |                 self.flag.store(true, Ordering::SeqCst);
35 |             } else {
36 |                 eprintln!("{:?} does not match {:?}", self.buf, expected);
37 |             }
38 | 
39 |             Ok(())
40 |         }
41 |     }
42 | 
43 |     let flag = Arc::new(AtomicBool::new(false));
44 | 
45 |     // Create a basic logger configuration
46 |     let (_max_level, logger) = fern::Dispatch::new()
47 |         .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
48 |         .level(log::LevelFilter::Info)
49 |         .chain(io::stdout())
50 |         .chain(Box::new(TestWriter {
51 |             buf: Vec::new(),
52 |             flag: flag.clone(),
53 |         }) as Box<dyn io::Write + Send>)
54 |         .into_log();
55 | 
56 |     let l = &*logger;
57 |     manual_log(l, Info, "Test information message");
58 | 
59 |     // ensure all File objects are dropped and OS buffers are flushed.
60 |     log::logger().flush();
61 | 
62 |     assert!(
63 |         flag.load(Ordering::SeqCst),
64 |         "raw Write test failed: did not match buffer"
65 |     );
66 | }
67 | 


--------------------------------------------------------------------------------