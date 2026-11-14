# Az ECOGRAMA eredete és a független megvalósítás eljárása

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · [Français](CLEANROOM.fr.md) · [Italiano](CLEANROOM.it.md) · [Español](CLEANROOM.es.md) · [Português](CLEANROOM.pt.md) · [Deutsch](CLEANROOM.de.md) · **Magyar**

Az ECOGRAMA **függetlenül, nyilvános specifikációkból, szabványokból és tankönyvekből valósult meg. Harmadik fél forráskódját nem másoltuk.** Ez a fájl azt rögzíti, mi történt; olyan mérnöki fegyelem, amely az eredetet ellenőrizhetővé teszi, nem jogi tanács és nem szigorú clean-room garancia (nincs különálló specifikációs csapat).

## Szabályok
1. **Megengedett források:** nyilvános szabványok és specifikációk, tankönyvek és lektorált cikkek, nyilvános dokumentáció, és más eszközök *felhasználó által látható* viselkedése.
2. **Tiltott források:** dekompilált vagy kiszivárgott kód, EULA-val korlátozott SDK-k, szabadalmazott formátumok visszafejtése, más eszközök generált táblái, nevei, felületi szövegei vagy mintatartalma.
3. **Modulonkénti eredet:** nevezd meg a nyilvános forrást a modul dokumentációs megjegyzésében, és adj egy sort az alábbi táblázathoz.
4. **Függőségek:** csak megengedő licencek (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); a `cargo deny check` a `deny.toml` listáját érvényesíti.
5. **Saját nevek és sémák** mindenhez, aminek nincs nyilvános szabványa.
6. **Rokon projektek** a `docs/LANDSCAPE.md` fájlban szerepelnek; a *dokumentációjuk* elolvasása a terjedelem felméréséhez rendben van, a kódjuk másolása nem. Ellenőrizd egy projekt licencét még a forrás elolvasása előtt is, ha újra megvalósíthatnád.

## Tájékoztatás az MI-segítségről
A kezdeti vázat és kezdő moduljának kódját MI-modellel (Claude, Anthropic) írtuk. A betanítási adatai nem a szerző felügyelete alatt állnak; a kód az alább megnevezett nyilvános források alapján készült. Aki erősebb biztosítékot igényel, kezelje ezt dokumentált maradék kockázatú, független megvalósításként.

## Eredetnapló
| Modul | Alap (nyilvános) | Megjegyzések |
|---|---|---|
| hann, dft_magnitude | tankönyvi DSP (Hann-ablak, a DFT definíciója) |  |
