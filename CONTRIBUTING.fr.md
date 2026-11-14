# Contribuer — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · **Français** · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Merci de votre aide. ECOGRAMA est un projet implémenté de façon indépendante : ne copiez ni code, ni ressources, ni textes d’autres outils ; travaillez à partir de spécifications publiques, de manuels et du comportement observé, et indiquez la source utilisée (voir `CLEANROOM.md`).

1. Choisissez une tâche dans `docs/issues/` (ou son ticket GitHub) et dites dans un commentaire que vous la prenez. Les tâches étiquetées `good first issue` ne demandent aucune connaissance préalable du projet.
2. Compilez et testez : `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Ajoutez un test pour chaque comportement ajouté ou corrigé. Les tests ne doivent pas nécessiter d’accès réseau ni de fichiers privés ; construisez les données d’exemple dans le test lui-même.
4. Les analyseurs et tout ce qui lit des données non fiables ne doivent jamais paniquer : renvoyez une erreur.
5. Ajoutez une ligne au tableau de provenance de `CLEANROOM.md` pour chaque nouveau module (quelle source publique il suit).
6. Notez les changements visibles par l’utilisateur dans `CHANGELOG.md`. La documentation suit le flux README en huit langues (anglais, roumain, français, italien, espagnol, portugais, allemand, hongrois) ; une correction de traduction est la bienvenue.
7. Commits : sujet court à l’impératif, signés (`git commit -s`, Developer Certificate of Origin). Problèmes de sécurité : voir `SECURITY.md` (en privé, pas dans un ticket public).
