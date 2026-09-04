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

**レンダリング済みの Web プロトタイプを、編集可能な Figma レイヤーに。数十画面、すべてのアプリ状態を、コマンド一つで。**  
*Headless Chrome が DOM を取得し、Figma プラグインがフレーム・テキスト・ベクターとして再構築します。*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 概要

**dom2figma** は、HTML ソースではなくブラウザが実際に描画した結果を取得し、それを本物のレイヤーとして Figma に移すコマンドラインツールです。塗り・線・角丸・シャドウ付きのフレーム、フォントと整列を持つテキストレイヤー、ベクターとしての SVG アイコン。

html.to.design のようなプラグインと違い、**プロトタイプ全体**を対象に設計されています。ルート一覧を巡回し、小さな JavaScript フックでアプリの状態（ペルソナ、ロール、テーマ）を切り替え、同一画面を重複排除し、ワンクリックで取り込める JSON を一つ生成します。プロトタイプを変えたら、もう一度コマンドを実行するだけ。

**Linux と macOS 向けの単一スタティックバイナリ**として配布。外部依存は Chrome または Chromium のみです。

---

## 🏗️ 仕組み

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   起動      ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ source を開く          │──────────►│ プラグイン / Scripter│
  │ root          │             │ 各グループ:            │           │  • グループごとに    │
  │ routes        │             │   setup JS を実行      │           │    セクション        │
  │ groups[]      │             │   各ルート:            │           │  • 画面ごとにフレーム│
  │ fix_css       │             │     遷移して待機       │           │  • 塗り・線・角丸・  │
  │ font          │             │     serialize.js 注入  │           │    シャドウ          │
  └───────────────┘             │     DOM → ツリー       │           │  • フォント付テキスト│
                                │     ハッシュ → 重複排除│           │  • SVG → ベクター    │
                                │     要素 PNG           │           └──────────────────────┘
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **キャプチャ。** `dom2figma capture` が Chrome を起動し、ソースを開き、各グループの `setup` を実行し、すべてのルートを訪れて可視 DOM をシリアライズします: ジオメトリ、計算済みスタイル、テキスト行、SVG。同一画面はツリーのハッシュで除外されます。
2. **インポート。** 同梱の Figma プラグイン（`dom2figma plugin`）が JSON を読み、Plugin API でセクション・フレーム・テキストノードを構築します。ブラウザ版 Figma には Scripter 経由の方法があります（`dom2figma serve` + `dom2figma scripter`）。

---

## ✨ 特徴

*   **プロトタイプ全体を一回で:** ルート × 状態、同一画面は自動で重複排除。
*   **アプリ状態をグループとして:** ペルソナ・ロール・テーマをグループごとの JavaScript `setup` フックで切り替え。
*   **3 つのナビゲーションモード:** `#/route` ルーター向けの `hash`、実パス向けの `url`、ルーターのないクリック駆動モックアップ向けの `js`。
*   **全高の画面:** 内部スクロールコンテナを展開し、ビューポートで切れずに画面全体を取得。
*   **忠実なテキスト:** Figma が使うものと同じフォントで描画するため、改行位置が一致。単一行テキストは自動幅。
*   **編集可能な出力:** CSS クラス由来のレイヤー名、辺ごとの線、角丸、線形グラデーション、ドロップ/インナーシャドウ、クリッピング、不透明度、テキストのケースと装飾。
*   **参照用 PNG** を JSON の隣に出力し、素早く目視確認。
*   **Claude Code プラグイン**を同梱: エージェントがモックアップを調査し、設定を書き、取得・検証し、インポート手順を提示。

---

## 📦 01. インストール

