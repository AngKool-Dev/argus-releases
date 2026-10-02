# ARGUS / EraLauncher — Agent Work Guide

## Project Overview

- **Repo root:** `D:\Projects\Active\Games\era-launcher`
- **Type:** Rust TUI (terminal UI) launcher using `crossterm` + `ratatui`
- **Version source:** `Cargo.toml` → `version = "0.1.13"`
- **Remote:** `https://github.com/AngKool-Dev/argus-releases.git`
- **Branch:** `master` (source of truth)
- **Binary:** `src/bin/era-launcher.rs`

## Key Source Files

| File | Purpose |
|------|---------|
| `src/argus/theme.rs` | All built-in themes + theme resolution |
| `src/argus/state.rs` | AppState, section labels, theme_options list |
| `src/argus/ui/mod.rs` | All TUI rendering (navbar, home, settings, mods, etc.) |
| `src/argus/app.rs` | Main event loop, update flow, key handlers |
| `src/argus/update.rs` | GitHub release check, download, helper spawn |
| `src/config.rs` | Config struct, Settings, InstanceConfig |
| `src/resources/era-launcher.rc` | Windows resource script for icon |
| `src/resources/era-launcher.ico` | Application icon |
| `build.rs` | Compiles `.rc` → `.res`/`.o` and links into exe |

## Build Instructions

### Prerequisites (one-time setup)

1. **Rust toolchain** — installed via `winget install Rustlang.Rust.MSVC`
   - Cargo is at `C:\Program Files\Rust stable MSVC 1.98\bin\cargo.exe`
2. **Visual Studio Build Tools 2022** — installed via `winget install Microsoft.VisualStudio.2022.BuildTools`
   - Add workload: `Microsoft.VisualStudio.Workload.VCTools`
   - Add components: `Microsoft.VisualStudio.Component.VC.Tools.x86.x64`, `Microsoft.VisualStudio.Component.Windows11SDK.22000`, `Microsoft.VisualStudio.Component.Windows10SDK.22621`
3. **Windows SDK libs** — `kernel32.lib` etc. needed by linker
   - Path: `C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\um\x64`
4. **GitHub CLI** (optional) — `winget install GitHub.cli` for creating releases

### Build commands

```powershell
# Refresh PATH for this session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

# Set compiler and library paths
$msvcPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64"
$sdkLib = "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\um\x64"
$env:Path = "$msvcPath;" + $env:Path
$env:LIB = "$sdkLib;" + [System.Environment]::GetEnvironmentVariable("LIB","Machine")

# Build
cd D:\Projects\Active\Games\era-launcher
cargo build --release
```

### Output

```
D:\Projects\Active\Games\era-launcher\target\release\era-launcher.exe
```

## Icon Embedding

- `build.rs` compiles `src/resources/era-launcher.rc` at build time
- `.rc` references `src/resources/era-launcher.ico` with a **relative path**
- `build.rs` searches for a resource compiler in this order:
  1. `WINDRES_PATH` environment variable
  2. `windres` in PATH (MSYS2/Git Bash)
  3. Hardcoded MSYS2/Git Bash paths
  4. `rc.exe` in Windows SDK (`C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\rc.exe`)
- If `rc.exe` is used, it generates a `.res` file; if `windres` is used, it generates an `.o` file
- Both are linked via `cargo:rustc-link-arg`

**Verify icon is embedded:**
```powershell
$bytes = [System.IO.File]::ReadAllBytes("D:\Projects\Active\Games\era-launcher\target\release\era-launcher.exe")
$found = $false
for ($i=0; $i -lt $bytes.Length - 8; $i++) {
    if ($bytes[$i] -eq 0x00 -and $bytes[$i+1] -eq 0x00 -and $bytes[$i+2] -eq 0x01 -and $bytes[$i+3] -eq 0x00) {
        $found = $true; Write-Host "ICO header found at offset: $i"; break
    }
}
if (-not $found) { Write-Host "ICO header NOT found" }
```

## Theme System

