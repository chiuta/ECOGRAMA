# Contributing / Contribuții

**EN.** Thanks for helping. ECOGRAMA is an independently implemented project: do not copy code, assets or text from other tools; work from public specifications, textbooks and observed behaviour, and say which source you used (see `CLEANROOM.md`).

1. Pick a task from `docs/issues/` (or its GitHub issue) and say in a comment that you take it. Tasks labelled `good first issue` need no prior knowledge of the project.
2. Build and test: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Add a test for every behaviour you add or fix. Tests must not need network access or private files; build sample data in the test itself.
4. Parsers and anything that reads untrusted input must never panic: return an error instead.
5. Add one line to the provenance table in `CLEANROOM.md` for every new module (what public source it follows).
6. Note user-visible changes in `CHANGELOG.md`. Documentation is bilingual where the project already is (English + Romanian); other languages follow the README pipeline.
7. Commits: short imperative subject, signed off (`git commit -s`, Developer Certificate of Origin). Security issues: see `SECURITY.md` (privately, not in a public issue).

**RO.** Mulțumim. ECOGRAMA este o implementare independentă: nu copia cod, resurse sau texte din alte unelte; lucrează din specificații publice, manuale și comportament observat și spune ce sursă ai folosit (vezi `CLEANROOM.md`). Pași: alege o sarcină din `docs/issues/` și anunț-o într-un comentariu; rulează testele, `clippy` și `fmt` de mai sus; adaugă test pentru fiecare comportament; parserele nu au voie să intre în panică pe date nevalide; adaugă o linie în tabelul de proveniență; notează schimbările în `CHANGELOG.md`; commit-uri semnate (`git commit -s`); vulnerabilitățile se raportează privat (`SECURITY.md`).
