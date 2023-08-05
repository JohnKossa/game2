extern crate sdl2;
extern crate gl;
mod input;
mod screens;
mod sound_manager;
mod game_context;

use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};
use rodio::{Decoder};
use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::image::{LoadTexture};
use sdl2::keyboard::Keycode;

use input::{InputState, ControllerType, ControllerSettings};
use screens::battle::{BattleContext, render_battle};
use sound_manager::SoundManager;
use game_context::{GameContext, GameObject, Player};

fn main() {
    let sdl_context = sdl2::init().expect("Unable to create sdl context");
    let controller_subsystem = sdl_context
        .game_controller()
        .expect("Unable to create game controller subsystem");
    let video_subsystem = sdl_context
        .video()
        .expect("Unable to initialize sdl video context");

    let file = BufReader::new(File::open("assets/sounds/Eyewitness.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let mut my_sound_manager = SoundManager::new();
    my_sound_manager
        .play("bg", source)
        .set_volume(0.5);

    // Attempt to open the controller
    let _controller = controller_subsystem
        .open(0)
        .expect("Failed to open controller");
    let controller_settings = ControllerSettings::new();
    let window = video_subsystem.window("Game Window", 1080, 720)
        .position_centered()
        .build()
        .expect("Failed to create window");
    let mut canvas = window.into_canvas()
        .software()
        .build()
        .expect("Failed to create canvas from window");

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture("assets/images/background.jpg")
        .expect("Unable to create background texture.");

    let mut events = sdl_context.event_pump()
        .expect("Unable to initialize sdl event pump");
    let target_frame_duration = Duration::from_millis(1000 / 60); // 60 FPS
    let mut input_state = InputState::new();

    let mut game_obj = GameObject{
        phase: GameContext::Battle(BattleContext::new()),
        player: Some(Player{})
    };

    'mainloop: loop {
        let frame_start = Instant::now();
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {keycode: Some(code),..} => {
                    input_state.device = ControllerType::Keyboard;
                    match code{
                        Keycode::W => { input_state.dpad_up = true },
                        Keycode::A => { input_state.dpad_left = true },
                        Keycode::D => { input_state.dpad_right = true },
                        Keycode::S => { input_state.dpad_down = true },
                        Keycode::I => { input_state.btn_up = true },
                        Keycode::J => { input_state.btn_left = true },
                        Keycode::K => { input_state.btn_down = true },
                        Keycode::L => { input_state.btn_right = true },
                        Keycode::E => { input_state.left_shoulder = true },
                        Keycode::U => { input_state.right_shoulder = true },
                        _ => ()
                    }
                },
                Event::KeyUp {keycode: Some(code),..} => {
                    input_state.device = ControllerType::Keyboard;
                    match code{
                        Keycode::W => { input_state.dpad_up = false },
                        Keycode::A => { input_state.dpad_left = false },
                        Keycode::D => { input_state.dpad_right = false },
                        Keycode::S => { input_state.dpad_down = false },
                        Keycode::I => { input_state.btn_up = false },
                        Keycode::J => { input_state.btn_left = false },
                        Keycode::K => { input_state.btn_down = false },
                        Keycode::L => { input_state.btn_right = false },
                        Keycode::E => { input_state.left_shoulder = false },
                        Keycode::U => { input_state.right_shoulder = false },
                        _ => ()
                    };
                },
                Event::ControllerAxisMotion { axis, value: val, .. } => {
                    input_state.device = ControllerType::Gamepad;
                    let dead_zone = controller_settings.dead_zone;
                    match axis {
                        Axis::LeftX => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.left_x_pos = 0;
                            }else{
                                input_state.left_x_pos = val;
                            }
                        },
                        Axis::LeftY => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.left_y_pos = 0;
                            }else{
                                input_state.left_y_pos = val;
                            }
                        },
                        Axis::RightX => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.right_x_pos = 0;
                            }else{
                                input_state.right_x_pos = val;
                            }
                        },
                        Axis::RightY => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.right_y_pos = 0;
                            }else{
                                input_state.right_y_pos = val;
                            }
                        },
                        Axis::TriggerLeft => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.trig_left_pos = 0;
                            }else{
                                input_state.trig_left_pos = val;
                            }
                        },
                        Axis::TriggerRight => {
                            if val < dead_zone && val > -dead_zone{
                                input_state.trig_right_pos = 0;
                            }else{
                                input_state.trig_right_pos = val;
                            }
                        },
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    input_state.device = ControllerType::Gamepad;
                    match button {
                        Button::A =>             { input_state.btn_down = true },
                        Button::X =>             { input_state.btn_left = true },
                        Button::Y =>             { input_state.btn_up = true },
                        Button::B =>             { input_state.btn_right = true },
                        Button::LeftShoulder =>  { input_state.left_shoulder = true },
                        Button::RightShoulder => { input_state.right_shoulder = true },
                        Button::DPadDown =>      { input_state.dpad_down = true },
                        Button::DPadLeft =>      { input_state.dpad_left = true },
                        Button::DPadRight =>     { input_state.dpad_right = true },
                        Button::DPadUp =>        { input_state.dpad_up = true },
                        Button::Start =>         { input_state.btn_start = true },
                        Button::Back =>          { input_state.btn_back = true },
                        Button::LeftStick =>     { input_state.left_stick = true },
                        Button::RightStick =>    { input_state.right_stick = true },
                        _ => ()
                    }
                },
                Event::ControllerButtonUp { button, .. } => {
                    input_state.device = ControllerType::Gamepad;
                    match button{
                        Button::A =>             { input_state.btn_down = false },
                        Button::X =>             { input_state.btn_left = false },
                        Button::Y =>             { input_state.btn_up = false },
                        Button::B =>             { input_state.btn_right = false },
                        Button::LeftShoulder =>  { input_state.left_shoulder = false },
                        Button::RightShoulder => { input_state.right_shoulder = false },
                        Button::DPadDown =>      { input_state.dpad_down = false },
                        Button::DPadLeft =>      { input_state.dpad_left = false },
                        Button::DPadRight =>     { input_state.dpad_right = false },
                        Button::DPadUp =>        { input_state.dpad_up = false },
                        Button::Start =>         { input_state.btn_start = false },
                        Button::Back =>          { input_state.btn_back = false },
                        Button::LeftStick =>     { input_state.left_stick = false },
                        Button::RightStick =>    { input_state.right_stick = false },
                        _ => ()
                    }
                },
                Event::ControllerDeviceAdded { .. } => {
                    println!("Controller added");
                },
                Event::ControllerDeviceRemoved { .. } => {
                    println!("Controller removed");
                },
                _ => {}
            }
        }
        println!("{:?}", input_state);

        game_obj.handle_tick(&input_state, &mut my_sound_manager);
        game_obj.render(&mut canvas, &background_texture);

        // Sleep if we finished this frame early so we lock to the desired framerate
        let frame_duration = frame_start.elapsed();
        if let Some(remaining_duration) = target_frame_duration.checked_sub(frame_duration) {
            std::thread::sleep(remaining_duration);
        } else {
            println!("Dropped framerate. Frame duration: {:?}, Target: {:?}", frame_duration, target_frame_duration);
        }
    }
}
