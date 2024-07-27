use std::f32::consts::PI;

use bevy::{
  prelude::*,
  sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;

use crate::prelude::*;

mod sprite;

enum EnemyVariant {
  Red,
  Purple,
  Gray,
}

// TODO: add damage
#[allow(dead_code)]
#[derive(Component)]
struct Enemy {
  attack_range: f32,
  variant: EnemyVariant,
}

#[derive(Component)]
struct AttackCone;

impl Default for Enemy {
  fn default() -> Self {
    let variant = thread_rng().gen_range(0..2);
    let variant = if variant == 0 {
      EnemyVariant::Gray
    } else if variant == 1 {
      EnemyVariant::Purple
    } else {
      EnemyVariant::Red
    };

    Self {
      attack_range: SPRITE_SIZE * 3.0,
      variant,
    }
  }
}

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

#[allow(dead_code)]
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Charging {
  attack_entity: Option<Entity>,
  charging_time: f32,
  range: f32,
}

impl Default for Charging {
  fn default() -> Self {
    Self {
      charging_time: ENEMY_CHARGING_TIME,
      range: ENEMY_CHARGING_RANGE,
      attack_entity: None,
    }
  }
}

#[allow(dead_code)]
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Ready {
  delay: f32,
}

impl Default for Ready {
  fn default() -> Self {
    Self {
      delay: ENEMY_READY_TIME,
    }
  }
}

// #[derive(Clone, Component)]
// #[component(storage = "SparseSet")]
// struct Buffering;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Delivering;

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
    app.add_systems(
      Update,
      charge.run_if(in_state(AppState::InGame)).after(spawn_enemy),
    );
    app.add_systems(
      Update,
      orient_towards_player
        .run_if(in_state(AppState::InGame))
        .after(charge),
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
  player_query: Query<(&Transform, Entity), With<Player>>,
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

  let in_attack_range =
    move |In(entity): In<Entity>, query: Query<&Transform>, enemy_query: Query<&Enemy>| {
      let current_player_transform = query.get(player_entity).unwrap().translation.truncate();
      let enemy_transform = query.get(entity).unwrap().translation.truncate();
      let enemy_data = enemy_query.get(entity).unwrap();

      let distance = current_player_transform.distance(enemy_transform);
      match distance <= enemy_data.attack_range {
        true => Ok(true),
        false => Err(false),
      }
    };

  let is_attack_charged = || false;
  let has_ready_time_elapsed = || false;

  // base circumference on BASE_ENEMIES
  let angle = rand::thread_rng().gen_range(0.0..360.0) * PI / 180.0;
  let circumference = BASE_ENEMIES * SPRITE_SIZE;
  let radius = (circumference / (2.0 * PI)) + rand::thread_rng().gen_range(1.5..30.0);

  let state_machine = StateMachine::default()
    .trans::<Idle, _>(
      near_player,
      Follow {
        target: player_entity,
        speed: rand::thread_rng().gen_range(16.0..24.0),
        angle,
        player_radius: radius,
      },
    )
    .trans::<Follow, _>(near_player.not(), Idle)
    .trans::<Idle, _>(in_attack_range, Charging::default())
    .trans::<Follow, _>(in_attack_range, Charging::default())
    .trans::<Charging, _>(is_attack_charged, Ready::default())
    .trans::<Ready, _>(has_ready_time_elapsed, Delivering)
    .trans::<Delivering, _>(|| true, Idle);

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
      transform: Transform::from_xyz(enemy_x, enemy_y, ENEMY_Z_INDEX),
      ..default()
    },
    Enemy::default(),
    // initialize with Idle state
    Idle,
  ));
}

fn idle(mut query: Query<(&mut Velocity, &Idle), With<Enemy>>) {
  for (mut velocity, _) in &mut query {
    velocity.linvel = Vec2::ZERO;
  }
}

fn charge(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut query: Query<
    (&mut Charging, Entity, &mut Velocity, &Transform),
    (With<Enemy>, Without<Player>),
  >,
  player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
  // spawn cone entity
  let player_transform = player_query.get_single().unwrap();

  for (mut charging, entity, mut velocity, transform) in &mut query {
    if charging.attack_entity.is_some() {
      continue;
    };

    velocity.linvel = Vec2::ZERO;

    let shape = Mesh2dHandle(meshes.add(CircularSector::new(charging.range, 1.0)));
    let material = materials.add(Color::srgb(0.0, 1.0, 0.0));

    // TODO: angle is not initialized correctly
    let angle =
      (player_transform.translation.truncate() - transform.translation.truncate()).to_angle();

    let cone = (
      MaterialMesh2dBundle {
        mesh: shape,
        material,
        transform: Transform {
          translation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: ENEMY_ATTACK_GIZMO_Z_INDEX,
          },
          rotation: Quat::from_rotation_z(angle),
          ..default()
        },
        ..default()
      },
      AttackCone,
    );

    let attack_entity = commands.spawn(cone).id();
    commands.entity(entity).push_children(&[attack_entity]);
    charging.attack_entity = Some(attack_entity);
  }
}

fn orient_towards_player(
  mut query: Query<(&mut Transform, &GlobalTransform), With<AttackCone>>,
  player_query: Query<&Transform, (With<Player>, Without<AttackCone>)>,
) {
  let player_transform = player_query.get_single().unwrap().translation.truncate();
  for (mut cone_transform, global_transform) in &mut query {
    let angle =
      (player_transform - global_transform.translation().truncate()).to_angle() - PI / 2.0;
    cone_transform.rotation = Quat::from_rotation_z(angle);
  }
}
