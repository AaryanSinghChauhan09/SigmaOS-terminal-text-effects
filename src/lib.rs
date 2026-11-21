// SPDX-License-Identifier: Apache-2.0
//!
//! SigmaOS-terminal-text-effects — terminal ASCII art & text effects.
//!
//! Renders text as ASCII-art banners using built-in bitmap fonts,
//! applies terminal effects (box borders, glow via ANSI 256-color,
//! rainbow gradient, invert/blink via ANSI codes), centers/pads output,
//! and supports multi-line composition. Deterministic — fonts are
//! embedded, no external data files needed.

use std::fmt;
use std::path::PathBuf;

/// A bitmap font: each char maps to rows of '#'/' '.
pub type Font = std::collections::BTreeMap<char, Vec<String>>;

/// Built-in 5-row font covering ASCII printable subset.
pub fn font_standard() -> Font {
    let mut f = Font::new();
    // Each glyph: 5 rows. Use a compact but readable font.
    let glyphs: &[(&str, [&str; 5])] = &[
        ("A", [" ### ", "#   #", "#####", "#   #", "#   #"]),
        ("B", ["#### ", "#   #", "#### ", "#   #", "#### "]),
        ("C", [" ####", "#    ", "#    ", "#    ", " ####"]),
        ("D", ["#### ", "#   #", "#   #", "#   #", "#### "]),
        ("E", ["#####", "#    ", "#### ", "#    ", "#####"]),
        ("F", ["#####", "#    ", "#### ", "#    ", "#    "]),
        ("G", [" ####", "#    ", "#  ##", "#   #", " ####"]),
        ("H", ["#   #", "#   #", "#####", "#   #", "#   #"]),
        ("I", ["#####", "  #  ", "  #  ", "  #  ", "#####"]),
        ("J", ["    #", "    #", "    #", "#   #", " ### "]),
        ("K", ["#   #", "#  # ", "###  ", "#  # ", "#   #"]),
        ("L", ["#    ", "#    ", "#    ", "#    ", "#####"]),
        ("M", ["#   #", "## ##", "# # #", "#   #", "#   #"]),
        ("N", ["#   #", "##  #", "# # #", "#  ##", "#   #"]),
        ("O", [" ### ", "#   #", "#   #", "#   #", " ### "]),
        ("P", ["#### ", "#   #", "#### ", "#    ", "#    "]),
        ("Q", [" ### ", "#   #", "# # #", "#  # ", " ## #"]),
        ("R", ["#### ", "#   #", "#### ", "#  # ", "#   #"]),
        ("S", [" ####", "#    ", " ### ", "    #", "#### "]),
        ("T", ["#####", "  #  ", "  #  ", "  #  ", "  #  "]),
        ("U", ["#   #", "#   #", "#   #", "#   #", " ### "]),
        ("V", ["#   #", "#   #", "#   #", " # # ", "  #  "]),
        ("W", ["#   #", "#   #", "# # #", "## ##", "#   #"]),
        ("X", ["#   #", " # # ", "  #  ", " # # ", "#   #"]),
        ("Y", ["#   #", " # # ", "  #  ", "  #  ", "  #  "]),
        ("Z", ["#####", "   # ", "  #  ", " #   ", "#####"]),
        ("0", [" ### ", "#  ##", "# # #", "##  #", " ### "]),
        ("1", ["  #  ", " ##  ", "  #  ", "  #  ", "#####"]),
        ("2", [" ### ", "#   #", "  ## ", " #   ", "#####"]),
        ("3", ["#### ", "    #", " ### ", "    #", "#### "]),
        ("4", ["   # ", "  ## ", " # # ", "#####", "   # "]),
        ("5", ["#####", "#    ", "#### ", "    #", "#### "]),
        ("6", [" ### ", "#    ", "#### ", "#   #", " ### "]),
        ("7", ["#####", "    #", "   # ", "  #  ", " #   "]),
        ("8", [" ### ", "#   #", " ### ", "#   #", " ### "]),
        ("9", [" ### ", "#   #", " ####", "    #", " ### "]),
        (" ", ["     ", "     ", "     ", "     ", "     "]),
        ("-", ["     ", "     ", "#####", "     ", "     "]),
        ("_", ["     ", "     ", "     ", "     ", "#####"]),
        (".", ["     ", "     ", "     ", " ##  ", " ##  "]),
        (",", ["     ", "     ", "     ", " ##  ", "#    "]),
        ("!", ["  #  ", "  #  ", "  #  ", "     ", "  #  "]),
        ("?", [" ### ", "#   #", "  ## ", "     ", "  #  "]),
        ("/", ["    #", "   # ", "  #  ", " #   ", "#    "]),
        ("\\", ["#    ", " #   ", "  #  ", "   # ", "    #"]),
        ("(", ["   # ", "  #  ", " #   ", "  #  ", "   # "]),
        (")", [" #   ", "  #  ", "   # ", "  #  ", " #   "]),
        ("+", ["     ", "  #  ", "#####", "  #  ", "     "]),
        ("=", ["     ", "#####", "     ", "#####", "     "]),
        ("*", ["     ", "# # #", " ### ", "# # #", "     "]),
        ("#", [" # # ", "#####", " # # ", "#####", " # # "]),
        ("@", [" ### ", "#   #", "# ###", "#    ", " ### "]),
        ("%", ["##  #", "## # ", "  #  ", " #   ", "#  ##"]),
        ("&", [" ##  ", "#  # ", " ##  ", "# #  ", " ## #"]),
    ];
    for (ch, rows) in glyphs {
        f.insert(ch.chars().next().unwrap(), rows.iter().map(|s| s.to_string()).collect());
    }
    // lowercase → uppercase mapping handled at render
    f
}

