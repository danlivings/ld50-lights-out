use crate::resources::Coordinates;
use crate::resources::MAX_LIGHTNESS;
use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TileComponent {
    pub coordinates: Coordinates,
    pub lightness: u8,
}

impl TileComponent {
    pub fn get_color(&self) -> Color {
        let c = (self.lightness as f32) / (MAX_LIGHTNESS as f32);

        Color::rgb(c, c, c)
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct BoardComponent;
