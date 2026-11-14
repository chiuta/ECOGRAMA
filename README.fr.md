# ECOGRAMA — Éditeur de spectrogramme et restauration audio : sélectionner sur le spectrogramme, réparer, exporter.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-14

🌐 https://ecograma.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- STFT à fenêtre et pas configurables, palettes de couleurs (viridis, magma, niveaux de gris) et lecture/écriture WAV (PCM 16/24 bits, flottant 32 bits).
- Une vue spectrogramme dans egui, avec zoom et déplacement.
- Sélections rectangulaires et au lasso qui produisent un masque temps–fréquence, atténué par resynthèse overlap-add.
- Réparation spectrale par interpolation des trames voisines et suppression du ronflement, qui détecte 50/60 Hz et les harmoniques.
- Un habillage de plug-in CLAP (VST3 plus tard) et une décision sur la relation avec `cathar` : dépendre, interopérer ou contribuer en amont.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
