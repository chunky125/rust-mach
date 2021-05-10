//
// Build script for GNU Mach in Rust Assembly files
//

use std::path::Path;

fn main() {

    println!("cargo:rerun-if-changed=src/aarch64/boot.S");

    // Set the include path
    let include_path = Path::new("src/aarch64/include");

    // Build the ASM entry components
    cc::Build::new()
        .file("src/aarch64/boot.S")
        .include(include_path)
        .warnings(true)
        .no_default_flags(false)
        // cc::build really does not want to detect the compiler automatically!
        .compiler("aarch64-linux-gnu-gcc")
        .compile("boot");
}
