# Changelog

All notable changes to this fork are documented here.

## [0.2.0] — Rust 2024 compatibility

Forked from [`not-fl3/cargo-quad-apk`](https://github.com/not-fl3/cargo-quad-apk)
at commit `e6ce7cd`. The goal of this release is to make the tool build and run
on a modern Rust toolchain (Rust 1.95, edition 2024) without changing its build
behavior.

### Fixed

- **Startup panic on modern `clap`.** `cli_run()` and `cli_logcat()` both declared
  the alias `"r"`. `clap` 4.5 rejects duplicate aliases with a debug assertion, so
  *every* invocation (including `--help`) panicked with
  `command 'logcat' alias 'r' is duplicated`. Removed the stray alias from
  `logcat`. (`run` keeps `r`; `build` keeps `b`.)
- **Edition-2024 user crates failed to build.** The tool injects miniquad's
  `mod_inject.rs` glue (`#[no_mangle] pub extern "C" fn quad_main()`) into the
  user's crate, where it is compiled at the *user crate's* edition. In edition
  2024 a bare `#[no_mangle]` is a hard error (`unsafe attribute used without
  unsafe`). The injector now rewrites it to `#[unsafe(no_mangle)]`, which is valid
  in every edition on Rust ≥ 1.82 — so edition-2024 *and* edition-2021 projects
  build. Verified end-to-end in CI by building an edition-2024 miniquad example
  into an APK (see `test-project/` and the "Android APK" workflow).

### Changed

- **Edition `2018` → `2024`.**
- **`cargo` library `0.87` → `=0.95.0`** (matches Rust 1.95). The `cargo` crate has
  no stable API; its version must track the toolchain — see `README.md`.
- **`compile_options` now takes `UserIntent`** instead of `CompileMode`
  (cargo's CLI/compile refactor). `UserIntent::Build` is passed where
  `CompileMode::Build` used to be; `CompileMode` is still used inside the
  `Executor` impl, which is unchanged.
- **Removed `--build-plan`** from `cargo quad-apk build`: cargo dropped the
  `arg_build_plan()` helper and the unstable `--build-plan` feature.
- **`std::env::set_var` calls wrapped in `unsafe`** (required by edition 2024).
  These are set single-threaded, before any compile job is spawned, so the usage
  is sound (documented inline with a `SAFETY:` note).
- Code formatted with `rustfmt` (edition 2024 style).
- Package version `0.1.4` → `0.2.0`.
- **License made consistent.** Upstream shipped an Apache-2.0 `LICENSE` file while
  declaring `MIT` in `Cargo.toml`. This fork is now explicitly dual-licensed
  **MIT OR Apache-2.0**: the Apache text moved to `LICENSE-APACHE`, `LICENSE-MIT`
  added, `Cargo.toml` set to `license = "MIT OR Apache-2.0"`.
- `repository` URL updated to the fork.

### Added

- `rust-toolchain.toml` pinning Rust `1.95.0` in lock-step with `cargo =0.95.0`.
- `README.md` documenting the `cargo`-crate ↔ toolchain version coupling, install
  paths, prerequisites and the `[package.metadata.android]` reference.
- GitHub Actions CI (`.github/workflows/ci.yml`): build + test + `rustfmt --check`
  on Linux and Windows.
- `.gitignore`, `.gitattributes`.

### Not changed (intentionally)

- The Android build pipeline (the custom `Executor`, NDK/SDK tool invocations,
  manifest generation, miniquad JNI glue injection) is byte-for-byte the upstream
  logic. Only the cargo-library API call sites required adapting.
- Long-standing upstream help-text typos are left as-is to keep the diff surgical.
