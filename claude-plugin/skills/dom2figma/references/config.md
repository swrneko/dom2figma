# Конфиг dom2figma.toml

| Поле | По умолчанию | Назначение |
|---|---|---|
| `source` | обязательно | путь к HTML или `http(s)://` URL |
| `root` | `body` | CSS-селектор снимаемого корня |
| `font` | `Inter` | шрифт съёмки, должен быть локально и в Figma |
| `force_font` | `true` | применить `font` ко всем элементам корня через `!important` |
| `viewport` | `1400×1000` | размер окна Chrome |
| `fix_css` | пусто | CSS перед съёмкой |
| `scroll_container` | нет | селектор внутреннего скролла, который раскрывается на всю высоту |
| `min_height` | `0` | базовая высота корня в px при раскрытии скролла |
| `settle_ms` | `400` | пауза после навигации |
| `navigation` | `hash` | `hash` \| `url` \| `js` |
| `out` | `out` | каталог результатов |
| `png` | `true` | сохранять PNG |
| `chrome` | автопоиск | путь к Chrome |
| `routes` | `[]` | общие маршруты |
| `groups` | `[]` | группы состояний: `name`, `setup` (JS), `setup_each_route`, `routes` |

## Прототип в рамке телефона с панелью персон (hash-роутер)

```toml
source = "prototype.html"
root = ".device"
navigation = "hash"
scroll_container = ".device__scroll"
min_height = 844
fix_css = """
.stand__panel{display:none!important}
html,body,#root,.stand{height:auto!important;min-height:0!important;overflow:visible!important}
.stand__stage{padding:0!important;display:block!important;overflow:visible!important}
.device{border:0!important;border-radius:0!important;box-shadow:none!important;margin:0!important}
"""
routes = ["/welcome", "/catalog", "/product/p01", "/cart"]

[[groups]]
name = "Гость"
setup = "[...document.querySelectorAll('.panel button')].find(b => b.textContent.includes('Гость')).click()"

[[groups]]
name = "Менеджер"
setup = "[...document.querySelectorAll('.panel button')].find(b => b.textContent.includes('Менеджер')).click()"
routes = ["/manager", "/manager/clients"]
```

## Обычный сайт или SPA с путями (dev-сервер)

```toml
source = "http://localhost:5173"
root = "body"
navigation = "url"
settle_ms = 800
routes = ["/", "/pricing", "/blog", "/login"]

[viewport]
width = 1440
height = 900

[[groups]]
name = "Авторизован"
setup = "localStorage.setItem('token', 'demo')"
setup_each_route = true          # url-навигация перезагружает страницу
routes = ["/dashboard", "/settings"]
```

## Мокап без роутера: экраны переключаются кликами

```toml
source = "mockup.html"
root = "#screen"
navigation = "js"
routes = [
  "document.querySelector('#nav-home').click()",
  "document.querySelector('#nav-catalog').click()",
  "document.querySelector('#open-cart').click()",
]
```

## Мобильная версия сайта

Задайте узкий viewport, адаптивная вёрстка перестроится сама:

```toml
[viewport]
width = 390
height = 844
```
