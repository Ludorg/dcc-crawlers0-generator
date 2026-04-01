# dcc-crawlers0-generator

[![Bluesky](https://img.shields.io/badge/Bluesky-@ludorg.bsky.social-0096FF?logo=bluesky&logoColor=white)](https://bsky.app/profile/ludorg.bsky.social) [![License](https://img.shields.io/github/license/ludorg/dcc-crawlers0-generator)](LICENSE) [![Language: Rust](https://img.shields.io/badge/language-Rust-121011?logo=rust&logoColor=white)](https://www.rust-lang.org)

Générateur de personnages "zero-level" pour Dungeon Crawl Classics (DCC). Ce dépôt fournit
deux utilitaires Rust : l'un pour générer les données de personnage (`lvl0_char_gen`) et l'autre
pour rendre des fiches imprimables à partir de templates (`sheet_gen`). Le contenu est fourni en
français et conçu pour être facilement personnalisable.

## Description

Pipeline en deux étapes :

1. `lvl0_char_gen` génère des fichiers JSON décrivant un personnage zero-level.
2. `sheet_gen` remplit une image-template en positionnant les textes selon un fichier de
   coordonnées pour produire une fiche PNG prête à l'impression.

## Fonctionnalités

- Génération de personnages zero-level en JSON.
- Rendu de fiches PNG à partir de templates et d'un fichier de coordonnées.
- Assets et templates organisés sous `data/` pour faciliter la personnalisation.
- Exemple de configuration TOML et fichier de coordonnées inclus.
- Script `run.sh` pour exécuter le pipeline rapidement.

## Personnalisation

- Modifiez/ajoutez des templates PNG, polices et traductions sous `data/`.
- Le fichier de coordonnées (format `clé X Y`) mappe les clés JSON aux positions sur la fiche.
- `sheet_gen` peut lire une configuration TOML (chemins d'entrée/sortie, template, police, coords).

## Exemples de configuration

- [data/sheet_gen/config_example.toml](data/sheet_gen/config_example.toml)
- [data/sheet_gen/fr/frozen-in-time/char_sheet_coord.example.txt](data/sheet_gen/fr/frozen-in-time/char_sheet_coord.example.txt)

## Prérequis

- Rust (stable) et Cargo
- Shell Unix pour exécuter les scripts fournis

## Installation & exécution

Construire le workspace :

```sh
cargo build --workspace
```

Générer un personnage :

```sh
cd lvl0_char_gen
cargo run --release
# sortie : output/personnage.json
```

Générer une fiche :

```sh
cd ../sheet_gen
cargo run --release
# par défaut lit output/personnage.json et écrit output/fiche_personnage.png
```

Lancer le pipeline complet :

```sh
./run.sh
```

Utilisation avancée (avec config TOML) :

```sh
# sheet_gen accepte maintenant un chemin de config TOML en premier argument
cargo run --bin sheet_gen -- data/sheet_gen/config_example.toml
```

## Structure du dépôt

- `lvl0_char_gen/` : génération des données JSON
- `sheet_gen/` : rendu des fiches depuis les JSON
- `data/` : assets, templates et exemples de config
- `run.sh` : script pour lancer le pipeline
- `Cargo.toml` : workspace
- `LICENSE` : licence du projet

## Développement

- Itérez avec `cargo run` dans la crate ciblée et vérifiez `output/`.
- Pour ajouter un champ sur la fiche, ajoutez la clé dans le fichier de coordonnées
  et mettez à jour le template PNG.


## Contribution

- Ouvrez une issue pour discuter des changements majeurs.
- Proposez une pull request pour corrections et améliorations.
- Ajoutez des exemples et tests pour les nouvelles fonctionnalités.

## Licence & contact

Ce projet est distribué sous licence MIT ([LICENSE](LICENSE)). Pour toute question,
ouvrez une issue ou contactez moi via GitHub.

## Remerciements

Merci d'utiliser `dcc-crawlers0-generator`.

## Note sur l'origine

Une partie du code et du contenu de ce `README.md` a été créée avec Copilot, puis relue et validée par l'auteur du dépôt.
