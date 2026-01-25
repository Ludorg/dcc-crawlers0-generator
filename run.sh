#!/bin/bash

cargo build --release

for i in {00..40}
do
    ./target/release/lvl0_char_gen 
    ./target/release/sheet_gen
    cp output/personnage.json output/personnage_$i.json
    cp output/fiche_personnage.png output/fiche_personnage_$i.png
done
