import { message } from '@tauri-apps/plugin-dialog';
import {error} from "@tauri-apps/plugin-log";

export const showErrorDialog = async (title: string, content: string) => {
    try {
        // Tauri 2 的 dialog API
        await message(content, {
            title: title,
            kind: 'error', // 类型: info, warning, error
        });
    } catch (e) {
        console.error("Tauri Dialog 调用失败，使用降级方案: ", e);
        // 如果原生弹窗失败，降级使用 alert
        alert(`${title}: ${content}`);
    }
};

// 简化的严重错误提示
export const alertSevere = (content: string) => {
    void error(`[前端严重错误] ${content}`);
    void showErrorDialog("发生错误", content);
};
