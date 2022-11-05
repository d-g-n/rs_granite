use std::collections::HashSet;

use bevy::prelude::*;
#[derive(Component, Eq, Hash, PartialEq, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: Color,
    pub bg: Color,
    pub layer: f32,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Blocker {}

#[derive(Component)]
pub struct Viewshed {
    pub dirty: bool,
    pub distance: u16,
    pub visible_tiles: HashSet<Position>,
}
