use crate::prelude::*;

#[system(for_each)]
#[filter(component::<MaximumVelocity>())]
#[filter(!component::<Velocity>())]
#[filter(!component::<NextPosition>())]
pub fn initialise_movement(
    entity: &Entity,
    position: &Position,
    buffer: &mut CommandBuffer
) {
    buffer.add_component(*entity, NextPosition(**position));
    buffer.add_component(*entity, Velocity::default());
}