use bevy::prelude::*;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Lights Out".to_string(),
        width: 800.,
        height: 800.,
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(BoardPlugin);

    app.run();
}