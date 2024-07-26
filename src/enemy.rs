use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    app.add_systems(OnEnter(AppState::InGame), spawn_enemy.after(init_player));
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
  query: Query<Entity, With<Player>>,
  // TODO: use UiAssets
) {
  let player_entity = query.get_single().unwrap();

  let near_player = move |In(entity): In<Entity>, transforms: Query<&Transform>| {
    let enemy_transform = transforms.get(entity).unwrap();
    let player_transform = transforms.get(player_entity).unwrap();

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
      transform: Transform::from_xyz(10., 10., 2.),
      ..default()
    },
    Enemy,
    // initialize with Idle state
    Idle,
  ));
}
