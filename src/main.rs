use bevy::prelude::*;
use iyes_loopless::prelude::*;

use rng::GameRNG;

mod camera;
mod game_logic;
mod rng;
mod screen;
mod sprites;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum InGameState {
    LoadMap,
    AwaitingInput,
    GameTurn,
    CleanUp,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum InMenuState {
    MainMenu,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    LoadAssets,
    Loading,
    InMenu { menu_state: InMenuState },
    InGame { game_state: InGameState },
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            // fill the entire browser window
            //fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameRNG::new())
        .add_loopless_state(GameState::LoadAssets)
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(sprites::SpritePlugin)
        .add_plugin(screen::ScreenPlugin)
        .add_plugin(game_logic::GameLogicPlugin)
        .run();
}
