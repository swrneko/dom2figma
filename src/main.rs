mod assets;
mod capture;
mod config;
mod doctor;
mod output;
mod serve;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

/// Снимает отрисованный DOM веб-прототипа и переносит его в Figma редактируемыми слоями.
#[derive(Parser)]
#[command(name = "dom2figma", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Создать пример конфига dom2figma.toml в текущем каталоге
    Init {
        /// Перезаписать существующий файл
        #[arg(long)]
        force: bool,
    },
    /// Снять экраны по конфигу в JSON (+ PNG)
    Capture {
        /// Путь к конфигу
        #[arg(short, long, default_value = "dom2figma.toml")]
        config: PathBuf,
        /// Снимать только эти группы (можно повторять)
        #[arg(short, long = "group")]
        groups: Vec<String>,
        /// Снимать только эти маршруты (можно повторять)
        #[arg(short, long = "route")]
        routes: Vec<String>,
        /// Имя выходного JSON без расширения
        #[arg(long, default_value = "screens")]
        name: String,
        /// Не сохранять PNG
        #[arg(long)]
        no_png: bool,
        /// Путь к Chrome/Chromium
        #[arg(long, env = "CHROME")]
        chrome: Option<PathBuf>,
    },
    /// Записать Figma-плагин (manifest.json, code.js, ui.html) в каталог
    Plugin {
        #[arg(short, long, default_value = "figma-plugin")]
        dir: PathBuf,
    },
    /// Собрать скрипт для плагина Scripter (импорт из браузерной Figma)
    Scripter {
        /// URL JSON, который раздаёт `dom2figma serve`
        #[arg(long, default_value = "http://localhost:8787/screens.json")]
        url: String,
        /// Файл для записи; без него — вывод в stdout
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Раздавать JSON из каталога с CORS для Scripter
    Serve {
        #[arg(short, long, default_value = "out")]
        dir: PathBuf,
        #[arg(short, long, default_value_t = 8787)]
        port: u16,
    },
    /// Проверить окружение: Chrome и шрифт
    Doctor {
        #[arg(short, long, default_value = "dom2figma.toml")]
        config: PathBuf,
        #[arg(long, env = "CHROME")]
        chrome: Option<PathBuf>,
    },
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ошибка: {e:#}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Init { force } => {
            let p = Path::new("dom2figma.toml");
            anyhow::ensure!(force || !p.exists(), "dom2figma.toml уже существует, используйте --force");
            std::fs::write(p, config::EXAMPLE)?;
            println!("Создан {}. Поправьте source, root и routes под проект.", p.display());
        }
        Cmd::Capture { config, groups, routes, name, no_png, chrome } => {
            let cfg = config::Config::load(&config)?;
            let base = config.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new(".")).to_path_buf();
            let report = doctor::run(chrome.as_deref().or(cfg.chrome.as_deref()), &cfg.font);
            if let Err(e) = &report.font {
                eprintln!("предупреждение: {e}. Переносы строк в Figma могут не совпасть.");
            }
            let opts = capture::CaptureOptions { only_groups: groups, only_routes: routes, out_name: name, png: !no_png, chrome };
            let (file, out) = capture::run(&cfg, &base, &opts)?;
            let size_mb = std::fs::metadata(&file).map(|m| m.len() as f64 / 1e6).unwrap_or(0.0);
            println!("\nГотово: {} экранов ({} уникальных) -> {} ({size_mb:.1} MB)", out.total(), out.unique(), file.display());
            println!("Дальше: `dom2figma plugin` и импорт JSON в Figma, либо `dom2figma serve` + `dom2figma scripter`.");
        }
        Cmd::Plugin { dir } => {
            let files = assets::write_plugin(&dir)?;
            for f in files {
                println!("  {}", f.display());
            }
            println!("Figma → Plugins → Development → Import plugin from manifest… → {}", dir.join("manifest.json").display());
        }
        Cmd::Scripter { url, out } => {
            let script = assets::build_scripter(&url)?;
            match out {
                Some(p) => {
                    std::fs::write(&p, script).with_context(|| format!("не удалось записать {}", p.display()))?;
                    println!("Записан {}. Вставьте его в Scripter и нажмите Run.", p.display());
                }
                None => print!("{script}"),
            }
        }
        Cmd::Serve { dir, port } => serve::serve(&dir, port)?,
        Cmd::Doctor { config, chrome } => {
            let font = config::Config::load(&config).map(|c| c.font).unwrap_or_else(|_| "Inter".to_string());
            let chrome_cfg = config::Config::load(&config).ok().and_then(|c| c.chrome);
            let report = doctor::run(chrome.as_deref().or(chrome_cfg.as_deref()), &font);
            match &report.chrome {
                Ok(p) => println!("Chrome:  OK  {}", p.display()),
                Err(e) => println!("Chrome:  НЕТ  {e}"),
            }
            match &report.font {
                Ok(p) => println!("Шрифт:   OK  {font} ({p})"),
                Err(e) => println!("Шрифт:   НЕТ  {e}"),
            }
            if !report.ok() {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
