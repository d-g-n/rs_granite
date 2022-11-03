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

pub struct DrunkardsWalkMapGenerator {
    pub target_num_drunkards: usize,
    pub drunkard_lifetime: usize,
    pub start_tile: GameTile,
}

impl DrunkardsWalkMapGenerator {
    pub fn new(
        target_num_drunkards: usize,
        drunkard_lifetime: usize,
        start_tile: GameTile,
    ) -> BoxedMapGenerator {
        Box::new(DrunkardsWalkMapGenerator {
            target_num_drunkards,
            drunkard_lifetime,
            start_tile,
        })
    }
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

    fn get_player_spawn(&self, in_map: GameMap, rng: &mut GameRNG) -> Option<Position> {
        None
    }
}
