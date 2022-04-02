use crate::components::MainCamera;
use crate::events::TileTriggerEvent;
use crate::resources::Board;
use bevy::input::ElementState;
use bevy::input::mouse::MouseButtonInput;
use bevy::log;
use bevy::prelude::*;

pub fn handle_mouse_input (
    windows: Res<Windows>,
    board: Res<Board>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_evw: EventWriter<TileTriggerEvent>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = windows.get(camera.window).unwrap();

    for event in button_evr.iter() {
        if event.state == ElementState::Pressed && event.button == MouseButton::Left {
            if let Some(screen_position) = window.cursor_position() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let gpu_coords = (screen_position / window_size) * 2.0 - Vec2::ONE;
                let gpu_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
                let world_position = gpu_to_world.project_point3(gpu_coords.extend(-1.0));
                let world_position: Vec2 = world_position.truncate();
            
                let tile_position = world_position / board.tile_size;
                log::info!("LMB pressed at {}", tile_position);

                let coordinates = (
                    tile_position.x.round() as i32,
                    tile_position.y.round() as i32
                ).into();

                tile_trigger_evw.send(TileTriggerEvent(coordinates));
            }
        }
    }
}