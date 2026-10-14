use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let obj_file = Path::new(&out_dir).join("era-launcher_res.o");
    let res_file = Path::new(&out_dir).join("era-launcher_res.res");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rc_file = Path::new(&manifest_dir).join("src/resources/era-launcher.rc");

    eprintln!("[build.rs] rc_file exists: {}", rc_file.exists());
    eprintln!("[build.rs] rc_file path: {}", rc_file.display());

    if !rc_file.exists() {
        panic!(
            "[build.rs] resource script missing: {}",
            rc_file.display()
        );
    }

    {
        if let Some(compiler) = find_windres() {
            eprintln!("[build.rs] Found compiler: {}", compiler.display());
            let rc_relative = rc_file
                .strip_prefix(&manifest_dir)
                .unwrap_or(Path::new("src/resources/era-launcher.rc"));

            let ok = if compiler.ends_with("rc.exe") {
                eprintln!("[build.rs] Using rc.exe, output: {}", res_file.display());
                let output = Command::new(&compiler)
                    .current_dir(&manifest_dir)
                    // rc.exe requires options BEFORE the input file, otherwise it
                    // fails with "RC1107: invalid usage".
                    .arg("/nologo")
                    .arg("/fo")
                    .arg(&res_file)
                    .arg(rc_relative)
                    .output();
                report("rc.exe", &output);
                match output {
                    Ok(o) if o.status.success() => {
                        println!("cargo:rustc-link-arg={}", res_file.display());
                        true
                    }
                    _ => false,
                }
            } else {
                eprintln!("[build.rs] Using windres, output: {}", obj_file.display());
                let output = Command::new(&compiler)
                    .current_dir(&manifest_dir)
                    // Same ordering rule as rc.exe: options first.
                    .arg("-O")
                    .arg("winres")
                    .arg("-o")
                    .arg(&obj_file)
                    .arg(rc_relative)
                    .output();
                report("windres", &output);
                match output {
                    Ok(o) if o.status.success() => {
                        println!("cargo:rustc-link-arg={}", obj_file.display());
                        true
                    }
                    _ => false,
                }
            };

            if !ok {
                // The compiler was found but failed. Previously this fell through
                // and produced an iconless exe with a successful-looking build.
                panic!(
                    "[build.rs] resource compiler failed to compile {} - the exe will have NO icon.\n\
                     See the [build.rs] output above for the compiler's own error message.",
                    rc_file.display()
                );
            }
        } else {
            // A silent skip here produced an iconless era-launcher.exe for weeks.
            // Fail loudly on Windows, warn everywhere else (e.g. cross-compiles).
            let msg = "[build.rs] No resource compiler found: the exe will have NO icon.\n\
                       Install one of:\n\
                       \x20 - Visual Studio Build Tools 2022 + 'Windows SDK' (gives rc.exe), or\n\
                       \x20 - Git for Windows / MSYS2 (gives windres.exe), or\n\
                       \x20 - set WINDRES_PATH to a windres.exe, or RC_PATH to an rc.exe\n\
                       See src/resources/era-launcher.rc and find_windres() in build.rs.";
            if env::var("CARGO_CFG_TARGET_OS").map(|v| v == "windows").unwrap_or(false) {
                panic!("{}", msg);
            }
            println!("cargo:warning={}", msg);
        }
    }
}

fn report(tool: &str, output: &Result<std::process::Output, std::io::Error>) {
    match output {
        Ok(o) => {
            eprintln!("[build.rs] {} status: {}", tool, o.status);
            eprintln!(
                "[build.rs] {} stdout: {}",
                tool,
                String::from_utf8_lossy(&o.stdout)
            );
            eprintln!(
                "[build.rs] {} stderr: {}",
                tool,
                String::from_utf8_lossy(&o.stderr)
            );
        }
        Err(e) => eprintln!("[build.rs] {} failed to launch: {}", tool, e),
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

    // GNU toolchains (Git for Windows, MSYS2) ship windres.
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

    // Windows SDK rc.exe. Do NOT hardcode a version: the previous build.rs
    // pinned 10.0.22621.0 and silently found nothing on any other SDK.
    // Enumerate <kit-root>\bin\<version>\x64\rc.exe and take the newest.
    let mut sdk_rcs: Vec<std::path::PathBuf> = Vec::new();
    for root in [
        r"C:\Program Files (x86)\Windows Kits\10\bin",
        r"C:\Program Files\Windows Kits\10\bin",
    ] {
        if let Ok(versions) = std::fs::read_dir(root) {
            for entry in versions.flatten() {
                let rc = entry.path().join("x64").join("rc.exe");
                if rc.exists() {
                    sdk_rcs.push(rc);
                }
            }
        }
    }
    // Also honour an explicit override for unusual layouts.
    if let Ok(p) = std::env::var("RC_PATH") {
        if std::path::Path::new(&p).exists() {
            sdk_rcs.push(std::path::PathBuf::from(p));
        }
    }
    sdk_rcs.sort();
    sdk_rcs.pop()
}
