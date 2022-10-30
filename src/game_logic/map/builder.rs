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
            player_spawn_position: Position {
                x: (width / 2) as i32,
                y: (height / 2) as i32,
            },
            history: Vec::new(),
        }
    }

    pub fn with_generator(
        &mut self,
        rng: &mut GameRNG,
        map_generator: BoxedMapGenerator,
    ) -> &mut MapBuilder {
        let final_map = map_generator.generate_map(self.get_map(), rng);

        self.map = final_map.clone();
        self.history.append(&mut final_map.history.clone());

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
}

pub struct MapWithSquareRoom {}

impl MapGenerator for MapWithSquareRoom {
    fn generate_map(&self, mut in_map: GameMap, rng: &mut GameRNG) -> GameMap {
        for x in 0..in_map.width / 2 {
            for y in 0..in_map.height / 2 {
                in_map.tiles[x][y] = GameTile::UnbreakableWall;
                in_map.snapshot();
            }
        }

        in_map
    }
}
