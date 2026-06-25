# cargo-quad-apk (Rust 2024 compatible fork)

[![CI](https://github.com/hoklims/cargo-quad-apk/actions/workflows/ci.yml/badge.svg)](https://github.com/hoklims/cargo-quad-apk/actions/workflows/ci.yml)
[![Android APK](https://github.com/hoklims/cargo-quad-apk/actions/workflows/android-apk.yml/badge.svg)](https://github.com/hoklims/cargo-quad-apk/actions/workflows/android-apk.yml)

A maintained, **Rust 2024 edition–compatible** fork of
[`not-fl3/cargo-quad-apk`](https://github.com/not-fl3/cargo-quad-apk) — the cargo
subcommand that builds Android `.apk` packages for
[miniquad](https://github.com/not-fl3/miniquad) / [macroquad](https://github.com/not-fl3/macroquad)
projects.

It is itself a fork of the old `android-rs-glue` crate, specialized for building
miniquad-based projects.

```sh
cargo quad-apk build --example my_game
```

---

## Why this fork exists

`cargo-quad-apk` does not call the `cargo` binary — it **links the `cargo` crate
as a library** and drives the build through cargo's internal API (a custom
`Executor` rewrites every `rustc` invocation to produce a `cdylib`, injects the
Android linker flags and the miniquad JNI glue code, then assembles the APK with
the NDK/SDK tools).

That internal API **is not stable**. The `cargo` team makes no guarantees about
it, and it changes every Rust release. As a result the upstream tool bit-rots:

- On a recent toolchain it **panics at startup** —
  `command 'logcat' alias 'r' is duplicated` — because two subcommands declared
  the same clap alias, which newer `clap` rejects with a debug assertion.
- It is still pinned to `edition = "2018"`.

This fork fixes those issues and, crucially, **documents and enforces the version
coupling** that makes the tool buildable at all (see below).

## The one rule: `cargo` library version must match your Rust toolchain

The published `cargo` **library** crate is versioned `0.N`, and version `0.N`
matches Rust **`1.N`**:

| `cargo` crate | Rust toolchain |
| ------------- | -------------- |
| `0.95.x`      | `1.95`         |
| `0.96.x`      | `1.96`         |
| `0.97.x`      | `1.97`         |

This repository pins both sides so they can never drift apart:

```toml
# Cargo.toml
cargo = "=0.95.0"
```

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.95.0"
```

**To move to a newer Rust:** bump *both* numbers together (e.g. `=0.96.0` +
`1.96.0`), then run `cargo build` and fix any internal-API drift the compiler
reports. Bumping only one side will fail to compile.

> The embedded `cargo` library understands every edition rustc does (including
> 2024), so once installed this tool builds miniquad/macroquad projects of **any**
> edition — your game crate does not have to be edition 2024.

## Prerequisites

- **Rust 1.95** (the repo's `rust-toolchain.toml` selects it automatically when
  you build inside the repo).
- **Android SDK** with `build-tools` and `platform-tools`, plus the matching
  `platforms/android-<version>/android.jar`.
- **Android NDK** (r23+; the tool uses `llvm-ar` / `llvm-readelf` / unified
  `ld`).
- A **JDK** (`javac`, `keytool`) on `PATH` or via `JAVA_HOME`.
- `zip` on `PATH`.

Environment variables the tool reads:

| Variable | Purpose |
| -------- | ------- |
| `NDK_HOME` | Path to the Android NDK (required) |
| `ANDROID_SDK_HOME` or `ANDROID_HOME` | Path to the Android SDK (required) |
| `JAVA_HOME` | Used to locate `javac` / `keytool` if not on `PATH` |

## Installation

### Recommended — build inside the repo (honors `rust-toolchain.toml`)

```sh
git clone https://github.com/hoklims/cargo-quad-apk
cd cargo-quad-apk
cargo install --path .
```

### With `cargo install --git`

`cargo install --git` builds in a temporary directory and uses your **active**
toolchain, which must match the pinned `cargo` crate. Pin it explicitly:

```sh
cargo +1.95.0 install --git https://github.com/hoklims/cargo-quad-apk
```

(If `1.95.0` is not installed: `rustup toolchain install 1.95.0`.)

## Usage

```sh
cargo quad-apk build [--release] [--example <name>] [--nosign] [--nostrip]
cargo quad-apk run   [--example <name>]    # build, install, and launch on a device
cargo quad-apk install
cargo quad-apk logcat
```

`build` produces APKs under
`target/android-artifacts/<debug|release>/apk/`.

## Supported `[package.metadata.android]` entries

```toml
# The target Android API level. "android_version" is the compile SDK version.
# Defaults to 36. (target_sdk_version defaults to android_version;
# min_sdk_version defaults to 18 — the minimum supported by rustc.)
android_version = 29
target_sdk_version = 29
min_sdk_version = 26

# Targets to build for.
# Defaults to: armv7-linux-androideabi, aarch64-linux-android, i686-linux-android
build_targets = [ "armv7-linux-androideabi", "aarch64-linux-android", "i686-linux-android", "x86_64-linux-android" ]

# The Java package name. Hyphens are converted to underscores.
# Defaults to rust.<target_name> for binaries, rust.<package_name>.example.<target_name> for examples.
package_name = "rust.cargo.apk.advanced"

# User-facing app name. Defaults to the target name.
label = "My Android App"

# Internal version number (integer). Defaults to 1.
version_code = 2

# User-facing version string. Defaults to the cargo package version.
version_name = "2.0"

# Path to a resources folder (optional).
res = "path/to/res_folder"

# Launcher icon (optional), e.g. "@mipmap/ic_launcher".
icon = "@mipmap/ic_launcher"

# Path to an assets folder (optional).
assets = "path/to/assets_folder"

# Run full-screen. Defaults to false.
fullscreen = false

# Max supported OpenGL ES version claimed by the manifest. Defaults to 2.0.
opengles_version_major = 3
opengles_version_minor = 2

# Extra XML attributes on the <application> tag.
[package.metadata.android.application_attributes]
"android:debuggable" = "true"
"android:hardwareAccelerated" = "true"

# Extra XML attributes on the <activity> tag.
[package.metadata.android.activity_attributes]
"android:screenOrientation" = "unspecified"

# uses-feature elements (keys: name, required, version).
[[package.metadata.android.feature]]
name = "android.hardware.vulkan.level"
version = "1"
required = false

# uses-permission elements.
[[package.metadata.android.permission]]
name = "android.permission.CAMERA"

# service elements.
[[package.metadata.android.service]]
name = ".ForegroundService"
foreground_service_type = "remoteMessaging"
exported = false
```

## Build tool environment variables

cargo-quad-apk exposes the NDK C/C++ toolchain to build scripts (so crates using
the `cc` / `cmake` crates build correctly):

- `CC` / `CXX` — NDK `clang` / `clang++` wrappers for the target & platform.
- `AR` — NDK `llvm-ar`.
- `CXXSTDLIB` — `c++` (NDK's full C++ standard library).
- `CMAKE_TOOLCHAIN_FILE`, `CMAKE_GENERATOR`, `CMAKE_MAKE_PROGRAM` — generated CMake
  toolchain pointing at the NDK.

## Building with Docker

Upstream publishes a Docker image for reproducible builds:

```sh
docker run --rm -v "$(pwd)":/root/src -w /root/src notfl3/cargo-apk \
  cargo quad-apk build --example quad
```

## Validation

This repo proves the full pipeline works, it does not just claim it:

- **`test-project/`** is a minimal miniquad app written in **edition 2024**.
- The **Android APK** workflow (`.github/workflows/android-apk.yml`) builds it into
  a real, signed `.apk` on every push — on a clean Ubuntu runner, with NDK r25 and
  the `aarch64-linux-android` target — then **uploads the APK as a downloadable
  artifact**. Grab it from the latest green run on the
  [Actions tab](https://github.com/hoklims/cargo-quad-apk/actions/workflows/android-apk.yml)
  ("quad-android-smoke-apk").
- The **CI** workflow builds + tests the tool and checks formatting on Linux and
  Windows.

> Scope: CI builds a debug APK for one ABI; it does not install or run it on a
> device/emulator. Building the edition-2024 example end-to-end is what verifies
> the JNI glue is injected in an edition-2024-safe way.

### Validating locally

You need the Android SDK + NDK (r25 recommended), a JDK, `zip` on `PATH`, and the
Rust target:

```sh
rustup target add aarch64-linux-android
export NDK_HOME=/path/to/android-sdk/ndk/25.2.9519653
export ANDROID_HOME=/path/to/android-sdk
cargo install --path .
cd test-project
cargo quad-apk build          # -> target/android-artifacts/debug/apk/*.apk
```

(On Windows the same applies; ensure a `zip` executable is on `PATH`.)

## Relationship to upstream & license

This is a compatibility fork; all original authorship is preserved (see
`Cargo.toml`). Dual-licensed under **MIT OR Apache-2.0** — see
[`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE). (Upstream
shipped an Apache-2.0 license file while declaring `MIT` in `Cargo.toml`; this
fork resolves that by offering both, the Rust ecosystem norm.)

See [`CHANGELOG.md`](CHANGELOG.md) for the exact set of changes relative to
`not-fl3/cargo-quad-apk`.

Credit to Pierre Krieger, Philip Alldredge and Fedor Logachev (not-fl3) for the
original work.
