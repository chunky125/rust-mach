//
// Build script for GNU Mach in Rust Assembly files
//

use std::{env, error::Error, fs::File, io::Write, path::Path, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out_dir.display());

    File::create(out_dir.join("ldscript"))?.write_all(include_bytes!("src/ldscript"))?;

    // Set the include path
    let include_path = Path::new("src/aarch64/include");

    // Build the ASM entry components
    Build::new()
        .file("src/aarch64/boot.S")
        .include(include_path)
        .warnings(true)
        .no_default_flags(false)
        .flag("-nostdlib")
        .flag("-fno-builtin")
        .flag("-nolibc")
        // cc::build really does not want to detect the compiler automatically!
        .compiler("aarch64-linux-gnu-gcc")
        .compile("boot");

    println!("cargo:rerun-if-changed=src/aarch64/boot.S");

    Build::new()
        .file("src/aarch64/memset.S")
        .include(include_path)
        .warnings(true)
        .no_default_flags(false)
        .flag("-nostdlib")
        .flag("-fno-builtin")
        .flag("-nolibc")
        // cc::build really does not want to detect the compiler automatically!
        .compiler("aarch64-linux-gnu-gcc")
        .compile("memset");

    println!("cargo:rerun-if-changed=src/aarch64/memset.S");

    Ok(())
}
