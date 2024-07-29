use crate::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use seldom_state::prelude::*;

use super::Score;

#[derive(Component)]
struct Meteor;

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
struct Falling;

#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
struct Impact;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, States, Reflect)]
pub enum CycleState {
  #[default]
  Standard,
  // TODO: choose a better name
  Meteors,
}

#[derive(Component)]
#[allow(dead_code)]
struct DelayTimer {
  timer: Timer,
}

#[derive(Component)]
struct FallSpeed(f32);

#[derive(Component)]
struct Cycle {
  start: Timer,
  meteors: usize,
  index: usize,
}

#[derive(Resource)]
struct MeteorSpawnDelay(Timer);

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
  fn build(&self, app: &mut App) {
    app.init_state::<CycleState>();

    app.add_systems(OnEnter(AppState::InGame), init_cycle);
    app.add_systems(
      Update,
      check_cycle_state
        .run_if(in_state(CycleState::Standard))
        .run_if(in_state(AppState::InGame)),
    );
    app.add_systems(
      Update,
      spawn_meteor
        .run_if(in_state(CycleState::Meteors))
        .run_if(in_state(AppState::InGame)),
    );
    app.add_systems(
      Update,
      check_impact
        .run_if(in_state(CycleState::Meteors))
        .run_if(in_state(AppState::InGame)),
    );
    app.add_systems(
      FixedUpdate,
      falling_meteor.run_if(in_state(AppState::InGame)),
    );
  }
}

fn check_cycle_state(
  mut next_state: ResMut<NextState<CycleState>>,
  time: Res<Time>,
  mut cycle: Query<&mut Cycle>,
) {
  let mut cycle = cycle.get_single_mut().unwrap();

  cycle.start.tick(time.delta());

  if cycle.start.finished() {
    next_state.set(CycleState::Meteors);
  }
}

fn init_cycle(mut commands: Commands, mut next_state: ResMut<NextState<CycleState>>) {
  commands.spawn((
    StateDespawnMarker,
    Cycle {
      start: Timer::from_seconds(CYCLE_DURATION, TimerMode::Once),
      meteors: CYCLE_WEIGHT,
      index: 1,
    },
  ));

  commands.insert_resource(MeteorSpawnDelay(Timer::from_seconds(
    rand::thread_rng().gen_range(METEOR_SPAWN_DELAY..METEOR_SPAWN_DELAY + 0.1),
    TimerMode::Once,
  )));
  next_state.set(CycleState::Standard);
}

fn spawn_meteor(
  mut commands: Commands,
  mut query: Query<&mut Cycle>,
  mut next_state: ResMut<NextState<CycleState>>,
  mut score: ResMut<Score>,
  player_query: Query<&Transform, With<Player>>,
  mut meteor_spawn_delay: ResMut<MeteorSpawnDelay>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  meteors_query: Query<Entity, With<Meteor>>,
  time: Res<Time>,
) {
  let mut cycle = query.get_single_mut().unwrap();
  meteor_spawn_delay.0.tick(time.delta());

  if meteor_spawn_delay.0.finished() {
    meteor_spawn_delay.0.reset();
  } else {
    return;
  }

  if cycle.meteors == 0 && meteors_query.iter().len() != 0 {
    return;
  };

  // If there are no more meteors to spawn, reset the cycle.
  if cycle.meteors == 0 && meteors_query.iter().len() == 0 {
    next_state.set(CycleState::Standard);
    cycle.start = Timer::from_seconds(CYCLE_DURATION, TimerMode::Once);
    cycle.index += 1;
    cycle.meteors = CYCLE_WEIGHT * cycle.index;
    score.0 += cycle.index.pow(3);
  };

  let is_on_ground = |In(entity): In<Entity>, query: Query<&Transform>| {
    let transform = query.get(entity).unwrap();
    transform.translation.z <= 0.
  };

  let state_machine = StateMachine::default().trans::<Falling, _>(is_on_ground, Impact);

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  let meteor_transform: Transform = {
    let player_position = player_query.get_single().unwrap().translation.truncate();

    let distance = rand::thread_rng().gen_range(0.0..100.0);
    let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
    let x = player_position.x + angle.cos() * distance;
    let y = player_position.y + angle.sin() * distance;
    Transform::from_xyz(x, y, 25.)
  };

  commands.spawn((
    StateDespawnMarker,
    Meteor,
    state_machine,
    MaterialMesh2dBundle {
      mesh: meshes.add(Circle::new(SPRITE_SIZE / 2.)).into(),
      material: materials.add(colors::RED_100),
      transform: meteor_transform,
      ..default()
    },
    Falling,
    FallSpeed(rand::thread_rng().gen_range(0.3..0.5)),
  ));

  cycle.meteors -= 1;
}

fn falling_meteor(
  #[allow(unused_variables)] time: Res<Time>,
  mut meteor_query: Query<(&mut Transform, &mut Handle<ColorMaterial>, &FallSpeed), With<Falling>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for (mut transform, color_material, fall_speed) in &mut meteor_query {
    transform.translation.z -= fall_speed.0;

    if let Some(material) = materials.get_mut(color_material.id()) {
      material.color = match transform.translation.z {
        z if z > 20. => colors::RED_100,
        z if z > 15. => colors::RED_200,
        z if z > 10. => colors::RED_300,
        z if z > 5. => colors::RED_400,
        _ => colors::RED_500,
      };
    }

    transform.scale = Vec3::splat(1.0 - transform.translation.z / 25.);
  }
}

fn check_impact(
  mut commands: Commands,
  impact_query: Query<(Entity, &Transform), With<Impact>>,
  player_query: Query<&Transform, With<Player>>,
  mut score: ResMut<Score>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  for (entity, transform) in &mut impact_query.iter() {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();
    let meteor_position = transform.translation.truncate();

    // This doesn't work as expected. + now it seems it does, but to be investigated
    if player_position.distance(meteor_position) < SPRITE_SIZE {
      next_state.set(AppState::GameOver);
    } else {
      score.0 += 3;
    }

    commands.entity(entity).despawn();
  }
}
