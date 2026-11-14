# ECOGRAMA — Editor de spectrogramă și restaurare audio: selectezi pe spectrogramă, repari, exporți.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 13.11.2026

🌐 https://ecograma.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

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
