use crate::prelude::*;

pub fn create_wall_collision() -> WallCollision {
    WallCollision
}

pub fn create_bat_collision(bat: Entity, index: PlayerIndex, bat_position: Vector) -> BatCollision {
    BatCollision { index, bat, bat_position }
}

pub fn create_goal_collision(index: PlayerIndex) -> GoalCollision {
    GoalCollision(index)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallCollision;

#[derive(Clone, Copy, Debug)]
pub struct BatCollision {
    pub bat: Entity,
    pub index: PlayerIndex,
    pub bat_position: Vector
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GoalCollision(pub PlayerIndex);

impl Deref for GoalCollision {
    type Target = PlayerIndex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
