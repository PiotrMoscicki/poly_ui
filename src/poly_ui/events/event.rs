// deps
use enum_map::Enum;
use nalgebra::Point2;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Enum, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyboardKey {
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Exclaim,
    Quotedbl,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Quote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,

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

    Delete,
    CapsLock,

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

    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,

    KpDivide,
    KpMultiply,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0,
    KpPeriod,
    Application,
    Power,
    KpEquals,

    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    KpComma,
    KpEqualsAs400,
    AltErase,
    Sysreq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    Kp00,
    Kp000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpTab,
    KpBackspace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpLess,
    KpGreater,
    KpAmpersand,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpColon,
    KpHash,
    KpSpace,
    KpAt,
    KpExclam,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClear,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpDecimal,
    KpHexadecimal,
    LCtrl,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    AcSearch,
    AcHome,
    AcBack,
    AcForward,
    AcStop,
    AcRefresh,
    AcBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
    Eject,
    Sleep,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Enum, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Enum, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct KeyPressEvent {
    key: KeyboardKey,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct KeyReleaseEvent {
    key: KeyboardKey,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MousePressEvent {
    button: MouseButton,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseReleaseEvent {
    button: MouseButton,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseMoveEvent {
    pos: Point2<u32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    KeyPressEvent(KeyPressEvent),
    KeyReleaseEvent(KeyReleaseEvent),
    MousePressEvent(MousePressEvent),
    MouseReleaseEvent(MouseReleaseEvent),
    MouseMoveEvent(MouseMoveEvent),
}
