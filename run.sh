#!/bin/bash

cargo build --release

for i in {00..40}
do
    ./target/release/lvl0_char_gen 
    ./target/release/sheet_gen
    cp output/personnage.json output/personnage_$i.json
    cp output/fiche_personnage.png output/fiche_personnage_$i.png
    ./target/release/import_foundry_gen output/personnage_$i.json > output/import_foundry_$i.txt
done
