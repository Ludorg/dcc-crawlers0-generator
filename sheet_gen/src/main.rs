use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (i32, i32);

#[derive(Debug, Deserialize, Serialize)]
struct Character {
    attributes: Vec<(String, i32, i32)>, // (Nom, Bonus, Valeur brute)
    total_bonus: i32,
    pv: i32,
    augure_num: i32,
    augure_titre: String,
    augure_effet: String,
    augure_bonus: i32,
    metier: String,
    arme: String,
    equipement: String,
    equipement_supp: String,
    starting_money: i32,
    armor_class: i32, // Nouvelle classe d'armure
    atk_cac: i32,
    atk_dis: i32,
    deg_cac: i32,
    deg_dis: i32,
    deg_0: i32,
    js_ref: i32,
    js_vig: i32,
    js_vol: i32,
    init: i32,
    nb_lang: i32,
    mvt: f32,
    degats_arme: String,
    action_dice: String,
    attack: String,
    crit_dice: String,
    crit_table: String,
    langs: String,
    title: String,
    level: i32,
}
// Exemple de format du fichier char_sheet_coord.txt
// augure_num x y
// attributes[Force].1 x y
// attributes[Force].2 x y

fn load_coordinates(file_path: &str) -> HashMap<String, Coord> {
    let mut coords = HashMap::new();
    let file = File::open(file_path).expect("Fichier char_sheet_coord manquant");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let key = parts[0].to_string();
            let x: i32 = parts[1].parse().expect("Coordonnée X invalide");
            let y: i32 = parts[2].parse().expect("Coordonnée Y invalide");
            coords.insert(key, (x, y));
        }
    }

    coords
}

fn format_bonus(bonus: i32) -> String {
    if bonus > 0 {
        format!("+{}", bonus)
    } else {
        bonus.to_string()
    }
}

fn main() {
    // Charge la fiche PNG vierge
    let mut img = image::open("./data/sheet_gen/fr/DCC_Fiche_Niv0.png")
        .expect("DCC_Fiche_Niv0.png manquante")
        .to_rgba8();

    // Charge la police
    let font_data =
        std::fs::read("./data/sheet_gen/assets/DejaVuSans.ttf").expect("Police manquante");
    let font = Font::try_from_vec(font_data).unwrap();
    let scale = Scale { x: 28.0, y: 28.0 }; // 40 pour fr/hyper-cube-of-myt

    // Charge le personnage
    let file = File::open("output/personnage.json").unwrap();
    let reader = BufReader::new(file);
    let character: Character = serde_json::from_reader(reader).unwrap();
    println!("{:?}", character);

    // Charge les coordonnées
    let coords = load_coordinates("./data/sheet_gen/fr/char_sheet_coord.txt");
    if coords.is_empty() {
        panic!("Le fichier char_sheet_coord.txt est vide ou manquant");
    }

    let black = Rgba([0u8, 0u8, 0u8, 255u8]);

    // Remplis les champs
    if let Some(&(x, y)) = coords.get("metier") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.metier);
    }
    if let Some(&(x, y)) = coords.get("pv") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &character.pv.to_string(),
        );
    }
    for (_i, (nom, bonus, val)) in character.attributes.iter().enumerate() {
        if let Some(&(x, y)) = coords.get(&format!("attributes[{}].1", nom)) {
            draw_text_mut(&mut img, black, x, y, scale, &font, &format_bonus(*bonus));
        }
        if let Some(&(x, y)) = coords.get(&format!("attributes[{}].2", nom)) {
            draw_text_mut(&mut img, black, x, y, scale, &font, &val.to_string());
        }
    }
    if let Some(&(x, y)) = coords.get("arme") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!("{}", &character.arme),
        );
    }
    if let Some(&(x, y)) = coords.get("degats_arme") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!("{}", &character.degats_arme),
        );
    }

    if let Some(&(x, y)) = coords.get("equipement") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.equipement);
    }
    if let Some(&(x, y)) = coords.get("equipement_supp") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &character.equipement_supp,
        );
    }
    if let Some(&(x, y)) = coords.get("starting_money") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!("{} pc", character.starting_money),
        );
    }
    if let Some(&(x, y)) = coords.get("augure_titre") {
        let augure_full = format!("{} : {}", character.augure_titre, character.augure_effet);
        draw_text_mut(&mut img, black, x, y, scale, &font, &augure_full);
    }
    if let Some(&(x, y)) = coords.get("augure_bonus") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!(
                "Bonus augure de naissance : {}",
                format_bonus(character.augure_bonus)
            ),
        );
    }

    if let Some(&(x, y)) = coords.get("augure_num") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!("{}", character.augure_num),
        );
    }

    if let Some(&(x, y)) = coords.get("armor_class") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &character.armor_class.to_string(),
        );
    }

    if let Some(&(x, y)) = coords.get("js_ref") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.js_ref),
        );
    }
    if let Some(&(x, y)) = coords.get("js_vig") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.js_vig),
        );
    }
    if let Some(&(x, y)) = coords.get("js_vol") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.js_vol),
        );
    }

    if let Some(&(x, y)) = coords.get("atk_cac") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.atk_cac),
        );
    }
    if let Some(&(x, y)) = coords.get("atk_dis") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.atk_dis),
        );
    }
    if let Some(&(x, y)) = coords.get("deg_cac") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.deg_cac),
        );
    }
    if let Some(&(x, y)) = coords.get("deg_dis") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.deg_dis),
        );
    }

    if let Some(&(x, y)) = coords.get("init") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format_bonus(character.init),
        );
    }

    if let Some(&(x, y)) = coords.get("action_dice") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.action_dice);
    }
    if let Some(&(x, y)) = coords.get("attack") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.attack);
    }
    if let Some(&(x, y)) = coords.get("crit_dice") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.crit_dice);
    }
    if let Some(&(x, y)) = coords.get("crit_table") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.crit_table);
    }

    if let Some(&(x, y)) = coords.get("langs") {
        let mut langs_str = character.langs.clone();
        if character.nb_lang > 0 {
            if !langs_str.is_empty() {
                langs_str.push_str(", ");
            }
            langs_str.push_str(&format!("{} autre(s) langue(s)", character.nb_lang));
        }
        draw_text_mut(&mut img, black, x, y, scale, &font, &langs_str);
    }

    if let Some(&(x, y)) = coords.get("mvt") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &format!("{} m", character.mvt),
        );
    }

    if let Some(&(x, y)) = coords.get("title") {
        draw_text_mut(&mut img, black, x, y, scale, &font, &character.title);
    }
    if let Some(&(x, y)) = coords.get("level") {
        draw_text_mut(
            &mut img,
            black,
            x,
            y,
            scale,
            &font,
            &character.level.to_string(),
        );
    }

    // Sauvegarde la fiche remplie
    img.save("output/fiche_personnage.png").unwrap();
}
