# ECOGRAMA — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **STFT con finestra e passo configurabili; testa contro `dft_magnitude`** — `good first issue`, `area:dsp`
   Usa `rustfft` dietro una funzionalità; la DFT naïf resta l’oracolo. Differenza massima per bin < 1e-3.

2. **Mappe di colore come dati: viridis, magma, scala di grigi** — `good first issue`, `data`, `area:gui`
   Tabelle da 256 voci in `data/*.csv`, un tipo `ColorMap` e un test che ogni tabella sia monotona in luminanza.

3. **Lettura e scrittura WAV (PCM a 16/24 bit e float a 32 bit)** — `good first issue`, `area:io`
   Implementa o scegli un crate con licenza permissiva; test di andata e ritorno; rifiuta intestazioni malformate senza andare in panic.

4. **Vista spettrogramma in egui: rendering, zoom e panoramica** — `help wanted`, `area:gui`
   Opzione di frequenza logaritmica; gestisci file da 10 minuti con il tiling. Schermata nella PR.

5. **Selezioni: rettangolo e lazo producono una maschera tempo–frequenza** — `help wanted`, `area:gui`
   La maschera è una struttura sparsa con bordi morbidi; testa unione e differenza di maschere.

6. **Attenua la selezione con risintesi overlap-add** — `help wanted`, `area:dsp`
   Test di andata e ritorno: nessuna selezione = identico al bit entro 1e-6; la selezione rimuove un tono sintetico di almeno 40 dB.

7. **Riparazione spettrale: riempi una selezione interpolando i frame circostanti** — `help wanted`, `area:dsp`
   Prima clic e brevi cadute. Testa con una sinusoide + un clic; riporta l’SNR prima/dopo.

8. **De-hum: rileva 50/60 Hz e le armoniche, applica notch stretti** — `good first issue`, `area:dsp`
   Test con ronzio sintetico + rumore; lascia intatta la banda del parlato (misurala).

9. **Wrapper di plug-in CLAP (VST3 più avanti)** — `help wanted`, `area:plugin`
   Controlla prima le licenze del framework di plug-in e dei binding VST3; scrivi l’esito in `docs/`.

10. **Confronta con `cathar` e decidi: dipendere, interoperare o contribuire a monte** — `help wanted`, `docs`
   Misura de-click e riparazione spettrale sugli stessi segnali sintetici; scrivi `docs/ADR-001.md`.
