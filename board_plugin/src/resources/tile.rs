use crate::resources::coordinates::Coordinates;
use bevy::utils::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "debug")]
use colored::{Color, Colorize};

pub const MAX_LIGHTNESS: u8 = 255;
const HALF_GRAY: u8 = MAX_LIGHTNESS / 2;

pub const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct TileMap {
    active_tiles: HashMap<Coordinates, Tile>
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            active_tiles: HashMap::from_iter([
                ((-1, -1).into(), Tile::new_gray(HALF_GRAY)),
                (( 0, -1).into(), Tile::new_gray(HALF_GRAY)),
                (( 1, -1).into(), Tile::new_gray(HALF_GRAY)),

                ((-1,  0).into(), Tile::new_gray(HALF_GRAY)),
                (( 0,  0).into(), Tile::new_white()),
                (( 1,  0).into(), Tile::new_gray(HALF_GRAY)),
                
                ((-1,  1).into(), Tile::new_gray(HALF_GRAY)),
                (( 0,  1).into(), Tile::new_gray(HALF_GRAY)),
                (( 1,  1).into(), Tile::new_gray(HALF_GRAY)),
            ]),
        }
    }

    pub fn tick_update(&mut self) {
        for tile in self.non_black_tiles_mut() {
            tile.lightness -= 1;
        }
    }

    pub fn make_tile_white(&mut self, x: i32, y: i32) {
        let coordinates: Coordinates = (x, y).into();

        if let Some(tile) = self.active_tiles.get_mut(&coordinates) {
            if tile.is_black() {
                return;
            }

            tile.lightness = MAX_LIGHTNESS;

            for offset in NEIGHBOUR_OFFSETS {
                let neighbour_coords = coordinates + offset.into();
                let neighbour = self.get_or_create_tile(neighbour_coords);

                neighbour.lightness = ((neighbour.lightness as u16 + MAX_LIGHTNESS as u16) / 2) as u8;
            }
        }
    }

    pub fn get_new_tiles(&self, existing_tile_coords: HashSet<Coordinates>) -> Vec<&Coordinates> {
        let tiles = &self.active_tiles;
        
        tiles.into_iter()
            .filter(|(coord, _)| { !existing_tile_coords.contains(coord) })
            .map(|(coord, _)| { coord })
            .collect()
    }

    pub fn non_black_tiles(&self) -> Vec<&Tile> {
        let tiles = &self.active_tiles;
        tiles.values()
            .filter(|tile| { tile.is_not_black() })
            .collect()
    }

    fn non_black_tiles_mut(&mut self) -> Vec<&mut Tile> {
        let tiles = &mut self.active_tiles;
        tiles.values_mut()
            .filter(|tile| { tile.is_not_black() })
            .collect()
    }

    fn get_or_create_tile(&mut self, coordinates: Coordinates) -> &mut Tile {
        if !self.active_tiles.contains_key(&coordinates) {
            let tile = &mut Tile::new_black();
            self.active_tiles.insert(coordinates, *tile);
        }

        self.active_tiles.get_mut(&coordinates).unwrap()
    }

    #[cfg(feature = "debug")]
    pub fn to_string(&self) -> String {
        let tiles = &self.active_tiles;
        tiles.into_iter()
            .map(|(coordinates, tile)| { format!("Tile {}: {}", coordinates, tile.to_string()) })
            .fold("TileMap:\n\t".to_string(), |a, b| a + &b + "\n\t")
            .trim()
            .to_string()
    }
}

impl Deref for TileMap {
    type Target = HashMap<Coordinates, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.active_tiles
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.active_tiles
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    pub lightness: u8,
}

impl Tile {
    pub fn new_white() -> Self {
        Tile::new_gray(MAX_LIGHTNESS)
    }

    pub fn new_black() -> Self {
        Tile::new_gray(0)
    }

    pub fn new_gray(lightness: u8) -> Self {
        Self {
            lightness: lightness,
        }
    }

    pub fn is_white(&self) -> bool {
        self.lightness >= MAX_LIGHTNESS
    }

    pub fn is_black(&self) -> bool {
        self.lightness <= 0
    }

    pub fn is_not_black(&self) -> bool {
        !self.is_black()
    }

    #[cfg(feature = "debug")]
    pub fn to_string(&self) -> String {
        fn get_fg_color(lightness: u8) -> Color {
            if lightness >= HALF_GRAY {
                Color::Black
            } else {
                Color::White
            }
        }

        fn get_bg_color(lightness: u8) -> Color {
            match lightness {
                MAX_LIGHTNESS => Color::BrightWhite,
                0 => Color::Black,
                l => Color::TrueColor {
                    r: l,
                    g: l,
                    b: l,
                },
            }
        }

        format!(
            "{:>3}",
            u8::to_string(&self.lightness)
                .color(get_fg_color(self.lightness))
                .on_color(get_bg_color(self.lightness))
        )
    }
}