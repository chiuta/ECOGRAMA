# ECOGRAMA — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **STFT mit konfigurierbarem Fenster und Hop; gegen `dft_magnitude` testen** — `good first issue`, `area:dsp`
   `rustfft` hinter einem Feature verwenden; die naive DFT bleibt das Orakel. Maximale Abweichung pro Bin < 1e-3.

2. **Farbkarten als Daten: Viridis, Magma, Graustufen** — `good first issue`, `data`, `area:gui`
   Tabellen mit 256 Einträgen in `data/*.csv`, ein Typ `ColorMap` und ein Test, dass jede Tabelle in der Helligkeit monoton ist.

3. **WAV lesen und schreiben (16/24-Bit-PCM und 32-Bit-Float)** — `good first issue`, `area:io`
   Selbst implementieren oder ein permissiv lizenziertes Crate wählen; Rundlauf-Tests; fehlerhafte Header ohne Panik ablehnen.

4. **Spektrogrammansicht in egui: Rendern, Zoomen und Verschieben** — `help wanted`, `area:gui`
   Option für logarithmische Frequenz; 10-Minuten-Dateien durch Kachelung bewältigen. Screenshot im PR.

5. **Auswahlen: Rechteck und Lasso erzeugen eine Zeit-Frequenz-Maske** — `help wanted`, `area:gui`
   Die Maske ist eine dünn besetzte Struktur mit weichen Kanten; Vereinigung/Differenz von Masken per Unit-Test prüfen.

6. **Die Auswahl mit Overlap-Add-Resynthese abschwächen** — `help wanted`, `area:dsp`
   Rundlauftest: keine Auswahl = bitgenau innerhalb von 1e-6; die Auswahl entfernt einen synthetischen Ton um ≥ 40 dB.

7. **Spektrale Reparatur: eine Auswahl durch Interpolation der umgebenden Frames füllen** — `help wanted`, `area:dsp`
   Zuerst Klicks und kurze Aussetzer. Mit Sinus + Klick testen; SNR vorher/nachher angeben.

8. **De-Hum: 50/60 Hz und Obertöne erkennen, schmale Notch-Filter anwenden** — `good first issue`, `area:dsp`
   Test mit synthetischem Brummen + Rauschen; das Sprachband unberührt lassen (messen).

9. **CLAP-Plug-in-Wrapper (VST3 später)** — `help wanted`, `area:plugin`
   Zuerst die Lizenzen des Plug-in-Frameworks und der VST3-Bindings prüfen; das Ergebnis in `docs/` festhalten.

10. **Mit `cathar` vergleichen und entscheiden: abhängen, interoperieren oder upstream beitragen** — `help wanted`, `docs`
   De-Click und spektrale Reparatur an denselben synthetischen Signalen messen; `docs/ADR-001.md` schreiben.
