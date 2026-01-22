use std::env;
use std::path::PathBuf;

use cbindgen;

fn main() {
  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  let package_name = env::var("CARGO_PKG_NAME").unwrap();
  let output_file = format!("lib{}.h", package_name);

  cbindgen::Builder::new()
    .with_crate(&crate_dir)
    .with_language(cbindgen::Language::C)
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file(&output_file);

  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  let profile_dir = out_dir
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .parent()
    .unwrap();

  let dest_path = profile_dir.join(&output_file);
  std::fs::rename(&output_file, dest_path).ok();
}
