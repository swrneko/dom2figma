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

**렌더링된 웹 프로토타입을 편집 가능한 Figma 레이어로. 수십 개의 화면, 모든 앱 상태, 명령 하나로.**  
*Headless Chrome이 DOM을 캡처하고, Figma 플러그인이 프레임·텍스트·벡터로 재구성합니다.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 소개

**dom2figma**는 HTML 소스가 아니라 브라우저가 실제로 렌더링한 결과를 캡처해 Figma에 진짜 레이어로 옮기는 명령줄 도구입니다. 채우기·선·모서리 반경·그림자가 있는 프레임, 폰트와 정렬이 있는 텍스트 레이어, 벡터로 변환된 SVG 아이콘.

html.to.design 같은 플러그인과 달리 **프로토타입 전체**를 위해 만들어졌습니다. 라우트 목록을 순회하고, 짧은 JavaScript 훅으로 앱 상태(페르소나, 역할, 테마)를 전환하고, 동일한 화면을 중복 제거해, 한 번의 클릭으로 가져올 수 있는 JSON 하나를 만듭니다. 프로토타입이 바뀌면 명령을 다시 실행하면 됩니다.

**Linux와 macOS용 단일 정적 바이너리**로 배포됩니다. 외부 의존성은 Chrome 또는 Chromium 하나뿐입니다.

---

## 🏗️ 동작 방식

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   실행      ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ source 열기            │──────────►│ 플러그인 / Scripter  │
  │ root          │             │ 각 그룹마다:           │           │  • 그룹별 섹션       │
  │ routes        │             │   setup JS 실행        │           │  • 화면별 프레임     │
  │ groups[]      │             │   각 라우트마다:       │           │  • 채우기, 선,       │
  │ fix_css       │             │     이동 후 대기       │           │    반경, 그림자      │
  │ font          │             │     serialize.js 주입  │           │  • 폰트 있는 텍스트  │
  └───────────────┘             │     DOM → 트리         │           │  • SVG → 벡터        │
                                │     해시 → 중복 제거   │           └──────────────────────┘
                                │     요소 PNG           │
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **캡처.** `dom2figma capture`가 Chrome을 실행하고 소스를 열어 각 그룹의 `setup`을 실행한 뒤, 모든 라우트를 방문해 보이는 DOM을 직렬화합니다: 지오메트리, 계산된 스타일, 텍스트 런, SVG. 동일한 화면은 트리 해시로 제거됩니다.
2. **가져오기.** 내장 Figma 플러그인(`dom2figma plugin`)이 JSON을 읽어 Plugin API로 섹션·프레임·텍스트 노드를 만듭니다. 브라우저 Figma에서는 Scripter 방식(`dom2figma serve` + `dom2figma scripter`)을 사용합니다.

---

## ✨ 기능

*   **한 번에 프로토타입 전체:** 라우트 × 상태, 동일 화면 자동 중복 제거.
*   **앱 상태를 그룹으로:** 페르소나·역할·테마를 그룹별 JavaScript `setup` 훅으로 전환.
*   **세 가지 내비게이션 모드:** `#/route` 라우터용 `hash`, 실제 경로용 `url`, 라우터 없이 클릭으로 움직이는 목업용 `js`.
*   **전체 높이 화면:** 내부 스크롤 컨테이너를 펼쳐 뷰포트에 잘리지 않고 화면 전체를 캡처.
*   **정확한 텍스트:** Figma가 사용할 것과 같은 폰트로 렌더링하므로 줄바꿈이 일치. 한 줄 텍스트는 자동 너비.
*   **편집 가능한 결과:** CSS 클래스에서 온 레이어 이름, 변별 선, 모서리 반경, 선형 그라디언트, 외부/내부 그림자, 클리핑, 불투명도, 텍스트 대소문자와 장식.
*   **참조용 PNG**를 JSON 옆에 저장해 빠른 시각 확인.
*   **Claude Code 플러그인** 포함: 에이전트가 목업을 분석하고, 설정을 작성하고, 캡처·검증하고, 가져오기 절차를 안내.

---

## 📦 01. 설치

