extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
    // Use system libassimp if it exists
    if let Ok(..) = pkg_config::Config::new().atleast_version("4.0.0").find("assimp") {
        return
    }

    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let mut config = Config::new("assimp");
    config
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_INSTALL_PDB", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("LIBRARY_SUFFIX", "");

    // On macOS, ensure the compiler can find C++ stdlib headers (<cmath> etc.).
    // The cmake crate reads CXXFLAGS from the process env when constructing CMAKE_CXX_FLAGS,
    // so we inject the CLT libc++ include path there directly.
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("xcrun").arg("--show-sdk-path").output() {
            let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let sdk_cxx = format!("{}/usr/include/c++/v1", sdk_path);
            if std::path::Path::new(&sdk_cxx).exists() {
                let existing = std::env::var("CXXFLAGS").unwrap_or_default();
                unsafe {
                    std::env::set_var("CXXFLAGS", format!("-I{} {}", sdk_cxx, existing).trim().to_string());
                }
            }
            config.define("CMAKE_OSX_SYSROOT", sdk_path);
        }
    }

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    // Link to assimp and its dependencies
    let debug_postfix = if env::var("DEBUG").unwrap() == "true" { "d" } else { "" };
    println!("cargo:rustc-link-lib=static=assimp{}", debug_postfix);
    println!("cargo:rustc-link-lib=static=IrrXML{}", debug_postfix);
    let zlib_static = dst.join("lib").join(format!("libzlibstatic{}.a", debug_postfix));
    if zlib_static.exists() {
        println!("cargo:rustc-link-lib=static=zlibstatic{}", debug_postfix);
    } else if !pkg_config::find_library("zlib").is_ok() {
        // cmake used the system zlib; link it directly
        println!("cargo:rustc-link-lib=z");
    }

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }


    println!("cargo:rerun-if-changed=build.rs");
}
