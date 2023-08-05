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

