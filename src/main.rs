#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::time::Duration;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();
    let entrance_timer = slint::Timer::default();
    entrance_timer.start(
        slint::TimerMode::SingleShot,
        Duration::from_millis(50), // 这里延迟 50ms，防止动画帧被吞
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_entrance_started(true); // 播放入场动画
            }
        },
    );

    ui.run()?;
    Ok(())
}
