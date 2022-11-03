use bevy::{prelude::*, reflect::Map};
use iyes_loopless::prelude::*;

use crate::{game_logic::components::Position, rng::GameRNG};

use super::game_map::{GameMap, GameMapTiles2D, GameTile};

pub type BoxedMapGenerator = Box<dyn MapGenerator>;

pub struct MapBuilder<'a> {
    map: GameMap,
    player_spawn_position: Position,
    history: Vec<GameMapTiles2D>,
    rng: &'a mut GameRNG,
}

impl<'a> MapBuilder<'a> {
    pub fn new(width: usize, height: usize, rng: &mut GameRNG) -> MapBuilder {
        MapBuilder {
            map: GameMap::new(width, height),
            player_spawn_position: Position { x: 0, y: 0 },
            history: Vec::new(),
            rng: rng,
        }
    }

    pub fn with_generator<'b>(
        &'b mut self,
        map_generator: BoxedMapGenerator,
    ) -> &'b mut MapBuilder<'a> {
        let mut final_map = map_generator.generate_map(self.get_map(), self.rng);

        self.history.append(&mut final_map.history.clone());

        final_map.clear_history();

        self.map = final_map.clone();

        if let Some(new_player_spawn) = map_generator.get_player_spawn(self.get_map(), self.rng) {
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
    fn get_player_spawn(&self, in_map: GameMap, rng: &mut GameRNG) -> Option<Position>;
}
