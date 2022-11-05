use crate::{
    game_logic::{
        components::Position,
        map::{
            builder::{BoxedMapGenerator, MapGenerator},
            game_map::{GameMap, GameTile},
        },
    },
    rng::GameRNG,
    utils::Rectangle,
};

pub struct BSPRoomMapGenerator {}

impl BSPRoomMapGenerator {
    pub fn new() -> BoxedMapGenerator {
        Box::new(BSPRoomMapGenerator {})
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

                    let dog_leg_x;

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
                }
            }
        }

        in_map
    }

    fn get_player_spawn(&self, in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}
