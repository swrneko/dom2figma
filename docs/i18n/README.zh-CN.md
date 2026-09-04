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

**把任何已渲染的网页原型变成可编辑的 Figma 图层。几十个页面、全部应用状态，一条命令。**  
*Headless Chrome 抓取 DOM，Figma 插件把它重建为框架、文本和矢量。*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 关于

**dom2figma** 是一个命令行工具，抓取的是浏览器实际渲染的结果而不是 HTML 源码，并把它作为真实图层放进 Figma：带填充、描边、圆角和阴影的框架，带字体和对齐的文本图层，作为矢量的 SVG 图标。

与 html.to.design 之类的插件不同，它面向**整个原型**：遍历路由列表，通过简短的 JavaScript 钩子切换应用状态（角色、人设、主题），去除重复页面，最终生成一个 JSON，一键导入。原型改了，再跑一次命令即可。

以**单个静态二进制文件**发布，支持 Linux 和 macOS。唯一的外部依赖是 Chrome 或 Chromium。

---

## 🏗️ 工作原理

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   启动      ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ 打开 source            │──────────►│ 插件或 Scripter      │
  │ root          │             │ 对每个分组:            │           │  • 每组一个 section  │
  │ routes        │             │   执行 setup JS        │           │  • 每页一个 frame    │
  │ groups[]      │             │   对每个路由:          │           │  • 填充、描边、      │
  │ fix_css       │             │     导航并等待         │           │    圆角、阴影        │
  │ font          │             │     注入 serialize.js  │           │  • 带字体的文本      │
  └───────────────┘             │     DOM → 树           │           │  • SVG → 矢量        │
                                │     哈希 → 去重        │           └──────────────────────┘
                                │     元素 PNG           │
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **抓取。** `dom2figma capture` 启动 Chrome，打开源，运行每个分组的 `setup`，访问每个路由并序列化可见 DOM：几何、计算样式、文本段、SVG。相同页面按树哈希丢弃。
2. **导入。** 自带的 Figma 插件（`dom2figma plugin`）读取 JSON，通过 Plugin API 构建 section、frame 和文本节点。浏览器版 Figma 可使用 Scripter 方案（`dom2figma serve` + `dom2figma scripter`）。

---

## ✨ 特性

*   **一次抓取整个原型：** 路由 × 状态，自动去除重复页面。
*   **应用状态即分组：** 人设、角色或主题通过每组的 JavaScript `setup` 钩子切换。
*   **三种导航模式：** `hash` 用于 `#/route` 路由器，`url` 用于真实路径，`js` 用于没有路由器、靠点击切换的原型。
*   **整页高度：** 内部滚动容器会被展开，页面完整抓取而不是被视口裁剪。
*   **忠实的文本：** 页面用 Figma 将使用的同一字体渲染，换行一致。单行文本自动宽度。
*   **可编辑输出：** 由 CSS 类命名的图层、四边独立描边、圆角、线性渐变、外/内阴影、裁剪、透明度、文本大小写和装饰线。
*   **参考 PNG** 与 JSON 并列，便于快速目检。
*   **附带 Claude Code 插件：** 代理自行分析原型、编写配置、抓取、校验并给出导入步骤。

---

## 📦 01. 安装

