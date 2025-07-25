// SPDX-FileCopyrightText: © The `magic-sys` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::process::ExitCode;

#[cfg(any(feature = "pkg-config", feature = "vcpkg"))]
enum LibraryResult<E, L> {
    Skipped(E),
    Failed(E),
    Success(L),
}

#[cfg(feature = "pkg-config")]
fn try_pkgconfig() -> LibraryResult<pkg_config::Error, pkg_config::Library> {
    let lib = pkg_config::Config::new()
        .atleast_version("5.39")
        .probe("libmagic");
    match lib {
        Err(err) => match err {
            pkg_config::Error::EnvNoPkgConfig(_) => LibraryResult::Skipped(err),
            _ => LibraryResult::Failed(err),
        },
        Ok(lib) => LibraryResult::Success(lib),
    }
}

#[cfg(feature = "vcpkg")]
fn try_vcpkg() -> LibraryResult<vcpkg::Error, vcpkg::Library> {
    let lib = vcpkg::find_package("libmagic");
    match lib {
        Err(err) => match err {
            vcpkg::Error::DisabledByEnv(_) => LibraryResult::Skipped(err),
            _ => LibraryResult::Failed(err),
        },
        Ok(lib) => {
            // workaround, see https://github.com/robo9k/rust-magic-sys/pull/16#issuecomment-949094327
            println!("cargo:rustc-link-lib=shlwapi");

            LibraryResult::Success(lib)
        }
    }
}

fn main() -> ExitCode {
    #[cfg(feature = "pkg-config")]
    {
        let lib = try_pkgconfig();
        match lib {
            LibraryResult::Skipped(err) => {
                println!("pkg-config skipped: {err}");
            }
            LibraryResult::Failed(err) => {
                println!("cargo:warning=pkg-config failed: {err}");
            }
            LibraryResult::Success(lib) => {
                let includes = std::env::join_paths(lib.include_paths.iter())
                    .expect("parsed include paths from `pkg-config` should be joinable");
                if includes.is_empty() {
                    let includedir = pkg_config::get_variable("libmagic", "includedir")
                        .expect("`pkg-config` package 'libmagic' should be found previously");
                    println!("cargo:include={includedir}");
                } else {
                    println!("cargo:include={}", includes.to_string_lossy());
                }

                println!("pkg-config success: {lib:?}");
                return ExitCode::SUCCESS;
            }
        }
    }
    #[cfg(not(feature = "pkg-config"))]
    println!("pkg-config feature disabled");

    #[cfg(feature = "vcpkg")]
    {
        let lib = try_vcpkg();
        match lib {
            LibraryResult::Skipped(err) => {
                println!("vcpkg skipped: {err}");
            }
            LibraryResult::Failed(err) => {
                println!("cargo:warning=vcpkg failed: {err}");
            }
            LibraryResult::Success(lib) => {
                let includes = std::env::join_paths(lib.include_paths.iter())
                    .expect("include paths from `vcpkg` should be joinable");
                if !includes.is_empty() {
                    println!("cargo:include={}", includes.to_string_lossy());
                }

                println!("vcpkg success: {lib:?}");
                return ExitCode::SUCCESS;
            }
        }
    }
    #[cfg(not(feature = "vcpkg"))]
    println!("vcpkg feature disabled");

    // if we're reach here, this means that either both pkg-config and vcpkg
    // got skipped or
    // both failed or
    // both features are disabled
    #[cfg(not(any(feature = "pkg-config", feature = "vcpkg")))]
    eprintln!(
        "the pkg-config and vcpkg features are both disabled, \
        this configuration requires you to override the build script: \
        https://crates.io/crates/magic#override"
    );

    eprintln!("could not link to `libmagic`");
    ExitCode::FAILURE
}
