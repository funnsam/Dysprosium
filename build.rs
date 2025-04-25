use std::{env, fs::File, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let sparams = Path::new(&out_dir).join("sparams.rs");
    search_params::write_search_params(&mut File::create(sparams).unwrap()).unwrap();
}
