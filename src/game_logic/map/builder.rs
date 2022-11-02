use bevy::{prelude::*, reflect::Map};
use iyes_loopless::prelude::*;

use crate::{game_logic::components::Position, rng::GameRNG};

use super::game_map::{GameMap, GameMapTiles2D, GameTile};

type BoxedMapGenerator = Box<dyn MapGenerator>;

pub struct MapBuilder {
    map: GameMap,
    player_spawn_position: Position,
    history: Vec<GameMapTiles2D>,
}

impl MapBuilder {
    pub fn new(width: usize, height: usize) -> MapBuilder {
        MapBuilder {
            map: GameMap::new(width, height),
            player_spawn_position: Position { x: 0, y: 0 },
            history: Vec::new(),
        }
    }

    pub fn with_generator(
        &mut self,
        rng: &mut GameRNG,
        map_generator: BoxedMapGenerator,
    ) -> &mut MapBuilder {
        let mut final_map = map_generator.generate_map(self.get_map(), rng);

        self.history.append(&mut final_map.history.clone());

        final_map.clear_history();

        self.map = final_map.clone();

        if let Some(new_player_spawn) = map_generator.get_player_spawn(self.get_map(), rng) {
            self.player_spawn_position = new_player_spawn;
        }

        self
    }

    pub fn get_map(&self) -> GameMap {
        self.map.clone()
    }

    pub fn get_spawn_position(&self) -> Position {
        self.player_spawn_position.clone()
    }

    pub fn get_history(&self) -> Vec<GameMapTiles2D> {
        self.history.clone()
    }
}

pub trait MapGenerator {
    fn generate_map(&self, in_map: GameMap, rng: &mut GameRNG) -> GameMap;
    fn get_player_spawn(&self, map: GameMap, rng: &mut GameRNG) -> Option<Position>;
}

pub struct FillRoomGenerator {
    pub tile: GameTile,
}

impl MapGenerator for FillRoomGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        for x in 0..in_map.width {
            for y in 0..in_map.height {
                let idx = in_map.xy_idx(x, y);

                in_map.tiles[idx] = self.tile;
            }
        }
        in_map.snapshot();

        in_map
    }

    fn get_player_spawn(&self, map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct DrunkardsWalkMapGenerator {
    pub target_num_drunkards: usize,
    pub drunkard_lifetime: usize,
    pub start_tile: GameTile,
}

impl MapGenerator for DrunkardsWalkMapGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        /*
        Pick a random point on a filled grid and mark it empty.
        Choose a random cardinal direction (N, E, S, W).
        Move in that direction, and mark it empty unless it already was.
        Repeat steps 2-3, until you have emptied as many grids as desired.

        */

        for i in 0..self.target_num_drunkards {
            let walls = in_map.get_tile_pos_by_type(self.start_tile);
            let mut random_wall_pos = walls[rng.rand_range(0..walls.len() as i32) as usize];
            let idx = in_map.xy_idx(random_wall_pos.0, random_wall_pos.1);

            in_map.tiles[idx] = GameTile::Floor;

            in_map.snapshot();

            let mut cur_tile_pos = (random_wall_pos.0 as i32, random_wall_pos.1 as i32);

            for n in 0..self.drunkard_lifetime {
                let rand_dir = rng.rand_range(0..4);

                let (new_x, new_y) = match rand_dir {
                    1 => (0, 1),
                    2 => (1, 0),
                    3 => (0, -1),
                    4 => (-1, 0),
                    _ => (0, 0),
                };

                let new_x = cur_tile_pos.0 + new_x;
                let new_y = cur_tile_pos.1 + new_y;

                if new_x < 0
                    || new_y < 0
                    || new_x >= in_map.width as i32
                    || new_y >= in_map.height as i32
                {
                    continue;
                }

                cur_tile_pos.0 = new_x;
                cur_tile_pos.1 = new_y;

                let idx = in_map.xy_idx(cur_tile_pos.0 as usize, cur_tile_pos.1 as usize);

                in_map.tiles[idx] = GameTile::Floor;
            }
            in_map.snapshot();
        }

        in_map
    }

    fn get_player_spawn(&self, map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct SymmetricalMapGenerator {
    pub horizontal_symmetry: bool,
    pub vertical_symmetry: bool,
}

impl MapGenerator for SymmetricalMapGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        if self.horizontal_symmetry {
            let initial_x = in_map.width / 2;

            for x in initial_x..in_map.width {
                for y in 0..in_map.height {
                    let idx = in_map.xy_idx(x, y);
                    in_map.tiles[idx] = in_map.tiles[in_map.xy_idx(2 * initial_x - x - 1, y)];
                }
            }

            in_map.snapshot();
        }

        if self.vertical_symmetry {
            let initial_y = in_map.height / 2;

            for x in 0..in_map.width {
                for y in initial_y..in_map.height {
                    let idx = in_map.xy_idx(x, y);

                    in_map.tiles[idx] = in_map.tiles[in_map.xy_idx(x, 2 * initial_y - y - 1)];
                }
            }

            in_map.snapshot();
        }

        in_map
    }

    fn get_player_spawn(&self, map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct RandomFreeSpaceSpawn {}

impl MapGenerator for RandomFreeSpaceSpawn {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        in_map
    }

    fn get_player_spawn(&self, mut map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        let floors = map.get_tile_pos_by_type(GameTile::Floor);
        let mut rand_floor_pos = floors[rng.rand_range(0..floors.len() as i32) as usize];

        Some(Position {
            x: rand_floor_pos.0 as i32,
            y: rand_floor_pos.1 as i32,
        })
    }
}

