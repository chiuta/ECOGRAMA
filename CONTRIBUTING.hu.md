# Közreműködés — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · **Magyar**

Köszönjük a segítséget. Az ECOGRAMA függetlenül megvalósított projekt: ne másolj kódot, erőforrást vagy szöveget más eszközökből; nyilvános specifikációkból, tankönyvekből és megfigyelt viselkedésből dolgozz, és add meg, melyik forrást használtad (lásd `CLEANROOM.md`).

1. Válassz egy feladatot a `docs/issues/` mappából (vagy a hozzá tartozó GitHub-issue-ból), és írd meg egy megjegyzésben, hogy vállalod. A `good first issue` címkéjű feladatokhoz nem kell előzetes projektismeret.
2. Fordítás és tesztelés: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Minden hozzáadott vagy javított viselkedéshez írj tesztet. A tesztek nem igényelhetnek hálózatot vagy privát fájlokat; a mintaadatokat magában a tesztben építsd fel.
4. Az értelmezők és mindaz, ami nem megbízható bemenetet olvas, soha nem pánikolhat: hibát adjon vissza.
5. Minden új modulhoz adj egy sort a `CLEANROOM.md` eredettáblájához (melyik nyilvános forrást követi).
6. A felhasználó számára látható változásokat jegyezd fel a `CHANGELOG.md` fájlban. A dokumentáció a README nyolcnyelvű folyamatát követi (angol, román, francia, olasz, spanyol, portugál, német, magyar); a fordításjavítás szívesen látott hozzájárulás.
7. Commitok: rövid felszólító tárgy, aláírva (`git commit -s`, Developer Certificate of Origin). Biztonsági problémák: lásd `SECURITY.md` (privátban, nem nyilvános issue-ban).
