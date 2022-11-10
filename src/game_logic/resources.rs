use super::components::Position;

pub struct PlayerResource {
    pub start_pos: Position,
    pub cur_pos: Position,
    pub move_waypoints: Vec<Position>,
}
