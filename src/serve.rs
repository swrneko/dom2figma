//! Локальный сервер для варианта импорта через Scripter: раздаёт JSON из каталога с CORS.

use anyhow::{Context, Result};
use std::path::Path;
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub fn serve(dir: &Path, port: u16) -> Result<()> {
    let addr = format!("127.0.0.1:{port}");
    let server = Server::http(&addr).map_err(|e| anyhow::anyhow!("не удалось слушать {addr}: {e}"))?;
    println!("Раздаю {} на http://localhost:{port}/  (Ctrl+C для остановки)", dir.display());
    for f in list_json(dir)? {
        println!("  http://localhost:{port}/{f}");
    }
    for req in server.incoming_requests() {
        let path = req.url().split('?').next().unwrap_or("/").trim_start_matches('/').to_string();
        let name = Path::new(&path).file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let cors = [
            Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap(),
            Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap(),
        ];
        if *req.method() == Method::Options {
            let mut r = Response::empty(StatusCode(204));
            for h in cors.iter().cloned() { r.add_header(h); }
            let _ = req.respond(r);
            continue;
        }
        let file = dir.join(&name);
        if !name.ends_with(".json") || !file.is_file() {
            let mut r = Response::from_string("not found").with_status_code(404);
            for h in cors.iter().cloned() { r.add_header(h); }
            let _ = req.respond(r);
            continue;
        }
        match std::fs::File::open(&file) {
            Ok(f) => {
                let mut r = Response::from_file(f);
                for h in cors.iter().cloned() { r.add_header(h); }
                r.add_header(Header::from_bytes("Content-Type", "application/json").unwrap());
                println!("{} GET {name}", chrono::Local::now().format("%H:%M:%S"));
                let _ = req.respond(r);
            }
            Err(e) => {
                let _ = req.respond(Response::from_string(e.to_string()).with_status_code(500));
            }
        }
    }
    Ok(())
}

fn list_json(dir: &Path) -> Result<Vec<String>> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .with_context(|| format!("каталог не найден: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_str().map(String::from))
        .filter(|n| n.ends_with(".json"))
        .collect();
    v.sort();
    Ok(v)
}
