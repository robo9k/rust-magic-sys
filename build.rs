// SPDX-FileCopyrightText: © The `magic-sys` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

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

#[cfg(feature = "bundled")]
fn try_bundled() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    let include_dir = out_dir.join("include");

    // First, copy magic.h.in into out_dir/include/magic.h, replacing the X.YY
    // string with the actual versionversion.
    std::fs::create_dir_all(&include_dir).unwrap();
    let mut data = std::fs::read_to_string("file/src/magic.h.in").unwrap();
    data = data.replace("X.YY", "5.45");
    std::fs::write(include_dir.join("magic.h"), &data).unwrap();

    cc::Build::new()
        .include("file/src")
        .include(include_dir)
        .define("HAVE_UNISTD_H", "1")
        .define("HAVE_INTTYPES_H", "1")
        .define("VERSION", "5.45")
        .file("file/src/buffer.c")
        .file("file/src/magic.c")
        .file("file/src/apprentice.c")
        .file("file/src/softmagic.c")
        .file("file/src/ascmagic.c")
        .file("file/src/encoding.c")
        .file("file/src/compress.c")
        .file("file/src/is_csv.c")
        .file("file/src/is_json.c")
        .file("file/src/is_simh.c")
        .file("file/src/is_tar.c")
        .file("file/src/readelf.c")
        .file("file/src/print.c")
        .file("file/src/fsmagic.c")
        .file("file/src/funcs.c")
        .file("file/src/apptype.c")
        .file("file/src/der.c")
        .file("file/src/cdf.c")
        .file("file/src/cdf_time.c")
        .file("file/src/readcdf.c")
        .file("file/src/fmtcheck.c")
        .compile("magic");
}

fn main() {
    #[cfg(feature = "bundled")]
    {
        let lib = try_bundled();
        return;
    }
    #[cfg(feature = "pkg-config")]
    {
        let lib = try_pkgconfig();
        match lib {
            LibraryResult::Skipped(err) => {
                println!("pkg-config skipped: {}", err);
            }
            LibraryResult::Failed(err) => {
                println!("cargo:warning=pkg-config failed: {}", err);
            }
            LibraryResult::Success(lib) => {
                println!("pkg-config success: {:?}", lib);
                return;
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
                println!("vcpkg skipped: {}", err);
            }
            LibraryResult::Failed(err) => {
                println!("cargo:warning=vcpkg failed: {}", err);
            }
            LibraryResult::Success(lib) => {
                println!("vcpkg success: {:?}", lib);
                return;
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
    println!(
        "the pkg-config and vcpkg features are both disabled, \
        this configuration requires you to override the build script: \
        https://crates.io/crates/magic#override"
    );

    panic!("could not link to `libmagic`");
}
