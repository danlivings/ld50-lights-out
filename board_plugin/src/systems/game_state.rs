use crate::events::*;
use crate::resources::FinalScore;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::log;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Init,
    MainMenu,
    InGame,
    Paused,
    GameOver,
}

impl GameState {
    pub fn transition(&self, game_state: &mut ResMut<State<GameState>>) {
        match game_state.replace(self.clone()) {
            Ok(_) => (),
            Err(e) => log::error!("Can't transition to {:?} state: {}", self, e),
        };
    }
}

pub fn handle_start_game_event(
    mut game_state: ResMut<State<GameState>>,
    mut start_game_evr: EventReader<StartGameEvent>,
) {
    for _ in start_game_evr.iter() {
        GameState::InGame.transition(&mut game_state);
    }
}

pub fn handle_exit_game_event(
    mut exit_game_evr: EventReader<ExitGameEvent>,
    mut exit_evw: EventWriter<AppExit>,
) {
    for _ in exit_game_evr.iter() {
        exit_evw.send(AppExit);
    }
}

pub fn handle_go_to_main_menu_event(
    mut game_state: ResMut<State<GameState>>,
    mut got_to_main_menu_evr: EventReader<GoToMainMenuEvent>,
) {
    for _ in got_to_main_menu_evr.iter() {
        GameState::MainMenu.transition(&mut game_state);
    }
}

pub fn handle_toggle_pause_event(
    mut game_state: ResMut<State<GameState>>,
    mut toggle_pause_evr: EventReader<TogglePauseEvent>,
) {
    for _ in toggle_pause_evr.iter() {
        match game_state.current() {
            GameState::InGame => match game_state.push(GameState::Paused) {
                Ok(_) => (),
                Err(e) => log::error!("Can't transition to {:?} state: {}", GameState::Paused, e),
            },
            GameState::Paused => match game_state.pop() {
                Ok(_) => (),
                Err(e) => log::error!("Can't transition to {:?} state: {}", GameState::InGame, e),
            },
            _ => {}, // Can't pause the game if the game isn't being played!
        };
    }
}

pub fn handle_game_over_event(
    mut game_state: ResMut<State<GameState>>,
    mut final_score: ResMut<FinalScore>,
    mut game_over_evr: EventReader<GameOverEvent>,
) {
    for event in game_over_evr.iter() {
        log::info!("Game over! Score: {}", event.0);
        final_score.0 = event.0;
        GameState::GameOver.transition(&mut game_state);
    }
}