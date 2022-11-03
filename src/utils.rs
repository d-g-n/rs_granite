#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn h_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    pub fn w_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x + self.width / 2), (self.y + self.height / 2))
    }

    pub fn all_squares(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        for x in self.x..(self.x + self.width) {
            for y in self.y..(self.y + self.height) {
                res.push((x, y));
            }
        }

        res
    }
}
