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

**Convierte cualquier prototipo web renderizado en capas editables de Figma. Decenas de pantallas, todos los estados de la app, un solo comando.**  
*Headless Chrome captura el DOM y un plugin de Figma lo reconstruye como frames, texto y vectores.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 Acerca de

**dom2figma** es una herramienta de línea de comandos que captura lo que el navegador realmente renderiza, no el HTML fuente, y lo lleva a Figma como capas reales: frames con rellenos, bordes, radios y sombras, capas de texto con fuentes y alineación, iconos SVG como vectores.

A diferencia de plugins como html.to.design, está pensada para **prototipos completos**: recorre una lista de rutas, cambia los estados de la aplicación (personas, roles, temas) mediante pequeños hooks de JavaScript, elimina pantallas idénticas y produce un único JSON que se importa con un clic. Cambias el prototipo, vuelves a ejecutar el comando.

Se distribuye como **un único binario estático para Linux y macOS**. La única dependencia externa es Chrome o Chromium.

---

## 🏗️ Cómo funciona

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   lanzar    ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ abrir source           │──────────►│ plugin o Scripter    │
  │ root          │             │ por cada grupo:        │           │  • sección por grupo │
  │ routes        │             │   ejecutar setup JS    │           │  • frame por pantalla│
  │ groups[]      │             │   por cada ruta:       │           │  • rellenos, bordes, │
  │ fix_css       │             │     navegar y esperar  │           │    radios, sombras   │
  │ font          │             │     inyectar serialize │           │  • texto con fuentes │
  └───────────────┘             │     DOM → árbol        │           │  • SVG → vectores    │
                                │     hash → dedupe      │           └──────────────────────┘
                                │     PNG del elemento   │
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Captura.** `dom2figma capture` lanza Chrome, abre la fuente, ejecuta el `setup` de cada grupo, visita cada ruta y serializa el DOM visible: geometría, estilos calculados, tramos de texto, SVG. Las pantallas idénticas se descartan por el hash del árbol.
2. **Importación.** El plugin de Figma incluido (`dom2figma plugin`) lee el JSON y construye secciones, frames y nodos de texto con la Plugin API. Para Figma en el navegador existe una variante con Scripter (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Características

*   **Todo el prototipo en una pasada:** rutas × estados, con deduplicación automática de pantallas idénticas.
*   **Estados de la app como grupos:** personas, roles o temas se cambian con un hook `setup` de JavaScript por grupo.
*   **Tres modos de navegación:** `hash` para routers `#/route`, `url` para rutas reales, `js` para maquetas basadas en clics sin router.
*   **Pantallas a altura completa:** los contenedores con scroll interno se expanden para capturar la pantalla entera, sin recortarla al viewport.
*   **Texto fiel:** la página se renderiza con la misma fuente que usará Figma, así los saltos de línea coinciden. Los textos de una línea tienen ancho automático.
*   **Salida editable:** capas nombradas por clases CSS, bordes por lado, radios de esquina, degradados lineales, sombras exteriores e interiores, recorte, opacidad, mayúsculas y decoración del texto.
*   **PNG de referencia** junto al JSON para una comprobación visual rápida.
*   **Plugin para Claude Code** incluido: el agente inspecciona la maqueta, escribe la configuración, captura, verifica y entrega los pasos de importación.

---

## 📦 01. Instalación

Descarga un binario desde [Releases](https://github.com/swrneko/dom2figma/releases) (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) y ponlo en tu `PATH`, o compila desde el código fuente:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Requisitos:**
*   **Chrome o Chromium.** Se detecta automáticamente en las ubicaciones habituales; se puede indicar con `--chrome`, la variable de entorno `CHROME` o `chrome = "..."` en la configuración.
*   **La fuente de captura** (por defecto `Inter`) instalada localmente. Linux: copia los TTF a `~/.local/share/fonts/` y ejecuta `fc-cache -f`. macOS: instálala con Catálogo Tipográfico. Ver [Fuentes](#-04-fuentes).

---

## 🚀 02. Inicio rápido

```bash
dom2figma init                 # crea dom2figma.toml con comentarios
$EDITOR dom2figma.toml         # define source, root, routes
dom2figma doctor               # ¿Chrome y fuente presentes?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, ejecuta el plugin y elige `out/screens.json`. Las pantallas se disponen en una sección por grupo en la página actual.

**Figma en el navegador (o Linux sin la app de escritorio):**

```bash
dom2figma serve                              # sirve out/*.json con CORS en http://localhost:8787
dom2figma scripter --out scripter.js         # script para el plugin Scripter
```

En Figma: Plugins → Scripter → New script → pega `scripter.js` → Run.

Ejecuciones parciales: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Configuración

```toml
source = "prototype.html"              # archivo HTML o URL http(s)
root = ".device"                       # selector de la raíz de pantalla, por defecto body
font = "Inter"                         # fuente de captura, debe existir localmente y en Figma
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # expandir el scroll interno a toda la altura
min_height = 844
fix_css = ".panel{display:none!important}"   # CSS inyectado antes de capturar
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Campo | Por defecto | Propósito |
|---|---|---|
| `source` | obligatorio | ruta al HTML o URL `http(s)://` |
| `root` | `body` | selector CSS de la raíz capturada |
| `font`, `force_font` | `Inter`, `true` | fuente de renderizado; `force_font` la aplica a todos los elementos bajo root |
| `viewport` | `1400×1000` | tamaño de la ventana del navegador; usa `390×844` para móviles |
| `navigation` | `hash` | `hash` asigna `location.hash`; `url` navega a `source + route`; `js` evalúa la ruta como JavaScript |
| `fix_css` | vacío | parche CSS: ocultar paneles, quitar marcos de dispositivo, dejar que la página crezca en altura |
| `scroll_container`, `min_height` | ninguno, `0` | expandir un contenedor con scroll interno para capturar las pantallas completas |
| `settle_ms` | `400` | pausa tras la navegación |
| `routes`, `groups` | | rutas y grupos de estado; un grupo tiene sus propias `routes`, un `setup` JS y `setup_each_route` para repetir setup tras cada navegación |
| `out`, `png`, `chrome` | `out`, `true`, auto | directorio de salida, guardar PNG, ruta de Chrome |

Un ejemplo totalmente comentado está en [`examples/dom2figma.toml`](../../examples/dom2figma.toml).

---

## 🔤 04. Fuentes

Chrome renderiza con la fuente instalada en la máquina, mientras que Figma sustituye la suya. Si difieren, los saltos de línea se desplazan. Por eso dom2figma renderiza con la fuente indicada en `font` y espera la misma fuente en Figma. **Inter y todas las Google Fonts están siempre disponibles en Figma.** `dom2figma doctor` comprueba el lado local.

---

## 📐 05. Qué se transfiere

**Se transfiere:** cajas rectangulares con colores sólidos, degradados lineales, bordes por lado, radios, sombras y recorte; texto con tamaño, peso, espaciado, alineación, mayúsculas y decoración; iconos SVG inline; valores y placeholders de campos; pseudoelementos de texto. Lo invisible se omite: `display:none`, opacidad cero, color de texto transparente.

**Se aproxima u omite:** imágenes rasterizadas por URL externa (las data URL funcionan), degradados radiales, filtros, modos de fusión, canvas y vídeo, transformaciones (solo el cuadro delimitador). **No se genera auto layout:** las capas se posicionan de forma absoluta. El resultado es una copia estructural exacta para que un diseñador trabaje sobre ella, no un sistema de diseño terminado.

---

## 🤖 06. Plugin para Claude Code

El repositorio incluye un plugin de Claude Code con la skill `dom2figma`. El agente inspecciona la maqueta (raíz, navegación, estados), escribe la configuración, captura una ruta de prueba, revisa los PNG, ejecuta la captura completa y devuelve las instrucciones de importación.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Después basta con pedir *«lleva este prototipo a Figma»* o invocar `/dom2figma:dom2figma`. Si falta el binario, la skill lo instala con `cargo install --git`.

---

## 🛠️ 07. Desarrollo

```bash
cargo test                      # tests unitarios
cargo test -- --ignored         # end-to-end con Chrome real sobre tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # ejecuta el plugin de Figma contra una API simulada, sin Figma
```

```text
src/capture.rs            controla Chrome mediante el DevTools Protocol
assets/serialize.js       se inyecta en la página y construye el árbol de capas
assets/figma-plugin/      plugin de Figma; el bloque LIB START/END se comparte con la variante Scripter
claude-plugin/            plugin y skill de Claude Code
.github/workflows/        ci.yml en cada push, release.yml en tags v*
```

---

## 📄 Licencia

MIT. Ver [LICENSE](../../LICENSE).
