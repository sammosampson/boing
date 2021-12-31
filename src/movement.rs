use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down
}
#[derive(Debug, Copy, Clone)]
pub struct Position(pub Vector);

impl Deref for Position {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NextPosition(pub Vector);

impl Deref for NextPosition {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NextPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Velocity(f32);

impl Deref for Velocity {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MaximumVelocity(pub f32);

impl Deref for MaximumVelocity {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaximumVelocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Heading(Vector);

impl Deref for Heading {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Heading {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector> for Heading {
    fn from(from: Vector) -> Self {
        Self(from)
    }
}
