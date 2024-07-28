use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

pub fn flip(query: Query<&Velocity>, mut flip_query: Query<(&mut Sprite, &Parent)>) {
  for (mut flippable, parent) in flip_query.iter_mut() {
    let parent_element = query.get(parent.get());
    if let Ok(usable_parent) = parent_element {
      flippable.flip_x = usable_parent.linvel.x < 0.001;
    }
  }
}

pub fn tick_despawn_timer(
  mut query: Query<(&mut DespawnTimer, Entity, &Parent)>,
  mut commands: Commands,
  time: Res<Time>,
) {
  for (mut timer, entity, parent) in query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      commands.entity(parent.get()).remove_children(&[entity]);
      commands.entity(entity).despawn_recursive();
    }
  }
}
