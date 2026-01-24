use rand::Rng;
use regex::Regex;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn roll_dice(sides: u8, count: u8, modifier: u8) -> u8 {
    let mut rng = rand::rng();
    (0..count).map(|_| rng.random_range(1..=sides)).sum::<u8>() + modifier
}

/// Récupère une entrée d'attribut à partir d'un fichier CSV (t1.1.csv)
fn get_attribute_data<P: AsRef<Path>>(
    file_path: P,
    target_value: i32,
) -> Option<(i32, i32, String, String, String)> {
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() >= 5
            && let (Ok(v1), Ok(v2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>())
            && v1 == target_value
        {
            return Some((
                v1,
                v2,
                parts[2].to_string(),
                parts[3].to_string(),
                parts[4].to_string(),
            ));
        }
    }
    None
}

/// Lance 1d4 et ajoute le bonus d'Endurance (minimum 1 PV)
fn roll_hp(endurance_bonus: i32) -> i32 {
    let base: i32 = roll_dice(4, 1, 0) as i32;
    std::cmp::max(1, base + endurance_bonus)
}

/// Détermine l'augure de naissance à partir d'un jet de 1d30 et du fichier t1.2.csv.
/// Retourne (num, titre, effet, attributs_cibles)
fn get_augure_naissance<P: AsRef<Path>>(
    file_path: P,
) -> Option<(i32, String, String, Vec<String>)> {
    let roll = roll_dice(30, 1, 0) as i32;
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() >= 3
            && let Ok(num) = parts[0].parse::<i32>()
            && num == roll
        {
            let attributs_cibles = if parts.len() >= 4 && !parts[3].is_empty() {
                parts[3].split('|').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };
            return Some((
                num,
                parts[1].to_string(),
                parts[2].to_string(),
                attributs_cibles,
            ));
        }
    }
    None
}

/// Détermine le métier de départ à partir d'un jet de 1d100 et du fichier t1.3.csv
fn get_metier<P: AsRef<Path>>(file_path: P) -> Option<(String, String, String)> {
    let roll = roll_dice(100, 1, 0) as i32;
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(\d+)(?:-(\d+))?").unwrap();

    for line in reader.lines().flatten() {
        //if let Ok(line) = line {
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() >= 4
            && let Some(cap) = re.captures(parts[0])
        {
            let min: i32 = cap[1].parse().ok()?;
            let max: i32 = cap
                .get(2)
                .and_then(|m| m.as_str().parse::<i32>().ok())
                .unwrap_or(min);
            if roll >= min && roll <= max {
                return Some((
                    parts[1].to_string(), // Métier
                    parts[2].to_string(), // Arme
                    parts[3].to_string(), // Équipement
                ));
            }
        }
        //}
    }
    None
}

/// Tire un équipement aléatoire sur la table 3-3 (t3.3.csv)
fn get_equipement_aleatoire<P: AsRef<Path>>(file_path: P) -> Option<String> {
    let roll = roll_dice(24, 1, 0) as i32; // Supposons 24 entrées (adapte selon ta table)
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() >= 2
            && let Ok(num) = parts[0].parse::<i32>()
            && num == roll
        {
            return Some(parts[1].to_string());
        }
    }
    None
}

/// Détermine l'argent de départ (5d12 pièces de cuivre)
fn roll_starting_money() -> i32 {
    roll_dice(12, 5, 0) as i32
}

/// Calcule la valeur de mouvement (mvt) en mètres selon le métier.
/// 9 mètres si le métier ne contient ni "halfelin", ni "nain" (casse non sensitive), sinon 6 mètres.
fn calc_mvt(metier: &str) -> f32 {
    let metier_lower = metier.to_lowercase();
    if metier_lower.contains("halfelin") || metier_lower.contains("nain") {
        6.0
    } else {
        9.0
    }
}

