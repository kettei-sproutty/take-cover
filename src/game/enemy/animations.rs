use bevy::prelude::*;

use super::Enemy;

#[derive(Component, Clone)]
pub struct AnimationIndices {
  pub first: usize,
  pub last: usize,
}

#[derive(Component, Clone)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
  time: Res<Time>,
  anim_query: Query<Option<&AnimationIndices>, With<Enemy>>,
  mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &Parent)>,
) {
  for (mut timer, mut atlas, parent) in &mut query {
    let indices = anim_query.get(parent.get()).unwrap_or(None);

    if let Some(animation_indices) = indices {
      timer.0.tick(time.delta());
      if timer.0.just_finished() {
        atlas.index =
          if atlas.index >= animation_indices.last || atlas.index < animation_indices.first {
            animation_indices.first
          } else {
            atlas.index + 1
          };
      }
    }
  }
}