/// Render text as ASCII art lines.
pub fn render_banner(text: &str, font: &Font) -> Vec<String> {
    let upper = text.to_uppercase();
    let mut rows = vec![String::new(); 5];
    let mut first = true;
    for ch in upper.chars() {
        if let Some(glyph) = font.get(&ch) {
            for (i, row) in glyph.iter().enumerate() {
                if !first { rows[i].push(' '); }
                rows[i].push_str(row);
            }
        } else {
            // unknown char → blank glyph width
            for i in 0..5 {
                if !first { rows[i].push(' '); }
                rows[i].push_str("     ");
            }
        }
        first = false;
    }
    // trim trailing whitespace each row
    rows.iter().map(|r| r.trim_end().to_string()).collect()
}

/// ANSI 256-color helpers.
pub fn ansi_fg_256(color: u8) -> String {
    format!("\x1b[38;5;{}m", color)
}
pub fn ansi_bg_256(color: u8) -> String {
    format!("\x1b[48;5;{}m", color)
}
pub const ANSI_RESET: &str = "\x1b[0m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_DIM: &str = "\x1b[2m";
pub const ANSI_INVERSE: &str = "\x1b[7m";
pub const ANSI_BLINK: &str = "\x1b[5m";
pub const ANSI_UNDERLINE: &str = "\x1b[4m";

/// Effect types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    None,
    /// Box-drawing border around art.
    Box,
    /// ANSI bold.
    Bold,
    /// ANSI dim.
    Dim,
    /// ANSI inverse video.
    Inverse,
    /// ANSI blink.
    Blink,
    /// ANSI underline.
    Underline,
    /// Rainbow gradient across lines (256-color).
    Rainbow,
    /// Single-color 256-color tint.
    Tint(u8),
    /// Glow: bold + bright color + padding spaces.
    Glow(u8),
}
impl Effect {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.to_lowercase().as_str() {
            "none" => Self::None,
            "box" => Self::Box,
            "bold" => Self::Bold,
            "dim" => Self::Dim,
            "inverse" => Self::Inverse,
            "blink" => Self::Blink,
            "underline" => Self::Underline,
            "rainbow" => Self::Rainbow,
            "glow" => Self::Glow(15),
            _ if s.starts_with("tint:") => Self::Tint(s[5..].parse().ok()?),
            _ if s.starts_with("glow:") => Self::Glow(s[5..].parse().ok()?),
            _ => return None,
        })
    }
}

