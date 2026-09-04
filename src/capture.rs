//! Управление Chrome: навигация по маршрутам, внедрение сериализатора, снятие JSON и PNG.

use crate::assets::SERIALIZE_JS;
use crate::config::{Config, Group, Navigation};
use crate::output::{slug, tree_hash, GroupOut, Output, Screen};
use anyhow::{anyhow, Context, Result};
use headless_chrome::protocol::cdp::{Emulation, Page, Runtime};
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

const FIX_CSS_ID: &str = "__dom2figma_fix";

pub struct CaptureOptions {
    /// Ограничить группы по имени (пусто — все).
    pub only_groups: Vec<String>,
    /// Ограничить маршруты (пусто — все из конфига).
    pub only_routes: Vec<String>,
    /// Имя выходного JSON без расширения.
    pub out_name: String,
    pub png: bool,
    pub chrome: Option<PathBuf>,
}

pub fn run(cfg: &Config, base_dir: &Path, opts: &CaptureOptions) -> Result<(PathBuf, Output)> {
    let source = cfg.source_url(base_dir)?;
    let out_dir = base_dir.join(&cfg.out);
    std::fs::create_dir_all(&out_dir)?;

    let chrome = crate::doctor::find_chrome(opts.chrome.as_deref().or(cfg.chrome.as_deref())).map_err(|e| anyhow!(e))?;
    let args: Vec<&OsStr> = vec![OsStr::new("--allow-file-access-from-files"), OsStr::new("--font-render-hinting=none"), OsStr::new("--hide-scrollbars")];
    let launch = LaunchOptionsBuilder::default()
        .headless(true)
        .path(Some(chrome))
        .window_size(Some((cfg.viewport.width, cfg.viewport.height)))
        .args(args)
        .idle_browser_timeout(Duration::from_secs(3600))
        .build()
        .map_err(|e| anyhow!("{e}"))?;
    let browser = Browser::new(launch).context("не удалось запустить Chrome")?;
    let tab = browser.new_tab()?;
    tab.set_default_timeout(Duration::from_secs(60));

    let mut groups = cfg.effective_groups();
    if !opts.only_groups.is_empty() {
        groups.retain(|g| opts.only_groups.iter().any(|n| n == &g.name));
        anyhow::ensure!(!groups.is_empty(), "ни одна группа не совпала с --group");
    }

    let mut session = Session { tab, cfg, source: source.clone(), png: opts.png && cfg.png };
    session.open(&source.to_string())?;

    let mut output = Output {
        generated_at: chrono::Local::now().to_rfc3339(),
        tool: format!("dom2figma {}", env!("CARGO_PKG_VERSION")),
        source: cfg.source.clone(),
        font: cfg.font.clone(),
        groups: Vec::new(),
    };
    let mut seen_global: HashMap<String, String> = HashMap::new();

    for group in &groups {
        println!("\n=== {} ===", group.name);
        let routes: Vec<&String> = group.routes.iter().filter(|r| opts.only_routes.is_empty() || opts.only_routes.contains(*r)).collect();
        session.enter_group(group)?;
        let png_dir = out_dir.join("png").join(slug(&group.name));
        if session.png {
            std::fs::create_dir_all(&png_dir)?;
        }
        let mut entry = GroupOut { name: group.name.clone(), screens: Vec::new() };
        let mut seen_local: HashMap<String, String> = HashMap::new();

        for route in routes {
            let actual = session.navigate(route, group)?;
            let data = session.serialize()?;
            let Some(tree) = data.get("tree").filter(|t| !t.is_null()) else {
                println!("  {route} -> пусто");
                continue;
            };
            let key = tree_hash(tree);
            if let Some(prev) = seen_local.get(&key) {
                println!("  {route} -> дубликат {prev}");
                continue;
            }
            seen_local.insert(key.clone(), actual.clone());
            let redirected = &actual != route;
            let name = slug(&actual);
            if session.png {
                if let Err(e) = session.screenshot(&png_dir.join(format!("{name}.png"))) {
                    eprintln!("  png не снят: {e}");
                }
            }
            let shared = seen_global.get(&key).cloned();
            let w = data["w"].as_f64().unwrap_or(0.0);
            let h = data["h"].as_f64().unwrap_or(0.0);
            println!(
                "  {route}{}  {w}x{h}{}",
                if redirected { format!(" -> {actual}") } else { String::new() },
                shared.as_ref().map(|s| format!("  (= {s})")).unwrap_or_default()
            );
            if shared.is_none() {
                seen_global.insert(key, format!("{}:{}", group.name, actual));
            }
            entry.screens.push(Screen { route: actual, requested: route.clone(), redirected, name, w, h, tree: tree.clone(), same_as: shared });
        }
        output.groups.push(entry);
    }

    let out_file = out_dir.join(format!("{}.json", opts.out_name));
    std::fs::write(&out_file, serde_json::to_vec(&output)?)?;
    Ok((out_file, output))
}

struct Session<'a> {
    tab: Arc<Tab>,
    cfg: &'a Config,
    source: url::Url,
    png: bool,
}

impl<'a> Session<'a> {
    fn open(&mut self, url: &str) -> Result<()> {
        self.tab.navigate_to(url)?;
        self.tab.wait_until_navigated()?;
        self.tab.wait_for_element(&self.cfg.root).with_context(|| format!("корень `{}` не появился на странице", self.cfg.root))?;
        self.inject()?;
        self.settle(600)?;
        Ok(())
    }

