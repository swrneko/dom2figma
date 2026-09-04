//! Встроенные ресурсы: JS-сериализатор, Figma-плагин, сборка варианта для Scripter.

use anyhow::{Context, Result};
use std::path::Path;

pub const SERIALIZE_JS: &str = include_str!("../assets/serialize.js");
pub const PLUGIN_MANIFEST: &str = include_str!("../assets/figma-plugin/manifest.json");
pub const PLUGIN_CODE: &str = include_str!("../assets/figma-plugin/code.js");
pub const PLUGIN_UI: &str = include_str!("../assets/figma-plugin/ui.html");
const SCRIPTER_HEADER: &str = include_str!("../assets/scripter-header.js");
const SCRIPTER_FOOTER: &str = include_str!("../assets/scripter-footer.js");

const LIB_START: &str = "// --- LIB START ---";
const LIB_END: &str = "// --- LIB END ---";

/// Записывает файлы Figma-плагина в каталог.
pub fn write_plugin(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    std::fs::create_dir_all(dir).with_context(|| format!("не удалось создать {}", dir.display()))?;
    let files = [("manifest.json", PLUGIN_MANIFEST), ("code.js", PLUGIN_CODE), ("ui.html", PLUGIN_UI)];
    let mut written = Vec::new();
    for (name, content) in files {
        let p = dir.join(name);
        std::fs::write(&p, content).with_context(|| format!("не удалось записать {}", p.display()))?;
        written.push(p);
    }
    Ok(written)
}

/// Собирает скрипт для плагина Scripter: общая библиотека из code.js плюс загрузка JSON по URL.
pub fn build_scripter(url: &str) -> Result<String> {
    let lib = PLUGIN_CODE
        .split(LIB_START)
        .nth(1)
        .and_then(|s| s.split(LIB_END).next())
        .context("в code.js нет маркеров LIB START/END")?;
    let safe_url = url.replace('\\', "\\\\").replace('\'', "\\'");
    Ok(format!("{}{}{}", SCRIPTER_HEADER.replace("__URL__", &safe_url), lib, SCRIPTER_FOOTER))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripter_contains_lib_and_url() {
        let s = build_scripter("http://localhost:8787/core.json").unwrap();
        assert!(s.contains("const URL = 'http://localhost:8787/core.json'"));
        assert!(s.contains("async function importAll("));
        assert!(!s.contains("figma.showUI"), "UI-часть плагина не должна попадать в Scripter");
        assert!(!s.contains("figma.ui.onmessage"));
        assert!(s.contains("const n = await importAll(data, OPTS)"));
    }

    #[test]
    fn scripter_escapes_quotes_in_url() {
        let s = build_scripter("http://x/a'b.json").unwrap();
        assert!(s.contains("http://x/a\\'b.json"));
    }

    #[test]
    fn plugin_code_supports_groups_and_legacy_personas() {
        assert!(PLUGIN_CODE.contains("data.groups || data.personas"));
        assert!(PLUGIN_UI.contains("data.groups || data.personas"));
    }

    #[test]
    fn serializer_is_idempotent_guarded() {
        assert!(SERIALIZE_JS.contains("if (window.__dom2figma) return;"));
        assert!(SERIALIZE_JS.contains("window.__dom2figma = {"));
    }

    #[test]
    fn write_plugin_creates_three_files() {
        let dir = std::env::temp_dir().join(format!("dom2figma-plugin-{}", std::process::id()));
        let files = write_plugin(&dir).unwrap();
        assert_eq!(files.len(), 3);
        assert!(dir.join("manifest.json").exists());
        let manifest: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.join("manifest.json")).unwrap()).unwrap();
        assert_eq!(manifest["main"], "code.js");
        std::fs::remove_dir_all(dir).ok();
    }
}
