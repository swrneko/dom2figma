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

**Turn any rendered web prototype into editable Figma layers. Dozens of screens, every app state, one command.**  
*Headless Chrome captures the DOM, a Figma plugin rebuilds it as frames, text and vectors.*

[English](README.md) · [Русский](docs/i18n/README.ru.md) · [简体中文](docs/i18n/README.zh-CN.md) · [日本語](docs/i18n/README.ja.md) · [한국어](docs/i18n/README.ko.md) · [Deutsch](docs/i18n/README.de.md) · [Español](docs/i18n/README.es.md) · [Français](docs/i18n/README.fr.md) · [Português](docs/i18n/README.pt-BR.md)

</div>

---

## 📖 About

**dom2figma** is a command-line tool that captures what the browser actually renders, not the HTML source, and moves it into Figma as real layers: frames with fills, strokes, radii and shadows, text layers with fonts and alignment, SVG icons as vectors.

Unlike plugins such as html.to.design, it is built for **whole prototypes**: it walks a list of routes, switches application states (personas, roles, themes) through small JavaScript hooks, de-duplicates identical screens and produces one JSON that imports in a single click. Change the prototype, run the command again.

Ships as a **single static binary for Linux and macOS**. The only external dependency is Chrome or Chromium.

---

## 🏗️ How it works

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   launch    ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ open source            │──────────►│ plugin or Scripter   │
  │ root          │             │ for each group:        │           │  • section per group │
  │ routes        │             │   run setup JS         │           │  • frame per screen  │
  │ groups[]      │             │   for each route:      │           │  • fills, strokes,   │
  │ fix_css       │             │     navigate & settle  │           │    radii, shadows    │
  │ font          │             │     inject serialize.js│           │  • text with fonts   │
  └───────────────┘             │     read DOM → tree    │           │  • SVG → vectors     │
                                │     hash → dedupe      │           └──────────────────────┘
                                │     element PNG        │
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Capture.** `dom2figma capture` launches Chrome, opens the source, runs each group's `setup`, visits every route and serializes the visible DOM: geometry, computed styles, text runs, SVG. Identical screens are dropped by tree hash.
2. **Import.** The bundled Figma plugin (`dom2figma plugin`) reads the JSON and builds sections, frames and text nodes through the Plugin API. For browser Figma there is a Scripter variant (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Features

*   **Whole prototype in one run:** routes × states, with automatic de-duplication of identical screens.
*   **App states as groups:** personas, roles or themes are switched by a JavaScript `setup` hook per group.
*   **Three navigation modes:** `hash` for `#/route` routers, `url` for real paths, `js` for click-driven mockups without a router.
*   **Full-height screens:** inner scroll containers are expanded so a screen is captured entirely, not clipped to the viewport.
*   **Faithful text:** the page is rendered with the same font Figma will use, so line breaks match. Single-line texts are auto-width.
*   **Editable output:** named layers from CSS classes, per-side strokes, corner radii, linear gradients, drop and inner shadows, clipping, opacity, text case and decoration.
*   **Reference PNGs** next to the JSON for quick visual checks.
*   **Claude Code plugin** included: the agent inspects the mockup, writes the config, captures, verifies and hands over import steps.

---

## 📦 01. Installation

Download a binary from [Releases](https://github.com/swrneko/dom2figma/releases) (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) and put it on your `PATH`, or build from source:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Requirements:**
*   **Chrome or Chromium.** Auto-detected in standard locations; override with `--chrome`, the `CHROME` env var or `chrome = "..."` in the config.
*   **The font you capture with** (default `Inter`) installed locally. Linux: drop TTFs into `~/.local/share/fonts/` and run `fc-cache -f`. macOS: install via Font Book. See [Fonts](#-04-fonts).

---

## 🚀 02. Quick start

```bash
dom2figma init                 # writes dom2figma.toml with comments
$EDITOR dom2figma.toml         # set source, root, routes
dom2figma doctor               # Chrome and font present?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, run the plugin and pick `out/screens.json`. Screens are laid out as one section per group on the current page.

**Browser Figma (or Linux without the desktop app):**

```bash
dom2figma serve                              # serves out/*.json with CORS on http://localhost:8787
dom2figma scripter --out scripter.js         # script for the Scripter plugin
```

In Figma: Plugins → Scripter → New script → paste `scripter.js` → Run.

Partial runs: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Configuration

```toml
source = "prototype.html"              # HTML file or http(s) URL
root = ".device"                       # screen root selector, default: body
font = "Inter"                         # capture font, must exist locally and in Figma
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # expand inner scroll to full height
min_height = 844
fix_css = ".panel{display:none!important}"   # CSS injected before capture
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Field | Default | Purpose |
|---|---|---|
| `source` | required | path to HTML or `http(s)://` URL |
| `root` | `body` | CSS selector of the captured root |
| `font`, `force_font` | `Inter`, `true` | rendering font; `force_font` applies it to every element under root |
| `viewport` | `1400×1000` | browser window size; use `390×844` for mobile layouts |
| `navigation` | `hash` | `hash` sets `location.hash`; `url` navigates to `source + route`; `js` evaluates the route as JavaScript |
| `fix_css` | empty | CSS patch: hide tool panels, remove device frames, let the page grow in height |
| `scroll_container`, `min_height` | none, `0` | expand an inner scroll container so screens are captured in full |
| `settle_ms` | `400` | pause after navigation |
| `routes`, `groups` | | routes and state groups; a group has its own `routes`, a JS `setup` and `setup_each_route` to re-run setup after each navigation |
| `out`, `png`, `chrome` | `out`, `true`, auto | output directory, save PNGs, Chrome path |

A fully commented example lives in [`examples/dom2figma.toml`](examples/dom2figma.toml).

---

## 🔤 04. Fonts

Chrome renders with whatever font is installed on the machine while Figma substitutes its own. If they differ, line breaks drift. dom2figma therefore renders with the font named in `font` and expects the same font in Figma. **Inter and every Google Font are always available in Figma.** `dom2figma doctor` checks the local side.

---

## 📐 05. What transfers

**Transfers:** rectangular boxes with solid colors, linear gradients, per-side strokes, radii, shadows and clipping; text with size, weight, letter spacing, alignment, case and decoration; inline SVG icons; input values and placeholders; text pseudo-elements. Invisible content is skipped: `display:none`, zero opacity, transparent text color.

**Approximated or skipped:** raster images by external URL (data URLs work), radial gradients, filters, blend modes, canvas and video, transforms (bounding box only). **No auto layout** is generated: layers are absolutely positioned. The result is an exact structural copy for a designer to build on, not a finished design system.

---

## 🤖 06. Claude Code plugin

The repository ships a Claude Code plugin with the `dom2figma` skill. The agent inspects the mockup (root, navigation, states), writes the config, captures a test route, checks the PNGs, runs the full capture and returns import instructions.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Then ask *"move this prototype into Figma"* or call `/dom2figma:dom2figma`. If the binary is missing, the skill installs it with `cargo install --git`.

---

## 🛠️ 07. Development

```bash
cargo test                      # unit tests
cargo test -- --ignored         # end-to-end with a real Chrome on tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # run the Figma plugin against a stub API, no Figma needed
```

```text
src/capture.rs            drives Chrome over the DevTools Protocol
assets/serialize.js       injected into the page, builds the layer tree
assets/figma-plugin/      Figma plugin; the LIB START/END block is shared with the Scripter build
claude-plugin/            Claude Code plugin and skill
.github/workflows/        ci.yml on every push, release.yml on v* tags
```

---

## 📄 License

MIT. See [LICENSE](LICENSE).
