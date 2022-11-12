use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::sprites::{SpriteAssets, SpriteSizes};

use super::structs::{ScreenContext, ScreenGlyph, ScreenTilePos};

pub fn init_screen(
    mut commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    sprite_sizes: Res<SpriteSizes>,
    sprite_assets: Res<SpriteAssets>,
    mut windows: ResMut<Windows>,
) {
    let primary_window = windows.primary_mut();
    primary_window.set_resolution(
        ctx.width as f32 * sprite_sizes.map_sprite_width,
        ctx.height as f32 * sprite_sizes.map_sprite_height,
    );

    for screen_tile in ctx.screen_vec.iter_mut() {
        let glyph_entity = create_sprite_entity(
            &mut commands,
            sprite_sizes.as_ref(),
            sprite_assets.as_ref(),
            &screen_tile.glyph,
            screen_tile.x,
            screen_tile.y,
            false,
            0.,
        );

        screen_tile.sprite_entities = vec![glyph_entity];
    }
}

pub fn render_screen(
    mut commands: Commands,
    mut ctx: ResMut<ScreenContext>,
    sprite_sizes: Res<SpriteSizes>,
    sprite_assets: Res<SpriteAssets>,
    mut query: Query<(
        Entity,
        &ScreenTilePos,
        &mut Visibility,
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut Sprite,
    )>,
) {
    if !ctx.is_changed() {
        return;
    }

    // to check and create missing sprites
    for screen_tile in ctx.screen_vec.iter_mut() {
        let text_count = screen_tile.tile_text.len() + 1;
        let entity_count = screen_tile.sprite_entities.len();

        // situations
        // text count = 1, no text defined, just the underlying glyph
        // entity

        if entity_count > text_count {
            // if entity count is greater than text, then we need to delete entities
            for i in 1..entity_count {
                let text_entity = &screen_tile.sprite_entities[i];

                commands.entity(*text_entity).despawn_recursive();
            }

            screen_tile.sprite_entities.truncate(1);
        } else if entity_count < text_count {
            // if entity count is less than text count, we need to create new text entities
            for i in 0..(text_count - 1) {
                let text_entry = &screen_tile.tile_text[i];

                let glyph_entity = create_sprite_entity(
                    &mut commands,
                    sprite_sizes.as_ref(),
                    sprite_assets.as_ref(),
                    text_entry,
                    screen_tile.x,
                    screen_tile.y,
                    true,
                    i as f32 * sprite_sizes.text_sprite_width,
                );

                screen_tile.sprite_entities.push(glyph_entity);
            }
        }
    }

    for (entity, screen_tile_pos, mut visibility, mut transform, mut fg_sprite, mut bg_sprite) in
        query.iter_mut()
    {
        let screen_tile = ctx.get_tile_mut(screen_tile_pos.x, screen_tile_pos.y);

        // the below block of code, as well as the backwards entity lookup is hideously slow

        let transform_position_x = (screen_tile.x * sprite_sizes.map_sprite_width as usize) as f32
            + (sprite_sizes.map_sprite_width / 2.);
        let transform_position_y = (screen_tile.y * sprite_sizes.map_sprite_height as usize) as f32
            + (sprite_sizes.map_sprite_height / 2.);

        // if the screen_tile has more than one entry, there's the base tile plus text
        let has_text_sprites = screen_tile.sprite_entities.len() > 1;
        // if the current entity is not in position 0, ie not the base tile, then it's text
        let is_entity_text = if has_text_sprites {
            screen_tile
                .sprite_entities
                .iter()
                .position(|e| e == &entity)
                .unwrap()
                > 0
        } else {
            false
        };

        let text_count = screen_tile.tile_text.len() + 1;
        let entity_count = screen_tile.sprite_entities.len();

        if has_text_sprites && is_entity_text && text_count == entity_count {
            visibility.is_visible = true;

            let text_entity_pos = screen_tile
                .sprite_entities
                .iter()
                .position(|e| e == &entity)
                .unwrap();

            let text_info = &screen_tile.tile_text[text_entity_pos - 1];

            fg_sprite.index = text_info.char as usize;
            fg_sprite.color = text_info.fg_color;
            bg_sprite.color = text_info.bg_color;
        } else if !is_entity_text && has_text_sprites {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = screen_tile.glyph.visible;
            fg_sprite.index = screen_tile.glyph.char as usize;
            transform.translation.x = transform_position_x;
            transform.translation.y = transform_position_y;
            fg_sprite.color = screen_tile.glyph.fg_color;
            bg_sprite.color = screen_tile.glyph.bg_color;
        }
    }

    ctx.clear();
}

fn create_sprite_entity(
    commands: &mut Commands,
    sprite_sizes: &SpriteSizes,
    sprite_assets: &SpriteAssets,
    glyph: &ScreenGlyph,
    x: usize,
    y: usize,
    is_text: bool,
    x_adjust: f32,
) -> Entity {
    let sprite_size = if is_text {
        Vec2::new(
            sprite_sizes.text_sprite_width,
            sprite_sizes.text_sprite_height,
        )
    } else {
        Vec2::new(
            sprite_sizes.map_sprite_width,
            sprite_sizes.map_sprite_height,
        )
    };

    let transform_position_x =
        (x * sprite_sizes.map_sprite_width as usize) as f32 + (sprite_size[0] / 2.) + x_adjust;
    let transform_position_y = (y * sprite_sizes.map_sprite_height as usize) as f32
        + (sprite_sizes.map_sprite_height / 2.);

    let mut char_sprite = TextureAtlasSprite::new(glyph.char.into());
    char_sprite.color = glyph.fg_color;

    commands
        .spawn()
        .insert(ScreenTilePos { x, y })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: glyph.bg_color,
                custom_size: Some(sprite_size),
                ..default()
            },
            ..default()
        })
        .insert_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(transform_position_x, transform_position_y, 1.0),
            visibility: Visibility { is_visible: true },
            sprite: char_sprite,
            texture_atlas: if is_text {
                sprite_assets.text_tex_atlas.clone()
            } else {
                sprite_assets.map_tex_atlas.clone()
            },
            ..default()
        })
        .id()
}
