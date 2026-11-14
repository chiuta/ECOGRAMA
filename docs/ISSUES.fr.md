# ECOGRAMA — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **STFT avec fenêtre et pas configurables ; tester contre `dft_magnitude`** — `good first issue`, `area:dsp`
   Utiliser `rustfft` derrière une fonctionnalité ; la DFT naïve reste l’oracle. Différence maximale par bin < 1e-3.

2. **Palettes de couleurs comme données : viridis, magma, niveaux de gris** — `good first issue`, `data`, `area:gui`
   Tables de 256 entrées dans `data/*.csv`, un type `ColorMap`, et un test vérifiant que chaque table est monotone en luminance.

3. **Lecture et écriture WAV (PCM 16/24 bits et flottant 32 bits)** — `good first issue`, `area:io`
   Implémenter ou choisir un crate sous licence permissive ; tests aller-retour ; rejeter les en-têtes mal formés sans paniquer.

4. **Vue spectrogramme dans egui : rendu, zoom et panoramique** — `help wanted`, `area:gui`
   Option de fréquence logarithmique ; gérer les fichiers de 10 minutes par tuilage. Capture d’écran dans la PR.

5. **Sélections : rectangle et lasso produisent un masque temps–fréquence** — `help wanted`, `area:gui`
   Le masque est une structure creuse à bords doux ; tester unitairement l’union et la différence de masques.

6. **Atténuer la sélection par resynthèse à recouvrement-addition** — `help wanted`, `area:dsp`
   Test aller-retour : sans sélection = identique au bit près à 1e-6 ; la sélection supprime un son synthétique d’au moins 40 dB.

7. **Réparation spectrale : remplir une sélection en interpolant les trames voisines** — `help wanted`, `area:dsp`
   Clics et courtes coupures d’abord. Tester avec une sinusoïde + un clic ; rapporter le SNR avant/après.

8. **De-hum : détecter 50/60 Hz et les harmoniques, appliquer des filtres coupe-bande étroits** — `good first issue`, `area:dsp`
   Test avec un ronflement synthétique + bruit ; laisser la bande de la parole intacte (la mesurer).

9. **Enveloppe de plug-in CLAP (VST3 plus tard)** — `help wanted`, `area:plugin`
   Vérifier d’abord les licences du framework de plug-in et des liaisons VST3 ; consigner le résultat dans `docs/`.

10. **Comparer avec `cathar` et décider : en dépendre, interopérer ou contribuer en amont** — `help wanted`, `docs`
   Mesurer le de-click et la réparation spectrale sur les mêmes signaux synthétiques ; écrire `docs/ADR-001.md`.
