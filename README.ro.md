# ECOGRAMA — Editor de spectrogramă și restaurare audio: selectezi pe spectrogramă, repari, exporți.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-14

🌐 https://ecograma.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- STFT cu fereastră și pas configurabile, hărți de culori (viridis, magma, tonuri de gri) și citire/scriere WAV (PCM 16/24 biți, float 32 biți).
- O vizualizare a spectrogramei în egui, cu zoom și derulare.
- Selecții dreptunghiulare și cu lasou care produc o mască timp–frecvență, atenuată prin resinteză overlap-add.
- Reparare spectrală prin interpolarea cadrelor învecinate și eliminarea zumzetului, care detectează 50/60 Hz și armonicele.
- Un înveliș de plugin CLAP (VST3 mai târziu) și o decizie despre relația cu `cathar`: dependență, interoperare sau contribuție în amonte.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
