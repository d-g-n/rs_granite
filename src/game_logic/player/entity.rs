use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game_logic::components::{Player, Position, Renderable},
    screen::ScreenContext,
};

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

#[derive(Default)]
pub struct HeldCounter {
    counter_ms: u128,
    passed_initial_threshold: bool,
}

pub fn handle_player_movement(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    ctx: Res<ScreenContext>,
    mut player_position: Query<(Entity, &mut Position), With<Player>>,
    time: Res<Time>,
    mut held_counter: Local<HeldCounter>,
) {
    let (direction_x, direction_y) =
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            (0, 1)
        } else if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            (-1, 0)
        } else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            (0, -1)
        } else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            (1, 0)
        } else {
            if held_counter.counter_ms != 0 {
                *held_counter = HeldCounter::default();
            }

            (0, 0)
        };

    let (_entity, mut player_pos) = player_position.single_mut();

    let new_x = player_pos.x + direction_x;
    let new_y = player_pos.y + direction_y;

    if new_x >= 0
        && new_y >= 0
        && new_x < ctx.width as i32
        && new_y < ctx.height as i32
        && held_counter.counter_ms == 0
    {
        player_pos.x = new_x;
        player_pos.y = new_y;
    }

    if !(direction_x == 0 && direction_y == 0) {
        held_counter.counter_ms += time.delta().as_millis();
    }

    if held_counter.passed_initial_threshold {
        if held_counter.counter_ms >= 100 {
            held_counter.counter_ms = 0;
        }
    } else {
        if held_counter.counter_ms >= 300 {
            held_counter.passed_initial_threshold = true;
        }
    }
}
