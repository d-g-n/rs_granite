use bevy::prelude::*;
#[derive(Component, Eq, Hash, PartialEq, Clone)]
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
