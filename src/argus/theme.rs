//! Theme — runtime-switchable color palettes for ARGUS.
//!
//! The settings value `theme` ("dark" | "light" | "system") is resolved to a
//! [`Theme`] palette and applied globally before each frame. Every drawing
//! function reads the active palette via [`current`] so switching themes
//! takes effect immediately (previously colors were hardcoded constants and
//! the setting did nothing visually).

use ratatui::prelude::Color;
use std::sync::RwLock;

/// A complete color palette for the terminal UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_panel: Color,
    pub bg_filled: Color,
    pub border: Color,
    pub border_focus: Color,
    pub border_dim: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub accent_fade: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_muted: Color,
    pub text_subtle: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub focus: Color,
    pub selection: Color,
    pub fabric: Color,
    pub forge: Color,
    pub vanilla: Color,
    pub divider: Color,
}

impl Theme {
    /// Deep dark palette with cyan-teal accents and subtle gradients.
    pub const DARK: Theme = Theme {
        name: "dark",
        bg: Color::Rgb(12, 12, 19),
        bg_dark: Color::Rgb(8, 8, 14),
        bg_panel: Color::Rgb(22, 22, 32),
        bg_filled: Color::Rgb(18, 18, 27),
        border: Color::Rgb(52, 52, 72),
        border_focus: Color::Rgb(70, 200, 140),
        border_dim: Color::Rgb(34, 34, 50),
        accent: Color::Rgb(70, 200, 140),
        accent_dim: Color::Rgb(55, 165, 120),
        accent_fade: Color::Rgb(35, 110, 85),
        text: Color::Rgb(225, 225, 235),
        text_dim: Color::Rgb(150, 150, 170),
        text_muted: Color::Rgb(100, 100, 120),
        text_subtle: Color::Rgb(70, 70, 90),
        success: Color::Rgb(70, 200, 140),
        warning: Color::Rgb(255, 195, 70),
        error: Color::Rgb(245, 90, 90),
        info: Color::Rgb(90, 175, 245),
        focus: Color::Rgb(100, 190, 255),
        selection: Color::Rgb(28, 28, 40),
        fabric: Color::Rgb(110, 130, 255),
        forge: Color::Rgb(245, 125, 60),
        vanilla: Color::Rgb(210, 210, 220),
        divider: Color::Rgb(38, 38, 55),
    };

    /// Light palette — same structure, readable on bright terminals.
    pub const LIGHT: Theme = Theme {
        name: "light",
        bg: Color::Rgb(245, 245, 248),
        bg_dark: Color::Rgb(230, 230, 235),
        bg_panel: Color::Rgb(255, 255, 255),
        bg_filled: Color::Rgb(248, 248, 252),
        border: Color::Rgb(205, 205, 215),
        border_focus: Color::Rgb(20, 140, 90),
        border_dim: Color::Rgb(218, 218, 228),
        accent: Color::Rgb(16, 130, 84),
        accent_dim: Color::Rgb(35, 120, 80),
        accent_fade: Color::Rgb(210, 235, 225),
        text: Color::Rgb(25, 25, 35),
        text_dim: Color::Rgb(105, 105, 120),
        text_muted: Color::Rgb(135, 135, 150),
        text_subtle: Color::Rgb(175, 175, 188),
        success: Color::Rgb(16, 130, 84),
        warning: Color::Rgb(180, 115, 0),
        error: Color::Rgb(210, 45, 45),
        info: Color::Rgb(25, 100, 200),
        focus: Color::Rgb(20, 90, 190),
        selection: Color::Rgb(235, 240, 238),
        fabric: Color::Rgb(75, 95, 215),
        forge: Color::Rgb(200, 90, 30),
        vanilla: Color::Rgb(75, 75, 85),
        divider: Color::Rgb(218, 218, 228),
    };

    /// Dracula — deep navy with vibrant pastel accents. Minecraft green
    /// reserved as the accent.
    pub const DRACULA: Theme = Theme {
        name: "dracula",
        bg: Color::Rgb(40, 42, 54),
        bg_dark: Color::Rgb(33, 34, 50),
        bg_panel: Color::Rgb(56, 58, 84),
        bg_filled: Color::Rgb(68, 71, 90),
        border: Color::Rgb(98, 114, 164),
        border_focus: Color::Rgb(139, 148, 255),
        border_dim: Color::Rgb(73, 76, 102),
        accent: Color::Rgb(80, 200, 120),
        accent_dim: Color::Rgb(60, 160, 90),
        accent_fade: Color::Rgb(40, 120, 70),
        text: Color::Rgb(248, 248, 242),
        text_dim: Color::Rgb(169, 183, 202),
        text_muted: Color::Rgb(98, 114, 143),
        text_subtle: Color::Rgb(66, 73, 92),
        success: Color::Rgb(80, 200, 120),
        warning: Color::Rgb(241, 250, 140),
        error: Color::Rgb(255, 85, 85),
        info: Color::Rgb(139, 148, 255),
        focus: Color::Rgb(189, 147, 249),
        selection: Color::Rgb(68, 71, 90),
        fabric: Color::Rgb(114, 137, 218),
        forge: Color::Rgb(255, 128, 64),
        vanilla: Color::Rgb(189, 147, 249),
        divider: Color::Rgb(74, 77, 92),
    };

