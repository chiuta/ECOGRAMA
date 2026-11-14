# Cum contribui — ECOGRAMA

[English](CONTRIBUTING.md) · **Română** · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Mulțumim pentru ajutor. ECOGRAMA este un proiect implementat independent: nu copia cod, resurse sau texte din alte unelte; lucrează din specificații publice, manuale și comportament observat și spune ce sursă ai folosit (vezi `CLEANROOM.md`).

1. Alege o sarcină din `docs/issues/` (sau din issue-ul ei de pe GitHub) și anunț într-un comentariu că o iei. Sarcinile marcate `good first issue` nu cer cunoștințe prealabile despre proiect.
2. Compilează și testează: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Adaugă un test pentru fiecare comportament pe care îl adaugi sau îl repari. Testele nu au voie să depindă de rețea sau de fișiere private; construiește datele de probă chiar în test.
4. Parserele și orice citește date nesigure nu au voie să intre în panică: întoarce o eroare.
5. Adaugă o linie în tabelul de proveniență din `CLEANROOM.md` pentru fiecare modul nou (ce sursă publică urmează).
6. Notează schimbările vizibile utilizatorului în `CHANGELOG.md`. Documentația urmează fluxul README în opt limbi (engleză, română, franceză, italiană, spaniolă, portugheză, germană, maghiară); o corectură de traducere este o contribuție binevenită.
7. Commit-uri: subiect scurt la imperativ, semnate (`git commit -s`, Developer Certificate of Origin). Probleme de securitate: vezi `SECURITY.md` (în privat, nu într-un issue public).
