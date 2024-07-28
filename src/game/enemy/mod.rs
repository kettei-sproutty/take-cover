use std::{f32::consts::PI, time::Duration};

use animations::{animate_sprite, AnimationIndices};
use bevy::{
  prelude::*,
  sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use rand::prelude::*;
use seldom_state::prelude::*;
use sprite::get_idle_animation;

use crate::prelude::*;

use super::common::{tick_despawn_timer, DespawnTimer};

mod animations;
mod sprite;

#[derive(Clone)]
enum EnemyVariant {
  Aqua,
  Purple,
  Green,
}

// TODO: add damage
#[derive(Clone, Component)]
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
      EnemyVariant::Aqua
    } else if variant == 1 {
      EnemyVariant::Purple
    } else {
      EnemyVariant::Green
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
  timer: Timer,
  range: f32,
}

impl Default for Charging {
  fn default() -> Self {
    Self {
      attack_entity: None,
      range: ENEMY_CHARGING_RANGE,
      timer: Timer::new(
        Duration::from_secs_f32(ENEMY_CHARGING_TIME),
        TimerMode::Once,
      ),
    }
  }
}

#[allow(dead_code)]
#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Ready {
  timer: Timer,
}

impl Default for Ready {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(ENEMY_READY_TIME, TimerMode::Once),
    }
  }
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Delivering {
  timer: Timer,
  radius: f32,
}

#[derive(Event)]
struct DeliveringEvent(Entity, f32);