- All themes are defined as `pub const` in `src/argus/theme.rs`
- Each theme is a `Theme` struct with color fields: `bg`, `bg_dark`, `bg_panel`, `bg_filled`, `border`, `border_focus`, `border_dim`, `accent`, `accent_dim`, `accent_fade`, `text`, `text_dim`, `text_muted`, `text_subtle`, `success`, `warning`, `error`, `info`, `focus`, `selection`, `fabric`, `forge`, `vanilla`, `divider`
- Resolution happens in `Theme::resolve(name)` which matches lowercase strings
- Available themes: `dark`, `light`, `system`, `dracula`, `tokyo-night`, `pink`, `midnight`, `forest`, `ocean`, `sunset`
- `system` reads Windows registry via `detect_system_theme()`
- Theme list exposed to TUI settings via `AppState.theme_options` in `src/argus/state.rs` (lines 504-515)

**To add a new theme:**
1. Define a `pub const THEME_NAME: Theme = Theme { ... };` in `src/argus/theme.rs`
2. Add resolution match arm in `Theme::resolve()`
3. Add the theme name string to `theme_options` in `src/argus/state.rs`

## Updater Mechanism

- Checks `https://github.com/AngKool-Dev/argus-releases/releases/latest` for latest tag
- Compares with `env!("CARGO_PKG_VERSION")` using `AppState::is_newer_version()`
- Throttle: only checks once per hour (`CHECK_INTERVAL = 1 hour`)
- Download URL pattern: `https://github.com/AngKool-Dev/argus-releases/download/<tag>/era-launcher.exe`
- Downloads to `era-launcher.new` next to current exe
- Spawns a detached `.bat` helper that:
  1. Waits up to 60s for the launcher to exit
  2. Copies `.new` over the original exe
  3. Deletes `.new`
  4. Writes marker file `era-launcher.exe.update-result`
  5. Relaunches the launcher
- The launcher clears the marker on startup if `ERA_LAUNCHER_UPDATE_PENDING` env var is set
- **Safety checks added in 0.1.13:**
  - Current exe size must be 1 MB - 15 MB before updating
  - Downloaded asset must be at least 1 MB

**Common update failure:** If the local exe is a debug build or outside the size range, the updater aborts with an error message.

## GitHub Release Process

1. Bump version in `Cargo.toml`
2. Commit and push to `origin/master`
3. Create and push tag:
   ```powershell
   git tag v0.1.14
   git push origin master --tags
   ```
4. Create GitHub Release with tag `v0.1.14`
5. Upload `target/release/era-launcher.exe` as the release asset
6. Verify asset is attached via API:
   ```powershell
   Invoke-RestMethod -Uri "https://api.github.com/repos/AngKool-Dev/argus-releases/releases/tags/v0.1.14"
   ```

**Important:** The updater reads from **GitHub Releases**, not git tags/branches. A release must exist for the updater to find it.

## Known Gotchas

1. **Windows linker setup:** MSVC linker `link.exe` and Windows SDK libs must be in PATH/LIB. See build commands above.
2. **Icon caching:** After rebuilding with a new icon, Windows Explorer may still show the old icon. Delete the exe and recopy, or restart Explorer.
3. **Update helper window:** In 0.1.13+, the helper is spawned with `CREATE_NO_WINDOW` via `cmd /c`. Older versions showed a console popup.
4. **Debug vs Release:** Debug builds are ~15 MB, release builds are ~7 MB. The updater rejects sizes outside 1-15 MB.
5. **Git safe.directory:** If git complains about ownership, run:
   ```powershell
   git config --global --add safe.directory "D:/Projects/Active/Games/era-launcher"
   ```

## Version Bump Checklist

When releasing a new version:
- [ ] Update `version` in `Cargo.toml`
- [ ] Commit: `git add Cargo.toml && git commit -m "Bump version to X.Y.Z"`
- [ ] Tag: `git tag vX.Y.Z`
- [ ] Push: `git push origin master --tags`
- [ ] Build release exe with the build commands above
- [ ] Create GitHub Release at `https://github.com/AngKool-Dev/argus-releases/releases/new`
- [ ] Upload the exe
- [ ] Test the updater from the previous version