pub struct ReplaceVisibleWallsWithBreakableMapGenerator {}

impl MapGenerator for ReplaceVisibleWallsWithBreakableMapGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        for x in 0..in_map.width {
            for y in 0..in_map.height {
                if in_map.tiles[in_map.xy_idx(x, y)] == GameTile::UnbreakableWall
                    && in_map.get_adjacent_count_by_type((x as i32, y as i32), GameTile::Floor) > 0
                {
                    let idx = in_map.xy_idx(x, y);

                    in_map.tiles[idx] = GameTile::Wall;
                }
            }
        }

        in_map.snapshot();

        in_map
    }

    fn get_player_spawn(&self, mut map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct BSPRoomMapGenerator {}

#[derive(Clone, Debug)]
struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn h_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    fn w_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    fn center(&self) -> (usize, usize) {
        ((self.x + self.width / 2), (self.y + self.height / 2))
    }

    fn all_squares(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        for x in self.x..(self.x + self.width) {
            for y in self.y..(self.y + self.height) {
                res.push((x, y));
            }
        }

        res
    }
}

#[derive(Clone, Debug)]
struct BSPPartition {
    rect: Rectangle,
    children: Box<Option<(BSPPartition, BSPPartition)>>,
}

impl BSPPartition {
    fn new(rect: Rectangle) -> BSPPartition {
        BSPPartition {
            rect,
            children: Box::new(None),
        }
    }
}

impl MapGenerator for BSPRoomMapGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        fn split_bsp(mut partition: BSPPartition, count: usize, rng: &mut GameRNG) -> BSPPartition {
            if count > 0 {
                let child_rects = split_random(&partition.rect, rng);
                partition.children = Box::new(Some((
                    split_bsp(
                        BSPPartition {
                            rect: child_rects.0,
                            children: Box::new(None),
                        },
                        count - 1,
                        rng,
                    ),
                    split_bsp(
                        BSPPartition {
                            rect: child_rects.1,
                            children: Box::new(None),
                        },
                        count - 1,
                        rng,
                    ),
                )))
            }

