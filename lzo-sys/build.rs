use std::path::PathBuf;
use std::{env, fs};

fn build_lzo(out_path: PathBuf) -> Vec<PathBuf> {
    let source = PathBuf::from("lzo");

    if !source.exists() {
        panic!("Folder 'lzo' does not exists. Maybe you forgot to clone the submodule?");
    }

    let mut build = cc::Build::new();
    build
        .include(source.join("src"))
        .include(source.join("include"));

    for f in fs::read_dir(source.join("src")).unwrap() {
        let file = f.unwrap();
        if file.file_name().to_string_lossy().ends_with(".c") {
            build.file(file.path());
        }
    }

    build.compile("lzo2");

    let out_include_path = out_path.join("lzo/include");

    fs::create_dir_all(out_include_path.join("lzo")).expect("Unable to create header output dir!");

    for f in fs::read_dir(source.join("include/lzo")).unwrap() {
        let file_path = f.unwrap().path();
        let src = file_path.to_owned();
        let dst = out_path.join(file_path);
        fs::copy(&src, &dst)
            .map_err(|err| {
                format!(
                    "Encountered error while copying {} to {}: {}",
                    src.display(),
                    dst.display(),
                    err
                )
            })
            .unwrap();
    }

    vec![out_include_path]
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LZO_STATIC");

    let want_static = cfg!(feature = "static") || env::var("LZO_STATIC").is_ok();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let headerpaths = if want_static {
        build_lzo(out_path)
    } else if let Ok(pkg) = pkg_config::probe_library("lzo2") {
        pkg.include_paths
    } else {
        build_lzo(out_path)
    };

    println!(
        "cargo:include={}",
        headerpaths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join(";")
    );
}
