# 🦀 dgui (desktop GUI)

A pure Rust, renderer-agnostic, platform-agnostic reactive retained-mode GUI framework.

[![GitHub](https://img.shields.io/badge/github-repo-blue?logo=github)](https://github.com/sumeeth05/dgui)
[![Crates.io](https://img.shields.io/crates/v/dgui.svg?color=orange)](https://crates.io/crates/dgui)
[![Docs](https://img.shields.io/badge/docs-dgui-green)](https://docs.rs/dgui/latest/dgui/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/sumeeth05/dgui/blob/main/LICENSE)

> [!WARNING]
> ⚠️ Note: DGUI is currently in active development and not recommended for production use.

## Features

- 🦀 Built entirely in Rust.
- 🎨 Renderer agnostic — works with any graphics backend.
- 🌍 Platform agnostic — compatible with any windowing or event library.
- ⚡ Reactive signals automatically update the UI.
- 🎮 Built for desktop applications, tools, and games.
- 🎭 CSS-inspired styling and animations.
- 📦 Modular architecture with easy to use API.
- 🚀 Lightweight, fast, and highly customizable.
- 🔀 Native Node Graph Support.

### Example

```rust
let count = Signal::create(0);

let ui = Widget::panel(
    vec![
        Widget::text(&count, None),

        Widget::button(
            vec![Widget::text("+", None)],
            || {
                count.set(|value| *value += 1);
            },
            None,
        ),

        Widget::button(
            vec![Widget::text("-", None)],
            || {
                count.set(|value| *value -= 1);
            },
            None,
        ),
    ],
    Styles::default(),
);


//In Application Loop

let layout = Layout::new(ui);

let mut draw = layout.build();

match layout.flags(){
    Flags::SIGNALED => {
       draw = layout.build();
    },
    Flags::UNSIGNALED => {
        draw
    },
    _ => {}
}
```

## 📄 License

Licensed under **MIT**.
