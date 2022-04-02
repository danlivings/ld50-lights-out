use crate::resources::TileMap;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub tile_size: f32,
    pub tile_padding: f32,
    pub entity: Entity,
}

impl Board {
    
}