use bevy::{prelude::*, reflect::Map};
use iyes_loopless::prelude::*;

use crate::{game_logic::components::Position, rng::GameRNG};

use super::{GameMap, GameMapTiles2D, GameTile};

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

pub struct SquareRoomMapGenerator {}

impl MapGenerator for SquareRoomMapGenerator {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        for x in 0..in_map.width {
            for y in 0..in_map.height / 2 {
                in_map.tiles[x][y] = GameTile::UnbreakableWall;
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
            let walls = in_map.get_tile_pos_by_type(GameTile::UnbreakableWall);
            let mut random_wall_pos = walls[rng.rand_range(0..walls.len() as i32) as usize];

            in_map.tiles[random_wall_pos.0][random_wall_pos.1] = GameTile::Floor;

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

                in_map.tiles[cur_tile_pos.0 as usize][cur_tile_pos.1 as usize] = GameTile::Floor;
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
                    in_map.tiles[x][y] = in_map.tiles[2 * initial_x - x - 1][y];
                }
            }

            in_map.snapshot();
        }

        if self.vertical_symmetry {
            let initial_y = in_map.height / 2;

            for x in 0..in_map.width {
                for y in initial_y..in_map.height {
                    in_map.tiles[x][y] = in_map.tiles[x][2 * initial_y - y - 1];
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
                if in_map.tiles[x][y] == GameTile::UnbreakableWall
                    && in_map.get_adjacent_count_by_type((x as i32, y as i32), GameTile::Floor) > 0
                {
                    in_map.tiles[x][y] = GameTile::Wall;
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
