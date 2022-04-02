use crate::resources::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct TileUpdateEvent;

#[derive(Debug, Copy, Clone)]
pub struct TileCreateEvent(pub Coordinates);
