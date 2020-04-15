use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use glob::glob;

fn main() {
    yarte::recompile::when_changed();

    Command::new("yarn").arg("docs:build").status().unwrap();


    for entry in glob("./docs/openapi/**/*").unwrap() {
        match entry {
            Ok(path) => println!("cargo:rerun-if-changed={}", path.display()),
            Err(e) => panic!(e)
        }
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let curr_dir = env::current_dir().unwrap();
    let dest_dir = Path::new(&out_dir).join("casbin_model.conf");
    let source_dir = curr_dir.join("config/casbin_model.conf");
    println!("cargo:rerun-if-changed={}", &source_dir.to_str().unwrap());
    fs::copy(source_dir, dest_dir).unwrap();
}
