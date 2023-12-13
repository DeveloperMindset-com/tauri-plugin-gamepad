use gilrs::{Axis, Button};

pub const BTN_UNKNOWN: u16 = 0;

pub const BTN_SOUTH: u16 = 1;
pub const BTN_EAST: u16 = 2;
pub const BTN_C: u16 = 3;
pub const BTN_NORTH: u16 = 4;
pub const BTN_WEST: u16 = 5;
pub const BTN_Z: u16 = 6;
pub const BTN_LT: u16 = 7;
pub const BTN_RT: u16 = 8;
pub const BTN_LT2: u16 = 9;
pub const BTN_RT2: u16 = 10;
pub const BTN_SELECT: u16 = 11;
pub const BTN_START: u16 = 12;
pub const BTN_MODE: u16 = 13;
pub const BTN_LTHUMB: u16 = 14;
pub const BTN_RTHUMB: u16 = 15;

pub const BTN_DPAD_UP: u16 = 16;
pub const BTN_DPAD_DOWN: u16 = 17;
pub const BTN_DPAD_LEFT: u16 = 18;
pub const BTN_DPAD_RIGHT: u16 = 19;

pub const AXIS_UNKNOWN: u16 = 0;

pub const AXIS_LSTICKX: u16 = 1;
pub const AXIS_LSTICKY: u16 = 2;
pub const AXIS_LEFTZ: u16 = 3;
pub const AXIS_RSTICKX: u16 = 4;
pub const AXIS_RSTICKY: u16 = 5;
pub const AXIS_RIGHTZ: u16 = 6;
pub const AXIS_DPADX: u16 = 7;
pub const AXIS_DPADY: u16 = 8;

pub fn button_from_u16(id: u16) -> Button {
    match id {
        BTN_SOUTH => Button::South,
        BTN_EAST => Button::East,
        BTN_C => Button::C,
        BTN_NORTH => Button::North,
        BTN_WEST => Button::West,
        BTN_Z => Button::Z,
        BTN_LT => Button::LeftTrigger,
        BTN_RT => Button::RightTrigger,
        BTN_LT2 => Button::LeftTrigger2,
        BTN_RT2 => Button::RightTrigger2,
        BTN_SELECT => Button::Select,
        BTN_START => Button::Start,
        BTN_MODE => Button::Mode,
        BTN_LTHUMB => Button::LeftThumb,
        BTN_RTHUMB => Button::RightThumb,
        BTN_DPAD_UP => Button::DPadUp,
        BTN_DPAD_DOWN => Button::DPadDown,
        BTN_DPAD_LEFT => Button::DPadLeft,
        BTN_DPAD_RIGHT => Button::DPadRight,
        BTN_UNKNOWN => Button::Unknown,
        _ => Button::Unknown,
    }
}

pub fn axis_from_u16(id: u16) -> Axis {
    match id {
        AXIS_LSTICKX => Axis::LeftStickX,
        AXIS_LSTICKY => Axis::LeftStickY,
        AXIS_LEFTZ => Axis::LeftZ,
        AXIS_RSTICKX => Axis::RightStickX,
        AXIS_RSTICKY => Axis::RightStickY,
        AXIS_RIGHTZ => Axis::RightZ,
        AXIS_DPADX => Axis::DPadX,
        AXIS_DPADY => Axis::DPadY,
        AXIS_UNKNOWN => Axis::Unknown,
        _ => Axis::Unknown,
    }
}
