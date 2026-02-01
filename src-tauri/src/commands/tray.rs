use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{command, AppHandle, Manager, WebviewWindow};
use std::sync::Mutex;
use std::fs as std_fs;

// 全局托盘状态
static TRAY_VISIBLE: AtomicBool = AtomicBool::new(false);

// 托盘句柄
static TRAY_HANDLE: Mutex<Option<tauri::tray::TrayIcon>> = Mutex::new(None);

// 窗口引用
static WINDOW_HANDLE: Mutex<Option<WebviewWindow>> = Mutex::new(None);

// --- 显示托盘 ---
#[command]
pub fn show_tray(app_handle: AppHandle) -> Result<(), String> {
    println!("show_tray 被调用");

    // 保存窗口引用
    if let Some(window) = app_handle.get_webview_window("main") {
        let mut win_handle = WINDOW_HANDLE.lock().unwrap();
        *win_handle = Some(window);
    }

    // 先检查状态，不持有锁
    if TRAY_VISIBLE.load(Ordering::SeqCst) {
        println!("托盘已经可见");
        return Ok(());
    }

    // 读取图标文件 - 尝试多个路径
    let icon_path_result = std::env::current_dir()
        .ok()
        .and_then(|p| {
            // 检查当前目录是否已经在 src-tauri 下
            let path_str = p.to_string_lossy();
            let icon_path = if path_str.ends_with("src-tauri") {
                p.join("icons/icon.png")
            } else {
                p.join("src-tauri/icons/icon.png")
            };
            println!("尝试读取图标: {:?}", icon_path);
            if icon_path.exists() {
                Some((icon_path.clone(), std_fs::read(&icon_path).ok()))
            } else {
                println!("图标文件不存在: {:?}", icon_path);
                None
            }
        });

    let icon_bytes: Vec<u8> = icon_path_result
        .and_then(|(_, bytes)| bytes)
        .ok_or_else(|| {
            println!("无法读取图标文件");
            "无法找到图标文件".to_string()
        })?;

    println!("图标大小: {} bytes", icon_bytes.len());

    // 解码 PNG 数据
    let decoded_image = image::load_from_memory(&icon_bytes)
        .map_err(|e| format!("无法解码图标: {:?}", e))?;

    // 调整大小到 32x32
    let resized_image = decoded_image.resize_exact(
        32,
        32,
        image::imageops::FilterType::Lanczos3
    );

    // 转换为 RGBA 格式
    let rgba_image = resized_image.to_rgba8();
    let rgba_bytes = rgba_image.as_raw().to_vec();

    println!("RGBA 数据大小: {} bytes", rgba_bytes.len());

    // 创建托盘菜单
    let menu = tauri::menu::Menu::new(&app_handle)
        .map_err(|e| format!("创建菜单失败: {:?}", e))?;
    let show_item = tauri::menu::MenuItem::with_id(&app_handle, "show", "显示窗口", true, None::<String>)
        .map_err(|e| format!("创建菜单项失败: {:?}", e))?;
    let quit_item = tauri::menu::MenuItem::with_id(&app_handle, "quit", "退出", true, None::<String>)
        .map_err(|e| format!("创建菜单项失败: {:?}", e))?;
    menu.append(&show_item)
        .map_err(|e| format!("添加菜单项失败: {:?}", e))?;
    menu.append(&quit_item)
        .map_err(|e| format!("添加菜单项失败: {:?}", e))?;

    // 创建托盘
    let tray_result = tauri::tray::TrayIconBuilder::new()
        .icon(tauri::image::Image::new_owned(rgba_bytes, 32, 32))
        .show_menu_on_left_click(false)
        .menu(&menu)
        .on_menu_event(|app_handle, event| {
            println!("托盘菜单事件: {:?}", event.id);
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .tooltip("HEIC Converter")
        .build(&app_handle);

    match tray_result {
        Ok(tray) => {
            println!("托盘创建成功");
            // 持有锁时更新状态
            let mut tray_guard = TRAY_HANDLE.lock().unwrap();
            *tray_guard = Some(tray);
            TRAY_VISIBLE.store(true, Ordering::SeqCst);
            Ok(())
        }
        Err(e) => {
            println!("托盘创建失败: {:?}", e);
            Err(format!("托盘创建失败: {:?}", e))
        }
    }
}

// --- 隐藏托盘 ---
#[command]
pub fn hide_tray() -> Result<(), String> {
    let mut tray_guard = TRAY_HANDLE.lock().unwrap();
    if let Some(_tray) = tray_guard.take() {
        // Tauri 2 中，TrayIcon 会自动清理，不需要手动 remove
    }
    TRAY_VISIBLE.store(false, Ordering::SeqCst);
    Ok(())
}