use tauri::{command, AppHandle, Manager, tray::{MouseButton, MouseButtonState, TrayIconEvent}};

// --- 显示托盘 ---
#[command]
pub fn show_tray(app_handle: AppHandle) -> Result<(), String> {
    println!("show_tray 被调用");

    // 检查托盘是否已存在
    if app_handle.tray_by_id("main-tray").is_some() {
        println!("托盘已经存在");
        return Ok(());
    }

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
    let tray_result = tauri::tray::TrayIconBuilder::with_id("main-tray")
        .icon(app_handle.default_window_icon().unwrap().clone())
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
        // 注意on_tray_icon_event在Linux中暂不可用
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("左键单击托盘图标");
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                println!("左键双击托盘图标");
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("未处理的托盘事件: {:?}", event);
            }
        })
        .tooltip("HEIC Converter")
        .build(&app_handle);

    match tray_result {
        Ok(_) => {
            println!("托盘创建成功");
            Ok(())
        }
        Err(e) => {
            println!("托盘创建失败: {:?}", e);
            Err(format!("托盘创建失败: {:?}", e))
        }
    }
}