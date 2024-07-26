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

    let direction = target_position - enemy_position;
    let distance = direction.length();

    let velocity = if distance > 0. {
      let direction = direction / distance;
      direction * follow.speed
    } else {
      Vec2::ZERO
    };

    rb_vels.linvel = velocity;
  }
}

fn spawn_enemy(
  mut commands: Commands,
  enemy_query: Query<&Enemy>,
  player_entity_query: Query<Entity, With<Player>>,
  player_transform_query: Query<&Transform, With<Player>>,
  // TODO: use UiAssets
) {
  if enemy_query.iter().count() > 3 {
    return;
  }

  let player_transform = *player_transform_query.single();
  let player_entity = player_entity_query.single();

  let near_player = move |In(entity): In<Entity>, transforms: Query<&Transform>| {
    let enemy_transform = transforms.get(entity).unwrap();

    let distance = player_transform
      .translation
      .truncate()
      .distance(enemy_transform.translation.truncate());

    // TODO: move to Enemy struct
    match distance <= 200. {
      true => Ok(distance),
      false => Err(distance),
    }
  };

  let state_machine = StateMachine::default()
    .trans::<Idle, _>(
      near_player,
      Follow {
        target: player_entity,
        speed: 5.,
      },
    )
    .trans::<Follow, _>(near_player.not(), Idle);

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  // we calculate the enemy position spawn based on the player position
  // enemy will spawn at a random position around the player
  // with a minumum radius of 100 and a maximum of 200
  let mut rng = rand::thread_rng();
  let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.);
  let distance = rng.gen_range(100.0..200.);
  let enemy_x = player_transform.translation.x + angle.cos() * distance;
  let enemy_y = player_transform.translation.y + angle.sin() * distance;

  // spawn enemy, define state machine behavior
  commands.spawn((
    // Despawn enemy on state change
    StateDespawnMarker,
    Collider::cuboid(SPRITE_SIZE / 2., SPRITE_SIZE / 2.),
    RigidBody::Dynamic,
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
