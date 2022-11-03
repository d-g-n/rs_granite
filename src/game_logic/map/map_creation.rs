use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game_logic::{
        components::{Blocker, Position, Renderable},
        map::{
            builder::MapBuilder,
            builders::{
                bsp::BSPRoomMapGenerator,
                spawns::RandomFreeSpaceSpawn,
                utils::{FillRoomGenerator, ReplaceVisibleWallsWithBreakableMapGenerator},
            },
            game_map::GameTile,
        },
        resources::PlayerResource,
    },
    rng::GameRNG,
    screen::ScreenContext,
    GameState, InGameState,
};

use super::game_map::{GameMap, GameMapTiles2D};

pub struct MapVisualisation {
    tick_count_ms: u128,
    visualisation_index: usize,
    map: GameMap,
    history: Vec<GameMapTiles2D>,
}

pub fn create_or_load_map(
    mut commands: Commands,
    ctx: Res<ScreenContext>,
    mut rng: ResMut<GameRNG>,
) {
    info!("load map");

    //let new_map = GameMap::new(ctx.width, ctx.height);

    let mut initial_map_builder = MapBuilder::new(ctx.width, ctx.height, rng.as_mut());

    let map_builder = initial_map_builder
        .with_generator(FillRoomGenerator::new(GameTile::UnbreakableWall))
        .with_generator(BSPRoomMapGenerator::new())
        .with_generator(ReplaceVisibleWallsWithBreakableMapGenerator::new())
        .with_generator(RandomFreeSpaceSpawn::new());

    let new_map = map_builder.get_map();

    commands.insert_resource(new_map);

    commands.insert_resource(PlayerResource {
        start_pos: map_builder.get_spawn_position(),
    });

    commands.insert_resource(MapVisualisation {
        tick_count_ms: 0,
        visualisation_index: 0,
        map: map_builder.get_map(),
        history: map_builder.get_history(),
    });

    info!("finish load map");
}

pub fn visualise_map(
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

    if map_vis.tick_count_ms > 250 {
        let current_frame = &map_vis.history[map_vis.visualisation_index];

        for x in 0..ctx.width {
            for y in 0..ctx.height {
                let mut cur_tile = ctx.get_tile(x, y);

                cur_tile.glyph = current_frame[map_vis.map.xy_idx(x, y)].get_char_rep();
                cur_tile.visible = true;
                cur_tile.bg_color = Color::BLACK;
                cur_tile.fg_color = Color::WHITE;
            }
        }

        map_vis.tick_count_ms = 0;
        map_vis.visualisation_index += 1;
    }
}

pub fn finalise_map_creation(mut commands: Commands, mut map: ResMut<GameMap>) {
    let mut tile_components = Vec::new();
    let mut tile_components_blockers = Vec::new();

    // no longer need to hold onto probably lengthly history
    map.clear_history();

    for x in 0..map.width {
        for y in 0..map.height {
            let game_tile = map.tiles[map.xy_idx(x, y)];

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