/// Apply an effect to rendered banner lines.
pub fn apply_effect(lines: &[String], effect: &Effect) -> Vec<String> {
    match effect {
        Effect::None => lines.to_vec(),
        Effect::Box => box_border(lines),
        Effect::Bold => lines.iter().map(|l| format!("{}{}{}", ANSI_BOLD, l, ANSI_RESET)).collect(),
        Effect::Dim => lines.iter().map(|l| format!("{}{}{}", ANSI_DIM, l, ANSI_RESET)).collect(),
        Effect::Inverse => lines.iter().map(|l| format!("{}{}{}", ANSI_INVERSE, l, ANSI_RESET)).collect(),
        Effect::Blink => lines.iter().map(|l| format!("{}{}{}", ANSI_BLINK, l, ANSI_RESET)).collect(),
        Effect::Underline => lines.iter().map(|l| format!("{}{}{}", ANSI_UNDERLINE, l, ANSI_RESET)).collect(),
        Effect::Rainbow => {
            // cycle through 6 hues across lines
            let colors = [196u8, 208, 226, 46, 51, 21];
            lines.iter().enumerate()
                .map(|(i, l)| format!("{}{}{}", ansi_fg_256(colors[i % colors.len()]), l, ANSI_RESET))
                .collect()
        }
        Effect::Tint(c) => {
            let code = ansi_fg_256(*c);
            lines.iter().map(|l| format!("{}{}{}", code, l, ANSI_RESET)).collect()
        }
        Effect::Glow(c) => {
            let code = ansi_fg_256(*c);
            lines.iter().map(|l| {
                let padded = format!(" {} ", l);
                format!("{}{}{}", code, ANSI_BOLD, padded)
            }).chain(std::iter::once(ANSI_RESET.to_string())).collect()
        }
    }
}

/// Wrap lines in a box-drawing border.
pub fn box_border(lines: &[String]) -> Vec<String> {
    if lines.is_empty() { return vec![]; }
    let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let mut out = vec![format!("┌{}┐", "─".repeat(w + 2))];
    for l in lines {
        let pad = w - l.chars().count();
        out.push(format!("│ {}{} │", l, " ".repeat(pad)));
    }
    out.push(format!("└{}┘", "─".repeat(w + 2)));
    out
}

/// Center each line to given width.
pub fn center(lines: &[String], width: usize) -> Vec<String> {
    lines.iter().map(|l| {
        let len = l.chars().count();
        if len >= width { return l.clone(); }
        let total = width - len;
        let left = total / 2;
        let right = total - left;
        format!("{}{}{}", " ".repeat(left), l, " ".repeat(right))
    }).collect()
}

/// The effects engine: compose render+effect+layout.
pub struct TextEffects {
    pub font: Font,
}
impl TextEffects {
    pub fn new() -> Self {
        Self { font: font_standard() }
    }

    /// Full pipeline: text → banner → effect → optional center.
    pub fn render(&self, text: &str, effect: &Effect, width: Option<usize>) -> Vec<String> {
        let mut lines = render_banner(text, &self.font);
        lines = apply_effect(&lines, effect);
        if let Some(w) = width {
            // center before ANSI codes get messy — center raw first
            let raw = render_banner(text, &self.font);
            let centered = center(&raw, w);
            lines = apply_effect(&centered, effect);
        }
        lines
    }

    /// Render to a single string.
    pub fn render_string(&self, text: &str, effect: &Effect, width: Option<usize>) -> String {
        self.render(text, effect, width).join("\n")
    }

    /// List available effects.
    pub fn list_effects() -> Vec<&'static str> {
        vec!["none","box","bold","dim","inverse","blink","underline","rainbow","tint:N","glow:N"]
    }
}
impl Default for TextEffects { fn default() -> Self { Self::new() } }

/// Persistence: saved presets (name → text+effect+width).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BannerPreset {
    pub name: String,
    pub text: String,
    pub effect: String,
    pub width: Option<usize>,
}

#[derive(Debug, Default)]
pub struct PresetStore {
    pub presets: Vec<BannerPreset>,
}
impl PresetStore {
    pub fn add(&mut self, p: BannerPreset) {
        self.presets.retain(|x| x.name != p.name);
        self.presets.push(p);
    }
    pub fn get(&self, name: &str) -> Option<&BannerPreset> {
        self.presets.iter().find(|p| p.name == name)
    }
    pub fn remove(&mut self, name: &str) -> bool {
        let n = self.presets.len();
        self.presets.retain(|p| p.name != name);
        self.presets.len() != n
    }
}

