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

pub struct RandomFreeSpaceSpawn {}

impl RandomFreeSpaceSpawn {
    pub fn new() -> BoxedMapGenerator {
        Box::new(RandomFreeSpaceSpawn {})
    }
}

impl MapGenerator for RandomFreeSpaceSpawn {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        in_map
    }

    fn get_player_spawn(&self, mut in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        let floors = in_map.get_tile_pos_by_type(GameTile::Floor);
        let mut rand_floor_pos = floors[rng.rand_range(0..floors.len() as i32) as usize];

        Some(Position {
            x: rand_floor_pos.0 as i32,
            y: rand_floor_pos.1 as i32,
        })
    }
}
