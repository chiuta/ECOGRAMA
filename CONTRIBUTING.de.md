# Mitmachen — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · **Deutsch** · [Magyar](CONTRIBUTING.hu.md)

Danke für deine Hilfe. ECOGRAMA ist ein unabhängig implementiertes Projekt: Kopiere keinen Code, keine Ressourcen und keine Texte aus anderen Werkzeugen; arbeite nach öffentlichen Spezifikationen, Lehrbüchern und beobachtetem Verhalten und gib an, welche Quelle du verwendet hast (siehe `CLEANROOM.md`).

1. Wähle eine Aufgabe aus `docs/issues/` (oder dem zugehörigen GitHub-Issue) und schreibe in einem Kommentar, dass du sie übernimmst. Aufgaben mit dem Label `good first issue` setzen keine Vorkenntnisse des Projekts voraus.
2. Bauen und testen: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Füge für jedes Verhalten, das du hinzufügst oder behebst, einen Test hinzu. Tests dürfen weder Netzwerkzugriff noch private Dateien benötigen; erzeuge Beispieldaten im Test selbst.
4. Parser und alles, was nicht vertrauenswürdige Eingaben liest, dürfen nie in Panik geraten: Gib stattdessen einen Fehler zurück.
5. Ergänze für jedes neue Modul eine Zeile in der Herkunftstabelle in `CLEANROOM.md` (welcher öffentlichen Quelle es folgt).
6. Halte für Nutzer sichtbare Änderungen in `CHANGELOG.md` fest. Die Dokumentation folgt der README-Pipeline in acht Sprachen (Englisch, Rumänisch, Französisch, Italienisch, Spanisch, Portugiesisch, Deutsch, Ungarisch); eine Korrektur einer Übersetzung ist willkommen.
7. Commits: kurzer Betreff im Imperativ, signiert (`git commit -s`, Developer Certificate of Origin). Sicherheitsprobleme: siehe `SECURITY.md` (privat, nicht in einem öffentlichen Issue).
