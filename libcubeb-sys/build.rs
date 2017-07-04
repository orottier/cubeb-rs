extern crate cmake;
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn main() {
    if env::var("LIBCUBEB_SYS_USE_PKG_CONFIG").is_ok() {
        if pkg_config::find_library("libcubeb").is_ok() {
            return;
        }
    }

    if !Path::new("libcubeb/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status();
    }

    let target = env::var("TARGET").unwrap();
    //    let host = env::var("HOST").unwrap();
    let windows = target.contains("windows");
    let darwin = target.contains("darwin");
    let mut cfg = cmake::Config::new("libcubeb");

    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));

    env::remove_var("DESTDIR");
    let dst = cfg.define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .build();

    if windows {
        // TBD
        println!("cargo:rustc-link-lib=static=cubeb");
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
    } else if darwin {
        println!("cargo:rustc-link-lib=static=cubeb");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-search=native={}", dst.display());
    } else {
        println!("cargo:rustc-link-lib=static=cubeb");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-search=native={}", dst.display());

        pkg_config::find_library("alsa").unwrap();
        pkg_config::find_library("libpulse").unwrap();
        pkg_config::find_library("jack").unwrap();
    }
}
