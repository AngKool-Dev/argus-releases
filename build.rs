use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let obj_file = Path::new(&out_dir).join("era-launcher_res.o");
    let res_file = Path::new(&out_dir).join("era-launcher_res.res");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rc_file = Path::new(&manifest_dir).join("src/resources/era-launcher.rc");

    if rc_file.exists() {
        if let Some(compiler) = find_windres() {
            let rc_relative = rc_file
                .strip_prefix(&manifest_dir)
                .unwrap_or(Path::new("src/resources/era-launcher.rc"));

            if compiler.ends_with("rc.exe") {
                let output = Command::new(&compiler)
                    .current_dir(&manifest_dir)
                    .arg(rc_relative)
                    .arg("/fo")
                    .arg(&res_file)
                    .output();

                if let Ok(output) = output {
                    if output.status.success() {
                        println!("cargo:rustc-link-arg={}", res_file.display());
                        return;
                    }
                }
            } else {
                let output = Command::new(&compiler)
                    .current_dir(&manifest_dir)
                    .arg(rc_relative)
                    .arg("-o")
                    .arg(&obj_file)
                    .output();

                if let Ok(output) = output {
                    if output.status.success() {
                        println!("cargo:rustc-link-arg={}", obj_file.display());
                        return;
                    }
                }
            }
        }
    }
}

fn find_windres() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("WINDRES_PATH") {
        return Some(std::path::PathBuf::from(p));
    }
    let output = Command::new("windres").arg("--version").output();
    if output.map(|o| o.status.success()).unwrap_or(false) {
        return Some(std::path::PathBuf::from("windres"));
    }

    let candidates = vec![
        r"C:\msys64\mingw64\bin\windres.exe",
        r"C:\msys64\ucrt64\bin\windres.exe",
        r"C:\msys64\clang64\bin\windres.exe",
        r"C:\Program Files\Git\mingw64\bin\windres.exe",
        r"C:\Program Files\Git\usr\bin\windres.exe",
    ];
    for candidate in candidates {
        if std::path::Path::new(candidate).exists() {
            return Some(std::path::PathBuf::from(candidate));
        }
    }

    let sdk_rc = r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\rc.exe";
    if std::path::Path::new(sdk_rc).exists() {
        return Some(std::path::PathBuf::from(sdk_rc));
    }

    None
}