/// Calcule la chaîne de dégâts de l'arme, modifiée par le bonus adéquat.
/// Si arme à distance (Portée ≠ "-"), ajoute le bonus deg_dis, sinon deg_cac.
/// Si arme à distance, ajoute la portée entre parenthèses.
/// Si le nom de l'arme contient "(comme X)", utilise X comme nom d'arme pour la recherche.
fn calculer_degats_arme<P: AsRef<std::path::Path>>(
    arme: &str,
    deg_cac: i32,
    deg_dis: i32,
    deg_0: i32,
    file_path: P,
) -> Option<String> {
    let file = File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    // Gère les cas "Perche (comme bâton)"
    let mut nom_arme = arme.trim();
    if let Some(start) = nom_arme.find("(comme ")
        && let Some(end) = nom_arme[start..].find(')')
    {
        let vrai_nom = &nom_arme[start + 7..start + end];
        nom_arme = vrai_nom.trim();
    }

    for line in reader.lines().skip(1).map_while(Result::ok) {
        let parts: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
        if parts.len() >= 4 && parts[0].eq_ignore_ascii_case(nom_arme) {
            let degats = parts[1];
            let portee = parts[2];
            let is_distance = portee != "–" && portee != "-";
            let mut bonus = if is_distance { deg_dis } else { deg_cac };
            if deg_0 != 0 {
                bonus += deg_0;
            }
            let mut degats_arme = if bonus == 0 {
                degats.to_string()
            } else if bonus > 0 {
                format!("{}+{}", degats, bonus)
            } else {
                format!("{}{}", degats, bonus)
            };
            if is_distance {
                degats_arme = format!("{} ({})", degats_arme, portee);
            }
            return Some(degats_arme);
        }
    }
    None
}

#[derive(Debug, Serialize)]
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

impl Character {
    /// Applique le bonus d'augure à l'attribut désigné (si présent)
    fn apply_augure_bonus(&mut self, attribut: &str, augure_bonus: i32) {
        match attribut {
            "armor_class" => self.armor_class += augure_bonus,
            "atk_cac" => self.atk_cac += augure_bonus,
            "atk_dis" => self.atk_dis += augure_bonus,
            "deg_cac" => self.deg_cac += augure_bonus,
            "deg_dis" => self.deg_dis += augure_bonus,
            "deg_0" => self.deg_0 += augure_bonus,
            "js_ref" => self.js_ref += augure_bonus,
            "js_vig" => self.js_vig += augure_bonus,
            "js_vol" => self.js_vol += augure_bonus,
            "init" => self.init += augure_bonus,
            "nb_lang" => self.nb_lang += augure_bonus,
            "mvt" => {
                self.mvt += augure_bonus as f32 * 1.5;
            }
            "pv" => self.pv += augure_bonus,
            _ => {} // Ignore si l'attribut n'est pas reconnu ou vide
        }
    }
}

