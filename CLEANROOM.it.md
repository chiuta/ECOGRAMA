# Provenienza di ECOGRAMA e procedura di implementazione indipendente

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · [Français](CLEANROOM.fr.md) · **Italiano** · [Español](CLEANROOM.es.md) · [Português](CLEANROOM.pt.md) · [Deutsch](CLEANROOM.de.md) · [Magyar](CLEANROOM.hu.md)

ECOGRAMA è **implementato in modo indipendente a partire da specifiche pubbliche, standard e manuali. Non è stato copiato alcun codice sorgente di terzi.** Questo file dice che cosa è stato fatto; è una disciplina ingegneristica che mantiene la provenienza verificabile, non una consulenza legale e non una garanzia rigorosa di tipo clean-room (non esiste un team di specifica separato).

## Regole
1. **Fonti ammesse:** standard e specifiche pubblici, manuali e articoli sottoposti a revisione paritaria, documentazione pubblica e comportamento *visibile all’utente* di altri strumenti.
2. **Fonti vietate:** codice decompilato o trapelato, SDK vincolati da EULA, reverse engineering di formati proprietari, tabelle generate, nomi, testi dell’interfaccia o contenuti di esempio di altri strumenti.
3. **Provenienza per modulo:** indica la fonte pubblica nel commento di documentazione del modulo e aggiungi una riga alla tabella qui sotto.
4. **Dipendenze:** solo licenze permissive (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` applica l’elenco in `deny.toml`.
5. **Nomi e schemi propri** per tutto ciò che non ha uno standard pubblico.
6. **Progetti affini:** elencati in `docs/LANDSCAPE.md`; leggere la loro *documentazione* per capire l’ambito va bene, copiare il loro codice no. Controlla la licenza di un progetto prima ancora di leggerne il sorgente, se potresti reimplementarlo.

## Dichiarazione sull’assistenza dell’IA
Lo scheletro iniziale e il suo modulo di partenza sono stati scritti con un modello di IA (Claude, Anthropic). I suoi dati di addestramento non sono sotto il controllo dell’autore; il codice è stato scritto a partire dalle fonti pubbliche indicate sotto. Un revisore che vuole una garanzia più forte dovrebbe considerarlo un’implementazione indipendente con un rischio residuo documentato.

## Registro di provenienza
| Modulo | Base (pubblica) | Note |
|---|---|---|
| hann, dft_magnitude | DSP da manuale (finestra di Hann, definizione della DFT) |  |
