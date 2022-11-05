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

        for viewshed_pos in new_viewshed.iter() {
            let idx = map.xy_idx_pos(viewshed_pos);

            map.viewed_tiles[idx] = true;
        }

        viewshed.visible_tiles = new_viewshed;
        viewshed.dirty = false;

        /*
        for (var row = 1; row < maxDistance; row++) {
          for (var col = 0; col <= row; col++) {
            var x = hero.x + col;
            var y = hero.y - row;

            paint(x, y);
          }
        } */
        /*
        let mut new_viewshed = HashSet::new();

        // can see ourselves
        new_viewshed.insert(position.clone());

        for octant in 0..1 {
            let mut shadow_line = ShadowLine {
                shadows: Vec::new(),
            };
            let mut full_shadow = false;

            for row in 1..viewshed.distance {
                for col in 0..=row {
                    let (row_o, col_o) = transform_octant(row as i32, col as i32, octant);
                    let x = position.x + col_o;
                    let y = position.y - row_o;

                    if !map.is_within_bounds(x, y) {
                        break;
                    }

                    if !full_shadow {
                        let mut projection = Shadow::project_tile(row.into(), col.into());

                        let visible = !shadow_line.is_in_shadow(&mut projection);

                        if visible {
                            new_viewshed.insert(Position { x, y });
                        }

                        if visible
                            && map.tiles[map.xy_idx(x as usize, y as usize)] == GameTile::Wall
                        {
                            shadow_line.add(&mut projection);
                            full_shadow = shadow_line.is_full_shadow();
                        }
                    }
                }
            }
        }

        viewshed.visible_tiles = new_viewshed;
        viewshed.dirty = false; */
    }
}

/*
Vec transformOctant(int row, int col, int octant) {
  switch (octant) {
    case 0: return Vec( col, -row);
    case 1: return Vec( row, -col);
    case 2: return Vec( row,  col);
    case 3: return Vec( col,  row);
    case 4: return Vec(-col,  row);
    case 5: return Vec(-row,  col);
    case 6: return Vec(-row, -col);
    case 7: return Vec(-col, -row);
  }
} */

fn transform_octant(row: i32, col: i32, octant: i32) -> (i32, i32) {
    // 2 is actually where i want to start
    match octant {
        //0 => (row, col),   // bottom right
        //1 => (row, -col),  // bottom left
        //2 => (col, -row),  // left bottom
        //3 => (-col, -row), // left top
        //4 => (-row, -col), // top left
        //5 => (-row, col),  // top right
        //6 => (-col, row),  // right top
        //7 => (col, row),   // right bottom
        0 => (col, -row),
        1 => (row, -col),
        2 => (row, col),
        3 => (col, row),
        4 => (-col, row),
        5 => (-row, col),
        6 => (-row, -col),
        7 => (-col, -row),
        _ => (row, col),
    }
}

#[derive(PartialEq, Clone)]
struct Shadow {
    pub start: i32,
    pub end: i32,
}

impl Shadow {
    pub fn project_tile(row: i32, col: i32) -> Shadow {
        let top_left = col / (row + 2);
        let bottom_right = (col + 1) / (row + 1);

        Shadow {
            start: top_left,
            end: bottom_right,
        }
    }

    pub fn contains(&self, other: &Shadow) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

struct ShadowLine {
    pub shadows: Vec<Shadow>,
}

impl ShadowLine {
    pub fn is_in_shadow(&self, projection: &mut Shadow) -> bool {
        for shadow in self.shadows.iter() {
            if shadow.contains(projection) {
                return true;
            }
        }

        return false;
    }

    pub fn is_full_shadow(&self) -> bool {
        self.shadows.len() == 1 && self.shadows[0].start == 0 && self.shadows[0].end == 1
    }

    pub fn add(&mut self, shadow: &mut Shadow) {
        let mut index = 0;
        for ele in self.shadows.iter() {
            if ele.start >= shadow.start {
                break;
            }
            index += 1;
        }

        let shadow_clone = self.shadows.clone();

        let mut overlapping_previous_opt: Option<usize> = None;
        if index > 0 && self.shadows[index - 1].end > shadow.start {
            overlapping_previous_opt = Some(index - 1);
        }

        let mut overlapping_next_opt: Option<usize> = None;
        if index < shadow_clone.len() && shadow_clone[index].start < shadow.end {
            overlapping_next_opt = Some(index);
        }

        if let Some(overlapping_next) = overlapping_next_opt {
            if let Some(overlapping_previous) = overlapping_previous_opt {
                self.shadows[overlapping_previous].end = shadow.end;
                self.shadows.remove(index);
            } else {
                self.shadows[overlapping_next].start = shadow.start;
            }
        } else {
            if let Some(overlapping_previous) = overlapping_previous_opt {
                self.shadows[overlapping_previous].end = shadow.end;
            } else {
                self.shadows.insert(index, shadow.clone());
            }
        }
    }
}
