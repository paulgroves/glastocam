# Webcam Glastonbury en Plein Écran

https://glastocam.foobarlol.lol

## À propos

Cette application web offre une expérience de visionnage améliorée pour la webcam du Festival de Glastonbury. Il s'agit d'une interface dédiée qui extrait la fonctionnalité webcam de la page principale de Glastonbury pour fournir une vue plein écran claire et sans distraction.

## Objectif

Créer une expérience de visionnage dédiée en plein écran pour permettre aux amateurs du festival de surveiller les terrains du festival à distance de manière optimale.

## Fonctionnalités principales

- **Flux webcam en direct** : Affichage en temps réel de la webcam du Festival de Glastonbury
- **Actualisation automatique** : L'image se rafraîchit automatiquement toutes les 5 minutes (300 secondes)
- **Mode plein écran** : Interface optimisée pour une expérience de visionnage immersive
- **Interface épurée** : Design minimaliste sans distractions pour une expérience de visionnage optimale
- **Positionnement intelligent** : L'application positionne automatiquement la vue sur la partie la plus intéressante de l'image
- **Cache-busting** : Utilisation de timestamps pour garantir des images fraîches à chaque actualisation

## Stack technique

- **HTML5, CSS3 et JavaScript pur** : Aucune librairie ou framework externe
- **Application web statique** : Fonctionne entièrement côté client
- **Aucun backend requis** : Pas de base de données ou de composants serveur
- **Intégration API externe** : Utilise l'API de webcam Panomax (panodata.panomax.com)

## Accès et utilisation

L'application est hébergée en tant que site statique et accessible directement à l'adresse :
**https://glastocam.foobarlol.lol**

Aucune installation ou configuration n'est nécessaire - il suffit d'ouvrir l'URL dans votre navigateur web.

## Détails techniques

### Intégration webcam
- **Source** : API Panomax (camera ID: 879)
- **URL de l'image** : `https://panodata.panomax.com/cams/879/recent_full.jpg`
- **Méthode de rafraîchissement** : Ajout de timestamps pour éviter la mise en cache
- **Intervalle d'actualisation** : 300 000 ms (5 minutes)

### Fonctionnalités JavaScript
- Rechargement automatique de l'image avec cache-busting
- Positionnement intelligent du viewport (scroll automatique vers la zone d'intérêt)
- Gestion des événements de chargement pour un affichage optimal

---

**Note** : Pas d'inquiétude, ce code est simplement copié et collé directement du cadre de la page officielle, aucun piratage ou quoi que ce soit de malveillant, juste une expérience de visionnage améliorée pour les connaisseurs.

Aimez la ferme, ne laissez aucune trace 💚