impl Default for Delivering {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(ENEMY_ATTACK_COOLDOWN, TimerMode::Once),
      radius: ENEMY_CHARGING_RANGE,
    }
  }
}

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
      animate_sprite
        .after(spawn_enemy)
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
      (charge, tick_charge_timer)
        .run_if(in_state(AppState::InGame))
        .after(spawn_enemy),
    );
    app.add_systems(
      Update,
      orient_towards_player
        .run_if(in_state(AppState::InGame))
        .after(charge),
    );
    app.add_systems(
      Update,
      (get_ready, tick_ready_timer)
        .run_if(in_state(AppState::InGame))
        .after(spawn_enemy),
    );
    app.add_systems(
      Update,
      (deliver, tick_delivery_timer)
        .run_if(in_state(AppState::InGame))
        .after(spawn_enemy),
    );

    app.add_event::<DeliveringEvent>();
    app.add_systems(
      Update,
      handle_delivering_event
        .run_if(in_state(AppState::InGame))
        .after(spawn_enemy),
    );
    app.add_systems(
      Update,
      (check_for_collisions, tick_despawn_timer).run_if(in_state(AppState::InGame)),
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
  asset_server: Res<AssetServer>,
  texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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

  let is_attack_charged =
    move |In(entity): In<Entity>, query: Query<(&Charging, Entity), With<Enemy>>| {
      let charge = query.get(entity);
      charge.is_ok_and(|c| c.0.timer.finished())
    };

  let has_ready_time_elapsed =
    move |In(entity): In<Entity>, query: Query<(&Ready, Entity), With<Enemy>>| {
      let ready = query.get(entity);
      ready.is_ok_and(|r| r.0.timer.finished())
    };
  let has_delivered =
    move |In(entity): In<Entity>, query: Query<(&Delivering, Entity), With<Enemy>>| {
      let delivered = query.get(entity);
      delivered.is_ok_and(|d| d.0.timer.finished())
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
        speed: rand::thread_rng().gen_range(16.0..24.0),
        angle,
        player_radius: radius,
      },
    )
    .trans::<Follow, _>(near_player.not(), Idle)
    .trans::<Idle, _>(in_attack_range, Charging::default())
    .trans::<Follow, _>(in_attack_range, Charging::default())
    .trans::<Charging, _>(is_attack_charged, Ready::default())
    .trans::<Ready, _>(has_ready_time_elapsed, Delivering::default())
    .trans::<Delivering, _>(has_delivered, Idle)
    .on_enter::<Idle>(|entity| {
      // TODO: is this removed from children? When bevy removes a component, it does not necessarily remove its link to the parent https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
      entity.insert(AnimationIndices { first: 0, last: 3 });
    })
    .on_enter::<Follow>(|entity| {
      entity.insert(AnimationIndices { first: 0, last: 3 });
    })
    .on_enter::<Charging>(|entity| {
      entity.insert(AnimationIndices { first: 7, last: 9 });
    })
    .on_enter::<Ready>(|entity| {
      entity.insert(AnimationIndices { first: 8, last: 9 });
    })
    .on_enter::<Delivering>(|entity| {
      entity.insert(AnimationIndices {
        first: 23,
        last: 27,
      });
    });

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  // we calculate the enemy position spawn based on the player position
  // enemy will spawn at a random position around the player
  // with a minimum radius of 100 and a maximum of 200
  let mut rng = rand::thread_rng();
  let distance = rng.gen_range(100.0..200.);
  let enemy_x = player_initial_transform.translation.x + angle.cos() * distance;
  let enemy_y = player_initial_transform.translation.y + angle.sin() * distance;

  let enemy = Enemy::default();
  let (texture, texture_atlas, timer) =
    get_idle_animation(&enemy.variant, asset_server, texture_atlas_layouts);

  // spawn enemy, define state machine behavior
  commands
    .spawn((
      // Despawn enemy on app state change
      StateDespawnMarker,
      Collider::cuboid(ENEMY_SPRITE_SIZE / 2., ENEMY_SPRITE_SIZE / 2.),
      CollisionGroups::new(ENEMY_GROUP, Group::empty()),
      // TODO: use transform and try removing any physics related thingy
      RigidBody::KinematicVelocityBased,
      Velocity::zero(),
      GravityScale(0.0),
      SpatialBundle::from_transform(Transform::from_xyz(enemy_x, enemy_y, ENEMY_Z_INDEX)),
      state_machine,
      enemy,
      // initialize with Idle state
      Idle,
    ))
    .with_children(|parent| {
      parent.spawn((
        StateDespawnMarker,
        SpriteBundle {
          sprite: Sprite {
            custom_size: Some(Vec2::new(ENEMY_SPRITE_SIZE, ENEMY_SPRITE_SIZE)),
            ..default()
          },
          texture,
          transform: Transform::from_xyz(
            ENEMY_SPRITE_SIZE * 0.5,
            ENEMY_SPRITE_SIZE * 0.5,
            ENEMY_ATTACK_GIZMO_Z_INDEX,
          ),
          ..default()
        },
        texture_atlas,
        timer,
      ));
    });
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
  let player_transform = player_query.get_single().unwrap();

  for (mut charging, entity, mut velocity, transform) in &mut query {
    if charging.attack_entity.is_some() {
      continue;
    };

    velocity.linvel = Vec2::ZERO;

    let shape = Mesh2dHandle(meshes.add(CircularSector::new(charging.range, 1.0)));
    let material = materials.add(Color::srgba(1.0, 0.0, 0.0, 0.2));

    // TODO: angle is not initialized correctly
    let angle =
      (player_transform.translation.truncate() - transform.translation.truncate()).to_angle();

    // spawn cone entity
    let cone = (
      StateDespawnMarker,
      MaterialMesh2dBundle {
        mesh: shape,
        material,
        transform: Transform {
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

fn tick_charge_timer(mut query: Query<&mut Charging, With<Enemy>>, time: Res<Time>) {
  for mut charging_data in query.iter_mut() {
    charging_data.timer.tick(time.delta());
  }
}

fn tick_delivery_timer(
  mut query: Query<(&mut Delivering, Entity), With<Enemy>>,
  time: Res<Time>,
  mut ev_writer: EventWriter<DeliveringEvent>,
) {
  for (mut delivering_data, entity) in query.iter_mut() {
    delivering_data.timer.tick(time.delta());
    if delivering_data.timer.just_finished() {
      ev_writer.send(DeliveringEvent(entity, delivering_data.radius));
    }
  }
}

fn tick_ready_timer(mut query: Query<&mut Ready, With<Enemy>>, time: Res<Time>) {
  for mut ready_data in query.iter_mut() {
    ready_data.timer.tick(time.delta());
  }
}

fn handle_delivering_event(
  mut delivering_event: EventReader<DeliveringEvent>,
  enemy_query: Query<(&Children, Entity), With<Enemy>>,
  cone_query: Query<&mut Parent, With<AttackCone>>,
  mut commands: Commands,
) {
  for evt in delivering_event.read() {
    // entity which fired the event
    let (entity, radius) = (evt.0, evt.1);
    for (children, enemy_entity) in &enemy_query {
      if entity != enemy_entity {
        continue;
      }

      for child in children {
        let is_cone = cone_query.get(*child).is_ok();
        if !is_cone {
          continue;
        }

        commands.entity(entity).remove_children(&[*child]);
        commands.entity(*child).despawn_recursive();

        // add collider for a frame
        let collider = commands
          .spawn(Collider::ball(radius))
          .insert(StateDespawnMarker)
          .insert(ActiveEvents::COLLISION_EVENTS)
          .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC)
          .insert(Sensor)
          .insert(CollisionGroups::new(ATTACK_GROUP, PLAYER_GROUP))
          .insert(DespawnTimer(Timer::from_seconds(
            ENEMY_DELIVER_TIME,
            TimerMode::Once,
          )))
          .id();
        commands.entity(entity).push_children(&[collider]);
      }
    }
  }
}

fn get_ready(
  enemies: Query<(&Ready, &Children), With<Enemy>>,
  material_query: Query<&mut Handle<ColorMaterial>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  time: Res<Time>,
) {
  for (_, children) in &enemies {
    for child in children.iter() {
      if let Ok(handle) = material_query.get(*child) {
        let material = materials.get_mut(handle).unwrap();
        let alpha = (time.elapsed_seconds() * READY_FLICKER_FREQUENCY)
          .sin()
          .abs()
          * READY_FLICKER_WAVELENGTH;
        material.color = Color::srgba(1.0, 0.0, 0.0, alpha);
      }
    }
  }
}

// instantiate collider
fn deliver() {}

fn check_for_collisions(
  mut collision_events: EventReader<CollisionEvent>,
  player_query: Query<Entity, With<Player>>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  for collision in collision_events.read() {
    if let CollisionEvent::Started(first_entity, entity, CollisionEventFlags::SENSOR) = collision {
      let p = player_query.get_single().unwrap();
      if p == *first_entity || p == *entity {
        next_state.set(AppState::GameOver);
      }
    }
  }
}
