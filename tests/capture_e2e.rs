//! Сквозной тест: реальный Chrome снимает фикстуру. Требует установленный Chrome/Chromium,
//! поэтому помечен `#[ignore]`: `cargo test -- --ignored`.

use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_dom2figma"))
}

#[test]
#[ignore]
fn captures_fixture_with_real_chrome() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let work = std::env::temp_dir().join(format!("dom2figma-e2e-{}", std::process::id()));
    std::fs::create_dir_all(&work).unwrap();
    std::fs::copy(fixtures.join("simple.html"), work.join("simple.html")).unwrap();
    std::fs::write(
        work.join("dom2figma.toml"),
        r##"
source = "simple.html"
root = "#app"
force_font = false
routes = ["/", "/second", "/"]
"##,
    )
    .unwrap();

    let out = Command::new(bin()).current_dir(&work).args(["capture", "--name", "t"]).output().expect("запуск dom2figma");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "stdout:\n{stdout}\nstderr:\n{stderr}");

    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(work.join("out/t.json")).unwrap()).unwrap();
    let screens = json["groups"][0]["screens"].as_array().unwrap();
    // Третий маршрут повторяет первый — отсеивается как дубликат.
    assert_eq!(screens.len(), 2, "{stdout}");
    assert_eq!(screens[0]["route"], "/");
    assert_eq!(screens[1]["route"], "/second");
    assert_eq!(screens[0]["w"], 320.0);

    let all = serde_json::to_string(&screens[0]["tree"]).unwrap();
    assert!(all.contains("\"Заголовок\""), "текст шапки");
    assert!(all.contains("\"Введите имя\""), "плейсхолдер поля");
    assert!(all.contains("\"NEW\""), "псевдоэлемент ::after");
    assert!(all.contains("\"type\":\"svg\""), "svg-иконка");
    assert!(!all.contains("скрытый текст"), "прозрачный текст не попадает");
    assert!(all.contains("\"shadows\":[{"), "тень карточки");
    assert!(work.join("out/png/default/home.png").exists());
    assert!(work.join("out/png/default/second.png").exists());
    std::fs::remove_dir_all(&work).ok();
}
