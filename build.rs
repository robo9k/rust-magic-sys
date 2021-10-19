fn env(name: &str) -> Option<std::ffi::OsString> {
    let target = std::env::var("TARGET").expect("Cargo didn't provide `TARGET` environment var");
    let target = target.to_uppercase().replace("-", "_");
    let prefixed_name = format!("{}_{}", target, name);
    println!("cargo:rerun-if-env-changed={}", prefixed_name);
    match std::env::var_os(prefixed_name) {
        Some(v) => Some(v),
        None => {
            println!("cargo:rerun-if-env-changed={}", name);
            std::env::var_os(name)
        }
    }
}

fn main() {
    if let Some(magic_dir) = env("MAGIC_DIR").map(std::path::PathBuf::from) {
        if !std::path::Path::new(&magic_dir).exists() {
            panic!("Magic library directory {:?} does not exist", magic_dir);
        }
        println!("cargo:rustc-link-search=native={}", magic_dir.to_string_lossy());

        let static_lib = magic_dir.join("libmagic.a");
        let shared_lib = magic_dir.join("libmagic.so");
        match env("MAGIC_STATIC").as_ref().and_then(|s| s.to_str()) {
            Some("false") | Some("FALSE") | Some("0") => {
                if !shared_lib.exists() {
                    panic!("No libmagic.so found in {:?}", magic_dir);
                }
                println!("cargo:rustc-link-lib=dylib=magic");
            }
            Some(_) => {
                if !static_lib.exists() {
                    panic!("No libmagic.a found in {:?}", magic_dir);
                }
                println!("cargo:rustc-link-lib=static=magic");
            }
            None => {
                match (static_lib.exists(), shared_lib.exists()) {
                    (false, false) => panic!("Neither libmagic.so, nor libmagic.a was found in {:?}", magic_dir),
                    (true, false) => println!("cargo:rustc-link-lib=static=magic"),
                    (false, true) => println!("cargo:rustc-link-lib=dylib=magic"),
                    (true, true) => panic!("Both a static and a shared library were found in {:?}\nspecify a choice with `MAGIC_STATIC=true|false`", magic_dir),
                }
            }
        }
    } else {
        if let Err(err) = vcpkg::find_package("libmagic") {
            println!("Could not find vcpkg package: {}", err);
        }

        println!("cargo:rustc-link-lib=dylib=magic");
    }
}