[Releases](https://github.com/swrneko/dom2figma/releases) からバイナリ（`dom2figma-linux-x86_64`、`dom2figma-macos-arm64`、`dom2figma-macos-x86_64`）をダウンロードして `PATH` に置くか、ソースからビルドします:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**要件:**
*   **Chrome または Chromium。** 標準の場所から自動検出。`--chrome`、環境変数 `CHROME`、設定の `chrome = "..."` で指定可能。
*   **取得に使うフォント**（既定は `Inter`）がローカルにインストール済みであること。Linux: TTF を `~/.local/share/fonts/` に置き `fc-cache -f`。macOS: Font Book でインストール。[フォント](#-04-フォント)を参照。

---

## 🚀 02. クイックスタート

```bash
dom2figma init                 # コメント付き dom2figma.toml を生成
$EDITOR dom2figma.toml         # source、root、routes を設定
dom2figma doctor               # Chrome とフォントは揃っている？
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`、プラグインを実行して `out/screens.json` を選択。画面は現在のページにグループごとのセクションとして配置されます。

**ブラウザ版 Figma（またはデスクトップアプリのない Linux）:**

```bash
dom2figma serve                              # out/*.json を CORS 付きで http://localhost:8787 に配信
dom2figma scripter --out scripter.js         # Scripter プラグイン用スクリプト
```

Figma で: Plugins → Scripter → New script → `scripter.js` を貼り付け → Run。

部分実行: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`。

---

## ⚙️ 03. 設定

```toml
source = "prototype.html"              # HTML ファイルまたは http(s) URL
root = ".device"                       # 画面ルートのセレクタ、既定は body
font = "Inter"                         # 取得フォント。ローカルと Figma の両方に必要
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # 内部スクロールを全高に展開
min_height = 844
fix_css = ".panel{display:none!important}"   # 取得前に注入する CSS
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| フィールド | 既定値 | 用途 |
|---|---|---|
| `source` | 必須 | HTML のパスまたは `http(s)://` URL |
| `root` | `body` | 取得ルートの CSS セレクタ |
| `font`, `force_font` | `Inter`, `true` | 描画フォント。`force_font` はルート配下の全要素に適用 |
| `viewport` | `1400×1000` | ブラウザウィンドウサイズ。モバイルは `390×844` |
| `navigation` | `hash` | `hash` は `location.hash` を設定、`url` は `source + route` へ遷移、`js` はルートを JavaScript として評価 |
| `fix_css` | 空 | CSS パッチ: ツールパネルを隠す、端末フレームを消す、ページを縦に伸ばす |
| `scroll_container`, `min_height` | なし, `0` | 内部スクロールコンテナを展開して画面全体を取得 |
| `settle_ms` | `400` | 遷移後の待機 |
| `routes`, `groups` | | ルートと状態グループ。グループは独自の `routes`、JS `setup`、遷移ごとに setup を再実行する `setup_each_route` を持つ |
| `out`, `png`, `chrome` | `out`, `true`, 自動 | 出力ディレクトリ、PNG 保存、Chrome パス |

コメント付きの完全な例は [`examples/dom2figma.toml`](../../examples/dom2figma.toml) にあります。

---

## 🔤 04. フォント

Chrome はマシンにインストール済みのフォントで描画し、Figma は自身のフォントで置き換えます。両者が異なると改行がずれます。そのため dom2figma は `font` で指定したフォントで描画し、Figma にも同じフォントがあることを前提とします。**Inter とすべての Google Fonts は Figma で常に利用可能です。** `dom2figma doctor` がローカル側を確認します。

---

## 📐 05. 変換されるもの

**変換される:** 単色・線形グラデーション・辺ごとの線・角丸・シャドウ・クリッピング付きの矩形ボックス。サイズ・ウェイト・字間・整列・ケース・装飾付きのテキスト。インライン SVG アイコン。入力欄の値とプレースホルダー。テキスト疑似要素。不可視の要素はスキップ: `display:none`、不透明度ゼロ、透明な文字色。

**近似またはスキップ:** 外部 URL のラスター画像（data URL は可）、放射グラデーション、フィルター、ブレンドモード、canvas と video、transform（バウンディングボックスのみ）。**Auto layout は生成されません:** レイヤーは絶対配置です。結果はデザイナーが手を入れるための正確な構造コピーであり、完成したデザインシステムではありません。

---

## 🤖 06. Claude Code プラグイン

リポジトリには `dom2figma` スキルを含む Claude Code プラグインが同梱されています。エージェントがモックアップ（ルート、ナビゲーション、状態）を調べ、設定を書き、テストルートを取得し、PNG を確認し、完全な取得を実行してインポート手順を返します。

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

あとは「このプロトタイプを Figma に移して」と頼むか、`/dom2figma:dom2figma` を呼び出すだけ。バイナリがなければスキルが `cargo install --git` でインストールします。

---

## 🛠️ 07. 開発

```bash
cargo test                      # ユニットテスト
cargo test -- --ignored         # tests/fixtures/simple.html に対する実 Chrome での E2E
node scripts/smoke-plugin.js out/screens.json   # スタブ API で Figma プラグインを実行、Figma 不要
```

```text
src/capture.rs            DevTools Protocol で Chrome を制御
assets/serialize.js       ページに注入され、レイヤーツリーを構築
assets/figma-plugin/      Figma プラグイン。LIB START/END ブロックは Scripter ビルドと共有
claude-plugin/            Claude Code プラグインとスキル
.github/workflows/        push ごとに ci.yml、v* タグで release.yml
```

---

## 📄 ライセンス

MIT。[LICENSE](../../LICENSE) を参照。
