use crate::{
    game_logic::{
        components::Position,
        map::{
            builder::{BoxedMapGenerator, MapGenerator},
            game_map::{GameMap, GameTile},
        },
    },
    rng::GameRNG,
};

pub struct FillRoomGenerator {
    pub tile: GameTile,
}

impl FillRoomGenerator {
    pub fn new(tile: GameTile) -> BoxedMapGenerator {
        Box::new(FillRoomGenerator { tile })
    }
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

    fn get_player_spawn(&self, mut in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct SymmetricalMapGenerator {
    pub horizontal_symmetry: bool,
    pub vertical_symmetry: bool,
}

impl SymmetricalMapGenerator {
    pub fn new(horizontal_symmetry: bool, vertical_symmetry: bool) -> BoxedMapGenerator {
        Box::new(SymmetricalMapGenerator {
            horizontal_symmetry,
            vertical_symmetry,
        })
    }
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

    fn get_player_spawn(&self, in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}

pub struct ReplaceVisibleWallsWithBreakableMapGenerator {}

impl ReplaceVisibleWallsWithBreakableMapGenerator {
    pub fn new() -> BoxedMapGenerator {
        Box::new(ReplaceVisibleWallsWithBreakableMapGenerator {})
    }
}

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

    fn get_player_spawn(&self, mut in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}
