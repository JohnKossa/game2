use sdl2::render::{WindowCanvas, Texture};
use crate::game_context::{GameContext, GameObject};
use crate::input::{InputState};
use crate::screens::battle::BattleContext;
use crate::sound_manager::SoundManager;

#[derive(Clone, Copy)]
pub enum StartScreenState{
    FadeIn(usize, usize),
    Waiting,
    FadeOut(usize, usize)
}

#[derive(Clone, Copy)]
pub struct StartScreenContext{
    pub state: StartScreenState
}
impl StartScreenContext{
    pub fn new() -> StartScreenContext{
        StartScreenContext{
            state: StartScreenState::FadeIn(0,30)
        }
    }
    pub fn handle_tick(game_obj: &mut GameObject, input_state: &InputState, sound_manager: &mut SoundManager){
        match game_obj.phase {
            GameContext::StartScreen(start_screen_context) => {
                match start_screen_context.state {
                    StartScreenState::FadeIn(a, b) if a == b => {
                        game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::Waiting})
                    },
                    StartScreenState::FadeIn(a, b) if a > b => unreachable!("Frame count above maximum"),
                    StartScreenState::FadeIn(a,b) => {
                        game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeIn(a+1, b)})
                    },
                    StartScreenState::Waiting => match input_state.btn_start{
                        true => {
                            game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeOut(0,30)})
                        },
                        false => (),
                    },
                    StartScreenState::FadeOut(a,b) if a==b => {
                        game_obj.phase = GameContext::Battle(BattleContext::from_game_object(game_obj))
                    },
                    StartScreenState::FadeOut(a, b) if a > b => unreachable!("Frame count above maximum"),
                    StartScreenState::FadeOut(a, b) => {
                        game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeOut(a+1,b)})
                    }
                }
            },
            _=>unreachable!("Should not be able to call handle_tick from start screen while not in start screen phase.")
        };
    }
}

pub fn render_start_screen(canvas: &mut WindowCanvas, background_texture: &Texture){
	canvas.clear();
	//let (width, height) = canvas.output_size().unwrap();
	//let bg_rect = Rect::from(0,0,width, height);
	canvas.copy(background_texture, None, None).expect("Couldn't draw background texture.");
	canvas.present();
}