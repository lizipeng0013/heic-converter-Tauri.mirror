use heic_converter_lib::commands::conversion::{confirm_output_folder, ensure_output_confirmed};

#[test]
fn test_output_folder_confirmation_roundtrip() {
    let confirmed = "/home/user/Downloads".to_string();
    confirm_output_folder(confirmed.clone()).expect("确认应成功");

    // 已确认的目录通过
    assert!(ensure_output_confirmed(&confirmed).is_ok());
    // 未确认的任意目录被拒绝
    let err = ensure_output_confirmed("/etc/evil").unwrap_err();
    assert!(err.contains("确认"));
    // 错误信息不泄露路径
    assert!(!err.contains('/'));
}
