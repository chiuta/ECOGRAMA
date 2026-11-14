# Contributing — ECOGRAMA

**English** · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Thanks for helping. ECOGRAMA is an independently implemented project: do not copy code, assets or text from other tools; work from public specifications, textbooks and observed behaviour, and say which source you used (see `CLEANROOM.md`).

1. Pick a task from `docs/issues/` (or its GitHub issue) and say in a comment that you take it. Tasks labelled `good first issue` need no prior knowledge of the project.
2. Build and test: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Add a test for every behaviour you add or fix. Tests must not need network access or private files; build sample data in the test itself.
4. Parsers and anything that reads untrusted input must never panic: return an error instead.
5. Add one line to the provenance table in `CLEANROOM.md` for every new module (what public source it follows).
6. Note user-visible changes in `CHANGELOG.md`. Documentation follows the README pipeline in eight languages (English, Romanian, French, Italian, Spanish, Portuguese, German, Hungarian); a translation fix is a welcome contribution.
7. Commits: short imperative subject, signed off (`git commit -s`, Developer Certificate of Origin). Security issues: see `SECURITY.md` (privately, not in a public issue).