            partition
        }

        fn split_random(rect: &Rectangle, rng: &mut GameRNG) -> (Rectangle, Rectangle) {
            let dir_rng = rng.rand_range_incl(0..=1);

            // nice rng range is 0.45 - 0.55 rand
            // so if it's width of 1 -> rect height, that range is
            // 5% - mid -> 5% + mid

            if dir_rng == 0 {
                // horizontal

                let left = Rectangle {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rng.rand_range_incl(
                        (0.3 * rect.height as f32) as i32..=(0.7 * rect.height as f32) as i32,
                    ) as usize,
                };

                let right = Rectangle {
                    x: rect.x,
                    y: rect.y + left.height,
                    width: rect.width,
                    height: rect.height - left.height,
                };

                if left.h_ratio() < 0.4 || right.h_ratio() < 0.4 {
                    return split_random(rect, rng);
                }

                (left, right)
            } else {
                // vertical

                let top = Rectangle {
                    x: rect.x,
                    y: rect.y,
                    width: rng.rand_range_incl(
                        (0.3 * rect.width as f32) as i32..=(0.7 * rect.width as f32) as i32,
                    ) as usize,
                    height: rect.height,
                };

                let bottom = Rectangle {
                    x: rect.x + top.width,
                    y: rect.y,
                    width: rect.width - top.width,
                    height: rect.height,
                };

                if top.w_ratio() < 0.4 || bottom.w_ratio() < 0.4 {
                    return split_random(rect, rng);
                }

                (top, bottom)
            }
        }

        let res = split_bsp(
            BSPPartition::new(Rectangle {
                x: 1,
                y: 1,
                width: in_map.width - 2,
                height: in_map.height - 2,
            }),
            4,
            rng,
        );

        fn collect_leaf(partition: &BSPPartition) -> Vec<Rectangle> {
            let mut res = Vec::new();
            let child = &*partition.children;

            // if a node has children, and those children do not have children, then those are siblings

            if let Some((l_part, r_part)) = child {
                res.append(&mut collect_leaf(l_part));
                res.append(&mut collect_leaf(r_part));
            } else {
                res.push(partition.rect.clone());
            }

            res
        }

        let leaves = collect_leaf(&res);
        let mut inner_rects = Vec::new();

        for (i, area) in leaves.iter().enumerate() {
            let room_x = area.x + rng.rand_range_incl(0..=area.width as i32 / 3) as usize;
            let room_y = area.y + rng.rand_range_incl(0..=area.height as i32 / 3) as usize;
            let room_w_p = area.width - (room_x - area.x);
            let room_h_p = area.height - (room_y - area.y);
            let room_w = room_w_p - rng.rand_range_incl(0..=room_w_p as i32 / 3) as usize;
            let room_h = room_h_p - rng.rand_range_incl(0..=room_h_p as i32 / 3) as usize;

            inner_rects.push(Rectangle {
                x: room_x,
                y: room_y,
                width: room_w,
                height: room_h,
            });

            in_map.draw_square(
                room_x,
                room_y,
                room_w,
                room_h,
                GameTile::Floor,
                GameTile::Floor,
            );

            in_map.snapshot();

            if i > 0 {
                let last = &inner_rects[i - 1];
                let cur = &inner_rects[i];

                let cur_all_squares = cur.all_squares();
                let last_all_squares = last.all_squares();

                // for all points within the last one, make a vec that's a pair where all x equal or all y equal

                let mut x_pairs: Vec<((usize, usize), (usize, usize))> = Vec::new();
                let mut y_pairs: Vec<((usize, usize), (usize, usize))> = Vec::new();

                for (cur_x, cur_y) in &cur_all_squares {
                    for (last_x, last_y) in &last_all_squares {
                        if *cur_x == *last_x {
                            x_pairs.push(((*cur_x, *cur_y), (*last_x, *last_y)));
                        }

                        if *cur_y == *last_y {
                            y_pairs.push(((*cur_x, *cur_y), (*last_x, *last_y)));
                        }
                    }
                }

                if !x_pairs.is_empty() {
                    let rand_x_pair = x_pairs[rng.rand_range(0..x_pairs.len() as i32) as usize];

                    if rand_x_pair.0 .1 < rand_x_pair.1 .1 {
                        in_map.draw_square(
                            rand_x_pair.0 .0,
                            rand_x_pair.0 .1,
                            1,
                            rand_x_pair.1 .1 - rand_x_pair.0 .1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    } else {
                        in_map.draw_square(
                            rand_x_pair.1 .0,
                            rand_x_pair.1 .1,
                            1,
                            rand_x_pair.0 .1 - rand_x_pair.1 .1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    }

                    in_map.snapshot();
                } else if !y_pairs.is_empty() {
                    let rand_y_pair = y_pairs[rng.rand_range(0..y_pairs.len() as i32) as usize];

                    if rand_y_pair.0 .0 < rand_y_pair.1 .0 {
                        in_map.draw_square(
                            rand_y_pair.0 .0,
                            rand_y_pair.0 .1,
                            rand_y_pair.1 .0 - rand_y_pair.0 .0,
                            1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    } else {
                        in_map.draw_square(
                            rand_y_pair.1 .0,
                            rand_y_pair.1 .1,
                            rand_y_pair.0 .0 - rand_y_pair.1 .0,
                            1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    }

                    in_map.snapshot();
                } else {
                    let (cur_rand_x, cur_rand_y) =
                        *&cur_all_squares[rng.rand_range(0..cur_all_squares.len() as i32) as usize];
                    let (last_rand_x, last_rand_y) = *&last_all_squares
                        [rng.rand_range(0..last_all_squares.len() as i32) as usize];

                    let mut dog_leg_x;

                    if last_rand_x < cur_rand_x {
                        let w = cur_rand_x - last_rand_x + 1;
                        dog_leg_x = last_rand_x + w;

                        in_map.draw_square(
                            last_rand_x,
                            last_rand_y,
                            w,
                            1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    } else {
                        let w = last_rand_x - cur_rand_x + 1;
                        dog_leg_x = cur_rand_x + w;

                        in_map.draw_square(
                            cur_rand_x,
                            cur_rand_y,
                            w,
                            1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    }

                    if last_rand_y < cur_rand_y {
                        // lower, grow up
                        in_map.draw_square(
                            dog_leg_x,
                            last_rand_y,
                            1,
                            cur_rand_y - last_rand_y + 1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    } else {
                        in_map.draw_square(
                            dog_leg_x,
                            cur_rand_y,
                            1,
                            last_rand_y - cur_rand_y + 1,
                            GameTile::Floor,
                            GameTile::Floor,
                        );
                    }

                    in_map.snapshot();

                    println!("dog leg")
                }
            }
        }

        in_map
    }

    fn get_player_spawn(&self, mut map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}
