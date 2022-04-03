use crate::resources::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct TileUpdateEvent;

#[derive(Debug, Copy, Clone)]
pub struct TileCreateEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct StartGameEvent;

#[derive(Debug, Copy, Clone)]
pub struct ExitGameEvent;

#[derive(Debug, Copy, Clone)]
pub struct TogglePauseEvent;

#[derive(Debug, Copy, Clone)]
pub struct GoToMainMenuEvent;

#[derive(Debug, Copy, Clone)]
pub struct GameOverEvent(pub u64);
