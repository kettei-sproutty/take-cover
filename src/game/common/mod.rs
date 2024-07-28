use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub fn flip(query: Query<&Velocity>, mut flip_query: Query<(&mut Sprite, &Parent)>) {
  for (mut flippable, parent) in flip_query.iter_mut() {
    let parent_element = query.get(parent.get());
    if let Ok(usable_parent) = parent_element {
      flippable.flip_x = usable_parent.linvel.x < 0.001;
    }
  }
}
