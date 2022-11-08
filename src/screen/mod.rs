use crate::{
    sprites::{SpriteAssets, SpriteSizes},
    GameState,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct ScreenGlyph {
    pub char: u16,
    pub fg_color: Color,
    pub bg_color: Color,
    pub visible: bool,
    pub layer: f32,
}

#[derive(Clone, Default, Debug)]
pub struct ScreenTile {
    pub x: usize,
    pub y: usize,
    pub glyph: ScreenGlyph,
    pub tile_text: Vec<ScreenGlyph>,
    pub sprite_entities: Vec<Entity>,
}

#[derive(Clone)]
pub struct ScreenContext {
    pub width: usize,
    pub height: usize,
    sprite_sizes: SpriteSizes,
    pub screen_vec: Vec<ScreenTile>,
}

#[derive(Component)]
pub struct ScreenTilePos {
    pub x: usize,
    pub y: usize,
}

pub(crate) struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenContext::new(
            80,
            50,
            SpriteSizes {
                map_sprite_width: 16.,
                map_sprite_height: 16.,
                text_sprite_width: 8.,
                text_sprite_height: 16.,
            },
        ))
        .add_enter_system(GameState::Loading, init_screen)
        .add_system(render_screen.run_if(
            move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                GameState::InMenu { .. } => true,
                GameState::InGame { .. } => true,
                _ => false,
            },
        ));
    }
}

#[derive(Default)]
pub struct ScreenTextBuilder {
    pub screen_text: Vec<(Color, Color, String)>,
    pub default_fg_colour: Color,
    pub default_bg_colour: Color,
    last_fg_colour: Color,
    last_bg_colour: Color,
}

/*
pub fn name(mut self, bar: String) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by value.
        self.bar = bar;
        self
    } */

impl ScreenTextBuilder {
    pub fn new() -> ScreenTextBuilder {
        ScreenTextBuilder {
            screen_text: Vec::new(),
            default_fg_colour: Color::WHITE,
            default_bg_colour: Color::BLACK,
            last_fg_colour: Color::WHITE,
            last_bg_colour: Color::BLACK,
        }
    }

    pub fn with_text(mut self, text: &str) -> ScreenTextBuilder {
        self.screen_text
            .push((self.last_fg_colour, self.last_bg_colour, text.to_owned()));
        self.last_bg_colour = self.default_bg_colour;
        self.last_fg_colour = self.default_fg_colour;
        self
    }

    pub fn with_fg_colour(mut self, fg_colour: Color) -> ScreenTextBuilder {
        self.last_fg_colour = fg_colour;
        self
    }

    pub fn with_bg_colour(mut self, bg_colour: Color) -> ScreenTextBuilder {
        self.last_bg_colour = bg_colour;
        self
    }

    pub fn build(self) -> Vec<(Color, Color, String)> {
        self.screen_text.clone()
    }
}

impl ScreenContext {
    pub fn new(width: usize, height: usize, sprite_sizes: SpriteSizes) -> ScreenContext {
        let mut new_screen_vec = vec![ScreenTile::default(); height * width];

        for x in 0..width {
            for y in 0..height {
                new_screen_vec[width * y + x].x = x;
                new_screen_vec[width * y + x].y = y;
            }
        }

        ScreenContext {
            width: width,
            height: height,
            screen_vec: new_screen_vec,
            sprite_sizes,
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn get_tile(&mut self, x: usize, y: usize) -> &mut ScreenTile {
        let idx = self.xy_idx(x, y);
        &mut self.screen_vec[idx]
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // this should return something to indicate the end width of the string, or end coord, or vec of screen tiles impacted
    pub fn draw_text<F>(&mut self, mut x: usize, y: usize, mut builder: F) -> (usize, usize)
    where
        F: FnMut(ScreenTextBuilder) -> ScreenTextBuilder,
    {
        if !self.is_in_bounds(x, y) {
            panic!("Can't create text outside of screen bounds");
        }

        // issue here with text drawing logic, it struggles to draw over existing text

        let builder_vec = builder(ScreenTextBuilder::new()).build();
        // 16 / 8 = 2
        let text_per_map_tile_width = self.sprite_sizes.map_sprite_width as usize
            / self.sprite_sizes.text_sprite_width as usize;

        let mut char_vec = Vec::new();

        'outer: for (oi, (fg, bg, string)) in builder_vec.iter().enumerate() {
            for (ii, ch) in string.chars().enumerate() {
                // char_idx will only ever be 0 or 1, but means that it should move over
                char_vec.push(ScreenGlyph {
                    char: ch as u16,
                    fg_color: *fg,
                    bg_color: *bg,
                    visible: true,
                    layer: 100.,
                });

                if char_vec.len() >= text_per_map_tile_width
                    || (builder_vec.len() == oi + 1 && string.len() == ii + 1)
                {
                    // put on screen here

                    if !self.is_in_bounds(x, y) {
                        break 'outer;
                    }

                    self.get_tile(x, y).tile_text = char_vec.clone();

                    x += 1;
                    char_vec.clear();
                }
            }
        }

        // -1 is due to += at end of loop
        (x - 1, y)
    }

    pub fn clear(&mut self) {
        for mut screen_tile in self.screen_vec.iter_mut() {
            screen_tile.glyph.char = 0;
            screen_tile.glyph.fg_color = Color::BLACK;
            screen_tile.glyph.fg_color = Color::BLACK;
            screen_tile.tile_text.clear();
        }
    }
}

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
        let screen_tile = ctx.get_tile(screen_tile_pos.x, screen_tile_pos.y);

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

        if has_text_sprites && is_entity_text {
            visibility.is_visible = true;
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
