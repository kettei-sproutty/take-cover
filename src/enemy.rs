use seldom_state::prelude::*;

use crate::{player::player::Player, prelude::*};

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Following {
  _target: Entity,
  _speed: f32,
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

#[derive(Bundle)]
pub struct EnemyBundle {
  sm: StateMachine,
  sprite: SpriteBundle,
}

impl EnemyBundle {}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  // init enemy state
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_enemy);
    app.add_systems(Update, following);
  }
}

fn following() {
  println!("following!")
}

fn spawn_enemy(
  mut commands: Commands,
  query: Query<Entity, With<Player>>,
  // TODO: use UiAssets
  asset_server: Res<AssetServer>,
) {
  let player_entity = query.get_single().unwrap();

  let near_player = move |In(entity): In<Entity>, transforms: Query<&Transform>| {
    let enemy_transform = transforms.get(entity).unwrap();
    let player_transform = transforms.get(player_entity).unwrap();

    let distance = player_transform
      .translation
      .truncate()
      .distance(enemy_transform.translation.truncate());

    match distance <= 300. {
      true => Ok(distance),
      false => Err(distance),
    }
  };

  // spawn enemy, define state machine behavior
  commands.spawn((
    EnemyBundle {
      sm: StateMachine::default()
        .trans::<Idle, _>(
          near_player,
          Following {
            _target: player_entity,
            _speed: 5.,
          },
        )
        .trans::<Following, _>(near_player.not(), Idle)
        .set_trans_logging(true),
      sprite: SpriteBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        texture: asset_server.load("textures/1-bit/colored.png"),
        ..default()
      },
    },
    // initialize with Idle state
    Idle,
  ));
}
