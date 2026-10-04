use heic_converter_lib::utils::locale::{current, message, with_current, Locale, CATALOG};

#[test]
fn catalog_all_locales_present_and_non_empty() {
    assert!(!CATALOG.is_empty());
    for entry in CATALOG {
        assert!(
            !entry.en.is_empty() && !entry.zh_hans.is_empty() && !entry.zh_hant.is_empty(),
            "empty translation for key {}",
            entry.key
        );
    }
}

#[test]
fn locale_parse_and_display_round_trip() {
    for (s, locale) in [
        ("en", Locale::En),
        ("zh-Hans", Locale::ZhHans),
        ("zh-Hant", Locale::ZhHant),
    ] {
        assert_eq!(Locale::parse(s), Some(locale));
        assert_eq!(locale.as_str(), s);
    }
    // 非内部键的裸系统标签不直接可解析（归一化在前端完成）
    assert_eq!(Locale::parse("zh-CN"), None);
    assert_eq!(Locale::parse(""), None);
}

#[test]
fn message_renders_per_locale_and_unknown_key_returns_key() {
    let key = "notification.conversion_done_body";
    let en = message(key, Locale::En);
    let hans = message(key, Locale::ZhHans);
    let hant = message(key, Locale::ZhHant);
    assert_ne!(en, hans);
    assert_ne!(hant, hans);
    assert_ne!(en, hant);
    // 未知键返回键本身（便于定位缺译）
    assert_eq!(message("nope.unknown_key", Locale::En), "nope.unknown_key");
}

#[test]
fn set_current_round_trip() {
    // with_current：全局锁内设置并自动恢复；恢复值在锁内捕获，锁外断言无竞争
    let (observed, restored) = with_current(Locale::ZhHant, || {
        assert_eq!(current(), Locale::ZhHant);
        current()
    });
    assert_eq!(observed, Locale::ZhHant);
    assert_eq!(restored, Locale::En);
}
