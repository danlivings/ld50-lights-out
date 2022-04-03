pub mod components;
pub mod resources;
pub mod systems;

mod events;
mod tick;
use bevy::log;
use bevy::prelude::*;
use components::*;
use events::*;
use resources::*;
use systems::*;
use tick::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::new());
        app.add_startup_system(Self::setup_camera);
        app.add_startup_system(Self::create_board);
        app.add_startup_system(scoring::setup_ui);

        app.insert_resource(ClearColor(Color::BLACK));
        app.add_system(input::handle_mouse_input);
        app.add_system(lighting::handle_tile_trigger);
        app.add_system(lighting::update);
        app.add_system(scoring::update_score);
        app.add_system(tile::update_tiles);
        app.add_system(tile::create_new_tiles);
        app.insert_resource(UpdateTickTimer::new(0.1));

        app.add_event::<TileTriggerEvent>();
        app.add_event::<TileUpdateEvent>();
        app.add_event::<TileCreateEvent>();

        #[cfg(feature = "debug")]
        self.register_inspectables(app);

        log::info!("Loaded board plugin.");
    }
}

impl BoardPlugin {
    fn setup_camera(mut commands: Commands) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d())
            .insert(MainCamera);
    }

    fn create_board(
        mut commands: Commands,
        mut tile_create_evw: EventWriter<TileCreateEvent>,
    ) {
        let tile_map = TileMap::new();

        let tile_size = 16.;
        let tile_padding = 2.;

        let entity = commands.spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(Vec3::new(0., 0., 0.)))
            .insert(GlobalTransform::default())
            .id();
        
        commands.insert_resource(Board {
            tile_map,
            tile_size,
            tile_padding,
            entity,
        });

        for coordinate in NEIGHBOUR_OFFSETS {
            tile_create_evw.send(TileCreateEvent(coordinate.into()));
        }
        tile_create_evw.send(TileCreateEvent(Coordinates::ZERO));
    }
    
    #[cfg(feature = "debug")]
    fn register_inspectables(&self, app: &mut App) {
        app.register_inspectable::<TileComponent>();
        app.register_inspectable::<Score>();
        app.register_inspectable::<Coordinates>();

        log::info!("Registered inspectable components.");
    }
}