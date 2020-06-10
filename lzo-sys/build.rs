use std::path::PathBuf;
use std::{env, fs};

fn build_lzo(out_path: PathBuf) -> Vec<PathBuf> {
    let source = PathBuf::from("lzo/src");

    if !source.exists() {
        panic!("Folder 'lzo/src' does not exists. Maybe you forgot to clone the submodule?");
    }

    let mut build = cc::Build::new();
    build.include(source);

    fs::read_dir("lzo/src").unwrap().for_each(|file| {
        let file_path = file.unwrap().path();
        if file_path.extension().unwrap() == "c" {
            build.file(file_path);
        }
    });

    build.compile("lzo2");

    let out_include_path = out_path.join("lzo/include");

    fs::create_dir_all(out_include_path.join("lzo")).unwrap();
    fs::read_dir("lzo/include/lzo").unwrap().for_each(|file| {
        let file_path = file.unwrap().path();
        fs::copy(file_path.to_owned(), out_path.join(file_path)).unwrap();
    });

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