[Releases](https://github.com/swrneko/dom2figma/releases)에서 바이너리(`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`)를 내려받아 `PATH`에 두거나, 소스에서 빌드합니다:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**요구 사항:**
*   **Chrome 또는 Chromium.** 표준 위치에서 자동 감지. `--chrome`, 환경 변수 `CHROME`, 설정의 `chrome = "..."`로 지정 가능.
*   **캡처에 쓰는 폰트**(기본 `Inter`)가 로컬에 설치되어 있어야 합니다. Linux: TTF를 `~/.local/share/fonts/`에 넣고 `fc-cache -f`. macOS: 서체 관리자로 설치. [폰트](#-04-폰트) 참고.

---

## 🚀 02. 빠른 시작

```bash
dom2figma init                 # 주석이 달린 dom2figma.toml 생성
$EDITOR dom2figma.toml         # source, root, routes 설정
dom2figma doctor               # Chrome과 폰트가 준비됐는지 확인
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, 플러그인을 실행하고 `out/screens.json`을 선택. 화면은 현재 페이지에 그룹별 섹션으로 배치됩니다.

**브라우저 Figma(또는 데스크톱 앱이 없는 Linux):**

```bash
dom2figma serve                              # out/*.json을 CORS와 함께 http://localhost:8787에서 제공
dom2figma scripter --out scripter.js         # Scripter 플러그인용 스크립트
```

Figma에서: Plugins → Scripter → New script → `scripter.js` 붙여넣기 → Run.

부분 실행: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. 설정

```toml
source = "prototype.html"              # HTML 파일 또는 http(s) URL
root = ".device"                       # 화면 루트 선택자, 기본 body
font = "Inter"                         # 캡처 폰트, 로컬과 Figma 양쪽에 있어야 함
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # 내부 스크롤을 전체 높이로 펼침
min_height = 844
fix_css = ".panel{display:none!important}"   # 캡처 전에 주입하는 CSS
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| 필드 | 기본값 | 용도 |
|---|---|---|
| `source` | 필수 | HTML 경로 또는 `http(s)://` URL |
| `root` | `body` | 캡처 루트의 CSS 선택자 |
| `font`, `force_font` | `Inter`, `true` | 렌더링 폰트; `force_font`는 루트 아래 모든 요소에 적용 |
| `viewport` | `1400×1000` | 브라우저 창 크기; 모바일 레이아웃은 `390×844` |
| `navigation` | `hash` | `hash`는 `location.hash` 설정, `url`은 `source + route`로 이동, `js`는 라우트를 JavaScript로 실행 |
| `fix_css` | 없음 | CSS 패치: 도구 패널 숨기기, 기기 프레임 제거, 페이지가 세로로 늘어나게 하기 |
| `scroll_container`, `min_height` | 없음, `0` | 내부 스크롤 컨테이너를 펼쳐 화면 전체를 캡처 |
| `settle_ms` | `400` | 이동 후 대기 시간 |
| `routes`, `groups` | | 라우트와 상태 그룹; 그룹은 자체 `routes`, JS `setup`, 이동마다 setup을 다시 실행하는 `setup_each_route`를 가짐 |
| `out`, `png`, `chrome` | `out`, `true`, 자동 | 출력 디렉터리, PNG 저장 여부, Chrome 경로 |

주석이 모두 달린 예시는 [`examples/dom2figma.toml`](../../examples/dom2figma.toml)에 있습니다.

---

## 🔤 04. 폰트

Chrome은 시스템에 설치된 폰트로 렌더링하고, Figma는 자체 폰트로 대체합니다. 둘이 다르면 줄바꿈이 어긋납니다. 그래서 dom2figma는 `font`에 지정한 폰트로 렌더링하고 Figma에도 같은 폰트가 있다고 가정합니다. **Inter와 모든 Google Fonts는 Figma에서 항상 사용할 수 있습니다.** `dom2figma doctor`가 로컬 쪽을 확인합니다.

---

## 📐 05. 변환되는 것

**변환됨:** 단색·선형 그라디언트·변별 선·반경·그림자·클리핑이 있는 사각 박스; 크기·굵기·자간·정렬·대소문자·장식이 있는 텍스트; 인라인 SVG 아이콘; 입력 값과 플레이스홀더; 텍스트 가상 요소. 보이지 않는 것은 건너뜀: `display:none`, 불투명도 0, 투명한 글자색.

**근사 또는 건너뜀:** 외부 URL 래스터 이미지(data URL은 가능), 방사형 그라디언트, 필터, 블렌드 모드, canvas와 video, transform(경계 상자만). **Auto layout은 생성되지 않음:** 레이어는 절대 위치입니다. 결과는 디자이너가 이어서 작업할 정확한 구조 복제본이며, 완성된 디자인 시스템이 아닙니다.

---

## 🤖 06. Claude Code 플러그인

저장소에는 `dom2figma` 스킬이 담긴 Claude Code 플러그인이 포함되어 있습니다. 에이전트가 목업(루트, 내비게이션, 상태)을 분석하고, 설정을 작성하고, 테스트 라우트를 캡처하고, PNG를 확인하고, 전체 캡처를 실행한 뒤 가져오기 안내를 돌려줍니다.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

그다음 "이 프로토타입을 Figma로 옮겨 줘"라고 요청하거나 `/dom2figma:dom2figma`를 호출하면 됩니다. 바이너리가 없으면 스킬이 `cargo install --git`으로 설치합니다.

---

## 🛠️ 07. 개발

```bash
cargo test                      # 단위 테스트
cargo test -- --ignored         # tests/fixtures/simple.html에 대해 실제 Chrome으로 E2E
node scripts/smoke-plugin.js out/screens.json   # 스텁 API로 Figma 플러그인 실행, Figma 불필요
```

```text
src/capture.rs            DevTools Protocol로 Chrome 제어
assets/serialize.js       페이지에 주입되어 레이어 트리 구성
assets/figma-plugin/      Figma 플러그인; LIB START/END 블록은 Scripter 빌드와 공유
claude-plugin/            Claude Code 플러그인과 스킬
.github/workflows/        푸시마다 ci.yml, v* 태그에 release.yml
```

---

## 📄 라이선스

MIT. [LICENSE](../../LICENSE) 참고.
