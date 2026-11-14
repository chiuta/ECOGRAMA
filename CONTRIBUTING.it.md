# Contribuire — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · **Italiano** · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Grazie per l’aiuto. ECOGRAMA è un progetto implementato in modo indipendente: non copiare codice, risorse o testi da altri strumenti; lavora a partire da specifiche pubbliche, manuali e comportamento osservato, e indica quale fonte hai usato (vedi `CLEANROOM.md`).

1. Scegli un compito da `docs/issues/` (o dalla sua issue su GitHub) e scrivi in un commento che lo prendi in carico. I compiti etichettati `good first issue` non richiedono conoscenze pregresse del progetto.
2. Compila e testa: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Aggiungi un test per ogni comportamento che aggiungi o correggi. I test non devono richiedere accesso alla rete né file privati; costruisci i dati di esempio nel test stesso.
4. I parser e tutto ciò che legge dati non attendibili non devono mai andare in panic: restituisci un errore.
5. Aggiungi una riga alla tabella di provenienza in `CLEANROOM.md` per ogni nuovo modulo (quale fonte pubblica segue).
6. Annota le modifiche visibili all’utente in `CHANGELOG.md`. La documentazione segue il flusso README in otto lingue (inglese, romeno, francese, italiano, spagnolo, portoghese, tedesco, ungherese); una correzione di traduzione è un contributo gradito.
7. Commit: oggetto breve all’imperativo, firmati (`git commit -s`, Developer Certificate of Origin). Problemi di sicurezza: vedi `SECURITY.md` (in privato, non in una issue pubblica).
