use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::controller::{Axis, Button};

#[derive(Debug)]
pub enum ControllerType{
	Keyboard,
	Gamepad
}

pub struct ControllerSettings{
	pub dead_zone: i16,
	pub left_trigger_activation_threshold: i16,
	pub right_trigger_activation_threshold: i16,
}

impl ControllerSettings{
	pub fn new() -> ControllerSettings{
		ControllerSettings{
			dead_zone: 2000,
			left_trigger_activation_threshold: 10000,
			right_trigger_activation_threshold: 10000,
		}
	}
}

#[derive(Debug)]
pub struct InputState{
	pub device: ControllerType,
	pub left_x_pos: i16,
	pub left_y_pos: i16,
	pub right_x_pos: i16,
	pub right_y_pos: i16,
	pub trig_left_pos: i16,
	pub trig_right_pos: i16,
	pub left_shoulder: bool,
	pub right_shoulder: bool,
	pub dpad_up: bool,
	pub dpad_down: bool,
	pub dpad_left: bool,
	pub dpad_right: bool,
	pub btn_left: bool,
	pub btn_right: bool,
	pub btn_up: bool,
	pub btn_down: bool,
	pub btn_start: bool,
	pub btn_back: bool,
	pub left_stick: bool,
	pub right_stick: bool,
	pub shutdown: bool,
}
impl InputState{
	pub fn new() -> InputState{
		InputState{
			device: ControllerType::Keyboard,
			left_x_pos: 0,
			left_y_pos: 0,
			right_x_pos: 0,
			right_y_pos: 0,
			trig_left_pos: 0,
			trig_right_pos: 0,
			left_shoulder: false,
			right_shoulder: false,
			dpad_up: false,
			dpad_down: false,
			dpad_left: false,
			dpad_right: false,
			btn_left: false,
			btn_right: false,
			btn_up: false,
			btn_down: false,
			btn_start: false,
			btn_back: false,
			left_stick: false,
			right_stick: false,
			shutdown: false
		}
	}
}

pub fn get_player_intent_vector(input: &InputState) -> Option<f32>{
	let pi = std::f32::consts::PI;
	if input.left_y_pos != 0 || input.left_x_pos != 0 {
		let theta = (-1.0 * input.left_y_pos as f32).atan2(input.left_x_pos as f32);
		return Some(theta);
	}
	match (&input.dpad_left, &input.dpad_right, &input.dpad_up, &input.dpad_down){
		(false, false, false, false) =>  None,
		(false, true, false, false)  =>  Some(0.0),
		(false, true, true, false)   =>  Some(0.25*pi),
		(false, false, true, false)  =>  Some(0.5*pi),
		(true, false, true, false)   =>  Some(0.75*pi),
		(true, false, false, false)  =>  Some(pi),
		(true, false, false, true)   =>  Some(1.25*pi),
		(false, false, false, true)  =>  Some(1.5*pi),
		(false, true, false, true)   =>  Some(1.75*pi),
		_                            =>  None
	}
}

