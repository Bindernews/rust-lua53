extern crate bindgen;
extern crate cc;

use std::io;
use std::env;
use std::path::PathBuf;
use std::process::Command;

trait CommandExt {
    fn execute(&mut self) -> io::Result<()>;
}

impl CommandExt for Command {
    /// Execute the command and return an error if it exited with a failure status.
    fn execute(&mut self) -> io::Result<()> {
        let status = self.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("The command\n\
            \t{:?}\n\
            did not run successfully.", self)))
        }
    }
}

fn main() {
    // Generate 'glue' bindings
    let bindings = bindgen::Builder::default()
        .header("lua-source/glue.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("glue.rs"))
        .expect("Couldn't write bindings!");


    // build archive
	let target_os = env::var("CARGO_CFG_TARGET_OS");
	let target_family = env::var("CARGO_CFG_TARGET_FAMILY");

	let mut config = cc::Build::new();

	if target_os == Ok("linux".to_string()) {
		config.define("LUA_USE_LINUX", None);
	} else if target_os == Ok("macos".to_string()) {
		config.define("LUA_USE_MACOSX", None);
	} else if target_family == Ok("unix".to_string()) {
		config.define("LUA_USE_POSIX", None);
	} else if target_family == Ok("windows".to_string()) {
		config.define("LUA_USE_WINDOWS", None);
	}

	if cfg!(debug_assertions) {
		config.define("LUA_USE_APICHECK", None);
	}

	config
		.include("lua")
		.file("lua-source/src/lapi.c")
		.file("lua-source/src/lauxlib.c")
		.file("lua-source/src/lbaselib.c")
		.file("lua-source/src/lbitlib.c")
		.file("lua-source/src/lcode.c")
		.file("lua-source/src/lcorolib.c")
		.file("lua-source/src/lctype.c")
		.file("lua-source/src/ldblib.c")
		.file("lua-source/src/ldebug.c")
		.file("lua-source/src/ldo.c")
		.file("lua-source/src/ldump.c")
		.file("lua-source/src/lfunc.c")
		.file("lua-source/src/lgc.c")
		.file("lua-source/src/linit.c")
		.file("lua-source/src/liolib.c")
		.file("lua-source/src/llex.c")
		.file("lua-source/src/lmathlib.c")
		.file("lua-source/src/lmem.c")
		.file("lua-source/src/loadlib.c")
		.file("lua-source/src/lobject.c")
		.file("lua-source/src/lopcodes.c")
		.file("lua-source/src/loslib.c")
		.file("lua-source/src/lparser.c")
		.file("lua-source/src/lstate.c")
		.file("lua-source/src/lstring.c")
		.file("lua-source/src/lstrlib.c")
		.file("lua-source/src/ltable.c")
		.file("lua-source/src/ltablib.c")
		.file("lua-source/src/ltm.c")
		.file("lua-source/src/lundump.c")
		.file("lua-source/src/lutf8lib.c")
		.file("lua-source/src/lvm.c")
		.file("lua-source/src/lzio.c")
		.compile("liblua5.3.a");
}
