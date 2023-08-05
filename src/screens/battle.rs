use std::fs::File;
use std::io::BufReader;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};
use rodio::{Decoder};
use crate::game_context::GameContext;
use super::super::input::{InputState, get_player_intent_vector};
use super::super::sound_manager::SoundManager;
use super::super::game_context::GameObject;

#[derive(Clone, Copy)]
pub enum BattleState{
	Starting,
	Live,
	Paused, //paused for quick-menu selection
	Finished
}

#[derive(Clone, Copy)]
pub struct BattleContext{
	pub state: BattleState,
	pub player: BattlePlayerContext,
}
impl BattleContext{
	pub fn new() -> BattleContext{
		BattleContext{
			state: BattleState::Live,//TODO change this to starting once we have state transitions
			player:BattlePlayerContext{
				position: Point::new(50,300),
				facing_vector: 0.0,
				state: PlayerState::Standing
			}
		}
	}
	pub fn from_game_object(game_object: &GameObject) -> BattleContext{
		BattleContext{
			state: BattleState::Live,//TODO change this to starting once we have state transitions
			player:BattlePlayerContext{
				position: Point::new(50,300),
				facing_vector: 0.0,
				state: PlayerState::Standing
			}
		}
	}
	pub fn handle_tick(game_obj: &mut GameObject, input_state: &InputState, my_sound_manager: &mut SoundManager){
		match game_obj.phase {
			GameContext::Battle(ref mut battle_context) =>{
				let battle_player = &mut battle_context.player;
				match battle_context.state {
					BattleState::Starting => (),
					BattleState::Live => {
						update_battle_player(battle_player, &input_state, my_sound_manager);
					},
					BattleState::Paused => (),
					BattleState::Finished => (),
				}
			},
			_ => unreachable!("Should not be able to call handle_tick from start screen while not in battle phase.")
		};
	}
}

#[derive(Clone, Copy)]
pub struct BattlePlayerContext{
	pub position: Point,
	pub facing_vector: f32,
	pub state: PlayerState,
}

#[derive(Clone, Copy)]
pub enum PlayerState{
	Standing,
	Running,
	Dashing(usize, usize),
	Attacking(usize, usize),
	Blocking,
	Casting(usize, usize)
}

fn update_battle_player(player: &mut BattlePlayerContext, input: &InputState, sound_manager: &mut SoundManager){
	match &player.state{
		PlayerState::Standing => {
			match (get_player_intent_vector(input), &input.btn_down, &input.right_shoulder){
				(_, true, _) => {
					player.state = PlayerState::Attacking(0,30);
				}
				(Some(x), false, _) => {
					player.facing_vector = x;
					const RUNNING_SPEED: f32 = 2.0;
					player.position.x += (player.facing_vector.cos() * RUNNING_SPEED) as i32;
					player.position.y -= (player.facing_vector.sin() * RUNNING_SPEED) as i32;
					player.state = PlayerState::Running;
				},
				(None, false, _) => (),
			};
		},
		PlayerState::Running => {
			match (get_player_intent_vector(input), &input.btn_down, &input.right_shoulder){
				(_, true, _) => {
					player.state = PlayerState::Attacking(0,30);
				},
				(Some(x), false, true) => {
					//set angle then start dashing
					player.facing_vector = x;
					player.state = PlayerState::Dashing(0, 30);
				},
				(Some(x), false, false) => {
					//still running
					player.facing_vector = x;
					const RUNNING_SPEED: f32 = 2.0;
					player.position.x += (player.facing_vector.cos() * RUNNING_SPEED) as i32;
					player.position.y -= (player.facing_vector.sin() * RUNNING_SPEED) as i32;
				},
				(None, false, _) => {
					player.state = PlayerState::Standing;
				},
			};
		},
		PlayerState::Dashing(framecount, max_frames) => {
			if *framecount == 0 {
				let file = BufReader::new(File::open("assets/sounds/chicken.ogg").unwrap());
				let source = Decoder::new(file).unwrap();
				sound_manager.play("chicken", source);
			}
			if framecount == max_frames{
				player.state = PlayerState::Standing;
			}else{
				const DASHING_SPEED: f32 = 5.0;
				player.position.x += (player.facing_vector.cos() * DASHING_SPEED) as i32;
				player.position.y -= (player.facing_vector.sin() * DASHING_SPEED) as i32;
				player.state = PlayerState::Dashing(framecount+1, *max_frames);
			}
		},
		PlayerState::Attacking(framecount, max_frames) => {
			if *framecount == 0{
				let file = BufReader::new(File::open("assets/sounds/sword_swing.ogg").unwrap());
				let source = Decoder::new(file).unwrap();
				sound_manager.play("swordswing", source);

			}
			if *framecount == 6{
				();
			}
			if *framecount == 24{
				();
			}
			if framecount == max_frames{
				player.state = PlayerState::Standing;
			}else{
				player.state = PlayerState::Attacking(framecount+1, *max_frames);
			}
		},
		PlayerState::Casting(framecount, max_frames) => (),
		PlayerState::Blocking => ()
	}
}

pub fn render_battle(canvas: &mut WindowCanvas, background_texture: &Texture, player: &BattlePlayerContext){
	canvas.clear();
	//let (width, height) = canvas.output_size().unwrap();
	//let bg_rect = Rect::from(0,0,width, height);
	canvas.copy(background_texture, None, None).expect("Couldn't draw background texture.");
	let player_rect = Rect::from_center(player.position, 50, 50);
	let player_color = match player.state{
		PlayerState::Standing => Color::RGB(0,255,0),
		PlayerState::Running => Color::RGB(255, 255, 0),
		PlayerState::Dashing(_,_) => Color::RGB(255,165,0),
		PlayerState::Attacking(_,_) => Color::RGB(255,0,0),
		PlayerState::Casting(_,_) => Color::RGB(255,0,255),
		PlayerState::Blocking => Color::RGB(40,40,40),
	};
	canvas.set_draw_color(player_color);
	canvas.fill_rect(player_rect).unwrap();
	canvas.present();
}