use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::StateMachine;

use crate::prelude::*;

use super::Score;

#[derive(Component)]
pub struct AttackComponent;

#[derive(Component)]
pub struct AttackTrailCollider;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct Attack;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
#[allow(dead_code)]
pub struct AttackEnd;

#[derive(Resource)]
pub struct AttackPositions(Vec<Vec2>);

#[derive(Component)]
pub struct AttackTrail;

#[derive(Component)]
pub struct Cooldown(pub Timer);

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), init_attack);
    app.add_systems(
      Update,
      track_mouse_movement
        .run_if(in_state(AppState::InGame))
        .after(init_attack),
    );
    app.add_systems(
      Update,
      check_attack
        .run_if(in_state(AppState::InGame))
        .after(track_mouse_movement),
    );
    app.add_systems(
      Update,
      check_for_collisions
        .run_if(in_state(AppState::InGame))
        .after(check_attack),
    );
  }
}

fn init_attack(mut commands: Commands) {
  let left_mouse_down =
    |input: Res<ButtonInput<MouseButton>>| input.just_pressed(MouseButton::Left);

  let left_mouse_up = |input: Res<ButtonInput<MouseButton>>| input.just_released(MouseButton::Left);

  let state_machine = StateMachine::default()
    .trans::<Idle, _>(left_mouse_down, Attack)
    .trans::<Attack, _>(left_mouse_up, Idle);

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  commands.insert_resource(AttackPositions(vec![]));

  commands.spawn((StateDespawnMarker, state_machine, AttackComponent, Idle));
}

fn track_mouse_movement(
  mut commands: Commands,
  mut mouse_position: ResMut<AttackPositions>,
  q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
  query: Query<Entity, With<Attack>>,
  q_windows: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(_entity) = query.get_single() {
    let (camera, camera_transform) = q_camera.single();

    if let Some(position) = q_windows
      .single()
      .cursor_position()
      .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
      .map(|ray| ray.origin.truncate())
    {
      mouse_position.0.push(position);

      commands.spawn((
        StateDespawnMarker,
        AttackTrail,
        SpriteBundle {
          sprite: Sprite {
            color: colors::PRIMARY_200,
            custom_size: Some(Vec2::new(2., 2.)),
            ..default()
          },
          transform: Transform::from_xyz(position.x, position.y, PLAYER_Z_INDEX),
          ..Default::default()
        },
      ));
    }
  }
}

pub fn check_attack(
  mut commands: Commands,
  mut positions: ResMut<AttackPositions>,
  query: Query<Entity, (With<AttackComponent>, With<Idle>)>,
  trail_query: Query<Entity, With<AttackTrail>>,
) {
  if positions.0.len() < 2 {
    return;
  }

  if let Ok(_entity) = query.get_single() {
    let mut vertices = positions.0.clone();
    vertices.push(vertices[0]);

    commands.spawn((
      StateDespawnMarker,
      AttackTrailCollider,
      Collider::polyline(vertices, None),
      CollisionGroups::new(ATTACK_TRAIL_GROUP, ENEMY_GROUP),
      ActiveCollisionTypes::all(),
      Sensor,
      Cooldown(Timer::from_seconds(0.1, TimerMode::Once)),
      CollidingEntities::default(),
      ActiveEvents::COLLISION_EVENTS,
    ));

    for trail_entity in trail_query.iter() {
      commands.entity(trail_entity).despawn();
    }

    positions.0.clear();
  }
}

fn check_for_collisions(
  mut commands: Commands,
  mut query: Query<(Entity, &CollidingEntities, &mut Cooldown), With<AttackTrailCollider>>,
  enemies: Query<Entity, With<Enemy>>,
  mut score: ResMut<Score>,
  time: Res<Time>,
) {
  for (collider_entity, colliders, mut cooldown) in &mut query {
    cooldown.0.tick(time.delta());

    for enemy_entity in &enemies {
      if colliders.contains(enemy_entity) {
        commands.entity(enemy_entity).despawn_recursive();
        commands.entity(collider_entity).despawn();
        score.0 += 3;
      }
    }

    if cooldown.0.finished() {
      // entity could have been despawned
      if commands.get_entity(collider_entity).is_some() {
        commands.entity(collider_entity).despawn();
      }
    }
  }
}
