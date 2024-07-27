use std::time::Instant;

use crate::prelude::*;
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
  // TODO: use a timer instead of Instant
  start: Instant,
  meteors: usize,
  index: usize,
}

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
    app.add_systems(Update, falling_meteor.run_if(in_state(AppState::InGame)));
  }
}

fn check_cycle_state(mut next_state: ResMut<NextState<CycleState>>, cycle: Query<&Cycle>) {
  let cycle = cycle.get_single().unwrap();

  if cycle.start.elapsed().as_secs_f32() >= CYCLE_DURATION {
    next_state.set(CycleState::Meteors);
  }
}

fn init_cycle(mut commands: Commands) {
  commands.spawn((
    StateDespawnMarker,
    Cycle {
      start: Instant::now(),
      meteors: CYCLE_WEIGHT,
      index: 1,
    },
  ));
}

fn spawn_meteor(
  mut commands: Commands,
  mut query: Query<&mut Cycle>,
  mut next_state: ResMut<NextState<CycleState>>,
  mut score: ResMut<Score>,
  player_query: Query<&Transform, With<Player>>,
  #[allow(unused_variables)] time: Res<Time>,
) {
  let mut cycle = query.get_single_mut().unwrap();

  // If there are no more meteors to spawn, reset the cycle.
  if cycle.meteors == 0 {
    next_state.set(CycleState::Standard);
    cycle.start = Instant::now();
    cycle.index += 1;
    cycle.meteors = CYCLE_WEIGHT * cycle.index;
    score.0 += cycle.index * 10;
  };

  // If the delay has not passed, do not spawn a meteor.
  // TODO: decrease the delay as the cycle index increases
  // TODO: add a random factor to the delay
  // if time.elapsed_wrapped() < METEOR_SPAWN_DELAY {
  //   return;
  // };

  let is_on_ground = |In(entity): In<Entity>, query: Query<&Transform>| {
    let transform = query.get(entity).unwrap();
    transform.translation.z < 0.
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
    Transform::from_xyz(x, y, 10.)
  };

  commands.spawn((
    StateDespawnMarker,
    Meteor,
    state_machine,
    SpriteBundle {
      sprite: Sprite {
        color: Color::srgb(0.0, 0.0, 1.0),
        custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
        ..default()
      },
      transform: meteor_transform,
      ..default()
    },
    Falling,
    FallSpeed(rand::thread_rng().gen_range(0.1..0.3)),
  ));

  cycle.meteors -= 1;
}

fn falling_meteor(
  #[allow(unused_variables)] time: Res<Time>,
  mut meteor_query: Query<(&mut Transform, &FallSpeed), With<Falling>>,
) {
  for (mut transform, fall_speed) in &mut meteor_query {
    transform.translation.z -= fall_speed.0;
    println!("Meteor z: {}", transform.translation.z);
    // TODO: change scale based on z
    // transform.scale = Vec3::splat(1.0 - transform.translation.z / 10.0);
  }
}

fn check_impact(
  mut commands: Commands,
  impact_query: Query<(Entity, &Transform), With<Impact>>,
  player_query: Query<&Transform, With<Player>>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  for (entity, transform) in &mut impact_query.iter() {
    let player_transform = player_query.single();
    let player_position = player_transform.translation.truncate();
    let meteor_position = transform.translation.truncate();

    // This doesn't work as expected.
    if player_position.distance(meteor_position) < SPRITE_SIZE {
      println!("Player hit by meteor");
      next_state.set(AppState::GameOver);
    }

    commands.entity(entity).despawn();
  }
}
