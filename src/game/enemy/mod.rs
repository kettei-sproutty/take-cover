use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;

use crate::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

#[allow(dead_code)]
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Follow {
  target: Entity,
  speed: f32,
  angle: f32,
  player_radius: f32,
}

// #[derive(Clone, Component)]
// #[component(storage = "SparseSet")]
// struct Charging;

// #[derive(Clone, Component)]
// #[component(storage = "SparseSet")]
// struct Ready;

// #[derive(Clone, Component)]
// #[component(storage = "SparseSet")]
// struct Buffering;

// #[derive(Clone, Component)]
// #[component(storage = "SparseSet")]
// struct Delivering;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  // init enemy state
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      spawn_enemy
        .after(init_player)
        .run_if(in_state(AppState::InGame)),
    );
    app.add_systems(
      Update,
      follow.run_if(in_state(AppState::InGame)).after(spawn_enemy),
    );
    app.add_systems(
      Update,
      idle.run_if(in_state(AppState::InGame)).after(spawn_enemy),
    );
  }
}

fn follow(
  mut enemy_query: Query<(&Follow, &mut Velocity, &Transform), With<Enemy>>,
  player_query: Query<&Transform, With<Player>>,
) {
  for (follow, mut rb_vels, transform) in enemy_query.iter_mut() {
    let player_transform = player_query.single();

    let target_position = player_transform.translation.truncate();
    let enemy_position = transform.translation.truncate();

    // describe a circle around the player
    let point_in_circumference = target_position
      + Vec2::new(
        follow.angle.cos() * follow.player_radius,
        follow.angle.sin() * follow.player_radius,
      );

    let direction = point_in_circumference - enemy_position;
    let distance = direction.length();

    let velocity = if distance < 8. {
      Vec2::ZERO
    } else {
      let direction = direction / distance;
      direction * follow.speed
    };

    rb_vels.linvel = velocity;
  }
}

fn spawn_enemy(
  mut commands: Commands,
  enemy_query: Query<&Enemy>,
  player_query: Query<(&Transform, Entity), With<Player>>, // TODO: use UiAssets
) {
  if enemy_query.iter().count() >= BASE_ENEMIES as usize {
    return;
  }

  let (player_initial_transform, player_entity) = player_query.get_single().unwrap();

  let near_player = move |In(entity): In<Entity>, transforms: Query<&Transform>| {
    let enemy_transform = transforms.get(entity).unwrap();
    let player_transform = transforms.get(player_entity).unwrap();

    let distance = f32::abs(
      player_transform
        .translation
        .truncate()
        .distance(enemy_transform.translation.truncate()),
    );

    // TODO: move to Enemy struct
    match distance <= 200. {
      true => Ok(true),
      false => Err(false),
    }
  };

  // base circumference on BASE_ENEMIES
  let angle = rand::thread_rng().gen_range(0.0..360.0) * PI / 180.0;
  let circumference = BASE_ENEMIES * SPRITE_SIZE;
  let radius = (circumference / (2.0 * PI)) + rand::thread_rng().gen_range(1.5..30.0);

  let state_machine = StateMachine::default()
    .trans::<Idle, _>(
      near_player,
      Follow {
        target: player_entity,
        speed: rand::thread_rng().gen_range(PLAYER_SPEED..24.0),
        angle,
        player_radius: radius,
      },
    )
    .trans::<Follow, _>(near_player.not(), Idle);

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  // we calculate the enemy position spawn based on the player position
  // enemy will spawn at a random position around the player
  // with a minimum radius of 100 and a maximum of 200
  let mut rng = rand::thread_rng();
  let distance = rng.gen_range(100.0..200.);
  let enemy_x = player_initial_transform.translation.x + angle.cos() * distance;
  let enemy_y = player_initial_transform.translation.y + angle.sin() * distance;

  // spawn enemy, define state machine behavior
  commands.spawn((
    // Despawn enemy on app state change
    StateDespawnMarker,
    Collider::cuboid(SPRITE_SIZE / 2., SPRITE_SIZE / 2.),
    // TODO: use transform and try removing any physics related thingy
    RigidBody::KinematicVelocityBased,
    Velocity::zero(),
    GravityScale(0.),
    state_machine,
    SpriteBundle {
      sprite: Sprite {
        color: Color::srgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
        ..default()
      },
      transform: Transform::from_xyz(enemy_x, enemy_y, 2.),
      ..default()
    },
    Enemy,
    // initialize with Idle state
    Idle,
  ));
}

fn idle(mut query: Query<(&mut Velocity, &Idle), With<Enemy>>) {
  for (mut velocity, _) in &mut query {
    velocity.linvel = Vec2::ZERO;
  }
}
