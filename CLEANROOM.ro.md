# Proveniența ECOGRAMA și procedura de implementare independentă

[English](CLEANROOM.md) · **Română** · [Français](CLEANROOM.fr.md) · [Italiano](CLEANROOM.it.md) · [Español](CLEANROOM.es.md) · [Português](CLEANROOM.pt.md) · [Deutsch](CLEANROOM.de.md) · [Magyar](CLEANROOM.hu.md)

ECOGRAMA este **implementat independent din specificații publice, standarde și manuale. Nu a fost copiat niciun cod sursă de la terți.** Acest fișier spune ce s-a făcut; este o disciplină de inginerie care păstrează proveniența verificabilă, nu un sfat juridic și nu o garanție strictă de tip clean-room (nu există o echipă separată pentru specificații).

## Reguli
1. **Surse permise:** standarde și specificații publice, manuale și articole evaluate de colegi, documentație publică și comportamentul *vizibil utilizatorului* al altor unelte.
2. **Surse interzise:** cod decompilat sau scurs, SDK-uri restricționate prin EULA, inginerie inversă a formatelor proprietare, tabele generate, nume, texte de interfață sau conținut de probă din alte unelte.
3. **Proveniență pe modul:** numește sursa publică în comentariul de documentație al modulului și adaugă o linie în tabelul de mai jos.
4. **Dependențe:** numai licențe permisive (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` impune lista din `deny.toml`.
5. **Nume și scheme proprii** pentru orice lucru fără standard public.
6. **Proiectele înrudite** sunt listate în `docs/LANDSCAPE.md`; citirea *documentației* lor pentru a înțelege domeniul este în regulă, copierea codului lor nu. Verifică licența unui proiect înainte să-i citești sursa, dacă l-ai putea reimplementa.

## Declarația privind asistența AI
Scheletul inițial și modulul lui de pornire au fost scrise cu un model AI (Claude, Anthropic). Datele lui de antrenare nu sunt sub controlul autorului; codul a fost scris pe baza surselor publice numite mai jos. Un evaluator care cere o asigurare mai puternică ar trebui să-l trateze ca pe o implementare independentă cu un risc rezidual documentat.

## Jurnal de proveniență
| Modul | Bază (publică) | Note |
|---|---|---|
| hann, dft_magnitude | DSP de manual (fereastra Hann, definiția DFT) |  |
