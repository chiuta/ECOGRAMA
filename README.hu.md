# ECOGRAMA — Spektrogramszerkesztő és hangrestaurálás: kijelölés a spektrogramon, javítás, exportálás.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-14

🌐 https://ecograma.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- STFT beállítható ablakkal és ugrással, színtérképek (viridis, magma, szürkeárnyalat) és WAV olvasás/írás (16/24 bites PCM, 32 bites lebegőpontos).
- Spektrogram-nézet egui-ban nagyítással és görgetéssel.
- Téglalap- és lasszós kijelölés, amely idő–frekvencia maszkot ad, overlap-add újraszintézissel csillapítva.
- Spektrális javítás a szomszédos keretek interpolálásával, és zúgáseltávolítás, amely felismeri az 50/60 Hz-et és a felharmonikusokat.
- CLAP plug-in burkoló (VST3 később) és döntés a `cathar` kapcsolatáról: függés, együttműködés vagy hozzájárulás a forráshoz.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
