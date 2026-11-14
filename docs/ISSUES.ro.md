# ECOGRAMA — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **STFT cu fereastră și pas configurabile; testează față de `dft_magnitude`** — `good first issue`, `area:dsp`
   Folosește `rustfft` în spatele unei funcționalități; DFT-ul naiv rămâne oracolul. Diferența maximă pe bin < 1e-3.

2. **Hărți de culoare ca date: viridis, magma, tonuri de gri** — `good first issue`, `data`, `area:gui`
   Tabele de 256 de intrări în `data/*.csv`, un tip `ColorMap` și un test că fiecare tabel este monoton în luminanță.

3. **Citire și scriere WAV (PCM 16/24 biți și float 32 biți)** — `good first issue`, `area:io`
   Implementează sau alege un crate cu licență permisivă; teste dus-întors; respinge antetele malformate fără panică.

4. **Vizualizare spectrogramă în egui: randare, zoom și deplasare** — `help wanted`, `area:gui`
   Opțiune de frecvență logaritmică; gestionează fișiere de 10 minute prin dale. Captură de ecran în PR.

5. **Selecții: dreptunghi și lasou produc o mască timp–frecvență** — `help wanted`, `area:gui`
   Masca este o structură rară cu margini moi; testează unitar reuniunea/diferența măștilor.

6. **Atenuează selecția cu resinteză overlap-add** — `help wanted`, `area:dsp`
   Test dus-întors: fără selecție = identic bit cu bit în limita 1e-6; selecția elimină un ton sintetic cu ≥ 40 dB.

7. **Reparare spectrală: umple o selecție interpolând cadrele din jur** — `help wanted`, `area:dsp`
   Întâi clicuri și goluri scurte. Testează cu o sinusoidă + click; raportează SNR înainte/după.

8. **De-hum: detectează 50/60 Hz și armonicele, aplică filtre notch înguste** — `good first issue`, `area:dsp`
   Test cu zumzet sintetic + zgomot; lasă banda vorbirii neatinsă (măsoară-o).

9. **Înveliș de plug-in CLAP (VST3 mai târziu)** — `help wanted`, `area:plugin`
   Verifică întâi licențele framework-ului de plug-in și ale legăturilor VST3; scrie constatarea în `docs/`.

10. **Compară cu `cathar` și decide: depinde, interoperează sau contribuie în amonte** — `help wanted`, `docs`
   Măsoară de-click și repararea spectrală pe aceleași semnale sintetice; scrie `docs/ADR-001.md`.
