#[non_exhaustive]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[non_exhaustive]
pub enum MouseEvent {
    Move {
        position: Position,
    },

    Down {
        button: MouseButton,
        position: Position,
        modifiers: Modifiers,
    },

    Up {
        button: MouseButton,
        position: Position,
        modifiers: Modifiers,
    },

    Scroll {
        dx: f32,
        dy: f32,
    },

    Enter,
    Leave,
}

pub struct Position {
    x: f64,
    y: f64,
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[non_exhaustive]
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

#[non_exhaustive]
pub enum Modifiers {
    SHIFT,
    CTRL,
    ALT,
    SUPER,
}

#[non_exhaustive]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,

    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,

    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,

    AltLeft,
    AltRight,

    Backspace,
    CapsLock,
    ContextMenu,

    ControlLeft,
    ControlRight,

    Enter,

    SuperLeft,
    SuperRight,

    ShiftLeft,
    ShiftRight,

    Space,
    Tab,

    Delete,
    End,
    Home,
    Insert,
    ScrollLock,
    Pause,

    PageDown,
    PageUp,

    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    NumLock,

    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadMultiply,
    NumpadSubtract,

    Escape,
    PrintScreen,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}
