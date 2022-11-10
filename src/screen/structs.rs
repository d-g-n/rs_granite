use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::sprites::SpriteSizes;

#[derive(Clone, Default, Debug)]
pub struct ScreenGlyph {
    pub char: u16,
    pub fg_color: Color,
    pub bg_color: Color,
    pub visible: bool,
    pub layer: f32,
}

#[derive(Clone, Default)]
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
    pub(in crate::screen) sprite_sizes: SpriteSizes,
    pub screen_vec: Vec<ScreenTile>,
}

#[derive(Component)]
pub struct ScreenTilePos {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct ScreenTextBuilder {
    pub screen_text: Vec<(Color, Color, String)>,
    pub default_fg_colour: Color,
    pub default_bg_colour: Color,
    pub(in crate::screen) last_fg_colour: Color,
    pub(in crate::screen) last_bg_colour: Color,
}
