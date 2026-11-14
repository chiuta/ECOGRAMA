# Herkunft von ECOGRAMA und Verfahren der unabhängigen Implementierung

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · [Français](CLEANROOM.fr.md) · [Italiano](CLEANROOM.it.md) · [Español](CLEANROOM.es.md) · [Português](CLEANROOM.pt.md) · **Deutsch** · [Magyar](CLEANROOM.hu.md)

ECOGRAMA ist **unabhängig aus öffentlichen Spezifikationen, Normen und Lehrbüchern implementiert. Es wurde kein Quellcode Dritter kopiert.** Diese Datei hält fest, was getan wurde; sie ist eine Ingenieursdisziplin, die die Herkunft nachprüfbar hält, keine Rechtsberatung und keine strenge Clean-Room-Garantie (es gibt kein getrenntes Spezifikationsteam).

## Regeln
1. **Erlaubte Quellen:** öffentliche Normen und Spezifikationen, Lehrbücher und begutachtete Fachartikel, öffentliche Dokumentation und das *für Nutzer sichtbare* Verhalten anderer Werkzeuge.
2. **Verbotene Quellen:** dekompilierter oder geleakter Code, durch EULA beschränkte SDKs, Reverse Engineering proprietärer Formate, generierte Tabellen, Namen, Oberflächentexte oder Beispielinhalte anderer Werkzeuge.
3. **Herkunft pro Modul:** Nenne die öffentliche Quelle im Doku-Kommentar des Moduls und ergänze eine Zeile in der Tabelle unten.
4. **Abhängigkeiten:** nur permissive Lizenzen (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` setzt die Liste in `deny.toml` durch.
5. **Eigene Namen und Schemata** für alles ohne öffentlichen Standard.
6. **Verwandte Projekte** stehen in `docs/LANDSCAPE.md`; ihre *Dokumentation* zur Abgrenzung zu lesen ist in Ordnung, ihren Code zu kopieren nicht. Prüfe die Lizenz eines Projekts, bevor du überhaupt dessen Quellcode liest, falls du es nachbauen könntest.

## Hinweis zur KI-Unterstützung
Das erste Gerüst und sein Startmodul wurden mit einem KI-Modell (Claude, Anthropic) geschrieben. Dessen Trainingsdaten unterliegen nicht der Kontrolle des Autors; der Code wurde nach den unten genannten öffentlichen Quellen geschrieben. Wer eine stärkere Gewähr braucht, sollte dies als unabhängige Implementierung mit einem dokumentierten Restrisiko betrachten.

## Herkunftsprotokoll
| Modul | Grundlage (öffentlich) | Hinweise |
|---|---|---|
| hann, dft_magnitude | Lehrbuch-DSP (Hann-Fenster, Definition der DFT) |  |
