use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct UiRoot(pub Entity);

#[derive(Component)]
pub struct UiButton<'a, T: Send + Sync + Copy>(pub &'a T);

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct UiHighlightable {
    pub default_color: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub root_entity: Entity,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct PauseMenu;