    /// Внедряет сериализатор (идемпотентно).
    fn inject(&self) -> Result<()> {
        self.eval(SERIALIZE_JS)?;
        Ok(())
    }

    fn eval(&self, expr: &str) -> Result<Value> {
        let res = self.tab.call_method(Runtime::Evaluate {
            expression: expr.to_string(),
            object_group: None,
            include_command_line_api: Some(false),
            silent: Some(false),
            context_id: None,
            return_by_value: Some(true),
            generate_preview: Some(false),
            user_gesture: Some(true),
            await_promise: Some(true),
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        })?;
        if let Some(exc) = res.exception_details {
            return Err(anyhow!("JS: {}", exc.exception.and_then(|e| e.description).unwrap_or(exc.text)));
        }
        Ok(res.result.value.unwrap_or(Value::Null))
    }

    fn settle(&self, ms: u64) -> Result<()> {
        self.eval(&format!("window.__dom2figma.settle({ms})"))?;
        Ok(())
    }

    fn apply_fix_css(&self) -> Result<()> {
        let css = serde_json::to_string(&self.cfg.injected_css())?;
        self.eval(&format!("window.__dom2figma.setCss('{FIX_CSS_ID}', {css})"))?;
        Ok(())
    }

    fn enter_group(&mut self, group: &Group) -> Result<()> {
        if let Some(setup) = &group.setup {
            // Служебные панели должны быть видимы во время setup — снимаем заплатку, потом возвращаем.
            self.eval(&format!("window.__dom2figma.setCss('{FIX_CSS_ID}', '')"))?;
            self.settle(100)?;
            self.eval(setup).with_context(|| format!("setup группы «{}»", group.name))?;
            self.settle(500)?;
        }
        self.apply_fix_css()?;
        self.settle(150)?;
        Ok(())
    }

    /// Переходит на маршрут и возвращает фактический маршрут.
    fn navigate(&mut self, route: &str, group: &Group) -> Result<String> {
        match self.cfg.navigation {
            Navigation::Hash => {
                let r = serde_json::to_string(route)?;
                self.eval(&format!("location.hash = '#' + {r}"))?;
            }
            Navigation::Url => {
                let target = self.source.join(route.trim_start_matches('/')).with_context(|| format!("плохой маршрут {route}"))?;
                self.tab.navigate_to(target.as_str())?;
                self.tab.wait_until_navigated()?;
                self.tab.wait_for_element(&self.cfg.root)?;
                self.inject()?;
                if group.setup_each_route {
                    if let Some(setup) = &group.setup {
                        self.eval(setup)?;
                    }
                }
                self.apply_fix_css()?;
            }
            Navigation::Js => {
                self.eval(route)?;
            }
        }
        self.settle(self.cfg.settle_ms)?;
        let root = serde_json::to_string(&self.cfg.root)?;
        let scroll = serde_json::to_string(&self.cfg.scroll_container)?;
        self.eval(&format!("window.__dom2figma.fit({root}, {scroll}, {})", self.cfg.min_height))?;
        self.settle(150)?;
        Ok(match self.cfg.navigation {
            Navigation::Hash => self.eval("decodeURIComponent(location.hash.slice(1))")?.as_str().unwrap_or(route).to_string(),
            Navigation::Url => {
                let cur = self.tab.get_url();
                let base = self.source.as_str();
                match cur.strip_prefix(base.trim_end_matches('/')) {
                    Some(rest) if !rest.is_empty() => rest.to_string(),
                    _ => cur,
                }
            }
            Navigation::Js => route.to_string(),
        })
    }

    fn serialize(&self) -> Result<Value> {
        let root = serde_json::to_string(&self.cfg.root)?;
        let v = self.eval(&format!("window.__dom2figma.serialize({root})"))?;
        if let Some(e) = v.get("error").and_then(|e| e.as_str()) {
            return Err(anyhow!("{e}"));
        }
        Ok(v)
    }

    /// PNG корня целиком: временно расширяем viewport до высоты документа.
    fn screenshot(&self, path: &Path) -> Result<()> {
        let doc_h = self.eval("window.__dom2figma.docHeight()")?.as_f64().unwrap_or(0.0).ceil() as u32;
        let vp = self.cfg.viewport;
        let height = doc_h.max(vp.height).min(16000);
        self.tab.call_method(metrics(vp.width, height))?;
        let result = (|| -> Result<()> {
            let el = self.tab.find_element(&self.cfg.root)?;
            let png = el.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
            std::fs::write(path, png)?;
            Ok(())
        })();
        self.tab.call_method(metrics(vp.width, vp.height))?;
        result
    }
}

fn metrics(width: u32, height: u32) -> Emulation::SetDeviceMetricsOverride {
    Emulation::SetDeviceMetricsOverride {
        width,
        height,
        device_scale_factor: 1.0,
        mobile: false,
        scale: None,
        screen_width: None,
        screen_height: None,
        position_x: None,
        position_y: None,
        dont_set_visible_size: None,
        screen_orientation: None,
        viewport: None,
        display_feature: None,
        device_posture: None,
    }
}