fn main() {
    let file_path = "./data/fr/t1.1.csv"; // Chemin vers le fichier CSV
    let attributes = [
        "Force",
        "Agilité",
        "Endurance",
        "Intelligence",
        "Présence",
        "Chance",
    ];
    let mut results = Vec::new();
    let mut total_bonus: i32;

    loop {
        total_bonus = 0;
        results.clear();

        for attribute in attributes.iter() {
            let dice_result = roll_dice(6, 3, 0); // Lancer 3d6

            match get_attribute_data(file_path, dice_result as i32) {
                Some(entry) => {
                    println!("{} ({}) : {:?}", attribute, dice_result, entry);
                    total_bonus += entry.1; // Ajouter la valeur du bonus (2e élément du tuple)
                    results.push((attribute, entry.1, dice_result as i32)); // Stocker les résultats en i32
                }
                None => println!(
                    "{} ({}) : Aucune correspondance trouvée",
                    attribute, dice_result
                ),
            }
        }

        if total_bonus > 1 {
            break; // Si la somme des bonus est suffisante, arrêter
        }

        println!(
            "Bonus insuffisant ({}) ! Relance des plus faibles en utilisant 2d6+6...",
            total_bonus
        );

        // Relancer les valeurs avec le plus faible bonus
        results.sort_by(|a, b| a.1.cmp(&b.1)); // Trier par bonus croissant
        for i in 0..results.len() {
            if results[i].1 < 0 {
                let dice_result = roll_dice(6, 2, 6); // Lancer 2d6+6
                match get_attribute_data(file_path, dice_result as i32) {
                    Some(entry) => {
                        println!(
                            "Nouvelle valeur pour {} ({}) : {:?}",
                            results[i].0, dice_result, entry
                        );
                        total_bonus = total_bonus - results[i].1 + entry.1; // Mettre à jour le bonus total
                        results[i] = (results[i].0, entry.1, dice_result as i32);
                        // Mettre à jour les résultats
                    }
                    None => println!(
                        "{} ({}) : Aucune correspondance trouvée",
                        results[i].0, dice_result
                    ),
                }
            }
            if total_bonus > 1 {
                break;
            }
        }
    }

    println!("Total des bonus final : {}", total_bonus);
    println!("Bonus validé !");

    // Supposons que results[2] correspond à Endurance
    let endurance_bonus = results[2].1;
    let pv = roll_hp(endurance_bonus);
    println!("Points de vie (PV) : {}", pv);

    let augure_path = "./data/fr/t1.2.csv";
    let chance_bonus = results[5].1; // "Chance" est le 6e attribut
    let augure_bonus = if chance_bonus > 1 { chance_bonus } else { 1 };

    let agility_bonus = results[1].1; // Supposons que "Agilité" est le 2e attribut
    let base_armor_class = 10; // Classe d'armure de base
    let armor_class = base_armor_class + agility_bonus;
    println!("Classe d'armure initiale (CA) : {}", armor_class);

    let (num, titre, effet, augure_fields) =
        if let Some((num, titre, effet, champs)) = get_augure_naissance(augure_path) {
            println!(
                "Augure de naissance (jet 1d30 = {}): {} — {}",
                num, titre, effet
            );
            println!(
                "Le modificateur de Chance appliqué à ce type de jet est : {}",
                augure_bonus
            );

            (num, titre, effet, champs)
        } else {
            println!("Aucun augure trouvé.");
            (0, String::new(), String::new(), Vec::new())
        };

    let metier_path = "./data/fr/t1.3.csv";

    let (metier, arme, equipement) = if let Some((metier, arme, equip)) = get_metier(metier_path) {
        println!(
            "Métier : {}\nArme : {}\nÉquipement : {}",
            metier, arme, equip
        );
        (metier, arme, equip)
    } else {
        println!("Aucun métier trouvé.");
        (String::new(), String::new(), String::new())
    };

    let mvt = calc_mvt(&metier);

    let equip_rand_path = "./data/fr/t3.3.csv";
    let equipement_supp = if let Some(equipement) = get_equipement_aleatoire(equip_rand_path) {
        println!("Équipement supplémentaire : {}", equipement);
        equipement
    } else {
        println!("Aucun équipement supplémentaire trouvé.");
        String::new()
    };

    let starting_money = roll_starting_money();
    println!("Argent de départ : {} pièces de cuivre", starting_money);

    let force_bonus = results[0].1; // Force est le 1er attribut
    let presence_bonus = results[4].1; // Présence est le 5e attribut
    let intelligence_bonus = results[3].1; // Intelligence est le 4e attribut
    let nb_lang = std::cmp::max(0, intelligence_bonus);

    let mut character = Character {
        attributes: results
            .iter()
            .map(|(nom, bonus, val)| (nom.to_string(), *bonus, *val))
            .collect(),
        total_bonus,
        pv,
        augure_num: num,
        augure_titre: titre,
        augure_effet: effet,
        augure_bonus,
        metier,
        arme,
        equipement,
        equipement_supp,
        starting_money,
        armor_class,
        atk_cac: force_bonus,
        atk_dis: agility_bonus,
        deg_cac: force_bonus,
        deg_dis: 0,
        deg_0: 0,
        js_ref: agility_bonus,
        js_vig: endurance_bonus,
        js_vol: presence_bonus,
        init: agility_bonus,
        nb_lang,
        mvt,
        degats_arme: String::new(),
        action_dice: "1d20".to_string(),
        attack: "+0".to_string(),
        crit_dice: "1d4".to_string(),
        crit_table: "I".to_string(),
        langs: String::new(),
        title: "-".to_string(),
        level: 0,
    };

    // Supposons que augure_field contient le nom du champ à modifier (ex: "armor_class", "atk_cac", etc.)
    if !augure_fields.is_empty() {
        for field in &augure_fields {
            character.apply_augure_bonus(field, augure_bonus);
        }
    }

    let degats_arme = calculer_degats_arme(
        &character.arme,
        character.deg_cac,
        character.deg_dis,
        character.deg_0,
        "./data/fr/t3.1.csv",
    )
    .unwrap_or_else(|| "Inconnu".to_string());

    character.degats_arme = degats_arme;

    let mut langs = "Commun".to_string();
    let metier_lower = character.metier.to_lowercase();
    if metier_lower.contains("nain") {
        langs.push_str(", Nain");
    }
    if metier_lower.contains("elfe") {
        langs.push_str(", Elfe");
    }
    if metier_lower.contains("halfelin") {
        langs.push_str(", Halfelin");
    }
    character.langs = langs;

    println!("{:#?}", character);

    let json = serde_json::to_string_pretty(&character).expect("Erreur de sérialisation JSON");
    let mut file = File::create("personnage.json").expect("Impossible de créer le fichier JSON");
    file.write_all(json.as_bytes())
        .expect("Erreur d'écriture JSON");
    println!("Personnage sauvegardé dans personnage.json");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_csv_entry_all_values() {
        // Création d'un fichier temporaire
        let mut temp_file =
            NamedTempFile::new().expect("Impossible de créer le fichier temporaire");

        // Contenu du fichier CSV simulé
        let csv_content = "\
        10; -2; Aucun ; Pas d'ajustement ; 5;\n\
        11; 0; Aucun ; Pas d'ajustement ; 3;\n\
        12; +1; Aucun ; Léger ajustement ; 2;\n\
        13; +1; Aucun ; Pas d'ajustement ; 4;\n\
        15; +1; +1; +1 sort ; 5;\n\
        16; +2; +2; +1 sort ; 5;\n\
        17; +2; +2; +2 sorts ; 5;\n\
        18; +3; +3; +2 sorts ; 5;\n";

        // Écriture dans le fichier temporaire
        temp_file
            .write_all(csv_content.as_bytes())
            .expect("Échec d'écriture");

        // Chemin du fichier temporaire
        let file_path = temp_file.path().to_str().unwrap();

        // Valeurs à tester
        let expected_values = vec![
            (10, -2, "Aucun", "Pas d'ajustement", "5"),
            (11, 0, "Aucun", "Pas d'ajustement", "3"),
            (12, 1, "Aucun", "Léger ajustement", "2"),
            (13, 1, "Aucun", "Pas d'ajustement", "4"),
            (15, 1, "+1", "+1 sort", "5"),
            (16, 2, "+2", "+1 sort", "5"),
            (17, 2, "+2", "+2 sorts", "5"),
            (18, 3, "+3", "+2 sorts", "5"),
        ];

        // Tester chaque valeur cible
        for (target, bonus, v3, desc1, desc2) in expected_values {
            let entry = get_attribute_data(file_path, target);
            assert!(entry.is_some(), "Ligne {} non trouvée !", target);

            let (v1, v2, v3_found, desc1_found, desc2_found) = entry.unwrap();
            assert_eq!(v1, target);
            assert_eq!(v2, bonus);
            assert_eq!(v3_found, v3.to_string());
            assert_eq!(desc1_found, desc1.to_string());
            assert_eq!(desc2_found, desc2.to_string());

            //println!("✅ Test réussi pour {} -> {:?}", target, entry);
        }
    }
}
