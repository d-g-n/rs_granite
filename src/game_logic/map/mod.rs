use std::borrow::BorrowMut;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game_logic::{
        components::{Blocker, Position, Renderable},
        map::builder::{MapBuilder, MapWithSquareRoom},
    },
    rng::GameRNG,
    screen::ScreenContext,
    GameState, InGameState,
};

pub mod builder;
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

type GameMapTiles2D = Vec<Vec<GameTile>>;
#[derive(Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: GameMapTiles2D,
    pub history: Vec<GameMapTiles2D>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> GameMap {
        let new_map = vec![vec![GameTile::Floor; height]; width];

        GameMap {
            width,
            height,
            tiles: new_map,
            history: Vec::new(),
        }
    }

    pub fn fill(&mut self, tile: GameTile) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.tiles[x][y] = tile;
            }
        }
    }

    pub fn snapshot(&mut self) {
        self.history.push(self.tiles.clone())
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
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
        )
        .add_system(visualise_map.run_in_state(GameState::InGame {
            game_state: InGameState::LoadMap,
        }))
        .add_exit_system(
            GameState::InGame {
                game_state: InGameState::LoadMap,
            },
            finalise_map_creation,
        );
    }
}

struct MapVisualisation {
    tick_count_ms: u128,
    visualisation_index: usize,
    history: Vec<GameMapTiles2D>,
}

fn create_or_load_map(mut commands: Commands, ctx: Res<ScreenContext>, mut rng: ResMut<GameRNG>) {
    info!("load map");

    //let new_map = GameMap::new(ctx.width, ctx.height);

    let mut initial_map_builder = MapBuilder::new(ctx.width, ctx.height);

    let map_builder =
        initial_map_builder.with_generator(rng.as_mut(), Box::new(MapWithSquareRoom {}));

    let new_map = map_builder.get_map();

    commands.insert_resource(new_map);

    commands.insert_resource(MapVisualisation {
        tick_count_ms: 0,
        visualisation_index: 0,
        history: map_builder.get_history(),
    });

    info!("finish load map");
}

fn visualise_map(
    mut commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    mut map_vis: ResMut<MapVisualisation>,
    //map: Res<GameMap>,
    time: Res<Time>,
) {
    if map_vis.visualisation_index >= map_vis.history.len() {
        commands.remove_resource::<MapVisualisation>();
        commands.insert_resource(NextState(GameState::InGame {
            game_state: InGameState::AwaitingInput,
        }));

        info!("finish vis");

        return;
    }

    map_vis.tick_count_ms += time.delta().as_millis();

    if map_vis.tick_count_ms > 100 {
        let current_frame = &map_vis.history[map_vis.visualisation_index];

        for x in 0..ctx.width {
            for y in 0..ctx.height {
                let mut cur_tile = ctx.get_tile(x, y);

                cur_tile.glyph = current_frame[x][y].get_char_rep();
                cur_tile.visible = true;
                cur_tile.bg_color = Color::BLACK;
                cur_tile.fg_color = Color::WHITE;
            }
        }

        map_vis.tick_count_ms = 0;
        map_vis.visualisation_index += 1;
    }
}

fn finalise_map_creation(mut commands: Commands, mut map: ResMut<GameMap>) {
    let mut tile_components = Vec::new();
    let mut tile_components_blockers = Vec::new();

    // no longer need to hold onto probably lengthly history
    map.clear_history();

    for x in 0..map.width {
        for y in 0..map.height {
            let game_tile = map.tiles[x][y];

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
}

fn draw_map(ctx: &ScreenContext, map: &GameMap) {}
