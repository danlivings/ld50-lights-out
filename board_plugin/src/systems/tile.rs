use crate::components::TileComponent;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::log;
use bevy::utils::HashSet;

pub fn update_tiles(
    board: Res<Board>,
    mut tile_query: Query<(&mut TileComponent, &mut Sprite)>,
    mut tile_update_evr: EventReader<TileUpdateEvent>,
    mut tile_create_evw: EventWriter<TileCreateEvent>,
) {
    let mut existing_tile_coords: HashSet<Coordinates> = HashSet::default();

    for _ in tile_update_evr.iter() {
        for (mut tile_component, mut sprite) in tile_query.iter_mut() {
            existing_tile_coords.insert(tile_component.coordinates);
            if let Some(tile) = board.tile_map.get(&tile_component.coordinates) {
                log::debug!("Updating tile {}", tile_component.coordinates);

                tile_component.lightness = tile.lightness;
                sprite.color = tile_component.get_color();
            }
        }
    }

    for new_tile_coords in board.tile_map.get_new_tiles(existing_tile_coords) {
        tile_create_evw.send(TileCreateEvent(*new_tile_coords));
    }
}

pub fn create_new_tiles(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_create_evr: EventReader<TileCreateEvent>,
) {
    for event in tile_create_evr.iter() {
        let coordinates = &event.0;
        let tile = board.tile_map.get(coordinates).unwrap();

        commands.entity(board.entity).with_children(|parent| {
            spawn_tile(parent, &tile, coordinates, board.tile_size, board.tile_padding);
        });

        log::info!("Created tile at {}", coordinates);
    }
}

fn spawn_tile(
    parent: &mut ChildBuilder,
    tile: &Tile,
    coordinates: &Coordinates,
    tile_size: f32,
    tile_padding: f32,
) {
    let tile_component = TileComponent {
        coordinates: *coordinates,
        lightness: tile.lightness,
    };

    let mut commands = parent.spawn();

    commands
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: tile_component.get_color(),
                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                coordinates.x as f32 * tile_size,
                coordinates.y as f32 * tile_size,
                1.,
            ),
            ..Default::default()
        })
        .insert(Name::new(format!("Tile {}", coordinates)))
        .insert(tile_component);
}
