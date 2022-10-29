

use crate::{
    sprites::{SpriteAssets, SpriteSizes},
    GameState,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Default)]
pub struct ScreenTile {
    pub x: usize,
    pub y: usize,
    pub glyph: u16,
    pub fg_color: Color,
    pub bg_color: Color,
    pub visible: bool,
    pub layer: f32,
}

#[derive(Clone)]
pub struct ScreenContext {
    pub width: usize,
    pub height: usize,
    pub screen_vec: Vec<Vec<ScreenTile>>,
}

#[derive(Component)]
pub struct ScreenTilePos {
    pub x: usize,
    pub y: usize,
}

pub(crate) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenContext::new(80, 50))
            .add_enter_system(GameState::Loading, init_screen)
            .add_system(
                render_screen.run_if(
                    move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                        GameState::InMenu { .. } => true,
                        GameState::InGame { .. } => true,
                        _ => false,
                    },
                ),
            );
    }
}

impl ScreenContext {
    pub fn new(width: usize, height: usize) -> ScreenContext {
        let mut new_screen_vec = vec![vec![ScreenTile::default(); width]; height];

        for (y, inner_vec) in new_screen_vec.iter_mut().enumerate() {
            for (x, mut screen_tile) in inner_vec.iter_mut().enumerate() {
                screen_tile.x = x;
                screen_tile.y = y;
            }
        }

        ScreenContext {
            width: width,
            height: height,
            screen_vec: new_screen_vec,
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, screen_tile: ScreenTile) {
        self.screen_vec[y][x] = screen_tile;
    }

    pub fn get_tile(&mut self, x: usize, y: usize) -> &mut ScreenTile {
        &mut self.screen_vec[y][x]
    }

    pub fn clear(&mut self) {
        for (y, inner_vec) in self.screen_vec.iter_mut().enumerate() {
            for (x, mut screen_tile) in inner_vec.iter_mut().enumerate() {
                screen_tile.glyph = 0;
                screen_tile.fg_color = Color::BLACK;
                screen_tile.fg_color = Color::BLACK;
            }
        }
    }
}

pub fn init_screen(
    mut commands: Commands,
    ctx: Res<ScreenContext>,
    sprite_sizes: Res<SpriteSizes>,
    sprite_assets: Res<SpriteAssets>,
    mut windows: ResMut<Windows>,
) {
    let primary_window = windows.primary_mut();
    primary_window.set_resolution(
        ctx.width as f32 * sprite_sizes.map_sprite_width,
        ctx.height as f32 * sprite_sizes.map_sprite_height,
    );

    for screen_vec in ctx.screen_vec.iter() {
        for screen_tile in screen_vec.iter() {
            let transform_position_x = (screen_tile.x * sprite_sizes.map_sprite_width as usize)
                as f32
                + (sprite_sizes.map_sprite_width / 2.);
            let transform_position_y = (screen_tile.y * sprite_sizes.map_sprite_height as usize)
                as f32
                + (sprite_sizes.map_sprite_height / 2.);

            let mut char_sprite = TextureAtlasSprite::new(screen_tile.glyph.into());
            char_sprite.color = screen_tile.fg_color;

            commands
                .spawn()
                .insert(ScreenTilePos {
                    x: screen_tile.x,
                    y: screen_tile.y,
                })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: screen_tile.bg_color,
                        custom_size: Some(Vec2::new(
                            sprite_sizes.map_sprite_width,
                            sprite_sizes.map_sprite_height,
                        )),
                        ..default()
                    },
                    ..default()
                })
                .insert_bundle(SpriteSheetBundle {
                    transform: Transform::from_xyz(transform_position_x, transform_position_y, 1.0),
                    visibility: Visibility { is_visible: true },
                    sprite: char_sprite,
                    texture_atlas: sprite_assets.map_tex_atlas.clone(),
                    ..default()
                });
        }
    }
}

pub fn render_screen(
    mut commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    sprite_sizes: Res<SpriteSizes>,
    mut query: Query<(
        Entity,
        &ScreenTilePos,
        &mut Visibility,
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut Sprite,
    )>,
) {
    //ctx.clear();

    for (_entity, screen_tile_pos, mut visibility, mut transform, mut fg_sprite, mut bg_sprite) in
        query.iter_mut()
    {
        let screen_tile = ctx.get_tile(screen_tile_pos.x, screen_tile_pos.y);

        let transform_position_x = (screen_tile.x * sprite_sizes.map_sprite_width as usize) as f32
            + (sprite_sizes.map_sprite_width / 2.);
        let transform_position_y = (screen_tile.y * sprite_sizes.map_sprite_height as usize) as f32
            + (sprite_sizes.map_sprite_height / 2.);

        transform.translation.x = transform_position_x;
        transform.translation.y = transform_position_y;
        fg_sprite.color = screen_tile.fg_color;
        bg_sprite.color = screen_tile.bg_color;
        fg_sprite.index = screen_tile.glyph as usize;
        visibility.is_visible = screen_tile.visible;
    }
}
