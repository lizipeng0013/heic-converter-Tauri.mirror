use heic_converter_lib::commands::conversion::{
    confirm_output_folder, ensure_output_confirmed, validate_convert_request,
};
use heic_converter_lib::utils::locale::{with_current, Locale};

#[test]
fn test_output_folder_confirmation_roundtrip() {
    let confirmed = "/home/user/Downloads".to_string();
    confirm_output_folder(confirmed.clone()).expect("确认应成功");

    // 已确认的目录通过
    assert!(ensure_output_confirmed(&confirmed, Locale::ZhHans).is_ok());
    // 未确认的任意目录被拒绝
    let err = ensure_output_confirmed("/etc/evil", Locale::ZhHans).unwrap_err();
    assert!(err.contains("确认"));
    // 错误信息不泄露路径
    assert!(!err.contains('/'));
}

#[test]
fn test_command_errors_render_in_english() {
    // 命令边界文案：En 环境触发拒绝显示英文（with_current 串行化并恢复原值）
    with_current(Locale::En, || {
        let err = confirm_output_folder(String::new()).unwrap_err();
        assert!(err.contains("Output folder cannot be empty"));
    });

    let err = ensure_output_confirmed("/anywhere", Locale::En).unwrap_err();
    assert!(err.contains("not confirmed"));
}

#[test]
fn test_convert_request_rejections_render_per_locale() {
    // 未选文件（ticket 05 AC：英文环境显示英文）
    let err = validate_convert_request(&[], "jpg", "/tmp/out", 90, Locale::En).unwrap_err();
    assert_eq!(err, "No files selected");
    let err = validate_convert_request(&[], "jpg", "/tmp/out", 90, Locale::ZhHans).unwrap_err();
    assert!(err.contains("没有选择任何文件"));
}
