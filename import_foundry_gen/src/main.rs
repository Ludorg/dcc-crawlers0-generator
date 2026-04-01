use anyhow::{Context, Result};
use serde_json::Value;
use std::env;
use std::fs;

fn french_attr_to_english(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "force" => "Strength".to_string(),
        "agilité" | "agilite" => "Agility".to_string(),
        "endurance" => "Stamina".to_string(),
        "présence" | "presence" => "Personality".to_string(),
        "intelligence" => "Intelligence".to_string(),
        "chance" => "Luck".to_string(),
        other => other.to_string(),
    }
}

fn fmt_attr(attr: &Value) -> Option<String> {
    // expect [name, bonus, value]
    if let Some(arr) = attr.as_array() {
        if arr.len() >= 3 {
            let name = arr[0].as_str().unwrap_or("?");
            let bonus = arr[1].as_i64().unwrap_or(0);
            let value = arr[2].as_i64().unwrap_or(0);
            let eng = french_attr_to_english(name);
            return Some(format!("{}: {} ({:+})", eng, value, bonus));
        }
    }
    None
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.json>", args[0]);
        std::process::exit(2);
    }
    let path = &args[1];
    let data = fs::read_to_string(path).with_context(|| format!("reading {}", path))?;
    let v: Value = serde_json::from_str(&data).with_context(|| "parsing json")?;

    // Header: level and occupation/metier
    let level = v.get("level").and_then(|x| x.as_i64()).unwrap_or(0);
    let metier = v.get("metier").and_then(|x| x.as_str()).unwrap_or("-");
    println!("{}-level Occupation: {}", level, metier);

    // Attributes
    if let Some(attrs) = v.get("attributes").and_then(|x| x.as_array()) {
        for a in attrs {
            if let Some(s) = fmt_attr(a) {
                println!("{}", s);
            }
        }
    }

    println!("");

    // AC and HP
    let ac = v.get("armor_class").and_then(|x| x.as_i64()).unwrap_or(0);
    let hp = v.get("pv").and_then(|x| x.as_i64()).unwrap_or(0);
    println!("AC: {}; HP: {}", ac, hp);

    // Weapon
    let arme = v.get("arme").and_then(|x| x.as_str()).unwrap_or("-");
    let atk_cac = v.get("atk_cac").and_then(|x| x.as_i64()).unwrap_or(0);
    let degats_raw = v.get("degats_arme").and_then(|x| x.as_str()).unwrap_or("-");
    let degats = match degats_raw.find('(') {
        Some(i) => degats_raw[..i].trim(),
        None => degats_raw,
    };
    println!("Weapon: {} {:+} ({})", arme, atk_cac, degats);

    // Speed, Init, Saves
    let mvt = v.get("mvt").and_then(|x| x.as_f64()).unwrap_or(0.0);
    let init = v.get("init").and_then(|x| x.as_i64()).unwrap_or(0);
    let js_ref = v.get("js_ref").and_then(|x| x.as_i64()).unwrap_or(0);
    let js_vig = v.get("js_vig").and_then(|x| x.as_i64()).unwrap_or(0);
    let js_vol = v.get("js_vol").and_then(|x| x.as_i64()).unwrap_or(0);
    println!(
        "Speed: {}; Init: {}; Ref: {}; Fort: {}; Will: {}",
        mvt, init, js_ref, js_vig, js_vol
    );

    println!("");

    // Equipment and Trade good (mapping requested)
    let equip = v.get("equipement").and_then(|x| x.as_str()).unwrap_or("-");
    let equip_supp = v
        .get("equipement_supp")
        .and_then(|x| x.as_str())
        .unwrap_or("");
    if !equip_supp.is_empty() {
        println!("Equipment: {}", equip_supp);
    }
    if equip != "-" && !equip.is_empty() {
        println!("Trade good: {}", equip);
    }

    // Starting funds
    let starting = v.get("starting_money").and_then(|x| x.as_i64());
    if let Some(s) = starting {
        println!("Starting Funds: {} cp", s);
    }

    // Lucky sign / augure
    if let Some(augure_titre) = v.get("augure_titre").and_then(|x| x.as_str()) {
        let augure_effet = v.get("augure_effet").and_then(|x| x.as_str()).unwrap_or("");
        let augure_bonus = v.get("augure_bonus").and_then(|x| x.as_i64()).unwrap_or(0);
        if !augure_effet.is_empty() {
            println!(
                "Lucky sign: {} ({}) ({:+})",
                augure_titre, augure_effet, augure_bonus
            );
        } else {
            println!("Lucky sign: {} ({:+})", augure_titre, augure_bonus);
        }
    }

    // Languages
    if let Some(langs) = v.get("langs").and_then(|x| x.as_str()) {
        println!("Languages: {}", langs);
    }

    // Empty line at the end
    println!("");

    Ok(())
}
