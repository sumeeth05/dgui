pub enum Event {
    Pointer(PointerEvent),
    Keyboard(KeyboardEvent),
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum PointerEvent {
    Move { x: f32, y: f32 },

    Down { button: MouseButton, x: f32, y: f32 },

    Up { button: MouseButton, x: f32, y: f32 },

    Scroll { dx: f32, dy: f32 },

    Enter,
    Leave,
}

pub enum KeyboardEvent {
    KeyDown {
        key: Key,
        modifiers: Modifiers,
        repeat: bool,
    },

    KeyUp {
        key: Key,
        modifiers: Modifiers,
    },

    Text(String),
}

pub enum Key {}

pub enum Modifiers {}
