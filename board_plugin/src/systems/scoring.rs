use crate::components::{Score, UiRoot};
use crate::events::GameOverEvent;
use crate::resources::Board;
use crate::tick::UpdateTickTimer;
use crate::utils::format_number;
use bevy::prelude::*;

pub fn setup_score_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ui_root_query: Query<&mut UiRoot>,
) {
    let ui_root = ui_root_query.single_mut();
    let mut ui_root_entity = commands.entity(ui_root.0);

    ui_root_entity
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(60.)),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(Name::new("Score Panel"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(16.)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "Score: 0",
                                TextStyle {
                                    font: asset_server.load("fonts/Lato/Lato-Light.ttf"),
                                    font_size: 32.,
                                    color: Color::WHITE,
                                },
                                Default::default()
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("Score Text"))
                        .insert(Score(0));
                });
        });
    
}

pub fn teardown_score_ui(
    mut commands: Commands,
    score_query: Query<Entity, With<Score>>,
    parent_query: Query<&Parent>,
) {
    for entity in score_query.iter() {
        if let Ok(parent) = parent_query.get(entity) {
            commands.entity(parent.0).despawn_recursive();
        }
    }
}

pub fn update_score(
    board: Res<Board>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTickTimer>,
    mut score_query: Query<(&mut Score, &mut Text)>,
    mut game_over_evw: EventWriter<GameOverEvent>,
) {
    if !timer.0.tick(time.time_since_startup()).just_finished() {
        return;
    }

    let (mut score, mut text) = score_query.single_mut();

    let score_increment = board.tile_map
        .non_black_tiles()
        .into_iter()
        .map(|tile| { f64::log(tile.lightness as f64 + 1., 2.) as u64 })
        .reduce(|a, b| { a + b })
        .unwrap_or(0);

    score.0 = score.0 + score_increment;
    text.sections[0].value = format!("Score: {}", format_number(score.0));

    if score_increment == 0 {
        game_over_evw.send(GameOverEvent(score.0));
    }
}
