use crate::prelude::*;

/// Despawn recursively all entities with the given component.
/// It is used in combo with the [`StateDespawnMarker`] component.
/// Example:
/// ```rust
/// fn setup_loadscreen(mut commands: Commands) {
///   commands.spawn((
///     StateDespawnMarker,
///     Camera2dBundle::default()
///   ));
/// }
/// ```
pub fn despawn_all_recursive<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
