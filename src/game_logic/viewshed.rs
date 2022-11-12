use std::collections::HashSet;

use bevy::{prelude::*, render::view};
use iyes_loopless::prelude::*;

use super::{
    components::{Position, Renderable, Viewshed},
    map::game_map::GameMap,
};

pub fn handle_viewshed_updating(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut viewshed_query: Query<(Entity, &Position, &mut Viewshed), Changed<Viewshed>>,
) {
    for (_entity, position, mut viewshed) in viewshed_query.iter_mut() {
        let mut new_viewshed = HashSet::new();

        // for a circle of radius viewshed.distance, draw a bres line to each tile on the circle

        new_viewshed.insert(position.clone());

        for i in (0..360).step_by(3) {
            let x_rot = (i as f32 * 0.01745).cos();
            let y_rot = (i as f32 * 0.01745).sin();

            let mut ox = position.x as f32 + 0.5;
            let mut oy = position.y as f32 + 0.5;

            for j in 0..viewshed.distance {
                if !map.is_within_bounds(ox as i32, oy as i32) {
                    continue;
                }

                let cell = map.tiles[map.xy_idx(ox as usize, oy as usize)];

                new_viewshed.insert(Position {
                    x: ox as i32,
                    y: oy as i32,
                });

                if cell.is_opaque() {
                    break;
                }

                ox += x_rot;
                oy += y_rot;
            }
        }

        /* for pos in bres_circle(position, viewshed.distance as i32) {
            let line = bresenhams_line(position, &pos);

            let mut stop_on_next = false;

            for line_pos in line {
                if stop_on_next {
                    break;
                }

                let is_opaque = map.is_opaque(line_pos.x, line_pos.y);
                if map.is_within_bounds(line_pos.x, line_pos.y) {
                    // can make this more efficient probably by just evaluating the line in here and bailing early
                    new_viewshed.insert(line_pos);
                }

                if is_opaque {
                    stop_on_next = true;
                }
            }
        } */

        for viewshed_pos in new_viewshed.iter() {
            let idx = map.xy_idx_pos(viewshed_pos);

            map.viewed_tiles[idx] = true;
        }

        viewshed.visible_tiles = new_viewshed;
        viewshed.dirty = false;
    }
}

fn bres_circle(center: &Position, radius: i32) -> Vec<Position> {
    let mut d = 3 - 2 * radius;
    let mut x = 0;
    let mut y = radius;

    let Position { x: p, y: q } = center;

    let mut out_pos = Vec::new();

    while x < y {
        out_pos.push((x + p, y + q));
        out_pos.push((y + p, x + q));
        out_pos.push((-y + p, x + q));
        out_pos.push((-x + p, y + q));
        out_pos.push((-x + p, -y + q));
        out_pos.push((-y + p, -x + q));
        out_pos.push((y + p, -x + q));
        out_pos.push((x + p, -y + q));

        if d < 0 {
            d = d + 4 * x + 6;
        } else {
            d = d + 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }

    out_pos
        .iter()
        .map(|(px, py)| Position { x: *px, y: *py })
        .collect()
}

fn bresenhams_line(start: &Position, end: &Position) -> Vec<Position> {
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();

    // slope bool indicates when slope >= 1

    fn get_line_positions(
        (mut x1, mut y1): (i32, i32),
        (x2, y2): (i32, i32),
        (dx, dy): (i32, i32),
        slope_decision: bool,
    ) -> Vec<Position> {
        let mut pk = 2 * dy - dx;

        let mut res = Vec::new();

        for _i in 0..=dx {
            if x1 < x2 {
                x1 += 1;
            } else {
                x1 -= 1;
            }

            if pk < 0 {
                if !slope_decision {
                    res.push(Position { x: x1, y: y1 });
                } else {
                    res.push(Position { x: y1, y: x1 });
                }

                pk = pk + 2 * dy;
            } else {
                if y1 < y2 {
                    y1 += 1;
                } else {
                    y1 -= 1;
                }

                if !slope_decision {
                    res.push(Position { x: x1, y: y1 });
                } else {
                    res.push(Position { x: y1, y: x1 });
                }

                pk = pk + 2 * dy - 2 * dx;
            }
        }

        res
    }

    if dx > dy {
        get_line_positions((start.x, start.y), (end.x, end.y), (dx, dy), false)
    } else {
        get_line_positions((start.y, start.x), (end.y, end.x), (dy, dx), true)
    }
}
