use crate::events::*;
use crate::resources::Board;
use crate::tick::UpdateTickTimer;
use bevy::log;
use bevy::prelude::*;


pub fn handle_tile_trigger(
    mut board: ResMut<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
    mut tile_update_evw: EventWriter<TileUpdateEvent>,
) {
    for event in tile_trigger_evr.iter() {
        let coordinates = &event.0;
        log::info!("Enlightening tile {}", coordinates);
        board.tile_map.make_tile_white(coordinates.x, coordinates.y);
    }

    tile_update_evw.send(TileUpdateEvent);
}

pub fn update(
    mut board: ResMut<Board>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTickTimer>,
    mut tile_update_evw: EventWriter<TileUpdateEvent>,
) {
    if !timer.0.tick(time.time_since_startup()).just_finished() {
        return;
    }

    board.tile_map.tick_update();
    tile_update_evw.send(TileUpdateEvent);
}
