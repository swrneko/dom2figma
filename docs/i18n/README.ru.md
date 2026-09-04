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

**Превращает любой отрисованный веб-прототип в редактируемые слои Figma. Десятки экранов, все состояния приложения, одна команда.**  
*Headless Chrome снимает DOM, плагин Figma собирает из него фреймы, текст и векторы.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 О проекте

**dom2figma** — консольный инструмент, который снимает то, что реально нарисовал браузер, а не исходный HTML, и переносит это в Figma настоящими слоями: фреймы с заливками, обводками, радиусами и тенями, текстовые слои со шрифтами и выравниванием, SVG-иконки как векторы.

В отличие от плагинов вроде html.to.design он рассчитан на **прототип целиком**: обходит список маршрутов, переключает состояния приложения (персоны, роли, темы) через небольшие JavaScript-хуки, отсеивает одинаковые экраны и выдаёт один JSON, который импортируется в один клик. Поменяли прототип — запустили команду ещё раз.

Поставляется **одним статическим бинарником под Linux и macOS**. Единственная внешняя зависимость — Chrome или Chromium.

---

## 🏗️ Как это работает

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   запуск    ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ открыть source         │──────────►│ плагин или Scripter  │
  │ root          │             │ для каждой группы:     │           │  • секция на группу  │
  │ routes        │             │   выполнить setup JS   │           │  • фрейм на экран    │
  │ groups[]      │             │   для каждого маршрута:│           │  • заливки, обводки, │
  │ fix_css       │             │     перейти, подождать │           │    радиусы, тени     │
  │ font          │             │     внедрить serialize │           │  • текст со шрифтами │
  └───────────────┘             │     DOM → дерево       │           │  • SVG → векторы     │
                                │     хэш → дедупликация │           └──────────────────────┘
                                │     PNG элемента       │
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Захват.** `dom2figma capture` запускает Chrome, открывает источник, выполняет `setup` каждой группы, обходит маршруты и сериализует видимый DOM: геометрию, вычисленные стили, текстовые строки, SVG. Одинаковые экраны отбрасываются по хэшу дерева.
2. **Импорт.** Встроенный плагин Figma (`dom2figma plugin`) читает JSON и строит секции, фреймы и текстовые узлы через Plugin API. Для браузерной Figma есть вариант через Scripter (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Возможности

*   **Прототип целиком за один прогон:** маршруты × состояния с автоматической дедупликацией одинаковых экранов.
*   **Состояния как группы:** персоны, роли или темы переключаются JavaScript-хуком `setup` у каждой группы.
*   **Три режима навигации:** `hash` для роутеров `#/route`, `url` для настоящих путей, `js` для мокапов на кликах без роутера.
*   **Экраны во всю высоту:** внутренние скролл-контейнеры раскрываются, экран снимается целиком, а не обрезается по окну.
*   **Точный текст:** страница рендерится тем же шрифтом, что подставит Figma, поэтому переносы строк совпадают. Однострочные тексты получают автоширину.
*   **Редактируемый результат:** имена слоёв из CSS-классов, обводки по сторонам, радиусы углов, линейные градиенты, внешние и внутренние тени, обрезка, прозрачность, регистр и подчёркивание текста.
*   **Референсные PNG** рядом с JSON для быстрой визуальной сверки.
*   **Плагин для Claude Code** в комплекте: агент изучает мокап, пишет конфиг, снимает, проверяет и выдаёт шаги импорта.

---

## 📦 01. Установка

Скачайте бинарник из [Releases](https://github.com/swrneko/dom2figma/releases) (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) и положите в `PATH`, либо соберите из исходников:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Требования:**
*   **Chrome или Chromium.** Ищется автоматически в стандартных местах; путь задаётся флагом `--chrome`, переменной `CHROME` или полем `chrome = "..."` в конфиге.
*   **Шрифт съёмки** (по умолчанию `Inter`), установленный локально. Linux: TTF в `~/.local/share/fonts/` и `fc-cache -f`. macOS: через Font Book. См. [Шрифты](#-04-шрифты).

---

## 🚀 02. Быстрый старт

```bash
dom2figma init                 # создаёт dom2figma.toml с комментариями
$EDITOR dom2figma.toml         # source, root, routes
dom2figma doctor               # Chrome и шрифт на месте?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, запустить плагин и выбрать `out/screens.json`. Экраны раскладываются секциями по группам на текущей странице.

**Браузерная Figma (или Linux без десктопного приложения):**

```bash
dom2figma serve                              # раздаёт out/*.json с CORS на http://localhost:8787
dom2figma scripter --out scripter.js         # скрипт для плагина Scripter
```

В Figma: Plugins → Scripter → New script → вставить `scripter.js` → Run.

Частичный прогон: `dom2figma capture --group "Гость" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Конфигурация

```toml
source = "prototype.html"              # HTML-файл или http(s) URL
root = ".device"                       # селектор корня экрана, по умолчанию body
font = "Inter"                         # шрифт съёмки, должен быть локально и в Figma
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # раскрыть внутренний скролл на всю высоту
min_height = 844
fix_css = ".panel{display:none!important}"   # CSS перед съёмкой
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Гость"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Менеджер"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Поле | По умолчанию | Назначение |
|---|---|---|
| `source` | обязательно | путь к HTML или `http(s)://` URL |
| `root` | `body` | CSS-селектор снимаемого корня |
| `font`, `force_font` | `Inter`, `true` | шрифт рендеринга; `force_font` применяет его ко всем элементам корня |
| `viewport` | `1400×1000` | размер окна браузера; `390×844` для мобильной вёрстки |
| `navigation` | `hash` | `hash` меняет `location.hash`; `url` переходит на `source + route`; `js` выполняет маршрут как JavaScript |
| `fix_css` | пусто | CSS-заплатка: скрыть панели, убрать рамки устройства, дать странице расти по высоте |
| `scroll_container`, `min_height` | нет, `0` | раскрыть внутренний скролл, чтобы экраны снимались целиком |
| `settle_ms` | `400` | пауза после навигации |
| `routes`, `groups` | | маршруты и группы состояний; у группы свои `routes`, JS `setup` и `setup_each_route` для повтора setup после каждой навигации |
| `out`, `png`, `chrome` | `out`, `true`, авто | каталог результатов, сохранять PNG, путь к Chrome |

Полностью прокомментированный пример: [`examples/dom2figma.toml`](../../examples/dom2figma.toml).

---

## 🔤 04. Шрифты

Chrome рендерит тем шрифтом, что установлен в системе, а Figma подставляет свой. Если они разные, переносы строк разъезжаются. Поэтому dom2figma рендерит шрифтом из `font` и ожидает тот же шрифт в Figma. **Inter и любой шрифт из Google Fonts в Figma есть всегда.** `dom2figma doctor` проверяет локальную сторону.

---

## 📐 05. Что переносится

**Переносится:** прямоугольные блоки с цветом, линейными градиентами, обводками по сторонам, радиусами, тенями и обрезкой; текст с размером, весом, межбуквенным интервалом, выравниванием, регистром и подчёркиванием; инлайновые SVG-иконки; значения и плейсхолдеры полей; текстовые псевдоэлементы. Невидимое пропускается: `display:none`, нулевая прозрачность, прозрачный цвет текста.

**Приближается или пропускается:** растровые картинки по внешним URL (data URL работают), радиальные градиенты, фильтры, blend modes, canvas и видео, трансформы (только ограничивающий прямоугольник). **Auto layout не строится:** слои позиционированы абсолютно. Результат — точная структурная копия для дизайнера, а не готовая дизайн-система.

---

## 🤖 06. Плагин для Claude Code

В репозитории лежит плагин Claude Code со скиллом `dom2figma`. Агент изучает мокап (корень, навигацию, состояния), пишет конфиг, снимает пробный маршрут, проверяет PNG, делает полный прогон и возвращает инструкцию по импорту.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Дальше достаточно попросить «перегони этот прототип в Figma» или вызвать `/dom2figma:dom2figma`. Если бинарника нет, скилл установит его через `cargo install --git`.

---

## 🛠️ 07. Разработка

```bash
cargo test                      # юнит-тесты
cargo test -- --ignored         # сквозной тест с реальным Chrome на tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # прогон плагина на заглушке API без Figma
```

```text
src/capture.rs            управляет Chrome через DevTools Protocol
assets/serialize.js       внедряется в страницу, строит дерево слоёв
assets/figma-plugin/      плагин Figma; блок LIB START/END общий со сборкой для Scripter
claude-plugin/            плагин и скилл Claude Code
.github/workflows/        ci.yml на каждый пуш, release.yml на теги v*
```

---

## 📄 Лицензия

MIT. См. [LICENSE](../../LICENSE).
