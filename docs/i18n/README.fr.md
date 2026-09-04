# 🎨 dom2figma

<div align="center">

![Rust](https://img.shields.io/badge/Rust-2021-000000?style=for-the-badge&logo=rust&logoColor=white)
![Chrome DevTools](https://img.shields.io/badge/Chrome-DevTools_Protocol-4285F4?style=for-the-badge&logo=googlechrome&logoColor=white)
![Figma](https://img.shields.io/badge/Figma-Plugin_API-F24E1E?style=for-the-badge&logo=figma&logoColor=white)
![Claude Code](https://img.shields.io/badge/Claude_Code-Plugin-D97757?style=for-the-badge&logo=anthropic&logoColor=white)
![Platforms](https://img.shields.io/badge/Linux_|_macOS-single_binary-333333?style=for-the-badge&logo=linux&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

[![CI](https://img.shields.io/github/actions/workflow/status/swrneko/dom2figma/ci.yml?style=for-the-badge&label=CI)](https://github.com/swrneko/dom2figma/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/swrneko/dom2figma?style=for-the-badge&label=Release)](https://github.com/swrneko/dom2figma/releases)

**Transforme n'importe quel prototype web rendu en calques Figma modifiables. Des dizaines d'écrans, tous les états de l'application, une seule commande.**  
*Headless Chrome capture le DOM, un plugin Figma le reconstruit en frames, textes et vecteurs.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 À propos

**dom2figma** est un outil en ligne de commande qui capture ce que le navigateur affiche réellement, et non le HTML source, puis le transfère dans Figma sous forme de vrais calques : frames avec remplissages, contours, rayons et ombres, calques de texte avec polices et alignement, icônes SVG en vecteurs.

Contrairement aux plugins comme html.to.design, il est conçu pour des **prototypes entiers** : il parcourt une liste de routes, bascule les états de l'application (personas, rôles, thèmes) via de petits hooks JavaScript, élimine les écrans identiques et produit un seul JSON importable en un clic. Le prototype change, on relance la commande.

Livré comme **un unique binaire statique pour Linux et macOS**. La seule dépendance externe est Chrome ou Chromium.

---

## 🏗️ Fonctionnement

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   lancer    ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ ouvrir source          │──────────►│ plugin ou Scripter   │
  │ root          │             │ pour chaque groupe :   │           │  • section par groupe│
  │ routes        │             │   exécuter setup JS    │           │  • frame par écran   │
  │ groups[]      │             │   pour chaque route :  │           │  • remplissages,     │
  │ fix_css       │             │     naviguer, attendre │           │    contours, rayons, │
  │ font          │             │     injecter serialize │           │    ombres            │
  └───────────────┘             │     DOM → arbre        │           │  • texte avec polices│
                                │     hash → dédoublonner│           │  • SVG → vecteurs    │
                                │     PNG de l'élément   │           └──────────────────────┘
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Capture.** `dom2figma capture` lance Chrome, ouvre la source, exécute le `setup` de chaque groupe, visite chaque route et sérialise le DOM visible : géométrie, styles calculés, segments de texte, SVG. Les écrans identiques sont écartés par hash de l'arbre.
2. **Import.** Le plugin Figma fourni (`dom2figma plugin`) lit le JSON et construit sections, frames et nœuds texte via la Plugin API. Pour Figma dans le navigateur, une variante Scripter existe (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Fonctionnalités

*   **Tout le prototype en une passe :** routes × états, avec dédoublonnage automatique des écrans identiques.
*   **États de l'application en groupes :** personas, rôles ou thèmes basculés par un hook JavaScript `setup` par groupe.
*   **Trois modes de navigation :** `hash` pour les routeurs `#/route`, `url` pour les vrais chemins, `js` pour les maquettes pilotées au clic sans routeur.
*   **Écrans en pleine hauteur :** les conteneurs à défilement interne sont déployés pour capturer l'écran entier, sans le tronquer au viewport.
*   **Texte fidèle :** la page est rendue avec la police que Figma utilisera, donc les retours à la ligne coïncident. Les textes d'une ligne sont en largeur automatique.
*   **Sortie modifiable :** calques nommés d'après les classes CSS, contours par côté, rayons d'angle, dégradés linéaires, ombres externes et internes, rognage, opacité, casse et décoration du texte.
*   **PNG de référence** à côté du JSON pour une vérification visuelle rapide.
*   **Plugin Claude Code** inclus : l'agent inspecte la maquette, écrit la configuration, capture, vérifie et remet les étapes d'import.

---

## 📦 01. Installation

Téléchargez un binaire depuis les [Releases](https://github.com/swrneko/dom2figma/releases) (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) et placez-le dans votre `PATH`, ou compilez depuis les sources :

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Prérequis :**
*   **Chrome ou Chromium.** Détecté automatiquement aux emplacements standard ; remplaçable via `--chrome`, la variable d'environnement `CHROME` ou `chrome = "..."` dans la configuration.
*   **La police de capture** (par défaut `Inter`) installée localement. Linux : déposez les TTF dans `~/.local/share/fonts/` puis `fc-cache -f`. macOS : installez via Livre des polices. Voir [Polices](#-04-polices).

---

## 🚀 02. Démarrage rapide

```bash
dom2figma init                 # écrit dom2figma.toml avec des commentaires
$EDITOR dom2figma.toml         # définir source, root, routes
dom2figma doctor               # Chrome et police présents ?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop :** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, lancez le plugin et choisissez `out/screens.json`. Les écrans sont disposés en une section par groupe sur la page courante.

**Figma dans le navigateur (ou Linux sans application de bureau) :**

```bash
dom2figma serve                              # sert out/*.json avec CORS sur http://localhost:8787
dom2figma scripter --out scripter.js         # script pour le plugin Scripter
```

Dans Figma : Plugins → Scripter → New script → coller `scripter.js` → Run.

Exécutions partielles : `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Configuration

```toml
source = "prototype.html"              # fichier HTML ou URL http(s)
root = ".device"                       # sélecteur de la racine d'écran, par défaut body
font = "Inter"                         # police de capture, doit exister localement et dans Figma
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # déployer le défilement interne sur toute la hauteur
min_height = 844
fix_css = ".panel{display:none!important}"   # CSS injecté avant la capture
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Champ | Défaut | Rôle |
|---|---|---|
| `source` | obligatoire | chemin du HTML ou URL `http(s)://` |
| `root` | `body` | sélecteur CSS de la racine capturée |
| `font`, `force_font` | `Inter`, `true` | police de rendu ; `force_font` l'applique à tous les éléments sous root |
| `viewport` | `1400×1000` | taille de la fenêtre du navigateur ; `390×844` pour le mobile |
| `navigation` | `hash` | `hash` définit `location.hash` ; `url` navigue vers `source + route` ; `js` évalue la route comme du JavaScript |
| `fix_css` | vide | patch CSS : masquer les panneaux, retirer les cadres d'appareil, laisser la page grandir en hauteur |
| `scroll_container`, `min_height` | aucun, `0` | déployer un conteneur à défilement interne pour capturer les écrans en entier |
| `settle_ms` | `400` | pause après la navigation |
| `routes`, `groups` | | routes et groupes d'états ; un groupe a ses propres `routes`, un `setup` JS et `setup_each_route` pour relancer setup après chaque navigation |
| `out`, `png`, `chrome` | `out`, `true`, auto | dossier de sortie, enregistrer les PNG, chemin de Chrome |

Un exemple entièrement commenté se trouve dans [`examples/dom2figma.toml`](../../examples/dom2figma.toml).

---

## 🔤 04. Polices

Chrome rend avec la police installée sur la machine tandis que Figma substitue la sienne. Si elles diffèrent, les retours à la ligne dérivent. dom2figma rend donc avec la police indiquée dans `font` et attend la même police dans Figma. **Inter et toutes les Google Fonts sont toujours disponibles dans Figma.** `dom2figma doctor` vérifie le côté local.

---

## 📐 05. Ce qui est transféré

**Transféré :** boîtes rectangulaires avec couleurs unies, dégradés linéaires, contours par côté, rayons, ombres et rognage ; texte avec taille, graisse, interlettrage, alignement, casse et décoration ; icônes SVG inline ; valeurs et placeholders des champs ; pseudo-éléments texte. L'invisible est ignoré : `display:none`, opacité nulle, couleur de texte transparente.

**Approximé ou ignoré :** images matricielles par URL externe (les data URL fonctionnent), dégradés radiaux, filtres, modes de fusion, canvas et vidéo, transformations (boîte englobante seulement). **Aucun auto layout** n'est généré : les calques sont positionnés en absolu. Le résultat est une copie structurelle exacte sur laquelle un designer peut travailler, pas un design system terminé.

---

## 🤖 06. Plugin Claude Code

Le dépôt fournit un plugin Claude Code avec le skill `dom2figma`. L'agent inspecte la maquette (racine, navigation, états), écrit la configuration, capture une route de test, vérifie les PNG, lance la capture complète et renvoie les instructions d'import.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Demandez ensuite *« passe ce prototype dans Figma »* ou appelez `/dom2figma:dom2figma`. Si le binaire manque, le skill l'installe avec `cargo install --git`.

---

## 🛠️ 07. Développement

```bash
cargo test                      # tests unitaires
cargo test -- --ignored         # bout en bout avec un vrai Chrome sur tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # exécute le plugin Figma contre une API factice, sans Figma
```

```text
src/capture.rs            pilote Chrome via le DevTools Protocol
assets/serialize.js       injecté dans la page, construit l'arbre de calques
assets/figma-plugin/      plugin Figma ; le bloc LIB START/END est partagé avec la variante Scripter
claude-plugin/            plugin et skill Claude Code
.github/workflows/        ci.yml à chaque push, release.yml sur les tags v*
```

---

## 📄 Licence

MIT. Voir [LICENSE](../../LICENSE).
