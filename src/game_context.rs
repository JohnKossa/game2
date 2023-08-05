use sdl2::render::{WindowCanvas, Texture};
use crate::game_context::GameContext::{Battle, StartScreen};
use crate::screens::start::render_start_screen;
use super::screens::battle::{BattleContext, render_battle};
use super::screens::start::StartScreenContext;
use super::input::{InputState, get_player_intent_vector};
use super::sound_manager::SoundManager;

#[derive(Clone, Copy)]
pub struct GameObject{
    pub phase: GameContext,
    pub player: Option<Player>,
}

impl GameObject{
    pub fn handle_tick(&mut self, input_state: &InputState, my_sound_manager: &mut SoundManager){
        match self.phase.clone(){
            Battle(_battle_context) =>{
                BattleContext::handle_tick(self, input_state, my_sound_manager);
            },
            StartScreen(_start_context)=>{
                StartScreenContext::handle_tick(self, input_state, my_sound_manager);
            },
            _ => {todo!("implement handle_tick for other game contexts")}
        }
    }

    pub fn render(self, canvas: &mut WindowCanvas, background_texture: &Texture){
        //will reach out to the draw functions of its phases
        match self.phase {
            Battle(battle)=> render_battle(canvas, background_texture, &battle.player),
            StartScreen(ctx) => render_start_screen(canvas, background_texture, &ctx),
            _ => todo!("implement render for other game phases")
        }
    }
}

#[derive(Clone, Copy)]
pub enum GameContext{
    StartScreen(StartScreenContext),
    Walking,
    PartyMenu,
    ForcedAction,
    Conversation,
    Battle(BattleContext),
}

#[derive(Clone, Copy)]
pub struct Player{
    //player stat things go here
}