pub fn read_input_event(input: &mut InputState, controller_settings: &ControllerSettings, event: &Event){
	match event {
		Event::KeyDown {keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => {input.shutdown = true;},
		Event::KeyDown {keycode: Some(code),..} => {
			input.device = ControllerType::Keyboard;
			match code{
				Keycode::W => { input.dpad_up = true },
				Keycode::A => { input.dpad_left = true },
				Keycode::D => { input.dpad_right = true },
				Keycode::S => { input.dpad_down = true },
				Keycode::I => { input.btn_up = true },
				Keycode::J => { input.btn_left = true },
				Keycode::K => { input.btn_down = true },
				Keycode::L => { input.btn_right = true },
				Keycode::E => { input.left_shoulder = true },
				Keycode::U => { input.right_shoulder = true },
				_ => ()
			}
		},
		Event::KeyUp {keycode: Some(code),..} => {
			input.device = ControllerType::Keyboard;
			match code{
				Keycode::W => { input.dpad_up = false },
				Keycode::A => { input.dpad_left = false },
				Keycode::D => { input.dpad_right = false },
				Keycode::S => { input.dpad_down = false },
				Keycode::I => { input.btn_up = false },
				Keycode::J => { input.btn_left = false },
				Keycode::K => { input.btn_down = false },
				Keycode::L => { input.btn_right = false },
				Keycode::E => { input.left_shoulder = false },
				Keycode::U => { input.right_shoulder = false },
				_ => ()
			};
		},
		Event::ControllerAxisMotion { axis, value: val, .. } => {
			input.device = ControllerType::Gamepad;
			let dead_zone = controller_settings.dead_zone;
			match (axis, val) {
				(Axis::LeftX, val) if val < &dead_zone && val > &-dead_zone => { input.left_x_pos = 0;},
				(Axis::LeftX, val) => {input.left_x_pos = *val;},
				(Axis::LeftY, val) if val < &dead_zone && val > &-dead_zone => { input.left_y_pos = 0;},
				(Axis::LeftY, val) => {input.left_y_pos = *val;},
				(Axis::RightX, val) if val < &dead_zone && val > &-dead_zone => { input.right_x_pos = 0;},
				(Axis::RightX, val) => {input.right_x_pos = *val;},
				(Axis::RightY, val) if val < &dead_zone && val > &-dead_zone => { input.right_y_pos = 0;},
				(Axis::RightY, val) => {input.right_y_pos = *val;},
				(Axis::TriggerLeft, val) if val < &dead_zone && val > &-dead_zone => { input.trig_left_pos = 0;},
				(Axis::TriggerLeft, val) => {input.trig_left_pos = *val;},
				(Axis::TriggerRight, val) if val < &dead_zone && val > &-dead_zone => { input.trig_right_pos = 0;},
				(Axis::TriggerRight, val) => {input.trig_right_pos = *val;},
			}
		}
		Event::ControllerButtonDown { button, .. } => {
			input.device = ControllerType::Gamepad;
			match button {
				Button::A =>             { input.btn_down = true },
				Button::X =>             { input.btn_left = true },
				Button::Y =>             { input.btn_up = true },
				Button::B =>             { input.btn_right = true },
				Button::LeftShoulder =>  { input.left_shoulder = true },
				Button::RightShoulder => { input.right_shoulder = true },
				Button::DPadDown =>      { input.dpad_down = true },
				Button::DPadLeft =>      { input.dpad_left = true },
				Button::DPadRight =>     { input.dpad_right = true },
				Button::DPadUp =>        { input.dpad_up = true },
				Button::Start =>         { input.btn_start = true },
				Button::Back =>          { input.btn_back = true },
				Button::LeftStick =>     { input.left_stick = true },
				Button::RightStick =>    { input.right_stick = true },
				_ => ()
			}
		},
		Event::ControllerButtonUp { button, .. } => {
			input.device = ControllerType::Gamepad;
			match button{
				Button::A =>             { input.btn_down = false },
				Button::X =>             { input.btn_left = false },
				Button::Y =>             { input.btn_up = false },
				Button::B =>             { input.btn_right = false },
				Button::LeftShoulder =>  { input.left_shoulder = false },
				Button::RightShoulder => { input.right_shoulder = false },
				Button::DPadDown =>      { input.dpad_down = false },
				Button::DPadLeft =>      { input.dpad_left = false },
				Button::DPadRight =>     { input.dpad_right = false },
				Button::DPadUp =>        { input.dpad_up = false },
				Button::Start =>         { input.btn_start = false },
				Button::Back =>          { input.btn_back = false },
				Button::LeftStick =>     { input.left_stick = false },
				Button::RightStick =>    { input.right_stick = false },
				_ => ()
			}
		},
		Event::ControllerDeviceAdded { .. } => { println!("Controller added"); },
		Event::ControllerDeviceRemoved { .. } => { println!("Controller removed"); },
		_ => {}
	}
}

