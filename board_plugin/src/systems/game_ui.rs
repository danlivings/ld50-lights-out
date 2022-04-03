use crate::components::*;
use crate::events::*;
use crate::resources::FinalScore;
use crate::utils::format_number;
use bevy::input::ElementState;
use bevy::input::mouse::*;
use bevy::log;
use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let entity = commands
        .spawn_bundle(NodeBundle {
            style: ui_style_fill_screen(),
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("UI"))
        .id();
    
        commands.entity(entity).insert(UiRoot(entity));
}

pub fn setup_main_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ui_root_query: Query<&mut UiRoot>,
) {
    let ui_root = ui_root_query.single_mut();

    let font = &asset_server.load("fonts/Lato/Lato-Light.ttf");
    let menu_panel_entity = build_main_menu_panel(&mut commands, font);

    commands.entity(ui_root.0).push_children(&[menu_panel_entity]);
}

pub fn setup_pause_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ui_root_query: Query<&mut UiRoot>,
) {
    let ui_root = ui_root_query.single_mut();

    let font = &asset_server.load("fonts/Lato/Lato-Light.ttf");
    let menu_panel_entity = build_pause_menu_panel(&mut commands, font);

    commands.entity(ui_root.0).push_children(&[menu_panel_entity]);
}

pub fn setup_game_over_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    final_score: Res<FinalScore>,
    mut ui_root_query: Query<&mut UiRoot>,
) {
    let ui_root = ui_root_query.single_mut();

    let font = &asset_server.load("fonts/Lato/Lato-Light.ttf");
    let menu_panel_entity = build_game_over_menu_panel(&mut commands, *final_score, font);

    commands.entity(ui_root.0).push_children(&[menu_panel_entity]);
}

fn ui_style_fill_screen() -> Style {
    Style {
        size: ui_fill_space(),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::FlexEnd,
        ..Default::default()
    }
}

fn ui_fill_space() -> Size<Val> {
    Size::new(Val::Percent(100.), Val::Percent(100.))
}

fn build_main_menu_panel(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    let menu_buttons_entity = build_main_menu_buttons(commands, font);

    let menu_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                margin: Rect {
                    left: Val::Px(128.),
                    right: Val::Px(128.),
                    top: Val::Px(96.),
                    bottom: Val::Px(96.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Main Menu Panel"))
        .with_children(|parent| {
            build_logo_panel(parent, font);
            build_help_text_panel(parent, font);
        })
        .id();

    commands.entity(menu_entity).push_children(&[menu_buttons_entity]);

    menu_entity
}

fn build_pause_menu_panel(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    let menu_buttons_entity = build_pause_menu_buttons(commands, font);

    let menu_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                position_type: PositionType::Absolute,
                margin: Rect {
                    left: Val::Px(128.),
                    right: Val::Px(128.),
                    top: Val::Px(96.),
                    bottom: Val::Px(96.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Pause Menu Panel"))
        .insert(PauseMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect::all(Val::Px(16.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "PAUSED",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Paused Text"));
        })
        .id();

    commands.entity(menu_entity).push_children(&[menu_buttons_entity]);

    menu_entity
}

fn build_game_over_menu_panel(
    commands: &mut Commands,
    final_score: FinalScore,
    font: &Handle<Font>,
) -> Entity {
    let menu_buttons_entity = build_game_over_menu_buttons(commands, font);

    let menu_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                position_type: PositionType::Absolute,
                margin: Rect {
                    left: Val::Px(128.),
                    right: Val::Px(128.),
                    top: Val::Px(96.),
                    bottom: Val::Px(96.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Game Over Menu Panel"))
        .insert(PauseMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect::all(Val::Px(16.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "GAME OVER",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Game Over Text"));
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect::all(Val::Px(16.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("Score: {}", format_number(final_score.0)),
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Score Text"));
        })
        .id();

    commands.entity(menu_entity).push_children(&[menu_buttons_entity]);

    menu_entity
}

fn build_logo_panel(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(300.), Val::Px(120.)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                padding: Rect::all(Val::Px(2.)),
                ..ui_style_fill_screen()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(Name::new("Logo"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: Rect {
                            left: Val::Px(23.),
                            right: Val::Px(23.),
                            top: Val::Px(8.),
                            bottom: Val::Px(8.),
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..ui_style_fill_screen()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .insert(Name::new("Logo Interior"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "LIGHTS",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                }
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("Title Text 1"));

                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "OUT",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                }
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("Title Text 2"));
                });
        });
}

fn build_help_text_panel(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(120.),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..ui_style_fill_screen()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        margin: Rect::all(Val::Px(40.)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Click gray squares to turn them white.\n\
                        The lighter the square, the more points it will give.\n\
                        If all squares turn black, the game ends.\n\n\
                        Press Esc or Space to pause.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        }
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Help Text"));
        })
        .insert(Name::new("Help Text Panel"));
}

fn build_main_menu_buttons(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    let start_game_button_entity = build_start_game_button(commands, font);
    let exit_game_button_entity = build_exit_game_button(commands, font);

    let menu_buttons_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                max_size: Size {
                    width: Val::Px(256.),
                    ..Default::default()
                },
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Main Menu Buttons"))
        .id();

    commands.entity(menu_buttons_entity)
        .push_children(&[start_game_button_entity, exit_game_button_entity]);

    menu_buttons_entity
}

fn build_pause_menu_buttons(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    let resume_game_button_entity = build_resume_game_button(commands, font);
    let return_main_menu_button_entity = build_return_main_menu_button(commands, font);
    let exit_game_button_entity = build_exit_game_button(commands, font);

    let menu_buttons_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                max_size: Size {
                    width: Val::Px(256.),
                    ..Default::default()
                },
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Pause Menu Buttons"))
        .id();

    commands.entity(menu_buttons_entity)
        .push_children(&[resume_game_button_entity, return_main_menu_button_entity, exit_game_button_entity]);

    menu_buttons_entity
}

fn build_game_over_menu_buttons(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    let restart_game_button_entity = build_restart_game_button(commands, font);
    let return_main_menu_button_entity = build_return_main_menu_button(commands, font);
    let exit_game_button_entity = build_exit_game_button(commands, font);

    let menu_buttons_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: ui_fill_space(),
                max_size: Size {
                    width: Val::Px(256.),
                    ..Default::default()
                },
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("Game Over Menu Buttons"))
        .id();

    commands.entity(menu_buttons_entity)
        .push_children(&[restart_game_button_entity, return_main_menu_button_entity, exit_game_button_entity]);

    menu_buttons_entity
}


fn build_start_game_button(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    build_button(commands, font, "START", "Start Game", &StartGameEvent)
}

fn build_exit_game_button(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    build_button(commands, font, "QUIT", "Exit Game", &ExitGameEvent)
}

fn build_resume_game_button(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    build_button(commands, font, "RESUME", "Resume Game", &TogglePauseEvent)
}

fn build_return_main_menu_button(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    build_button(commands, font, "MAIN MENU", "Main Menu", &GoToMainMenuEvent)
}

fn build_restart_game_button(
    commands: &mut Commands,
    font: &Handle<Font>,
) -> Entity {
    build_button(commands, font, "RESTART", "Restart", &StartGameEvent)
}

fn build_button<T: Send + Sync + Copy>(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    name: &str,
    on_click_event: &'static T,
) -> Entity {
    let root_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(64.)),
                padding: Rect::all(Val::Px(2.)),
                margin: Rect {
                    bottom: Val::Px(8.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .insert(Name::new(format!("{} Button", name)))
        .insert(UiButton(on_click_event))
        .id();
    
    commands.entity(root_entity)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: ui_fill_space(),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::BLACK.into(),
                    ..Default::default()
                })
                .insert(Name::new(format!("{} Button Interior", name)))
                .insert(UiHighlightable {
                    default_color: Color::BLACK,
                    hover_color: Color::WHITE,
                    pressed_color: Color::WHITE,
                    root_entity,
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 48.,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new(format!("{} Button Text", name)))
                        .insert(UiHighlightable {
                            default_color: Color::WHITE,
                            hover_color: Color::BLACK,
                            pressed_color: Color::BLACK,
                            root_entity,
                        });
                });
        });

    root_entity
}

