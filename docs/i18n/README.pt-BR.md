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

**Transforme qualquer protótipo web renderizado em camadas editáveis do Figma. Dezenas de telas, todos os estados do app, um único comando.**  
*O Headless Chrome captura o DOM e um plugin do Figma o reconstrói como frames, texto e vetores.*

[English](../../README.md) · [Русский](README.ru.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt-BR.md)

</div>

---

## 📖 Sobre

**dom2figma** é uma ferramenta de linha de comando que captura o que o navegador realmente renderiza, não o HTML de origem, e leva isso para o Figma como camadas reais: frames com preenchimentos, bordas, raios e sombras, camadas de texto com fontes e alinhamento, ícones SVG como vetores.

Diferente de plugins como o html.to.design, ela foi feita para **protótipos inteiros**: percorre uma lista de rotas, alterna estados da aplicação (personas, papéis, temas) por meio de pequenos hooks em JavaScript, remove telas idênticas e gera um único JSON importável com um clique. Mudou o protótipo, rode o comando de novo.

Distribuída como **um único binário estático para Linux e macOS**. A única dependência externa é o Chrome ou Chromium.

---

## 🏗️ Como funciona

```text
   dom2figma.toml                 headless Chrome                        Figma
  ┌───────────────┐   iniciar   ┌────────────────────────┐   JSON    ┌──────────────────────┐
  │ source        ├────────────►│ abrir source           │──────────►│ plugin ou Scripter   │
  │ root          │             │ para cada grupo:       │           │  • seção por grupo   │
  │ routes        │             │   executar setup JS    │           │  • frame por tela    │
  │ groups[]      │             │   para cada rota:      │           │  • preenchimentos,   │
  │ fix_css       │             │     navegar e aguardar │           │    bordas, raios,    │
  │ font          │             │     injetar serialize  │           │    sombras           │
  └───────────────┘             │     DOM → árvore       │           │  • texto com fontes  │
                                │     hash → dedupe      │           │  • SVG → vetores     │
                                │     PNG do elemento    │           └──────────────────────┘
                                └────────────────────────┘
                                 out/screens.json + out/png/
```

1. **Captura.** `dom2figma capture` inicia o Chrome, abre a origem, executa o `setup` de cada grupo, visita cada rota e serializa o DOM visível: geometria, estilos computados, trechos de texto, SVG. Telas idênticas são descartadas pelo hash da árvore.
2. **Importação.** O plugin do Figma incluído (`dom2figma plugin`) lê o JSON e constrói seções, frames e nós de texto pela Plugin API. Para o Figma no navegador há uma variante com Scripter (`dom2figma serve` + `dom2figma scripter`).

---

## ✨ Recursos

*   **Protótipo inteiro em uma passada:** rotas × estados, com deduplicação automática de telas idênticas.
*   **Estados do app como grupos:** personas, papéis ou temas são alternados por um hook `setup` em JavaScript por grupo.
*   **Três modos de navegação:** `hash` para roteadores `#/route`, `url` para caminhos reais, `js` para mockups guiados por clique, sem roteador.
*   **Telas em altura total:** contêineres com rolagem interna são expandidos para capturar a tela inteira, sem cortar no viewport.
*   **Texto fiel:** a página é renderizada com a mesma fonte que o Figma usará, então as quebras de linha coincidem. Textos de uma linha têm largura automática.
*   **Saída editável:** camadas nomeadas pelas classes CSS, bordas por lado, raios de canto, gradientes lineares, sombras externas e internas, recorte, opacidade, caixa e decoração do texto.
*   **PNGs de referência** ao lado do JSON para conferência visual rápida.
*   **Plugin para Claude Code** incluído: o agente inspeciona o mockup, escreve a configuração, captura, verifica e entrega os passos de importação.

---

## 📦 01. Instalação

