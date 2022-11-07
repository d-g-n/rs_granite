use std::collections::HashSet;

use bevy::{prelude::*, render::view};

use crate::{
    game_logic::{
        components::{Blocker, Player, Position, Renderable, Viewshed},
        map::{game_map::GameMap, pathfinding::astar_next_step},
        resources::PlayerResource,
    },
    screen::ScreenContext,
};

pub fn setup_player(mut commands: Commands, player_res: Res<PlayerResource>) {
    commands
        .spawn()
        .insert(Player {})
        .insert(player_res.start_pos.clone())
        .insert(Renderable {
            glyph: '@' as u16,
            fg: Color::YELLOW,
            bg: Color::BLACK,
            layer: 100.0,
        })
        .insert(Blocker {})
        .insert(Viewshed {
            dirty: true,
            distance: 8,
            visible_tiles: HashSet::new(),
        });
}

#[derive(Default)]
pub struct HeldCounter {
    counter_ms: u128,
    passed_initial_threshold: bool,
}

pub fn handle_player_movement(
    mut _commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut ctx: ResMut<ScreenContext>,
    time: Res<Time>,
    map: Res<GameMap>,
    mut held_counter: Local<HeldCounter>,
    mut player_position_query: Query<(Entity, &mut Position, &mut Viewshed), With<Player>>,
    blocker_position_query: Query<(Entity, &Position), (With<Blocker>, Without<Player>)>,
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

    let (_entity, mut player_pos, mut viewshed) = player_position_query.single_mut();

    let new_x = player_pos.x + direction_x;
    let new_y = player_pos.y + direction_y;

    for (_entity, position) in blocker_position_query.iter() {
        if position.x == new_x && position.y == new_y {
            return;
        }
    }

    if new_x >= 0
        && new_y >= 0
        && new_x < ctx.width as i32
        && new_y < ctx.height as i32
        && held_counter.counter_ms == 0
        && (direction_x != 0 || direction_y != 0)
    {
        player_pos.x = new_x;
        player_pos.y = new_y;

        viewshed.dirty = true;
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
