use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref TEMPLATES: RwLock<HashMap<String, String>> = RwLock::new({
        let mut m = HashMap::new();
        m.insert("not_found".to_string(), "指定されたリソースが見つかりませんでした".to_string());
        m.insert("permission_denied".to_string(), "権限がありません".to_string());
        m.insert("success".to_string(), "処理が正常に完了しました".to_string());
        m.insert("cancelled".to_string(), "操作がキャンセルされました".to_string());
        m
    });
}

pub fn add_template(key: &str, value: &str) -> Result<(), String> {
    match TEMPLATES.write() {
        Ok(mut templates) => {
            templates.insert(key.to_string(), value.to_string());
            Ok(())
        }
        Err(_) => Err("テンプレート更新中にエラーが発生しました".to_string()),
    }
}

pub fn get_template(key: &str) -> Option<String> {
    match TEMPLATES.read() {
        Ok(templates) => templates.get(key).cloned(),
        Err(_) => None,
    }
}

pub fn remove_template(key: &str) -> bool {
    match TEMPLATES.write() {
        Ok(mut templates) => templates.remove(key).is_some(),
        Err(_) => false,
    }
}