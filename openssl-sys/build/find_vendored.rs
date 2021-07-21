#[cfg(feature = "vendored-libressl")]
use libressl_src as openssl_src;
#[cfg(feature = "vendored-openssl")]
use openssl_src;
use std::path::PathBuf;

pub fn get_openssl(_target: &str) -> (PathBuf, PathBuf) {
    let artifacts = openssl_src::Build::new().build();
    println!("cargo:vendored=1");
    println!(
        "cargo:root={}",
        artifacts.lib_dir().parent().unwrap().display()
    );

    (
        artifacts.lib_dir().to_path_buf(),
        artifacts.include_dir().to_path_buf(),
    )
}
