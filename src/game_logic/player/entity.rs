use bevy::prelude::*;

use crate::game_logic::components::{Player, Position, Renderable};

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {})
        .insert(Position { x: 0, y: 0 })
        .insert(Renderable {
            glyph: '@' as u16,
            fg: Color::YELLOW,
            bg: Color::BLACK,
            layer: 100.0,
        });
}
