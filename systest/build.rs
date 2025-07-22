fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("magic.h");

    cfg.rust_version(1, 64, 0); // hardcoded MSRV

    if let Some(s) = std::env::var_os("DEP_MAGIC_INCLUDE") {
        cfg.include(s);
    }

    // #warning _FORTIFY_SOURCE requires compiling with optimization (-O)
    cfg.flag("-U_FORTIFY_SOURCE");

    cfg.skip_const(
        |s| s.starts_with("FILE_"), // not actually part of magic.h but defined manually for use as `action` with `magic_getpath`
    );

    cfg.skip_struct(
        |s| s == ("magic_set"), // opaque, can't test size / align / roundtrip
    );

    cfg.generate("../src/lib.rs", "all.rs");
}
