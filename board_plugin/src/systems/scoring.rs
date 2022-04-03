use crate::components::Score;
use crate::resources::Board;
use crate::tick::UpdateTickTimer;
use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("UI"))
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

pub fn update_score(
    board: Res<Board>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTickTimer>,
    mut score_query: Query<(&mut Score, &mut Text)>,
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
}

fn format_number(n: u64) -> String {
    let n_string = n.to_string();

    let mut chars = vec![];

    let mut i = 0;
    for n_char in n_string.chars().rev() {
        if i != 0 && i % 3 == 0 {
            chars.push(' ');
        }
        chars.push(n_char);
        i = i + 1;
    }

    chars.into_iter()
        .rev()
        .collect()
}