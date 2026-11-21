//! SigmaOS-terminal-text-effects — banner CLI.
//!
//! Usage:
//!   sigmaos-banner "TEXT" [--effect box|bold|dim|inverse|blink|underline|rainbow|tint:N|glow:N]
//!   sigmaos-banner "TEXT" --width N --effect box
//!   sigmaos-banner save NAME "TEXT" EFFECT [WIDTH]
//!   sigmaos-banner list | show NAME | remove NAME
//!
//! Presets: ~/.local/share/sigmaos/banners.json

use sigmaos_terminal_text_effects::{Effect, PresetStore, TextEffects, BannerPreset};
use std::path::PathBuf;
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: sigmaos-banner <text|save|list|show|remove|effects> [args]");
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() { usage(); }
    let db = PresetStore::default_path();
    let mut ps = PresetStore::load(&db).unwrap_or_default();
    let te = TextEffects::new();

    let result: Result<(), String> = (|| -> Result<(), String> { match args[0].as_str() {
        "effects" => {
            for e in TextEffects::list_effects() { println!("{}", e); }
            Ok(())
        }
        "list" => {
            for p in &ps.presets {
                println!("{:<16} {:<20} {:<10} width={:?}", p.name, p.text, p.effect, p.width);
            }
            Ok(())
        }
        "save" => {
            if args.len() < 4 { return Err("save NAME TEXT EFFECT [WIDTH]".into()); }
            let width = args.get(4).and_then(|s| s.parse().ok());
            ps.add(BannerPreset { name: args[1].clone(), text: args[2].clone(),
                effect: args[3].clone(), width });
            println!("saved {}", args[1]);
            Ok(())
        }
        "show" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let p = ps.get(name).ok_or_else(|| format!("no preset: {}", name))?;
            let eff = Effect::parse(&p.effect).unwrap_or(Effect::None);
            print!("{}", te.render_string(&p.text, &eff, p.width));
            println!();
            Ok(())
        }
        "remove" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if ps.remove(name) { println!("removed"); Ok(()) }
            else { Err(format!("no preset: {}", name)) }
        }
        text => {
            // parse --effect X --width N flags
            let mut effect = Effect::None;
            let mut width: Option<usize> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--effect" => { i+=1; effect = Effect::parse(args.get(i).map(|s|s.as_str()).unwrap_or("none")).unwrap_or(Effect::None); }
                    "--width" => { i+=1; width = args.get(i).and_then(|s| s.parse().ok()); }
                    other => return Err(format!("unknown flag: {}", other)),
                }
                i += 1;
            }
            print!("{}", te.render_string(text, &effect, width));
            println!();
            Ok(())
        }
    } })();

    match result {
        Ok(()) => { if let Err(e)=ps.save(&db) { eprintln!("sigmaos-banner: warn: save: {e}"); } }
        Err(e) => { eprintln!("sigmaos-banner: {}", e); exit(1); }
    }
}
