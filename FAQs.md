### Installing nightly for experimental features
https://www.geeksforgeeks.org/how-to-install-rust-nightly-on-macos/
<hr>

### Cargo build --target <target.json>
```
➜  rust-os git:(master) ✗ cargo build --target x86_64-rust_os.json
   
  Compiling rust-os v0.1.0 (/Users/prateek.jha/Personal/rust-os)
error[E0463]: can't find crate for `core`
  |
  = note: the `x86_64-rust_os` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-rust_os`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error[E0463]: can't find crate for `compiler_builtins`

error[E0463]: can't find crate for `core`
 --> src/main.rs:4:5
  |
4 | use core::panic::PanicInfo;
  |     ^^^^ can't find crate
  |
  = note: the `x86_64-rust_os` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-rust_os`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error: requires `sized` lang_item

For more information about this error, try `rustc --explain E0463`.
error: could not compile `rust-os` due to 4 previous errors
```
https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std
<hr>

### How to switch between toolchains?
``> rustup override set nightly``

OR

``> rustup override set default``

https://stackoverflow.com/questions/58226545/how-to-switch-between-rust-toolchains
<hr>
