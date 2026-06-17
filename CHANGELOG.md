# Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.


## 0.2.0

### Ajouté

- Thème clair en plus du thème sombre, avec bouton de bascule dans le header.
  Le choix est persisté dans `localStorage` et restauré au démarrage.
- Raccourci `Ctrl/Cmd + L` pour focus + sélection complète du champ de saisie
  actif (input ligne ou textarea colonne/conversion), quel que soit l'onglet ouvert.

### Modifié

- Refonte de la feuille de style en variables CSS (`--bg`, `--text`,
  `--accent`, etc.) pour centraliser les couleurs et permettre la bascule de
  thème sans dupliquer les règles.
- Contraste des textes secondaires (libellés, footer, historique vide,
  raccourcis clavier)

## 0.1.0 

Version initiale 
