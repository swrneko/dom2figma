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

**Verwandelt jeden gerenderten Web-Prototyp in editierbare Figma-Ebenen. Dutzende Screens, jeder App-Zustand, ein Befehl.**  
*Headless Chrome erfasst das DOM, ein Figma-Plugin baut es als Frames, Text und Vektoren nach.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 Über das Projekt

**dom2figma** ist ein Kommandozeilenwerkzeug, das erfasst, was der Browser tatsächlich rendert, nicht den HTML-Quelltext, und es als echte Ebenen nach Figma bringt: Frames mit Füllungen, Konturen, Radien und Schatten, Textebenen mit Schriften und Ausrichtung, SVG-Icons als Vektoren.

Anders als Plugins wie html.to.design ist es für **ganze Prototypen** gebaut: Es geht eine Routenliste durch, schaltet App-Zustände (Personas, Rollen, Themes) über kleine JavaScript-Hooks um, entfernt identische Screens und erzeugt eine JSON-Datei, die sich mit einem Klick importieren lässt. Prototyp geändert, Befehl erneut ausführen.

Ausgeliefert als **einzelne statische Binary für Linux und macOS**. Die einzige externe Abhängigkeit ist Chrome oder Chromium.

---

## 🏗️ Funktionsweise

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   Start     ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ source öffnen          │──────────►│ Plugin oder Scripter │
  │ root          │             │ für jede Gruppe:       │           │  • Section je Gruppe │
  │ routes        │             │   setup-JS ausführen   │           │  • Frame je Screen   │
  │ groups[]      │             │   für jede Route:      │           │  • Füllungen,        │
  │ fix_css       │             │     navigieren, warten │           │    Konturen, Radien, │
  │ font          │             │     serialize.js       │           │    Schatten          │
  └───────────────┘             │     DOM → Baum         │           │  • Text mit Schriften│
                                │     Hash → Dedupe      │           │  • SVG → Vektoren    │
                                │     Element-PNG        │           └──────────────────────┘
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Erfassen.** `dom2figma capture` startet Chrome, öffnet die Quelle, führt das `setup` jeder Gruppe aus, besucht jede Route und serialisiert das sichtbare DOM: Geometrie, berechnete Styles, Textläufe, SVG. Identische Screens werden per Baum-Hash verworfen.
2. **Importieren.** Das mitgelieferte Figma-Plugin (`dom2figma plugin`) liest die JSON und baut Sections, Frames und Textknoten über die Plugin API. Für Figma im Browser gibt es eine Scripter-Variante (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Funktionen

*   **Ganzer Prototyp in einem Lauf:** Routen × Zustände mit automatischer Deduplizierung identischer Screens.
*   **App-Zustände als Gruppen:** Personas, Rollen oder Themes werden per JavaScript-`setup`-Hook je Gruppe umgeschaltet.
*   **Drei Navigationsmodi:** `hash` für `#/route`-Router, `url` für echte Pfade, `js` für klickgesteuerte Mockups ohne Router.
*   **Screens in voller Höhe:** Innere Scroll-Container werden aufgeklappt, sodass ein Screen komplett erfasst und nicht am Viewport abgeschnitten wird.
*   **Originaltreuer Text:** Die Seite wird mit derselben Schrift gerendert, die Figma verwendet, daher stimmen Zeilenumbrüche. Einzeilige Texte bekommen automatische Breite.
*   **Editierbare Ausgabe:** Ebenennamen aus CSS-Klassen, seitenweise Konturen, Eckenradien, lineare Verläufe, äußere und innere Schatten, Clipping, Deckkraft, Textcase und -dekoration.
*   **Referenz-PNGs** neben der JSON für schnelle Sichtprüfung.
*   **Claude-Code-Plugin** inklusive: Der Agent analysiert das Mockup, schreibt die Konfiguration, erfasst, prüft und liefert die Importschritte.

---

## 📦 01. Installation

Binary aus den [Releases](https://github.com/swrneko/dom2figma/releases) laden (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) und in den `PATH` legen, oder aus dem Quellcode bauen:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Voraussetzungen:**
*   **Chrome oder Chromium.** Wird an Standardorten automatisch gefunden; überschreibbar per `--chrome`, Umgebungsvariable `CHROME` oder `chrome = "..."` in der Konfiguration.
*   **Die Erfassungsschrift** (Standard `Inter`) lokal installiert. Linux: TTFs nach `~/.local/share/fonts/` und `fc-cache -f`. macOS: über Schriftsammlung installieren. Siehe [Schriften](#-04-schriften).

---

## 🚀 02. Schnellstart

```bash
dom2figma init                 # schreibt dom2figma.toml mit Kommentaren
$EDITOR dom2figma.toml         # source, root, routes setzen
dom2figma doctor               # Chrome und Schrift vorhanden?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, Plugin starten und `out/screens.json` wählen. Screens werden als eine Section je Gruppe auf der aktuellen Seite angeordnet.

**Figma im Browser (oder Linux ohne Desktop-App):**

```bash
dom2figma serve                              # liefert out/*.json mit CORS unter http://localhost:8787
dom2figma scripter --out scripter.js         # Skript für das Scripter-Plugin
```

In Figma: Plugins → Scripter → New script → `scripter.js` einfügen → Run.

Teilläufe: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Konfiguration

```toml
source = "prototype.html"              # HTML-Datei oder http(s)-URL
root = ".device"                       # Selektor der Screen-Wurzel, Standard: body
font = "Inter"                         # Erfassungsschrift, lokal und in Figma nötig
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # inneren Scroll auf volle Höhe aufklappen
min_height = 844
fix_css = ".panel{display:none!important}"   # vor der Erfassung injiziertes CSS
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Feld | Standard | Zweck |
|---|---|---|
| `source` | Pflicht | Pfad zur HTML oder `http(s)://`-URL |
| `root` | `body` | CSS-Selektor der erfassten Wurzel |
| `font`, `force_font` | `Inter`, `true` | Renderschrift; `force_font` wendet sie auf alle Elemente unter root an |
| `viewport` | `1400×1000` | Browserfenster; `390×844` für mobile Layouts |
| `navigation` | `hash` | `hash` setzt `location.hash`; `url` navigiert zu `source + route`; `js` wertet die Route als JavaScript aus |
| `fix_css` | leer | CSS-Patch: Werkzeugleisten ausblenden, Geräterahmen entfernen, Seite in der Höhe wachsen lassen |
| `scroll_container`, `min_height` | keiner, `0` | inneren Scroll-Container aufklappen, damit Screens vollständig erfasst werden |
| `settle_ms` | `400` | Pause nach der Navigation |
| `routes`, `groups` | | Routen und Zustandsgruppen; eine Gruppe hat eigene `routes`, ein JS-`setup` und `setup_each_route`, um setup nach jeder Navigation zu wiederholen |
| `out`, `png`, `chrome` | `out`, `true`, auto | Ausgabeverzeichnis, PNGs speichern, Chrome-Pfad |

Ein vollständig kommentiertes Beispiel liegt in [`examples/dom2figma.toml`](../../examples/dom2figma.toml).

---

## 🔤 04. Schriften

Chrome rendert mit der auf dem Rechner installierten Schrift, Figma ersetzt sie durch seine eigene. Unterscheiden sie sich, verschieben sich Zeilenumbrüche. Deshalb rendert dom2figma mit der in `font` genannten Schrift und erwartet dieselbe Schrift in Figma. **Inter und alle Google Fonts sind in Figma immer verfügbar.** `dom2figma doctor` prüft die lokale Seite.

---

## 📐 05. Was übertragen wird

**Übertragen:** rechteckige Boxen mit Farben, linearen Verläufen, seitenweisen Konturen, Radien, Schatten und Clipping; Text mit Größe, Gewicht, Laufweite, Ausrichtung, Case und Dekoration; Inline-SVG-Icons; Eingabewerte und Platzhalter; Text-Pseudoelemente. Unsichtbares wird übersprungen: `display:none`, Deckkraft null, transparente Textfarbe.

**Angenähert oder übersprungen:** Rasterbilder per externer URL (Data-URLs funktionieren), radiale Verläufe, Filter, Blend Modes, Canvas und Video, Transforms (nur Bounding Box). **Kein Auto Layout** wird erzeugt: Ebenen sind absolut positioniert. Das Ergebnis ist eine exakte strukturelle Kopie zum Weiterarbeiten, kein fertiges Designsystem.

---

## 🤖 06. Claude-Code-Plugin

Das Repository enthält ein Claude-Code-Plugin mit dem Skill `dom2figma`. Der Agent analysiert das Mockup (Wurzel, Navigation, Zustände), schreibt die Konfiguration, erfasst eine Testroute, prüft die PNGs, führt die vollständige Erfassung aus und liefert Importanweisungen.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Danach genügt *„bring diesen Prototyp nach Figma“* oder der Aufruf `/dom2figma:dom2figma`. Fehlt die Binary, installiert der Skill sie mit `cargo install --git`.

---

## 🛠️ 07. Entwicklung

```bash
cargo test                      # Unit-Tests
cargo test -- --ignored         # End-to-End mit echtem Chrome auf tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # Figma-Plugin gegen eine Stub-API laufen lassen, ohne Figma
```

```text
src/capture.rs            steuert Chrome über das DevTools Protocol
assets/serialize.js       wird in die Seite injiziert, baut den Ebenenbaum
assets/figma-plugin/      Figma-Plugin; der Block LIB START/END wird mit dem Scripter-Build geteilt
claude-plugin/            Claude-Code-Plugin und Skill
.github/workflows/        ci.yml bei jedem Push, release.yml bei v*-Tags
```

---

## 📄 Lizenz

MIT. Siehe [LICENSE](../../LICENSE).
