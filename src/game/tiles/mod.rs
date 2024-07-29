use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use super::TILE_SPRITE_SIZE;

#[derive(Component)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
  wall: Wall,
  collider: Collider,
  rb: RigidBody,
}

impl Default for WallBundle {
  fn default() -> Self {
    Self {
      wall: Wall,
      collider: Collider::cuboid(TILE_SPRITE_SIZE * 0.5, TILE_SPRITE_SIZE * 0.5),
      rb: RigidBody::Fixed,
    }
  }
}
