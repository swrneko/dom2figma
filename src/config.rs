//! Конфиг проекта `dom2figma.toml`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Путь к HTML-файлу или URL (http/https).
    pub source: String,
    /// CSS-селектор корня экрана. По умолчанию `body`.
    #[serde(default = "d_root")]
    pub root: String,
    /// Шрифт, которым снимается страница и который подставит Figma.
    #[serde(default = "d_font")]
    pub font: String,
    /// Принудительно применять `font` ко всем элементам корня.
    #[serde(default = "d_true")]
    pub force_font: bool,
    #[serde(default)]
    pub viewport: Viewport,
    /// CSS, внедряемый перед съёмкой (скрыть служебные панели, убрать рамки).
    #[serde(default)]
    pub fix_css: String,
    /// Селектор внутреннего скролл-контейнера, который нужно раскрыть на всю высоту.
    #[serde(default)]
    pub scroll_container: Option<String>,
    /// Минимальная высота корня в px (0 — не трогать).
    #[serde(default)]
    pub min_height: u32,
    /// Пауза после навигации, мс.
    #[serde(default = "d_settle")]
    pub settle_ms: u64,
    #[serde(default)]
    pub navigation: Navigation,
    /// Каталог результатов.
    #[serde(default = "d_out")]
    pub out: PathBuf,
    /// Сохранять PNG каждого экрана.
    #[serde(default = "d_true")]
    pub png: bool,
    /// Путь к Chrome/Chromium; иначе автопоиск.
    #[serde(default)]
    pub chrome: Option<PathBuf>,
    /// Маршруты по умолчанию для всех групп.
    #[serde(default)]
    pub routes: Vec<String>,
    /// Группы состояний (персоны, роли, темы). Пусто — одна группа `default`.
    #[serde(default)]
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self { width: 1400, height: 1000 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Navigation {
    /// `location.hash = '#' + route`
    #[default]
    Hash,
    /// Полная навигация на `source + route`.
    Url,
    /// Маршрут — JS-выражение, которое выполняется в странице.
    Js,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub name: String,
    /// JS, переключающий состояние (клик по персоне, установка localStorage и т.п.).
    #[serde(default)]
    pub setup: Option<String>,
    /// Повторять `setup` после каждой навигации (нужно при `navigation = "url"`, если состояние не сохраняется).
    #[serde(default)]
    pub setup_each_route: bool,
    /// Маршруты группы; пусто — берутся общие `routes`.
    #[serde(default)]
    pub routes: Vec<String>,
}

fn d_root() -> String {
    "body".into()
}
fn d_font() -> String {
    "Inter".into()
}
fn d_true() -> bool {
    true
}
fn d_settle() -> u64 {
    400
}
fn d_out() -> PathBuf {
    PathBuf::from("out")
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| format!("не удалось прочитать {}", path.display()))?;
        Self::parse(&text).with_context(|| format!("ошибка в {}", path.display()))
    }

    pub fn parse(text: &str) -> Result<Self> {
        let cfg: Config = toml::from_str(text)?;
        cfg.validate()?;
        Ok(cfg)
    }

    fn validate(&self) -> Result<()> {
        anyhow::ensure!(!self.source.trim().is_empty(), "`source` пуст");
        let has_routes = !self.routes.is_empty() || self.groups.iter().any(|g| !g.routes.is_empty());
        anyhow::ensure!(has_routes, "не задано ни одного маршрута: заполните `routes` или `groups[].routes`");
        for g in &self.groups {
            anyhow::ensure!(!g.name.trim().is_empty(), "у группы пустое имя");
        }
        Ok(())
    }

    /// Группы с подставленными маршрутами. Без групп — одна `default`.
    pub fn effective_groups(&self) -> Vec<Group> {
        if self.groups.is_empty() {
            return vec![Group { name: "default".into(), setup: None, setup_each_route: false, routes: self.routes.clone() }];
        }
        self.groups
            .iter()
            .map(|g| Group { routes: if g.routes.is_empty() { self.routes.clone() } else { g.routes.clone() }, ..g.clone() })
            .collect()
    }

    /// `source` как URL: http(s) остаётся, путь превращается в file://.
    pub fn source_url(&self, base_dir: &Path) -> Result<url::Url> {
        if self.source.starts_with("http://") || self.source.starts_with("https://") {
            return Ok(url::Url::parse(&self.source)?);
        }
        let p = base_dir.join(&self.source);
        let abs = p.canonicalize().with_context(|| format!("файл не найден: {}", p.display()))?;
        url::Url::from_file_path(&abs).map_err(|_| anyhow::anyhow!("не удалось построить file:// URL для {}", abs.display()))
    }

    /// CSS, внедряемый перед съёмкой: пользовательский плюс принудительный шрифт.
    pub fn injected_css(&self) -> String {
        let mut css = self.fix_css.clone();
        if self.force_font {
            css.push_str(&format!("\n{root},{root} *{{font-family:'{font}',sans-serif!important}}", root = self.root, font = self.font));
        }
        css
    }
}

pub const EXAMPLE: &str = include_str!("../examples/dom2figma.toml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_config_gets_defaults() {
        let cfg = Config::parse("source = \"index.html\"\nroutes = [\"/\"]").unwrap();
        assert_eq!(cfg.root, "body");
        assert_eq!(cfg.font, "Inter");
        assert_eq!(cfg.viewport.width, 1400);
        assert_eq!(cfg.navigation, Navigation::Hash);
        assert!(cfg.png);
        let groups = cfg.effective_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "default");
        assert_eq!(groups[0].routes, vec!["/"]);
    }

    #[test]
    fn groups_inherit_common_routes() {
        let cfg = Config::parse(
            r#"
source = "x.html"
routes = ["/a", "/b"]
[[groups]]
name = "Гость"
[[groups]]
name = "Менеджер"
routes = ["/m"]
"#,
        )
        .unwrap();
        let g = cfg.effective_groups();
        assert_eq!(g[0].routes, vec!["/a", "/b"]);
        assert_eq!(g[1].routes, vec!["/m"]);
    }

    #[test]
    fn rejects_config_without_routes() {
        assert!(Config::parse("source = \"x.html\"").is_err());
    }

    #[test]
    fn rejects_unknown_fields() {
        assert!(Config::parse("source = \"x.html\"\nroutes = [\"/\"]\nfoo = 1").is_err());
    }

    #[test]
    fn injected_css_forces_font() {
        let cfg = Config::parse("source = \"x.html\"\nroutes = [\"/\"]\nroot = \".app\"\nfix_css = \".panel{display:none}\"").unwrap();
        let css = cfg.injected_css();
        assert!(css.contains(".panel{display:none}"));
        assert!(css.contains(".app,.app *{font-family:'Inter'"));
    }

    #[test]
    fn example_config_is_valid() {
        let cfg = Config::parse(EXAMPLE).unwrap();
        assert!(!cfg.effective_groups().is_empty());
    }

    #[test]
    fn source_url_handles_http_and_files() {
        let cfg = Config::parse("source = \"https://example.com/app/\"\nroutes = [\"/\"]").unwrap();
        assert_eq!(cfg.source_url(Path::new(".")).unwrap().as_str(), "https://example.com/app/");
        let dir = std::env::temp_dir();
        let file = dir.join("dom2figma-test.html");
        std::fs::write(&file, "<html></html>").unwrap();
        let cfg = Config::parse("source = \"dom2figma-test.html\"\nroutes = [\"/\"]").unwrap();
        assert!(cfg.source_url(&dir).unwrap().as_str().starts_with("file://"));
    }
}
