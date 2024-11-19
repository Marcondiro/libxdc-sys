use std::ffi::OsString;
use std::fs::{create_dir_all, remove_dir, rename, FileType};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

const LIBXDC_SOURCE: &str = "libxdc";
const LIBXDC_STATIC_LIB: &str = "libxdc.a";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    check_submodule(LIBXDC_SOURCE);

    // In libxdc makefile, the ODIR variable is used only for intermediate artifacts, the lib is
    // always built in the source tree, therefore we have to copy the library and build in the
    // `out_dir`
    let libxdc_artifacts_dir = Path::new(&out_dir).join("libxdc_artifacts");
    create_dir_all(libxdc_artifacts_dir.join("src")).expect("Failed to create libxdc artifacts dir");
    let libxdc_src = Path::new(LIBXDC_SOURCE).join("src");
    for e in fs::read_dir(libxdc_src)
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Ok(t) = e.file_type() {
                t.is_file()
            } else {
                false
            }
        })
    {
        fs::copy(e.path(), libxdc_artifacts_dir.join("src").join(e.file_name())).unwrap();
    }
    fs::copy(Path::new(LIBXDC_SOURCE).join("Makefile"), libxdc_artifacts_dir.join("Makefile")).unwrap();
    fs::copy(Path::new(LIBXDC_SOURCE).join("libxdc.h"), libxdc_artifacts_dir.join("libxdc.h")).unwrap();


    let make_result = Command::new("make")
        .current_dir(&libxdc_artifacts_dir)
        .arg(LIBXDC_STATIC_LIB)
        .status()
        .unwrap();
    if !make_result.success() {
        panic!("Make failed");
    }

    // libxdc creates an empty "build" dir anyway
    let empty_build = Path::new(LIBXDC_SOURCE).join("build");
    if empty_build.exists() {
        let _ = remove_dir(empty_build).inspect_err(|_| {
            println!("cargo:warning=Failed to delete build directory in libxdc source tree")
        });
    }

    println!("cargo:rustc-link-lib=static=xdc");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&libxdc_artifacts_dir).display()
    );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate libxdc bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
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
