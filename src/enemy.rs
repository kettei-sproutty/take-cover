use bevy::prelude::*;
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

fn follow(// mut transforms: Query<&mut Transform>,
  // follows: Query<(Entity, &Follow), With<Enemy>>,
  // time: Res<Time>,
) {
  // for (entity, follow) in &follows {
  //   let (speed, target) = (follow.speed, follow.target);
  //   println!("{entity} is following {target} at {speed}");
  // }
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

  // spawn enemy, define state machine behavior
  commands.spawn((
    StateMachine::default()
      .trans::<Idle, _>(
        near_player,
        Follow {
          target: player_entity,
          speed: 5.,
        },
      )
      .trans::<Follow, _>(near_player.not(), Idle)
      .set_trans_logging(true),
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
