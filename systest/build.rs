fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = ctest::TestGenerator::new();

    cfg.header("magic.h");

    // note that is the edition of systest, not magic-sys
    cfg.edition(2024);

    if let Some(s) = std::env::var_os("DEP_MAGIC_INCLUDE") {
        cfg.include(s);
    }

    // #warning _FORTIFY_SOURCE requires compiling with optimization (-O)
    cfg.flag("-U_FORTIFY_SOURCE");

    cfg.skip_const(
        |s| s.ident().starts_with("FILE_"), // not actually part of magic.h but defined manually for use as `action` with `magic_getpath`
    );

    cfg.skip_struct(
        |s| s.ident() == ("magic_set"), // opaque, can't test size / align / roundtrip
    );

    ctest::generate_test(&mut cfg, "../src/lib.rs", "all.rs")?;

    Ok(())
}
