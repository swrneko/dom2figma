//! Формат результата и вспомогательные функции: slug маршрута, хэш дерева для дедупликации.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub generated_at: String,
    pub tool: String,
    pub source: String,
    pub font: String,
    pub groups: Vec<GroupOut>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupOut {
    pub name: String,
    pub screens: Vec<Screen>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screen {
    /// Фактический маршрут после навигации.
    pub route: String,
    /// Запрошенный маршрут.
    pub requested: String,
    pub redirected: bool,
    /// Имя файла без расширения.
    pub name: String,
    pub w: f64,
    pub h: f64,
    pub tree: Value,
    /// `группа:маршрут` экрана с тем же содержимым, если он уже снят.
    pub same_as: Option<String>,
}

impl Output {
    pub fn total(&self) -> usize {
        self.groups.iter().map(|g| g.screens.len()).sum()
    }
    pub fn unique(&self) -> usize {
        self.groups.iter().flat_map(|g| &g.screens).filter(|s| s.same_as.is_none()).count()
    }
}

/// Имя файла для маршрута: `/orders/o1?x=1` → `orders_o1_x_1`, `/` → `home`.
pub fn slug(route: &str) -> String {
    let path = route.split('#').last().unwrap_or(route);
    let path = path.strip_prefix("http://").or_else(|| path.strip_prefix("https://")).unwrap_or(path);
    let mut out = String::new();
    let mut prev_sep = true;
    for ch in path.chars() {
        if ch.is_alphanumeric() || ch == '-' || ch == '.' {
            out.push(ch);
            prev_sep = false;
        } else if !prev_sep {
            out.push('_');
            prev_sep = true;
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() { "home".into() } else { out }
}

/// Стабильный хэш дерева экрана.
pub fn tree_hash(tree: &Value) -> String {
    let bytes = serde_json::to_vec(tree).unwrap_or_default();
    blake3::hash(&bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn slug_examples() {
        assert_eq!(slug("/"), "home");
        assert_eq!(slug(""), "home");
        assert_eq!(slug("/welcome"), "welcome");
        assert_eq!(slug("/orders/o1"), "orders_o1");
        assert_eq!(slug("/manager/client/c_alexander/orders"), "manager_client_c_alexander_orders");
        assert_eq!(slug("/search?q=шёлк&page=2"), "search_q_шёлк_page_2");
        assert_eq!(slug("https://example.com/a/b/"), "example.com_a_b");
        assert_eq!(slug("file:///x/y.html#/cart"), "cart");
    }

    #[test]
    fn tree_hash_is_stable_and_order_independent() {
        let a = json!({"type": "frame", "x": 1, "children": [{"type": "text", "text": "a"}]});
        let b = json!({"children": [{"text": "a", "type": "text"}], "x": 1, "type": "frame"});
        assert_eq!(tree_hash(&a), tree_hash(&b));
        let c = json!({"type": "frame", "x": 2, "children": []});
        assert_ne!(tree_hash(&a), tree_hash(&c));
    }

    #[test]
    fn output_counts() {
        let mk = |same: Option<&str>| Screen {
            route: "/".into(),
            requested: "/".into(),
            redirected: false,
            name: "home".into(),
            w: 390.0,
            h: 844.0,
            tree: json!({}),
            same_as: same.map(String::from),
        };
        let out = Output {
            generated_at: String::new(),
            tool: String::new(),
            source: String::new(),
            font: String::new(),
            groups: vec![GroupOut { name: "a".into(), screens: vec![mk(None), mk(None)] }, GroupOut { name: "b".into(), screens: vec![mk(Some("a:/"))] }],
        };
        assert_eq!(out.total(), 3);
        assert_eq!(out.unique(), 2);
    }
}
