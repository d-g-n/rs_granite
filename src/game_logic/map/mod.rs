use std::borrow::BorrowMut;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game_logic::{
        components::{Blocker, Position, Renderable},
        map::builder::{
            DrunkardsWalkMapGenerator, MapBuilder, RandomFreeSpaceSpawn,
            ReplaceVisibleWallsWithBreakableMapGenerator, SquareRoomMapGenerator,
            SymmetricalMapGenerator,
        },
        resources::PlayerResource,
    },
    rng::GameRNG,
    screen::ScreenContext,
    GameState, InGameState,
};

pub mod builder;
pub mod pathfinding;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
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

    pub fn get_tile_pos_by_type(&mut self, tile: GameTile) -> Vec<(usize, usize)> {
        let mut res_vec = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[x][y] == tile {
                    res_vec.push((x, y));
                }
            }
        }

        res_vec
    }

    pub fn get_adjacent_tiles(&mut self, pos: (i32, i32)) -> Vec<(usize, usize, GameTile)> {
        let (x, y) = pos;
        let adjacent_coords = vec![
            (x, y + 1),     // North
            (x, y - 1),     // South
            (x + 1, y),     // East
            (x - 1, y),     // West
            (x + 1, y + 1), // NE
            (x - 1, y + 1), // NW
            (x + 1, y - 1), // SE
            (x - 1, y - 1), // SW
        ];

        let mut res_vec = Vec::new();

        for (new_x, new_y) in adjacent_coords {
            if new_x < 0 || new_y < 0 || new_x >= self.width as i32 || new_y >= self.height as i32 {
                continue;
            }
            res_vec.push((
                new_x as usize,
                new_y as usize,
                self.tiles[new_x as usize][new_y as usize],
            ))
        }

        res_vec
    }

    pub fn get_adjacent_count_by_type(&mut self, pos: (i32, i32), tile: GameTile) -> usize {
        let adjacent_tiles = self.get_adjacent_tiles(pos);
        let mut res_count = 0;

        for (_x, _y, adjacent_tile) in adjacent_tiles {
            if adjacent_tile == tile {
                res_count += 1;
            }
        }

        res_count
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

    let map_builder = initial_map_builder
        .with_generator(rng.as_mut(), Box::new(SquareRoomMapGenerator {}))
        .with_generator(
            rng.as_mut(),
            Box::new(DrunkardsWalkMapGenerator {
                target_num_drunkards: 50,
                drunkard_lifetime: 50,
            }),
        )
        .with_generator(
            rng.as_mut(),
            Box::new(SymmetricalMapGenerator {
                horizontal_symmetry: true,
                vertical_symmetry: true,
            }),
        )
        .with_generator(rng.as_mut(), Box::new(RandomFreeSpaceSpawn {}))
        .with_generator(
            rng.as_mut(),
            Box::new(ReplaceVisibleWallsWithBreakableMapGenerator {}),
        );

    let new_map = map_builder.get_map();

    commands.insert_resource(new_map);

    commands.insert_resource(PlayerResource {
        start_pos: map_builder.get_spawn_position(),
    });

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
