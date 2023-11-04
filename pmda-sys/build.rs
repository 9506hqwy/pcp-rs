use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let include_path = match pkg_config::probe_library("libpcp") {
        Ok(lib) => lib.include_paths.first().unwrap().clone(),
        _ => {
            let home = env::var("PCP_HOME").unwrap();
            let home = Path::new(&home);

            println!("cargo:rustc-link-lib=libpcp");
            println!(
                "cargo:rustc-link-search=native={}",
                home.join("lib").display()
            );

            home.join("include").join("pcp")
        }
    };

    println!("cargo:rustc-link-lib=pcp_pmda");

    let macro_header = Path::new("src").join("macro.h");
    let macro_src = Path::new("src").join("macro.c");

    println!("cargo:rerun-if-changed={}", macro_header.display());
    println!("cargo:rerun-if-changed={}", macro_src.display());

    cc::Build::new()
        .file(macro_src.to_str().unwrap())
        .include(&include_path)
        .compile("macro");

    let bindings = bindgen::Builder::default()
        .header(include_path.join("pmapi.h").to_str().unwrap())
        .header(include_path.join("pmda.h").to_str().unwrap())
        .header(macro_header.to_str().unwrap())
        .clang_arg(format!("-I{}", include_path.display()))
        .rustified_enum(".*")
        .blocklist_item("IPPORT_.*")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unbale to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
