# RustyDroid-Lib
An Android Library with Rust integration!

### Prerequisites
The Android NDK, Rust 1.85 (edition 2024) or higher and Linux.

### Usage
Configure the (absolute) path of the NDK-clang in `rust/.cargo/config.toml` file. This config is applicable for all crates in `rust` folder. Replace all '<>'.

You can add native android dependencies to crates individually using `build.rustflags` key in individual crate's config.toml.

Use this command to compile a rust crate:
```
cargo b --release --target <arch>
```


Move the generated libs from `target` to the `main/jniLibs/<target_abi>` folders

Failure to do any of these will lead to an incomplete library.

And sorry, Android Studio cant help managing rust sources. Prefer VSCode/RustRover for Rust.

To build the library, use:
```
./gradlew assemble
```
