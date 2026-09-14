// 海绵酱控制台 · 入口
// 隐藏 Windows 控制台窗口（release 模式）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    yunzai_app_lib::run();
}
