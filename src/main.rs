// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::time::Duration;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // 启动入场动画：窗口渲染就绪后再触发，确保 animate 能完整捕获属性从初始值
    // 到目标值的变化。复刻 PCL-CE 的 "Form Show" 序列（FormMain_Loaded 中触发）。
    // 用 SingleShot Timer 延迟一帧（50ms），等待 Slint 完成首帧渲染与 binding 评估。
    let ui_weak = ui.as_weak();
    let entrance_timer = slint::Timer::default();
    entrance_timer.start(
        slint::TimerMode::SingleShot,
        Duration::from_millis(50),
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_entrance_started(true);
            }
        },
    );
    // entrance_timer 在 ui.run() 阻塞期间一直保持引用，SingleShot 触发后自动停止，
    // 之后随 main 退出一起释放，不会泄漏。

    ui.run()?;
    Ok(())
}
