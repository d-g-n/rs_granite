use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game_logic::components::Position;

use super::GameMap;
use pathfinding::prelude::astar;

impl Position {
    fn is_valid(&self, map: &GameMap) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < map.width as i32 && self.y < map.height as i32
    }

    fn distance(&self, other: &Position) -> u32 {
        ((self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32)
    }

    fn distance_squared(&self, other: &Position) -> u32 {
        ((self.x.abs_diff(other.x) ^ 2 + self.y.abs_diff(other.y) ^ 2) as u32)
    }

    fn distance_diagonal(&self, other: &Position) -> f32 {
        let dx: f32 = self.x.abs_diff(other.x) as f32;
        let dy: f32 = self.y.abs_diff(other.y) as f32;
        let d2: f32 = 2.0_f32.sqrt();

        ((dx + dy) + (d2 - 2.) * f32::min(dx, dy))
    }

    fn successors(&self, map: &GameMap) -> Vec<(Position, u32)> {
        let &Position { x, y } = self;

        let res = vec![
            Position { x: x, y: y + 1 },     // North
            Position { x: x, y: y - 1 },     // South
            Position { x: x + 1, y: y },     // East
            Position { x: x - 1, y: y },     // West
            Position { x: x + 1, y: y + 1 }, // NE
            Position { x: x - 1, y: y + 1 }, // NW
            Position { x: x + 1, y: y - 1 }, // SE
            Position { x: x - 1, y: y - 1 }, // SW
        ]
        .into_iter()
        .filter(|p| p.is_valid(map))
        .filter(|p| !map.tiles[p.y as usize][p.x as usize].is_blocker())
        .map(|p| (p, 1))
        .collect();

        res
    }
}

pub fn astar_next_step(
    map: &GameMap,
    from: Position,
    to: Position,
) -> Option<(Vec<Position>, u32)> {
    astar(
        &from,
        |p| p.successors(map),
        |p| p.distance(&to),
        |p| *p == to,
    )
}