Baixe um binário em [Releases](https://github.com/swrneko/dom2figma/releases) (`dom2figma-linux-x86_64`, `dom2figma-macos-arm64`, `dom2figma-macos-x86_64`) e coloque no seu `PATH`, ou compile a partir do código:

```bash
cargo install --git https://github.com/swrneko/dom2figma
```

**Requisitos:**
*   **Chrome ou Chromium.** Detectado automaticamente nos locais padrão; pode ser indicado com `--chrome`, a variável de ambiente `CHROME` ou `chrome = "..."` na configuração.
*   **A fonte de captura** (padrão `Inter`) instalada localmente. Linux: copie os TTF para `~/.local/share/fonts/` e rode `fc-cache -f`. macOS: instale pelo Catálogo de Fontes. Veja [Fontes](#-04-fontes).

---

## 🚀 02. Início rápido

```bash
dom2figma init                 # cria dom2figma.toml com comentários
$EDITOR dom2figma.toml         # defina source, root, routes
dom2figma doctor               # Chrome e fonte presentes?
dom2figma capture              # → out/screens.json + out/png/
dom2figma plugin               # → figma-plugin/{manifest.json,code.js,ui.html}
```

**Figma Desktop:** Plugins → Development → *Import plugin from manifest…* → `figma-plugin/manifest.json`, execute o plugin e escolha `out/screens.json`. As telas são dispostas em uma seção por grupo na página atual.

**Figma no navegador (ou Linux sem o app desktop):**

```bash
dom2figma serve                              # serve out/*.json com CORS em http://localhost:8787
dom2figma scripter --out scripter.js         # script para o plugin Scripter
```

No Figma: Plugins → Scripter → New script → cole `scripter.js` → Run.

Execuções parciais: `dom2figma capture --group "Guest" --route "/cart" --no-png --name quick`.

---

## ⚙️ 03. Configuração

```toml
source = "prototype.html"              # arquivo HTML ou URL http(s)
root = ".device"                       # seletor da raiz da tela, padrão body
font = "Inter"                         # fonte de captura, deve existir localmente e no Figma
navigation = "hash"                    # hash | url | js
scroll_container = ".device__scroll"   # expandir a rolagem interna para a altura total
min_height = 844
fix_css = ".panel{display:none!important}"   # CSS injetado antes da captura
routes = ["/", "/catalog", "/cart"]

[[groups]]
name = "Guest"
setup = "document.querySelector('#persona-guest').click()"

[[groups]]
name = "Manager"
setup = "localStorage.setItem('role', 'manager'); location.reload()"
routes = ["/manager", "/manager/clients"]
```

| Campo | Padrão | Finalidade |
|---|---|---|
| `source` | obrigatório | caminho do HTML ou URL `http(s)://` |
| `root` | `body` | seletor CSS da raiz capturada |
| `font`, `force_font` | `Inter`, `true` | fonte de renderização; `force_font` aplica a todos os elementos sob root |
| `viewport` | `1400×1000` | tamanho da janela do navegador; use `390×844` para layouts móveis |
| `navigation` | `hash` | `hash` define `location.hash`; `url` navega para `source + route`; `js` avalia a rota como JavaScript |
| `fix_css` | vazio | patch CSS: ocultar painéis, remover molduras de dispositivo, deixar a página crescer em altura |
| `scroll_container`, `min_height` | nenhum, `0` | expandir um contêiner de rolagem interna para capturar telas inteiras |
| `settle_ms` | `400` | pausa após a navegação |
| `routes`, `groups` | | rotas e grupos de estado; um grupo tem suas próprias `routes`, um `setup` JS e `setup_each_route` para repetir o setup após cada navegação |
| `out`, `png`, `chrome` | `out`, `true`, auto | diretório de saída, salvar PNGs, caminho do Chrome |

Um exemplo totalmente comentado está em [`examples/dom2figma.toml`](../../examples/dom2figma.toml).

---

## 🔤 04. Fontes

O Chrome renderiza com a fonte instalada na máquina, enquanto o Figma substitui pela sua. Se forem diferentes, as quebras de linha se deslocam. Por isso o dom2figma renderiza com a fonte indicada em `font` e espera a mesma fonte no Figma. **Inter e todas as Google Fonts estão sempre disponíveis no Figma.** `dom2figma doctor` verifica o lado local.

---

## 📐 05. O que é transferido

**Transferido:** caixas retangulares com cores sólidas, gradientes lineares, bordas por lado, raios, sombras e recorte; texto com tamanho, peso, espaçamento, alinhamento, caixa e decoração; ícones SVG inline; valores e placeholders de campos; pseudoelementos de texto. O invisível é ignorado: `display:none`, opacidade zero, cor de texto transparente.

**Aproximado ou ignorado:** imagens raster por URL externa (data URLs funcionam), gradientes radiais, filtros, modos de mesclagem, canvas e vídeo, transformações (apenas a caixa delimitadora). **Nenhum auto layout** é gerado: as camadas ficam posicionadas de forma absoluta. O resultado é uma cópia estrutural exata para o designer trabalhar em cima, não um design system pronto.

---

## 🤖 06. Plugin para Claude Code

O repositório traz um plugin do Claude Code com a skill `dom2figma`. O agente inspeciona o mockup (raiz, navegação, estados), escreve a configuração, captura uma rota de teste, confere os PNGs, roda a captura completa e devolve as instruções de importação.

```
/plugin marketplace add swrneko/dom2figma
/plugin install dom2figma@dom2figma
```

Depois basta pedir *"leve este protótipo para o Figma"* ou chamar `/dom2figma:dom2figma`. Se o binário estiver ausente, a skill o instala com `cargo install --git`.

---

## 🛠️ 07. Desenvolvimento

```bash
cargo test                      # testes unitários
cargo test -- --ignored         # ponta a ponta com Chrome real em tests/fixtures/simple.html
node scripts/smoke-plugin.js out/screens.json   # roda o plugin do Figma contra uma API simulada, sem Figma
```

```text
src/capture.rs            controla o Chrome via DevTools Protocol
assets/serialize.js       injetado na página, constrói a árvore de camadas
assets/figma-plugin/      plugin do Figma; o bloco LIB START/END é compartilhado com a variante Scripter
claude-plugin/            plugin e skill do Claude Code
.github/workflows/        ci.yml a cada push, release.yml em tags v*
```

---

## 📄 Licença

MIT. Veja [LICENSE](../../LICENSE).
