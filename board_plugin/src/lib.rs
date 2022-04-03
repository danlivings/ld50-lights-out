pub mod components;
pub mod resources;
pub mod systems;
pub mod utils;

mod events;
mod tick;
use bevy::log;
use bevy::prelude::*;
use components::*;
use events::*;
use game_state::*;
use resources::*;
use systems::*;
use tick::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Init);

        app.insert_resource(ClearColor(Color::BLACK));
        app.insert_resource(UpdateTickTimer::new(0.1));
        app.insert_resource(FinalScore(0));

        self.setup_global_systems(app);
        self.setup_init_systems(app);
        self.setup_main_menu_systems(app);
        self.setup_in_game_systems(app);
        self.setup_paused_systems(app);
        self.setup_game_over_systems(app);

        app.add_event::<TileTriggerEvent>();
        app.add_event::<TileUpdateEvent>();
        app.add_event::<TileCreateEvent>();
        app.add_event::<StartGameEvent>();
        app.add_event::<ExitGameEvent>();
        app.add_event::<TogglePauseEvent>();
        app.add_event::<GoToMainMenuEvent>();
        app.add_event::<GameOverEvent>();

        #[cfg(feature = "debug")]
        self.register_inspectables(app);

        log::info!("Loaded board plugin.");
    }
}

impl BoardPlugin {
    pub fn setup_global_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(game_state::handle_start_game_event)
                .with_system(game_state::handle_exit_game_event)
                .with_system(game_state::handle_go_to_main_menu_event)
                .with_system(game_state::handle_game_over_event)
                .with_system(game_ui::handle_ui_highlight)
                .with_system(input::handle_keyboard_input)
                .with_system(game_state::handle_toggle_pause_event)
        );
    }

    pub fn setup_init_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Init)
                .label("Initialize UI")
                .with_system(game_ui::setup_ui)
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::Init)
                .after("Initialize UI")
                .with_system(|mut game_state: ResMut<State<GameState>>| {
                    GameState::MainMenu.transition(&mut game_state);
                })
        );
    }

    pub fn setup_main_menu_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(game_ui::setup_main_menu_ui)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(game_ui::handle_ui_button_click::<StartGameEvent>)
                .with_system(game_ui::handle_ui_button_click::<ExitGameEvent>)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(game_ui::teardown_main_menu_ui)
        );
    }
    
    pub fn setup_in_game_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(Self::setup_camera)
                .with_system(Self::create_board)
                .with_system(scoring::setup_score_ui)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(input::handle_mouse_input)
                .with_system(lighting::handle_tile_trigger)
                .with_system(lighting::update)
                .with_system(scoring::update_score)
                .with_system(tile::update_tiles)
                .with_system(tile::create_new_tiles)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::InGame)
                .with_system(scoring::teardown_score_ui)
                .with_system(Self::teardown_camera)
                .with_system(Self::teardown_board)
        );
    }
    
    pub fn setup_paused_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Paused)
                .with_system(game_ui::setup_pause_menu_ui)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Paused)
                .with_system(game_ui::handle_ui_button_click::<TogglePauseEvent>)
                .with_system(game_ui::handle_ui_button_click::<GoToMainMenuEvent>)
                .with_system(game_ui::handle_ui_button_click::<ExitGameEvent>)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Paused)
                .with_system(game_ui::teardown_pause_menu_ui)
        );
    }
    
    pub fn setup_game_over_systems(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(game_ui::setup_game_over_menu_ui)
        );
        app.add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(game_ui::handle_ui_button_click::<StartGameEvent>)
                .with_system(game_ui::handle_ui_button_click::<GoToMainMenuEvent>)
                .with_system(game_ui::handle_ui_button_click::<ExitGameEvent>)
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(game_ui::teardown_game_over_menu_ui)
        );
    }

    fn setup_camera(mut commands: Commands) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d())
            .insert(MainCamera);
    }

    fn teardown_camera(
        mut commands: Commands,
        camera_query: Query<Entity, With<MainCamera>>,
    ) {
        for entity in camera_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
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
            .insert(BoardComponent)
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

    fn teardown_board(
        mut commands: Commands,
        board_query: Query<Entity, With<BoardComponent>>
    ) {
        commands.remove_resource::<Board>();
        
        for entity in board_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    #[cfg(feature = "debug")]
    fn register_inspectables(&self, app: &mut App) {
        app.register_inspectable::<TileComponent>();
        app.register_inspectable::<Score>();
        app.register_inspectable::<Coordinates>();
        app.register_inspectable::<UiHighlightable>();
        app.register_inspectable::<UiRoot>();

        log::info!("Registered inspectable components.");
    }
}