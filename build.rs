use std::env;
use std::ffi::OsString;
use std::fs::{create_dir_all, remove_dir, rename};
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBXDC_SOURCE: &str = "libxdc";
const LIBXDC_STATIC_LIB: &str = "libxdc.a";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    check_submodule(LIBXDC_SOURCE);

    let libxdc_artifacts_dir = PathBuf::from(&out_dir).join("libxdc_artifacts");
    create_dir_all(&libxdc_artifacts_dir).expect("Failed to create libxdc artifacts dir");
    let mut odir = OsString::from("ODIR=");
    odir.push(libxdc_artifacts_dir.as_os_str());
    Command::new("make")
        .current_dir(LIBXDC_SOURCE)
        .args([odir, LIBXDC_STATIC_LIB.into()])
        .output()
        .unwrap();
    // libxdc creates an empty "build" dir anyway
    let empty_build = Path::new(LIBXDC_SOURCE).join("build");
    if empty_build.exists() {
        let _ = remove_dir(empty_build).inspect_err(|_| {
            println!("cargo:warning=Failed to delete build directory in libxdc source tree")
        });
    }
    // In libxdc makefile ODIR variable is used only for intermediate artifacts, the lib needs to
    // be moved
    rename(
        PathBuf::from(LIBXDC_SOURCE).join(LIBXDC_STATIC_LIB),
        PathBuf::from(&libxdc_artifacts_dir).join(LIBXDC_STATIC_LIB),
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", libxdc_artifacts_dir.display());
    println!("cargo:rustc-link-lib=static=xdc");

    let bindings = bindgen::Builder::default()
        .header(
            PathBuf::from(LIBXDC_SOURCE)
                .join("libxdc.h")
                .to_string_lossy(),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate libxdc bindings");

    bindings
        .write_to_file(PathBuf::from(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn check_submodule(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() || path.iter().next().is_none() {
        let error = format!("{dir} directory not found or empty");
        println!("cargo:warning={error}");
        println!(
            "cargo:warning=Hint: Please get the submodules with `git submodule update --init --recursive`"
        );
        panic!("{error}");
    }
}