    /// Tokyo Night (Night) — cool blue-grey with warm accents.
    /// Minecraft green as accent.
    pub const TOKYO_NIGHT: Theme = Theme {
        name: "tokyo-night",
        bg: Color::Rgb(26, 27, 39),
        bg_dark: Color::Rgb(22, 23, 42),
        bg_panel: Color::Rgb(41, 41, 74),
        bg_filled: Color::Rgb(51, 52, 76),
        border: Color::Rgb(86, 95, 137),
        border_focus: Color::Rgb(114, 137, 218),
        border_dim: Color::Rgb(60, 66, 102),
        accent: Color::Rgb(80, 200, 120),
        accent_dim: Color::Rgb(60, 160, 90),
        accent_fade: Color::Rgb(40, 120, 70),
        text: Color::Rgb(192, 202, 245),
        text_dim: Color::Rgb(122, 129, 165),
        text_muted: Color::Rgb(69, 71, 92),
        text_subtle: Color::Rgb(48, 50, 65),
        success: Color::Rgb(115, 215, 148),
        warning: Color::Rgb(241, 187, 106),
        error: Color::Rgb(247, 118, 141),
        info: Color::Rgb(114, 137, 218),
        focus: Color::Rgb(122, 170, 255),
        selection: Color::Rgb(41, 41, 74),
        fabric: Color::Rgb(114, 137, 218),
        forge: Color::Rgb(255, 148, 76),
        vanilla: Color::Rgb(192, 202, 245),
        divider: Color::Rgb(51, 53, 72),
    };

    /// Pink — soft rose tones with warm accents.
    pub const PINK: Theme = Theme {
        name: "pink",
        bg: Color::Rgb(28, 18, 24),
        bg_dark: Color::Rgb(20, 12, 16),
        bg_panel: Color::Rgb(36, 24, 30),
        bg_filled: Color::Rgb(44, 30, 36),
        border: Color::Rgb(54, 38, 46),
        border_focus: Color::Rgb(224, 128, 188),
        border_dim: Color::Rgb(38, 26, 32),
        accent: Color::Rgb(224, 108, 168),
        accent_dim: Color::Rgb(200, 88, 148),
        accent_fade: Color::Rgb(160, 68, 120),
        text: Color::Rgb(240, 220, 230),
        text_dim: Color::Rgb(180, 150, 165),
        text_muted: Color::Rgb(120, 95, 108),
        text_subtle: Color::Rgb(80, 60, 72),
        success: Color::Rgb(108, 200, 150),
        warning: Color::Rgb(240, 180, 120),
        error: Color::Rgb(240, 120, 120),
        info: Color::Rgb(140, 160, 240),
        focus: Color::Rgb(200, 140, 255),
        selection: Color::Rgb(44, 30, 36),
        fabric: Color::Rgb(140, 120, 255),
        forge: Color::Rgb(255, 140, 100),
        vanilla: Color::Rgb(220, 200, 210),
        divider: Color::Rgb(50, 34, 42),
    };

    /// Midnight — deep navy with cool blue accents.
    pub const MIDNIGHT: Theme = Theme {
        name: "midnight",
        bg: Color::Rgb(10, 12, 20),
        bg_dark: Color::Rgb(6, 8, 14),
        bg_panel: Color::Rgb(18, 22, 32),
        bg_filled: Color::Rgb(26, 30, 42),
        border: Color::Rgb(34, 40, 56),
        border_focus: Color::Rgb(100, 140, 255),
        border_dim: Color::Rgb(24, 28, 40),
        accent: Color::Rgb(100, 140, 255),
        accent_dim: Color::Rgb(80, 120, 220),
        accent_fade: Color::Rgb(50, 80, 160),
        text: Color::Rgb(220, 225, 240),
        text_dim: Color::Rgb(150, 158, 180),
        text_muted: Color::Rgb(100, 108, 130),
        text_subtle: Color::Rgb(60, 66, 90),
        success: Color::Rgb(80, 200, 160),
        warning: Color::Rgb(240, 190, 90),
        error: Color::Rgb(255, 100, 100),
        info: Color::Rgb(100, 160, 255),
        focus: Color::Rgb(120, 180, 255),
        selection: Color::Rgb(26, 30, 42),
        fabric: Color::Rgb(120, 140, 255),
        forge: Color::Rgb(255, 140, 80),
        vanilla: Color::Rgb(200, 210, 230),
        divider: Color::Rgb(28, 34, 48),
    };

