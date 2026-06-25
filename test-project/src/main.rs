//! Minimal miniquad application used as the end-to-end build target for
//! cargo-quad-apk. It does nothing but open a window and clear it — enough to
//! exercise the full APK pipeline (Rust -> cdylib via the NDK, JNI glue
//! injection, manifest generation, aapt2/d8/javac, signing).

use miniquad::*;

struct Stage;

impl EventHandler for Stage {
    fn update(&mut self) {}
    fn draw(&mut self) {}
}

fn main() {
    miniquad::start(conf::Conf::default(), || Box::new(Stage));
}
