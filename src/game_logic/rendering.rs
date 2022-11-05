use std::collections::HashMap;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::screen::ScreenContext;

use super::{
    components::{Player, Position, Renderable, Viewshed},
    map::game_map::{GameMap, GameTile},
};

pub fn handle_renderable(
    mut _commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    map: ResMut<GameMap>,
    mut query: Query<(Entity, &Position, &Renderable)>,
    mut viewshed_visibility_query: Query<&Viewshed, With<Player>>,
) {
    let mut position_visibility_history: HashMap<Position, f32> = HashMap::new();

    let player_viewshed = viewshed_visibility_query.single();

    ctx.clear();

    for (_entity, position, renderable) in query.iter_mut() {
        let mut screen_tile = ctx.get_tile(position.x as usize, position.y as usize);

        if !position_visibility_history.contains_key(position) {
            position_visibility_history.insert(position.clone(), renderable.layer);
        }

        if let Some(last_layer) = position_visibility_history.get(position) {
            if renderable.layer < *last_layer {
                continue;
            } else {
                position_visibility_history.insert(position.clone(), renderable.layer);
            }
        }

        let glyph = if renderable.glyph == GameTile::Wall.get_char_rep() {
            smooth_wall_rendering(&map, position.x, position.y)
        } else {
            renderable.glyph
        };

        screen_tile.visible = player_viewshed.visible_tiles.contains(position)
            || map.viewed_tiles[map.xy_idx_pos(position)];

        screen_tile.glyph = glyph;
        screen_tile.layer = renderable.layer;

        if player_viewshed.visible_tiles.contains(position) {
            screen_tile.fg_color = renderable.fg;
            screen_tile.bg_color = renderable.bg;
        } else {
            let linear_fg = renderable.fg.r() * 0.2126
                + renderable.fg.g() * 0.7152
                + renderable.fg.b() * 0.0722;

            let linear_bg = renderable.bg.r() * 0.2126
                + renderable.bg.g() * 0.7152
                + renderable.bg.b() * 0.0722;

            screen_tile.fg_color = Color::rgb(linear_fg, linear_fg, linear_fg);
            screen_tile.bg_color = Color::rgb(linear_bg, linear_bg, linear_bg);
        }
    }
}

fn smooth_wall_rendering(map: &GameMap, x: i32, y: i32) -> u16 {
    if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 {
        return 35;
    }
    let mut mask: u8 = 0;

    let x = x as usize;
    let y = y as usize;

    fn is_revealed_and_wall(map: &GameMap, x: usize, y: usize) -> bool {
        if x >= map.width || y >= map.height {
            false
        } else {
            let idx = map.xy_idx(x, y);

            map.viewed_tiles[idx] && map.tiles[idx] == GameTile::Wall
        }
    }

    if is_revealed_and_wall(map, x, y + 1) {
        mask += 1;
    }
    if y > 0 && is_revealed_and_wall(map, x, y - 1) {
        mask += 2;
    }
    if x > 0 && is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}