    /// Forest — deep greens with earthy accents.
    pub const FOREST: Theme = Theme {
        name: "forest",
        bg: Color::Rgb(16, 22, 18),
        bg_dark: Color::Rgb(10, 14, 12),
        bg_panel: Color::Rgb(22, 30, 26),
        bg_filled: Color::Rgb(30, 40, 34),
        border: Color::Rgb(38, 50, 44),
        border_focus: Color::Rgb(88, 200, 140),
        border_dim: Color::Rgb(28, 36, 32),
        accent: Color::Rgb(88, 180, 130),
        accent_dim: Color::Rgb(70, 150, 110),
        accent_fade: Color::Rgb(45, 100, 75),
        text: Color::Rgb(220, 230, 222),
        text_dim: Color::Rgb(150, 168, 155),
        text_muted: Color::Rgb(100, 118, 105),
        text_subtle: Color::Rgb(60, 72, 65),
        success: Color::Rgb(88, 200, 140),
        warning: Color::Rgb(210, 180, 90),
        error: Color::Rgb(220, 100, 90),
        info: Color::Rgb(90, 160, 200),
        focus: Color::Rgb(110, 190, 150),
        selection: Color::Rgb(30, 40, 34),
        fabric: Color::Rgb(100, 140, 220),
        forge: Color::Rgb(220, 120, 70),
        vanilla: Color::Rgb(190, 200, 180),
        divider: Color::Rgb(32, 42, 36),
    };

    /// Ocean — deep blues with aquatic accents.
    pub const OCEAN: Theme = Theme {
        name: "ocean",
        bg: Color::Rgb(14, 20, 28),
        bg_dark: Color::Rgb(8, 12, 18),
        bg_panel: Color::Rgb(20, 28, 38),
        bg_filled: Color::Rgb(28, 38, 50),
        border: Color::Rgb(36, 48, 62),
        border_focus: Color::Rgb(80, 170, 220),
        border_dim: Color::Rgb(26, 34, 46),
        accent: Color::Rgb(80, 170, 220),
        accent_dim: Color::Rgb(60, 145, 190),
        accent_fade: Color::Rgb(35, 90, 140),
        text: Color::Rgb(220, 230, 240),
        text_dim: Color::Rgb(150, 165, 180),
        text_muted: Color::Rgb(100, 115, 130),
        text_subtle: Color::Rgb(55, 66, 80),
        success: Color::Rgb(80, 200, 170),
        warning: Color::Rgb(220, 190, 90),
        error: Color::Rgb(240, 110, 100),
        info: Color::Rgb(100, 150, 240),
        focus: Color::Rgb(110, 180, 255),
        selection: Color::Rgb(28, 38, 50),
        fabric: Color::Rgb(100, 130, 240),
        forge: Color::Rgb(240, 130, 80),
        vanilla: Color::Rgb(200, 210, 225),
        divider: Color::Rgb(30, 40, 52),
    };

    /// Sunset — warm oranges and reds with dark backgrounds.
    pub const SUNSET: Theme = Theme {
        name: "sunset",
        bg: Color::Rgb(26, 18, 14),
        bg_dark: Color::Rgb(18, 12, 8),
        bg_panel: Color::Rgb(34, 24, 18),
        bg_filled: Color::Rgb(44, 32, 24),
        border: Color::Rgb(56, 40, 30),
        border_focus: Color::Rgb(240, 140, 80),
        border_dim: Color::Rgb(38, 28, 20),
        accent: Color::Rgb(240, 140, 80),
        accent_dim: Color::Rgb(210, 120, 60),
        accent_fade: Color::Rgb(160, 80, 35),
        text: Color::Rgb(240, 225, 210),
        text_dim: Color::Rgb(185, 160, 140),
        text_muted: Color::Rgb(125, 105, 90),
        text_subtle: Color::Rgb(75, 62, 52),
        success: Color::Rgb(120, 190, 110),
        warning: Color::Rgb(240, 190, 80),
        error: Color::Rgb(240, 110, 90),
        info: Color::Rgb(140, 170, 240),
        focus: Color::Rgb(255, 170, 100),
        selection: Color::Rgb(44, 32, 24),
        fabric: Color::Rgb(140, 120, 240),
        forge: Color::Rgb(240, 110, 70),
        vanilla: Color::Rgb(215, 200, 185),
        divider: Color::Rgb(46, 34, 26),
    };

