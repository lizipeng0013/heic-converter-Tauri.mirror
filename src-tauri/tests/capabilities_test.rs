use serde_json::Value;

fn load_permissions() -> Vec<Value> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/capabilities/default.json");
    let raw = std::fs::read_to_string(path).expect("读取能力配置失败");
    let value: Value = serde_json::from_str(&raw).expect("解析能力配置失败");
    value["permissions"]
        .as_array()
        .expect("permissions 应为数组")
        .clone()
}

fn permission_ids(perms: &[Value]) -> Vec<String> {
    perms
        .iter()
        .map(|p| match p {
            Value::String(s) => s.clone(),
            Value::Object(o) => o["identifier"].as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        })
        .collect()
}

#[test]
fn test_unused_broad_permissions_removed() {
    let perms = load_permissions();
    let ids = permission_ids(&perms);

    // 审计发现的未使用宽权限必须保持移除状态（防回归）
    assert!(
        !ids.contains(&"fs:allow-write-file".to_string()),
        "fs:allow-write-file（$HOME/** 写权限）不应被授予"
    );
    assert!(
        !ids.contains(&"opener:allow-open-path".to_string()),
        "opener:allow-open-path（无 scope 打开路径）不应被授予"
    );
    assert!(
        !ids.contains(&"notification:default".to_string()),
        "notification:default（全量通知权限）不应被授予"
    );
}

#[test]
fn test_stat_permission_has_scope() {
    let perms = load_permissions();
    let stat = perms
        .iter()
        .find(|p| p.is_object() && p["identifier"] == "fs:allow-stat")
        .expect("fs:allow-stat 应存在");

    let allow = stat["allow"]
        .as_array()
        .expect("fs:allow-stat 必须带精确 path scope");
    assert!(!allow.is_empty(), "fs:allow-stat 的 scope 不应为空");

    // 不允许出现无差别全量 HOME 读取
    for entry in allow {
        let path = entry["path"].as_str().unwrap_or_default();
        assert_ne!(path, "$HOME/**", "stat scope 不应覆盖整个 $HOME");
    }
}
