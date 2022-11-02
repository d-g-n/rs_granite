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
            GameTile::DownStairs => 31 as u16,
            GameTile::UpStairs => 30 as u16,
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

pub type GameMapTiles2D = Vec<GameTile>;

#[derive(Clone)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub tiles: GameMapTiles2D,
    pub history: Vec<GameMapTiles2D>,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> GameMap {
        let new_map = vec![GameTile::Floor; width * height];

        GameMap {
            width,
            height,
            tiles: new_map,
            history: Vec::new(),
        }
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    pub fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn fill(&mut self, tile: GameTile) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.xy_idx(x, y);

                self.tiles[idx] = tile;
            }
        }
    }

    pub fn snapshot(&mut self) {
        self.history.push(self.tiles.clone())
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn draw_square(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fill_tile: GameTile,
        border_tile: GameTile,
    ) {
        for width_x in 0..width {
            for height_y in 0..height {
                let new_x = x + width_x;
                let new_y = y + height_y;

                if !self.is_within_bounds(new_x, new_y) {
                    continue;
                }

                let idx = self.xy_idx(new_x, new_y);

                if width_x == 0 || width_x == width - 1 || height_y == 0 || height_y == height - 1 {
                    // border
                    self.tiles[idx] = border_tile;
                } else {
                    // fill
                    self.tiles[idx] = fill_tile;
                }
            }
        }
    }

    pub fn get_tile_pos_by_type(&mut self, tile: GameTile) -> Vec<(usize, usize)> {
        let mut res_vec = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if self.tiles[self.xy_idx(x, y)] == tile {
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
                self.tiles[self.xy_idx(new_x as usize, new_y as usize)],
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
