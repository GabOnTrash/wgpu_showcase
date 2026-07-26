use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // This tells Cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = env::var("OUT_DIR")?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;

    let res_path = PathBuf::from(manifest_dir).join("res");

    let paths_to_copy = vec![res_path];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