    /// Resolve a settings string ("dark" | "light" | "system" |
    /// "dracula" | "tokyo-night" | "pink" | "midnight" | "forest" |
    /// "ocean" | "sunset") to a palette.
    pub fn resolve(name: &str) -> Theme {
        match name.to_lowercase().as_str() {
            "light" => Theme::LIGHT,
            "system" => detect_system_theme(),
            "dracula" => Theme::DRACULA,
            "tokyo-night" | "tokyonight" | "tokyo night" => Theme::TOKYO_NIGHT,
            "pink" => Theme::PINK,
            "midnight" => Theme::MIDNIGHT,
            "forest" => Theme::FOREST,
            "ocean" => Theme::OCEAN,
            "sunset" => Theme::SUNSET,
            _ => Theme::DARK,
        }
    }
}

/// Detect the OS-wide light/dark preference. Windows reads the registry;
/// other platforms currently default to dark.
fn detect_system_theme() -> Theme {
    if cfg!(windows) {
        let output = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output();
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            // The value line ends with something like "REG_DWORD 0x1".
            for token in text.split_whitespace() {
                if token == "0x1" {
                    return Theme::LIGHT;
                }
                if token == "0x0" {
                    return Theme::DARK;
                }
            }
        }
    }
    Theme::DARK
}

static CURRENT: RwLock<Theme> = RwLock::new(Theme::DARK);

/// Cache of the last applied settings string so `apply` can skip redundant
/// resolution. Without this, the "system" theme spawned a `reg query`
/// subprocess on EVERY rendered frame.
static APPLIED_NAME: RwLock<Option<String>> = RwLock::new(None);

/// Apply a theme by settings name. Safe to call every frame — repeated
/// calls with the same name are no-ops.
pub fn apply(name: &str) {
    {
        let cached = APPLIED_NAME.read().ok();
        if let Some(guard) = cached {
            if guard.as_deref() == Some(name) {
                return;
            }
        }
    }
    let theme = Theme::resolve(name);
    if let Ok(mut guard) = CURRENT.write() {
        *guard = theme;
    }
    if let Ok(mut guard) = APPLIED_NAME.write() {
        *guard = Some(name.to_string());
    }
}

/// Get a copy of the currently active palette.
pub fn current() -> Theme {
    CURRENT.read().map(|t| *t).unwrap_or(Theme::DARK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_dark() {
        assert_eq!(Theme::resolve("dark"), Theme::DARK);
        assert_eq!(Theme::resolve("unknown"), Theme::DARK);
        assert_eq!(Theme::resolve(""), Theme::DARK);
    }

    #[test]
    fn test_resolve_dracula() {
        assert_eq!(Theme::resolve("dracula"), Theme::DRACULA);
        assert_eq!(Theme::resolve("Dracula"), Theme::DRACULA);
    }

    #[test]
    fn test_resolve_tokyo_night() {
        assert_eq!(Theme::resolve("tokyo-night"), Theme::TOKYO_NIGHT);
        assert_eq!(Theme::resolve("tokyonight"), Theme::TOKYO_NIGHT);
        assert_eq!(Theme::resolve("tokyo night"), Theme::TOKYO_NIGHT);
    }

    #[test]
    fn test_resolve_light() {
        assert_eq!(Theme::resolve("light"), Theme::LIGHT);
        assert_eq!(Theme::resolve("LIGHT"), Theme::LIGHT);
    }

    #[test]
    fn test_resolve_system_returns_valid_palette() {
        let t = Theme::resolve("system");
        assert!(t == Theme::DARK || t == Theme::LIGHT);
    }

    #[test]
    fn test_resolve_pink() {
        assert_eq!(Theme::resolve("pink"), Theme::PINK);
        assert_eq!(Theme::resolve("PINK"), Theme::PINK);
    }

    #[test]
    fn test_resolve_midnight() {
        assert_eq!(Theme::resolve("midnight"), Theme::MIDNIGHT);
    }

    #[test]
    fn test_resolve_forest() {
        assert_eq!(Theme::resolve("forest"), Theme::FOREST);
    }

    #[test]
    fn test_resolve_ocean() {
        assert_eq!(Theme::resolve("ocean"), Theme::OCEAN);
    }

    #[test]
    fn test_resolve_sunset() {
        assert_eq!(Theme::resolve("sunset"), Theme::SUNSET);
    }

    #[test]
    fn test_palettes_differ() {
        assert_ne!(Theme::DARK.bg, Theme::LIGHT.bg);
        assert_ne!(Theme::DARK.text, Theme::LIGHT.text);
        assert_ne!(Theme::PINK.bg, Theme::MIDNIGHT.bg);
    }

    #[test]
    fn test_apply_and_current_roundtrip() {
        apply("light");
        assert_eq!(current().name, "light");
        apply("dark");
        assert_eq!(current().name, "dark");
    }
}
