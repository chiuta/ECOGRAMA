# Provenance d’ECOGRAMA et procédure d’implémentation indépendante

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · **Français** · [Italiano](CLEANROOM.it.md) · [Español](CLEANROOM.es.md) · [Português](CLEANROOM.pt.md) · [Deutsch](CLEANROOM.de.md) · [Magyar](CLEANROOM.hu.md)

ECOGRAMA est **implémenté de façon indépendante à partir de spécifications publiques, de normes et de manuels. Aucun code source tiers n’a été copié.** Ce fichier dit ce qui a été fait ; c’est une discipline d’ingénierie qui garde la provenance vérifiable, pas un avis juridique et pas une garantie stricte de type clean-room (il n’existe pas d’équipe de spécification distincte).

## Règles
1. **Sources autorisées :** normes et spécifications publiques, manuels et articles évalués par des pairs, documentation publique et comportement *visible par l’utilisateur* d’autres outils.
2. **Sources interdites :** code décompilé ou divulgué, SDK soumis à EULA, rétro-ingénierie de formats propriétaires, tables générées, noms, textes d’interface ou contenus d’exemple d’autres outils.
3. **Provenance par module :** nommez la source publique dans le commentaire de documentation du module et ajoutez une ligne au tableau ci-dessous.
4. **Dépendances :** licences permissives uniquement (MIT, Apache-2.0, BSD, ISC, Zlib, CC0) ; `cargo deny check` applique la liste de `deny.toml`.
5. **Noms et schémas propres** pour tout ce qui n’a pas de norme publique.
6. **Projets apparentés** : listés dans `docs/LANDSCAPE.md` ; lire leur *documentation* pour cerner le périmètre est acceptable, copier leur code ne l’est pas. Vérifiez la licence d’un projet avant même de lire sa source si vous pourriez le réimplémenter.

## Déclaration d’assistance par IA
Le squelette initial et son module de départ ont été écrits avec un modèle d’IA (Claude, Anthropic). Ses données d’entraînement ne sont pas sous le contrôle de l’auteur ; le code a été écrit à partir des sources publiques citées ci-dessous. Un relecteur qui souhaite une assurance plus forte doit le considérer comme une implémentation indépendante avec un risque résiduel documenté.

## Journal de provenance
| Module | Base (publique) | Notes |
|---|---|---|
| hann, dft_magnitude | DSP de manuel (fenêtre de Hann, définition de la DFT) |  |
