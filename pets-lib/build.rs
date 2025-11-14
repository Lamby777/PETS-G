use glob::glob;
use jsonnet::JsonnetVm;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// thank you https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

// this is pretty much only here for jsonnet stuff

fn main() {
    let pattern = "../pets-gd/registries/**/*.jsonnet";
    for path in glob(pattern).unwrap().flatten() {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut vm = JsonnetVm::new();

    let base_dir = Path::new("../pets-gd/registries");
    for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file()
            && path.extension().and_then(|s| s.to_str()) == Some("jsonnet")
        {
            let file_name = match path.file_stem().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => {
                    p!("could not get file name for {path:?}");
                    continue;
                }
            };

            let output_dir = path.parent().unwrap_or(base_dir);
            let output_path = output_dir.join(format!("{}.json", file_name));

            // read and compile jsonnet
            let json_result = match vm.evaluate_file(path) {
                Ok(json) => json,
                Err(e) => {
                    p!("error compiling {}: {e}", path.display());
                    continue;
                }
            };

            // write the compiled JSON to the new file.
            if let Err(e) = fs::write(&output_path, &*json_result) {
                p!("error writing to {}: {e}", output_path.display());
            } else {
                // success
                // p!("compiled {} to {}", path.display(), output_path.display());
            }
        }
    }
}
