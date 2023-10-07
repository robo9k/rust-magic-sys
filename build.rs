// SPDX-FileCopyrightText: © The `magic-sys` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("5.39")
        .probe("libmagic");
    match lib {
        Err(err) => match err {
            pkg_config::Error::EnvNoPkgConfig(_) => {
                println!("pkg-config skipped: {}", err);
            }
            _ => {
                println!("cargo:warning=pkg-config failed: {}", err);
            }
        },
        Ok(lib) => {
            println!("pkg-config success: {:?}", lib);
            return;
        }
    }

    let lib = vcpkg::find_package("libmagic");
    match lib {
        Err(err) => match err {
            vcpkg::Error::DisabledByEnv(_) => {
                println!("vcpkg skipped: {}", err);
            }
            _ => {
                println!("cargo:warning=vcpkg failed: {}", err);
            }
        },
        Ok(lib) => {
            println!("vcpkg success: {:?}", lib);
            return;
        }
    }

    // if we're reach here, this means that either both pkg-config and vcpkg got skipped or both failed
    panic!("could not link to `libmagic` with neither `pkg-config` nor `vcpkg`");
}
