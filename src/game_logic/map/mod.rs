use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game_logic::components::{Blocker, Position, Renderable},
    rng::GameRNG,
    screen::ScreenContext,
    GameState, InGameState,
};

mod builder;
pub mod pathfinding;

#[derive(Clone, Copy)]
pub enum GameTile {
    Floor,
    Wall,
    UnbreakableWall,
    DownStairs,
    UpStairs,
}

impl GameTile {
    pub fn get_char_rep(&self) -> u16 {
        match self {
            GameTile::Floor => '.' as u16,
            GameTile::Wall => '#' as u16,
            GameTile::UnbreakableWall => 178 as u16, // ▓, doesn't like this one
            GameTile::DownStairs => '▼' as u16,
            GameTile::UpStairs => '▲' as u16,
        }
    }

    pub fn is_blocker(&self) -> bool {
        match self {
            GameTile::Floor => false,
            GameTile::Wall => true,
            GameTile::UnbreakableWall => true,
            GameTile::DownStairs => false,
            GameTile::UpStairs => false,
        }
    }
}

#[derive(Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<GameTile>>,
}

impl GameMap {
    pub fn new(width: usize, height: usize, rng: &mut GameRNG) -> GameMap {
        let mut new_map = vec![vec![GameTile::UnbreakableWall; width]; height];
        for x in 0..width {
            for y in 0..height {
                if rng.rand_dice("d10") > 8 {
                    continue;
                }
                new_map[y][x] = GameTile::Floor;
            }
        }
        GameMap {
            width,
            height,
            tiles: new_map,
        }
    }
}

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            GameState::InGame {
                game_state: InGameState::LoadMap,
            },
            create_or_load_map,
        );
    }
}

fn create_or_load_map(mut commands: Commands, ctx: Res<ScreenContext>, mut rng: ResMut<GameRNG>) {
    info!("load map");

    let new_map = GameMap::new(ctx.width, ctx.height, &mut rng);
    let tiles = new_map.tiles.clone();

    commands.insert_resource(new_map.clone());

    let mut tile_components = Vec::new();
    let mut tile_components_blockers = Vec::new();

    for x in 0..new_map.width {
        for y in 0..new_map.height {
            let game_tile = tiles[y][x];

            if game_tile.is_blocker() {
                tile_components_blockers.push((
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    Renderable {
                        glyph: game_tile.get_char_rep(),
                        fg: Color::WHITE,
                        bg: Color::BLACK,
                        layer: 0.0,
                    },
                    Blocker {},
                ));
            } else {
                tile_components.push((
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    Renderable {
                        glyph: game_tile.get_char_rep(),
                        fg: Color::WHITE,
                        bg: Color::BLACK,
                        layer: 0.0,
                    },
                ));
            }
        }
    }

    commands.spawn_batch(tile_components);
    commands.spawn_batch(tile_components_blockers);

    commands.insert_resource(NextState(GameState::InGame {
        game_state: InGameState::AwaitingInput,
    }));
}