从 [Releases](https://github.com/swrneko/dom2figma/releases) 下载二进制文件（`dom2figma-linux-x86_64`、`dom2figma-macos-arm64`、`dom2figma-macos-x86_64`）并放入 `PATH`，或从源码构建：

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**要求：**
*   **Chrome 或 Chromium。** 会在标准位置自动检测；可用 `--chrome`、环境变量 `CHROME` 或配置中的 `chrome = "..."` 指定。
*   **抓取所用字体**（默认 `Inter`）已本地安装。Linux：把 TTF 放到 `~/.local/share/fonts/` 并运行 `fc-cache -f`。macOS：通过字体册安装。见[字体](#-04-字体)。

---

## 🚀 02. 快速开始

```bash
dom2figma init                 # 生成带注释的 dom2figma.toml
$EDITOR dom2figma.toml         # 设置 source、root、routes
dom2figma doctor               # Chrome 和字体是否就位？
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop：** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`，运行插件并选择 `out/screens.json`。页面按分组以 section 排列在当前页。

**浏览器版 Figma（或没有桌面应用的 Linux）：**

```bash
dom2figma serve                              # 在 http://localhost:8787 以 CORS 方式提供 out/*.json
dom2figma scripter --out scripter.js         # Scripter 插件用的脚本
```

在 Figma 中：Plugins → Scripter → New script → 粘贴 `scripter.js` → Run。

部分抓取：`dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`。

---

## ⚙️ 03. 配置

```toml
source = "prototype.html"              # HTML 文件或 http(s) URL
root = ".device"                       # 页面根选择器，默认 body
font = "Inter"                         # 抓取字体，需本地和 Figma 中都存在
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # 将内部滚动展开到完整高度
min_height = 844
fix_css = ".panel{display:none!important}"   # 抓取前注入的 CSS
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| 字段 | 默认值 | 用途 |
|---|---|---|
| `source` | 必填 | HTML 路径或 `http(s)://` URL |
| `root` | `body` | 抓取根的 CSS 选择器 |
| `font`, `force_font` | `Inter`, `true` | 渲染字体；`force_font` 把它应用到根下所有元素 |
| `viewport` | `1400×1000` | 浏览器窗口尺寸；移动端布局用 `390×844` |
| `navigation` | `hash` | `hash` 设置 `location.hash`；`url` 导航到 `source + route`；`js` 把路由当作 JavaScript 执行 |
| `fix_css` | 空 | CSS 补丁：隐藏工具面板、去掉设备边框、让页面在高度上自由生长 |
| `scroll_container`, `min_height` | 无, `0` | 展开内部滚动容器以完整抓取页面 |
| `settle_ms` | `400` | 导航后的等待 |
| `routes`, `groups` | | 路由和状态分组；分组有自己的 `routes`、JS `setup`，以及每次导航后重跑 setup 的 `setup_each_route` |
| `out`, `png`, `chrome` | `out`, `true`, 自动 | 输出目录、是否保存 PNG、Chrome 路径 |

完整带注释的示例见 [`examples/dom2figma.toml`](../../examples/dom2figma.toml)。

---

## 🔤 04. 字体

Chrome 用机器上安装的字体渲染，Figma 则替换为自己的字体。两者不同时换行会漂移。因此 dom2figma 用 `font` 指定的字体渲染，并要求 Figma 中有同一字体。**Inter 和所有 Google Fonts 在 Figma 中始终可用。** `dom2figma doctor` 检查本地一侧。

---

## 📐 05. 哪些会被转换

**会转换：** 带纯色、线性渐变、四边描边、圆角、阴影和裁剪的矩形块；带字号、字重、字距、对齐、大小写和装饰线的文本；内联 SVG 图标；输入框的值和占位符；文本伪元素。不可见内容跳过：`display:none`、零透明度、透明文字颜色。

**近似或跳过：** 外部 URL 的位图（data URL 可用）、径向渐变、滤镜、混合模式、canvas 和视频、变换（仅边界框）。**不生成 auto layout：** 图层为绝对定位。结果是供设计师继续加工的精确结构副本，而不是成品设计系统。

---

## 🤖 06. Claude Code 插件

仓库附带一个 Claude Code 插件及 `dom2figma` 技能。代理会分析原型（根、导航、状态）、编写配置、抓取一个测试路由、检查 PNG、执行完整抓取并返回导入说明。

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

然后说一句"把这个原型搬进 Figma"，或调用 `/dom2figma:dom2figma`。若二进制文件缺失，技能会通过 `cargo install --git` 安装。

---

## 🛠️ 07. 开发

```bash
cargo test                      # 单元测试
cargo test -- --ignored         # 用真实 Chrome 对 tests/fixtures/simple.html 做端到端测试
node scripts/smoke-plugin.js out/screens.json   # 在桩 API 上运行 Figma 插件，无需 Figma
```

```text
src/capture.rs            通过 DevTools Protocol 驱动 Chrome
assets/serialize.js       注入页面，构建图层树
assets/figma-plugin/      Figma 插件；LIB START/END 块与 Scripter 构建共享
claude-plugin/            Claude Code 插件与技能
.github/workflows/        每次推送运行 ci.yml，v* 标签运行 release.yml
```

---

## 📄 许可证

MIT。见 [LICENSE](../../LICENSE)。
