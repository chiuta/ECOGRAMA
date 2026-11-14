# ECOGRAMA — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **STFT beállítható ablakkal és ugrással; tesztelés a `dft_magnitude` ellen** — `good first issue`, `area:dsp`
   Használd a `rustfft`-et feature mögött; a naiv DFT marad az orákulum. Maximális eltérés binenként < 1e-3.

2. **Színtérképek adatként: viridis, magma, szürkeárnyalat** — `good first issue`, `data`, `area:gui`
   256 elemű táblák a `data/*.csv`-ben, egy `ColorMap` típus, és egy teszt, hogy minden tábla monoton a luminanciában.

3. **WAV olvasása és írása (16/24 bites PCM és 32 bites float)** — `good first issue`, `area:io`
   Implementáld, vagy válassz megengedő licencű crate-et; oda-vissza tesztek; a hibás fejléceket paníkolás nélkül utasítsd el.

4. **Spektrogramnézet egui-ban: renderelés, nagyítás és görgetés** — `help wanted`, `area:gui`
   Logaritmikus frekvencia opció; a 10 perces fájlokat csempézéssel kezeld. Képernyőkép a PR-ban.

5. **Kijelölések: téglalap és lasszó időfrekvencia-maszkot állít elő** — `help wanted`, `area:gui`
   A maszk ritka struktúra lágy szélekkel; egységteszttel ellenőrizd a maszkok egyesítését/különbségét.

6. **A kijelölés csillapítása overlap-add reszintézissel** — `help wanted`, `area:dsp`
   Oda-vissza teszt: kijelölés nélkül = bitre azonos 1e-6-on belül; a kijelölés ≥ 40 dB-lel eltávolít egy szintetikus hangot.

7. **Spektrális javítás: a kijelölés kitöltése a környező keretek interpolálásával** — `help wanted`, `area:dsp`
   Először kattogások és rövid kimaradások. Teszteld szinusz + kattanás jellel; jelentsd az SNR-t előtte/utána.

8. **De-hum: a 50/60 Hz és a harmonikusok felismerése, keskeny notch szűrők alkalmazása** — `good first issue`, `area:dsp`
   Teszt szintetikus zümmögéssel + zajjal; a beszédsávot hagyd érintetlenül (mérd meg).

9. **CLAP bővítmény-burkoló (VST3 később)** — `help wanted`, `area:plugin`
   Előbb ellenőrizd a bővítmény-keretrendszer és a VST3 kötések licencét; az eredményt írd a `docs/`-ba.

10. **Hasonlítsd össze a `cathar`-ral, és döntsd el: függés, együttműködés vagy upstream hozzájárulás** — `help wanted`, `docs`
   Mérd a de-click és a spektrális javítás teljesítményét ugyanazokon a szintetikus jeleken; írd meg a `docs/ADR-001.md`-t.
