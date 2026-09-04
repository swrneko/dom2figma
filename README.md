# dom2figma

CLI, который снимает отрисованный DOM веб-прототипа или сайта и переносит его в Figma редактируемыми слоями: фреймы с заливками, обводками и тенями, текстовые слои с шрифтами, SVG-иконки как векторы. Один бинарник под Linux и macOS, из внешних зависимостей только Chrome или Chromium.

В отличие от плагинов вроде html.to.design снимает десятки экранов за один запуск, умеет состояния приложения (персоны, роли, темы) и воспроизводится одной командой после правок прототипа.

## Как это работает

1. `dom2figma capture` запускает headless Chrome, открывает источник, для каждой группы состояний выполняет `setup`, обходит маршруты и на каждом снимает дерево видимых элементов: координаты, стили, тексты, SVG. Повторяющиеся экраны отсеиваются по хэшу дерева. Результат: `out/screens.json` и референсные PNG.
2. Figma-плагин (`dom2figma plugin`) читает этот JSON и строит секции, фреймы и текстовые слои через Plugin API. Для браузерной Figma есть вариант через плагин Scripter (`dom2figma serve` + `dom2figma scripter`).

## Установка

Готовые бинарники лежат в релизах на GitHub (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`). Из исходников:

```bash
cargo install --path .
```

Нужен Chrome или Chromium. Бинарник ищет его в стандартных местах, путь можно задать флагом `--chrome`, переменной `CHROME` или в конфиге.

## Быстрый старт

```bash
dom2figma init                 # создаёт dom2figma.toml с примером и комментариями
$EDITOR dom2figma.toml         # source, root, routes
dom2figma doctor               # Chrome и шрифт на месте?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

Импорт в Figma Desktop: Plugins → Development → Import plugin from manifest… → `figma-plugin/manifest.json`, затем запустить плагин и выбрать `out/screens.json`. Экраны раскладываются секциями по группам на текущей странице.

Импорт в браузерной Figma (или на Linux без десктопного приложения):

```bash
dom2figma serve                              # раздаёт out/*.json на http://localhost:8787 с CORS
dom2figma scripter --out scripter.js         # скрипт для плагина Scripter
```

В Figma: Plugins → Scripter → New script → вставить содержимое `scripter.js` → Run.

## Конфиг

```toml
source = "prototype.html"        # HTML-файл или URL
root = ".device"                 # корень экрана, по умолчанию body
font = "Inter"                   # шрифт съёмки и Figma; должен быть установлен локально
navigation = "hash"              # hash | url | js
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

| Поле | Назначение |
|---|---|
| `source` | путь к HTML или `http(s)://` URL |
| `root` | CSS-селектор снимаемого корня; для сайта `body` |
| `font`, `force_font` | шрифт, которым рендерится страница; при `force_font = true` применяется ко всем элементам корня |
| `navigation` | `hash` меняет `location.hash`; `url` переходит на `source + route`; `js` выполняет маршрут как JS-выражение |
| `viewport` | размер окна браузера, по умолчанию 1400×1000 |
| `fix_css` | CSS-заплатка: скрыть служебные панели, убрать рамки, дать странице расти по высоте |
| `scroll_container`, `min_height` | раскрыть внутренний скролл, чтобы экран снимался целиком |
| `settle_ms` | пауза после навигации |
| `routes`, `groups` | маршруты и группы состояний; у группы свои `routes` и JS `setup`, флаг `setup_each_route` повторяет `setup` после каждой навигации |
| `out`, `png`, `chrome` | каталог результатов, сохранять ли PNG, путь к Chrome |

Команда `capture` принимает `--group` и `--route` для частичного прогона, `--name` для имени JSON, `--no-png`.

## Шрифт

Chrome рендерит страницу тем шрифтом, который установлен в системе, а Figma подставляет свой. Если они разные, переносы строк разъезжаются. Поэтому съёмка идёт указанным в `font` шрифтом, и он должен быть и в системе, и в Figma. Inter и любой шрифт из Google Fonts в Figma есть всегда. Локально: на Linux положить TTF в `~/.local/share/fonts/` и выполнить `fc-cache -f`, на macOS установить через Font Book. `dom2figma doctor` проверяет наличие.

## Что переносится и что нет

Переносится: прямоугольные блоки с цветом, линейными градиентами, обводками по сторонам, радиусами, тенями и обрезкой; текст с размером, весом, межбуквенным интервалом, выравниванием, регистром и подчёркиванием; SVG-иконки; значения и плейсхолдеры полей; текстовые псевдоэлементы. Невидимое не переносится: display none, нулевая прозрачность, прозрачный цвет текста.

Не переносится или приближается: растровые картинки по внешним URL (только data URL), радиальные градиенты, фильтры, blend modes, canvas и видео, трансформы (даётся ограничивающий прямоугольник). Auto layout не строится, слои позиционируются абсолютно: результат это точная структурная копия для дизайнера, а не готовая дизайн-система.

## Плагин для Claude Code

В репозитории лежит плагин со скиллом `dom2figma`: агент сам разбирается в устройстве мокапа, пишет конфиг, снимает экраны, сверяет PNG и выдаёт инструкцию по импорту. Установка в Claude Code:

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

После этого достаточно попросить «перегони этот прототип в Figma» или вызвать `/dom2figma:dom2figma`. Сам бинарник скилл при отсутствии ставит через `cargo install --git https://github.com/swrneko/dom2figma`.

## Разработка

```bash
cargo test                      # юнит-тесты
cargo test -- --ignored         # сквозной тест с реальным Chrome на tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # прогон плагина на заглушке Figma API без Figma
```

Структура: `src/capture.rs` управляет Chrome, `assets/serialize.js` внедряется в страницу и строит дерево, `assets/figma-plugin/` собирает слои в Figma. Блок между маркерами `LIB START/END` в `code.js` общий для плагина и Scripter-варианта.