// ---- persistence ----
impl PresetStore {
    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        let j = serde_json::to_string_pretty(&self.presets).map_err(|e| e.to_string())?;
        if let Some(d) = path.parent() { std::fs::create_dir_all(d).map_err(|e| e.to_string())?; }
        std::fs::write(path, j).map_err(|e| e.to_string())
    }
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let txt = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let presets: Vec<BannerPreset> = serde_json::from_str(&txt).map_err(|e| e.to_string())?;
        Ok(Self { presets })
    }
    pub fn default_path() -> PathBuf {
        if let Ok(p) = std::env::var("SIGMAOS_BANNER_FILE") { return PathBuf::from(p); }
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()))
            .join(".local/share/sigmaos/banners.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_has_alphabet() {
        let f = font_standard();
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars() {
            assert!(f.contains_key(&c), "missing glyph {}", c);
        }
    }

    #[test]
    fn render_basic() {
        let f = font_standard();
        let lines = render_banner("HI", &f);
        assert_eq!(lines.len(), 5);
        // each line non-empty
        assert!(lines.iter().all(|l| !l.is_empty()));
        // 'H' and 'I' glyphs have different patterns
        assert_ne!(lines[0], lines[1]);
    }

    #[test]
    fn render_lowercase_uppercases() {
        let f = font_standard();
        let a = render_banner("abc", &f);
        let b = render_banner("ABC", &f);
        assert_eq!(a, b);
    }

    #[test]
    fn render_unknown_char_blank() {
        let f = font_standard();
        let lines = render_banner("A€B", &f);
        assert_eq!(lines.len(), 5);
        // doesn't panic, produces output
        assert!(lines.iter().any(|l| l.contains('#')));
    }

    #[test]
    fn box_border_test() {
        let lines = vec!["abc".to_string(), "de".to_string()];
        let boxed = box_border(&lines);
        assert_eq!(boxed.len(), 4); // top + 2 + bottom
        assert!(boxed[0].starts_with('┌'));
        assert!(boxed[3].starts_with('└'));
        assert!(boxed[1].starts_with('│'));
    }

    #[test]
    fn effects_apply() {
        let lines = vec!["test".to_string()];
        assert_eq!(apply_effect(&lines, &Effect::None).len(), 1);
        assert!(apply_effect(&lines, &Effect::Bold)[0].contains("\x1b[1m"));
        assert!(apply_effect(&lines, &Effect::Inverse)[0].contains("\x1b[7m"));
        assert!(apply_effect(&lines, &Effect::Tint(196))[0].contains("38;5;196"));
        let rb = apply_effect(&lines, &Effect::Rainbow);
        assert!(rb[0].contains("38;5;"));
    }

    #[test]
    fn center_lines() {
        let lines = vec!["ab".to_string(), "abcd".to_string()];
        let c = center(&lines, 10);
        assert_eq!(c[0].len(), 10);
        assert_eq!(c[1].len(), 10);
        assert!(c[0].starts_with(' '));
    }

    #[test]
    fn effect_parse() {
        assert_eq!(Effect::parse("box"), Some(Effect::Box));
        assert_eq!(Effect::parse("bold"), Some(Effect::Bold));
        assert_eq!(Effect::parse("tint:196"), Some(Effect::Tint(196)));
        assert_eq!(Effect::parse("glow:15"), Some(Effect::Glow(15)));
        assert_eq!(Effect::parse("bogus"), None);
    }

    #[test]
    fn text_effects_pipeline() {
        let te = TextEffects::new();
        let out = te.render("OK", &Effect::Box, Some(40));
        assert!(!out.is_empty());
        // box adds border lines
        assert!(out.iter().any(|l| l.contains('┌') || l.contains('│')));
    }

    #[test]
    fn preset_store() {
        let mut ps = PresetStore::default();
        ps.add(BannerPreset { name:"welcome".into(), text:"HI".into(),
            effect:"box".into(), width:Some(40) });
        assert!(ps.get("welcome").is_some());
        ps.add(BannerPreset { name:"welcome".into(), text:"BYE".into(),
            effect:"bold".into(), width:None });
        assert_eq!(ps.presets.len(), 1);
        assert_eq!(ps.get("welcome").unwrap().text, "BYE");
        assert!(ps.remove("welcome"));
        assert!(ps.get("welcome").is_none());
    }

    #[test]
    fn preset_persistence() {
        let dir = std::env::temp_dir().join(format!("sigmaos-banner-{}", std::process::id()));
        let p = dir.join("banners.json");
        let mut ps = PresetStore::default();
        ps.add(BannerPreset { name:"test".into(), text:"HELLO".into(),
            effect:"rainbow".into(), width:None });
        ps.save(&p).unwrap();
        let ps2 = PresetStore::load(&p).unwrap();
        assert_eq!(ps2.presets.len(), 1);
        assert_eq!(ps2.presets[0].name, "test");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
