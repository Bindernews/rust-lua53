# rust-lua53 [![Build Status](https://travis-ci.org/jcmoyer/rust-lua53.svg?branch=master)](https://travis-ci.org/jcmoyer/rust-lua53) [![Documentation](https://docs.rs/lua/badge.svg)](https://docs.rs/lua)
Aims to be complete Rust bindings for Lua 5.3 and beyond. Currently, `master`
is tracking Lua `5.3.5`.

Requires a Unix-like environment. On Windows, [MSYS2](https://msys2.github.io/)
is supported.

You will need:
- wget (fetch on FreeBSD/Dragonfly, curl on MacOS)
- tar
- make
- gcc

### Using crates.io

Add this to your `Cargo.toml`:

```
[dependencies]
lua = "*"
```

### Using git

Add this to your `Cargo.toml`:

```
[dependencies.lua]
git = "https://github.com/jcmoyer/rust-lua53"
```

# Example

```rust
extern crate lua;

fn main() {
  let mut state = lua::State::new();
  state.open_libs();
  state.do_string("print('hello world!')");
}
```

# Generating Glue Code
To reduce compile times and build complexity this project ships pre-generated C-Rust bindings.
They can be found in `src/ffi/` and are named `glue_*.rs`. 

To generate new bindings (e.g. for a new platform or updated version) run `cargo install bindgen`.

```
$ cd $PROJECT_DIR
$ bindgen lua-source/glue.h -o src/ffi/<GLUE_FILE>
```

## Running bindgen on Windows
Currently `bindgen` requires `libclang.dll`. This may either be compiled from source or downloaded
prebuilt as part of the `Zig` project. See [this link][1] for details.

# License
Licensed under the MIT License, which is the same license Lua is distributed
under. Refer to `LICENSE.md` for more information.


[1]: https://stackoverflow.com/questions/51310020/is-there-a-pre-built-clang-library-for-windows/51312174