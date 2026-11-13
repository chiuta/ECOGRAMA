# ECOGRAMA — Editor di spettrogrammi e restauro audio: selezioni sullo spettrogramma, ripari, esporti.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0-alpha.1 · Ultimo aggiornamento: 13.11.2026

## Stato

Scheletro iniziale (pre-alfa): il workspace compila e i test passano in CI. Ogni funzione prevista è un'attività aperta in `docs/issues/`.

## Compilare e testare

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribuire

Leggi `CONTRIBUTING.md`, scegli un'attività con etichetta `good first issue` in `docs/issues/` e apri una pull request. Nessun contributo è troppo piccolo: una riga di tabella, un test o una traduzione contano.

## Implementazione indipendente

Scritta a partire da specifiche pubbliche e manuali; nessun codice sorgente di terzi è stato copiato (vedi `CLEANROOM.md`).

## Licenza

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sostegno

Trade-free: niente tracciamento, niente pubblicità. Se ti è utile, puoi sostenere l'autore: https://buymeacoffee.com/echo.of.the.strings