pub fn teardown_main_menu_ui(
    mut commands: Commands,
    child_ui_items_query: Query<&Children, With<UiRoot>>,
) {
    for entity in child_ui_items_query.single().iter() {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn teardown_pause_menu_ui(
    mut commands: Commands,
    pause_menu_ui_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn teardown_game_over_menu_ui(
    mut commands: Commands,
    child_ui_items_query: Query<&Children, With<UiRoot>>,
) {
    for entity in child_ui_items_query.single().iter() {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn handle_ui_highlight(
    windows: Res<Windows>,
    mut highlightable_node_query: Query<(&mut UiColor, &UiHighlightable)>,
    mut highlightable_text_query: Query<(&mut Text, &UiHighlightable)>,
    node_transform_query: Query<(&Node, &GlobalTransform)>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.get_primary().unwrap();
    let position = window.cursor_position();
    if let Some(position) = position {
        let mut is_hover = true;
        for event in button_evr.iter() {
            if let ElementState::Pressed = event.state {
                is_hover = false;
            }
        }

        for (mut ui_color, highlightable) in highlightable_node_query.iter_mut() {
            if let Ok((node, transform)) = node_transform_query.get(highlightable.root_entity) {
                if is_position_in_node(position, node, transform) {
                    if is_hover {
                        ui_color.0 = highlightable.hover_color.into();
                    } else {
                        ui_color.0 = highlightable.pressed_color.into();
                    };
                } else {
                    ui_color.0 = highlightable.default_color.into();
                }
            }
        }

        for (mut text, highlightable) in highlightable_text_query.iter_mut() {
            if let Ok((node, transform)) = node_transform_query.get(highlightable.root_entity) {
                for mut section in text.sections.iter_mut() {
                    if is_position_in_node(position, node, transform) {
                        if is_hover {
                            section.style.color = highlightable.hover_color;
                        } else {
                            section.style.color = highlightable.pressed_color;
                        }
                    } else {
                        section.style.color = highlightable.default_color;
                    }
                }
            }
        }
    }
}

pub fn handle_ui_button_click<T: Send + Sync + Copy>(
    windows: Res<Windows>,
    button_query: Query<(&UiButton<'static, T>, &Node, &GlobalTransform)>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut evw: EventWriter<T>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        if let ElementState::Released = event.state {
            let position = window.cursor_position();
            if let Some(position) = position {
                log::info!("LMB released at {}", position);
                for (ui_button, node, transform) in button_query.iter() {
                    if is_position_in_node(position, node, transform) {
                        let event = *ui_button.0;
                        evw.send(event);
                        log::info!("Sending {}", std::any::type_name::<T>());
                    }
                }
            }
        }
    }
}

fn is_position_in_node(position: Vec2, node: &Node, transform: &GlobalTransform) -> bool {
    let xmin = transform.translation.x - (node.size.x / 2.);
    let xmax = transform.translation.x + (node.size.x / 2.);
    let ymin = transform.translation.y - (node.size.y / 2.);
    let ymax = transform.translation.y + (node.size.y / 2.);

    position.x >= xmin
        && position.x <= xmax
        && position.y >= ymin
        && position.y <= ymax
}