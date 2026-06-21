#![windows_subsystem = "windows"]

use gpui::{
    App, Bounds, Context, CursorStyle, MouseButton, SharedString, TitlebarOptions, Window,
    WindowBounds, WindowControlArea, WindowDecorations, WindowOptions, div, prelude::*, px, rgb,
    size,
};

struct TitleBar {
    title: SharedString,
    should_move: bool,
}

impl TitleBar {
    fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            should_move: false,
        }
    }

    fn on_mouse_down(
        &mut self,
        _event: &gpui::MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.should_move = true;
    }

    fn on_mouse_move(
        &mut self,
        _event: &gpui::MouseMoveEvent,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        if self.should_move {
            self.should_move = false;
            window.start_window_move();
        }
    }

    fn on_mouse_up(
        &mut self,
        _event: &gpui::MouseUpEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.should_move = false;
    }

    fn on_mouse_down_out(
        &mut self,
        _event: &gpui::MouseDownEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.should_move = false;
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(48.0))
            .w_full()
            .bg(rgb(0x1171d1))
            .cursor(CursorStyle::Arrow)
            .window_control_area(WindowControlArea::Drag)
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_down_out(cx.listener(Self::on_mouse_down_out))
            .child(
                div()
                    .px(px(16.0))
                    .text_color(rgb(0xffffff))
                    .text_size(px(14.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(self.title.clone()),
            )
    }
}

/// 主窗口
struct MainWindow {}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(cx.new(|_cx| TitleBar::new("PCL-rs")))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .text_size(px(16.0))
                    .text_color(rgb(0xcccccc))
                    .child("Welcome to PCL-rs"),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(550.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| MainWindow {}),
        )
        .unwrap();
        cx.activate(true);
    });
}
