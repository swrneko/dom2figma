//! Проверка окружения: Chrome и шрифт.

use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Report {
    pub chrome: Result<PathBuf, String>,
    pub font: Result<String, String>,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.chrome.is_ok() && self.font.is_ok()
    }
}

pub fn run(chrome: Option<&Path>, font: &str) -> Report {
    Report { chrome: find_chrome(chrome), font: check_font(font) }
}

pub fn find_chrome(explicit: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return if p.exists() { Ok(p.to_path_buf()) } else { Err(format!("указанный Chrome не найден: {}", p.display())) };
    }
    headless_chrome::browser::default_executable().map_err(|e| format!("{e}. Укажите путь флагом --chrome, переменной CHROME или в конфиге `chrome = \"...\"`"))
}

/// Проверяет, установлен ли шрифт: fontconfig на Linux, каталоги шрифтов на macOS.
pub fn check_font(font: &str) -> Result<String, String> {
    if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap_or_default();
        let dirs = [format!("{home}/Library/Fonts"), "/Library/Fonts".into(), "/System/Library/Fonts".into(), "/System/Library/Fonts/Supplemental".into()];
        let needle = font.to_lowercase().replace(' ', "");
        for d in dirs {
            if let Ok(rd) = std::fs::read_dir(&d) {
                for e in rd.flatten() {
                    let n = e.file_name().to_string_lossy().to_lowercase().replace(' ', "");
                    if n.starts_with(&needle) && (n.ends_with(".ttf") || n.ends_with(".otf") || n.ends_with(".ttc")) {
                        return Ok(e.path().display().to_string());
                    }
                }
            }
        }
        return Err(format!("шрифт «{font}» не найден в ~/Library/Fonts и /Library/Fonts. Установите его через Font Book"));
    }
    match Command::new("fc-list").arg(":").arg("family").output() {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout);
            let hit = text.lines().find(|l| l.split(',').any(|f| f.trim().eq_ignore_ascii_case(font)));
            match hit {
                Some(l) => Ok(l.trim().to_string()),
                None => Err(format!("шрифт «{font}» не найден (fc-list). Положите TTF в ~/.local/share/fonts/ и выполните fc-cache -f")),
            }
        }
        Err(_) => Err("fc-list недоступен, проверить шрифт не удалось (установите fontconfig)".into()),
    }
}
