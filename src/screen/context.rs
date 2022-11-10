use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::sprites::SpriteSizes;

use super::structs::{ScreenContext, ScreenGlyph, ScreenTextBuilder, ScreenTile};

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
            return (x, y);
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
