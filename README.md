# dcc-crawlers0-generator

[![Bluesky](https://img.shields.io/badge/Bluesky-@ludorg.bsky.social-0096FF?logo=bluesky&logoColor=white)](https://bsky.app/profile/ludorg.bsky.social)

Générateur de personnages "zero-level" pour Dungeon Crawl Classics (DCC), ciblé pour une sortie JSON
en français. Ce dépôt contient des outils et des crates Rust pour générer des feuilles de personnages
et des variantes de sorties utilisables par d'autres outils.

Table des matières
- Description
- Fonctionnalités
- Prérequis
- Installation & exécution
- Structure du dépôt
- Développement
- Contribution
- Licence

Description
-----------
Ce projet automatise la génération de personnages de niveau 0 (zero-level) pour DCC. Il produit des
fichiers JSON prêts à être utilisés par des feuilles ou d'autres systèmes de traitement. Le contenu
est principalement en français et conçu pour être facilement traduit.

Fonctionnalités
- Génération de personnages zero-level en JSON
- Organisation des assets et des templates sous `data/`
- Deux utilitaires/crates principaux : `lvl0_char_gen` et `sheet_gen`
- Script d'exécution simple `run.sh` pour lancer la génération

Personnalisation de la sortie
----------------------------
La sortie est entièrement personnalisable :

- Templates et assets : vous pouvez modifier ou ajouter des templates, traductions et assets
	sous le dossier `data/` pour changer l'apparence et le contenu des feuilles générées.
- Options de la crate `sheet_gen` : la crate de génération de fiche (`sheet_gen`) applique
	des templates et formats configurables. Selon l'implémentation, elle accepte des
	paramètres ou fichiers de configuration (voir la doc et les options de la crate).
- Formats de sortie : le pipeline produit du JSON standardisé dans `output/`; vous pouvez
	transformer ce JSON vers d'autres formats en ajoutant des scripts ou un module de
	post-traitement.

# dcc-crawlers0-generator

[![License](https://img.shields.io/github/license/ludorg/dcc-crawlers0-generator)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-121011?logo=rust&logoColor=white)](https://www.rust-lang.org)

Générateur de personnages "zero-level" pour Dungeon Crawl Classics (DCC).
Ce dépôt contient des utilitaires Rust pour produire des données de personnages (JSON)
et générer des fiches imprimables à partir de templates. Le contenu fourni est en français
et pensé pour être facilement personnalisable et traduit.

Table des matières
- Description
- Fonctionnalités
- Personnalisation
- Exemples de configuration
- Prérequis
- Installation & exécution
- Structure du dépôt
- Développement
- Contribution
- Licence & contact

Description
-----------
Le projet fournit un pipeline simple en deux étapes :

1) génération des données de personnage (crate `lvl0_char_gen`) — produit des fichiers JSON
	 décrivant un personnage zero-level ;
2) génération de la fiche (crate `sheet_gen`) — remplit une image-template en positionnant
	 les textes selon un fichier de coordonnées pour produire une fiche PNG prête à l'impression.

Fonctionnalités
---------------
- Génération automatique de personnages de niveau 0 en JSON;
- Génération de fiches PNG à partir de templates et coordonnées personnalisables;
- Organisation des assets, templates et traductions dans `data/`;
- Exemple de configuration TOML et fichier de coordonnées pour adapter la sortie;
- Script `run.sh` pour lancer rapidement le pipeline depuis la racine.

Personnalisation
---------------
La sortie est entièrement personnalisable :

- Templates et assets : modifiez ou ajoutez des images PNG, polices et traductions sous
	`data/` pour changer l'apparence des fiches ;
- Fichier de coordonnées : mappez les clés JSON aux positions X/Y sur la fiche via
	un fichier texte (clé x y) ;
- Options de `sheet_gen` : la crate lit les chemins (template, police, coords, entrée, sortie)
	depuis une configuration TOML ou des arguments ; adaptez-les pour obtenir le rendu souhaité;
- Post-traitement : transformez le JSON en d'autres formats (PDF, HTML) avec des scripts
	additionnels si besoin.

Exemples de configuration
-------------------------
Un exemple minimal de configuration TOML est inclus :

- [data/sheet_gen/config_example.toml](data/sheet_gen/config_example.toml)

Exemple de fichier de coordonnées (format clé X Y) :

- [data/sheet_gen/fr/frozen-in-time/char_sheet_coord.example.txt](data/sheet_gen/fr/frozen-in-time/char_sheet_coord.example.txt)

Ces fichiers montrent comment indiquer le chemin du JSON d'entrée, le template PNG,
la police à utiliser, le fichier de coordonnées et la sortie désirée.

Prérequis
---------
- Rust (stable) et Cargo
- Outils classiques Unix (shell) pour exécuter les scripts fournis

Installation & exécution
------------------------
Construire les crates :

```sh
cargo build --workspace
```

Générer un personnage (exemple) :

```sh
cd lvl0_char_gen
cargo run --release
# le JSON généré sera écrit dans output/personnage.json
```

Générer une fiche à partir du JSON :

```sh
cd ../sheet_gen
cargo run --release
# par défaut, sheet_gen lit output/personnage.json et écrit output/fiche_personnage.png
```

Ou lancer le pipeline global depuis la racine (script fourni) :

```sh
./run.sh
```

Utilisation avancée (avec fichier de configuration) :

```sh
# exemple : passer un fichier TOML de configuration à sheet_gen (adaptation nécessaire)
cargo run --bin sheet_gen -- --config data/sheet_gen/config_example.toml
```

Structure du dépôt
------------------
- lvl0_char_gen/ : crate responsable de la génération des données JSON
- sheet_gen/ : crate responsable du rendu des fiches depuis les JSON
- data/ : assets, templates, fichiers de configuration et exemples
- output/ et _output/ : exemples de sorties JSON et PNG
- run.sh : script wrapper pour lancer le pipeline
- Cargo.toml : workspace et dépendances
- LICENSE : licence du projet

Développement
-------------
- Itérez localement avec `cargo run` dans la crate ciblée ; vérifiez les fichiers de
	sortie dans `output/` ;
- Pour ajouter un nouveau champ sur la fiche, ajoutez la clé correspondante dans le
	fichier de coordonnées et mettez à jour le template PNG si nécessaire ;
- Si vous souhaitez que `sheet_gen` accepte directement un fichier TOML, je peux
	ajouter un petit parser d'arguments et charger la configuration automatiquement.

Contribution
------------
Les contributions sont bienvenues :

- Ouvrez une issue pour discuter des changements majeurs ;
- Faites une pull request pour les corrections et améliorations ;
- Ajoutez des exemples et des tests pour les nouvelles fonctionnalités.

Licence & contact
-----------------
Le projet est couvert par la licence indiquée dans [LICENSE](LICENSE).
Pour toute question, ouvrez une issue ou contactez le mainteneur via la plateforme de
gestion du code.

Remerciements
-------------
Merci d'utiliser dcc-crawlers0-generator. Dites-moi si vous voulez que j'ajoute :

- support `--config` dans `sheet_gen` ;
- un script d'exécution dédié `scripts/make_sheet.sh` ;
- une traduction complète en anglais du README.
Les fichiers générés se trouvent dans les dossiers `output/` et `_output/` (exemples fournis).

Structure du dépôt
- [lvl0_char_gen](lvl0_char_gen) : crate principale pour générer les personnages
- [sheet_gen](sheet_gen) : génération / formatage des feuilles
- [data](data) : données sources, assets et templates (fr)
- [output](output) et [_output](/_output) : exemples de JSON générés
- [run.sh](run.sh) : script simple pour lancer le processus
- [Cargo.toml](Cargo.toml) : workspace / dépendances
- [LICENSE](LICENSE) : licence du projet

Développement
-------------
- Utilisez `cargo build` puis `cargo run` dans la crate souhaitée pour itérer.
- Respectez le format JSON existant dans `output/` lors de modifications.
- Si vous ajoutez de nouvelles données dans `data/`, documentez-les et fournissez des exemples.

Contribution
------------
Les contributions sont bienvenues : ouvrez une issue pour discuter des changements majeurs,
ou proposez une pull request pour corrections et améliorations. Merci d'ajouter des tests
ou exemples quand c'est pertinent.

Licence
-------
Ce projet est distribué sous la licence MIT indiquée dans le fichier [LICENSE](LICENSE).

Contact
-------
Pour toute question, ouvrez une issue sur le dépôt ou contactez moi via Github.

Merci d'utiliser `dcc-crawlers0-generator`.
