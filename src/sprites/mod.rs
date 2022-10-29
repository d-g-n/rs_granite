use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use iyes_loopless::prelude::*;

use crate::GameState;

pub(crate) struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpriteSizes {
            map_sprite_width: 16.,
            map_sprite_height: 16.,
            text_sprite_width: 8.,
            text_sprite_height: 16.,
        })
        .add_loading_state(
            LoadingState::new(GameState::LoadAssets)
                .continue_to_state(GameState::Loading)
                .with_collection::<SpriteAssets>(),
        )
        .add_enter_system(GameState::Loading, setup_sprites);
    }
}

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 16,
        rows = 16,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "sprites/map/16x16-RogueYun-AgmEdit.png")]
    pub map_tex_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 8.,
        tile_size_y = 16.,
        columns = 16,
        rows = 16,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "sprites/text/8x16-VGA.png")]
    pub text_tex_atlas: Handle<TextureAtlas>,
}

pub struct SpriteSizes {
    pub map_sprite_width: f32,
    pub map_sprite_height: f32,
    pub text_sprite_width: f32,
    pub text_sprite_height: f32,
}

fn setup_sprites(mut commands: Commands) {
    /* commands.insert_resource(NextState(GameState::InMenu {
        menu_state: InMenuState::MainMenu,
    })) */

    commands.insert_resource(NextState(GameState::InGame {
        game_state: crate::InGameState::LoadMap,
    }))
}
