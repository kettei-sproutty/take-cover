use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::{prelude::StateMachine, trigger::IntoTrigger};

use crate::prelude::*;

#[derive(Component)]
pub struct Player {
  speed: f32,
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Move;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), init_player);
    app.add_systems(
      Update,
      move_player
        .run_if(in_state(AppState::InGame))
        .after(init_player),
    );
  }
}

pub fn init_player(mut commands: Commands) {
  let has_moved =
    move |In(entity): In<Entity>,
          query: Query<&KinematicCharacterControllerOutput, With<Player>>| {
      let ctrl = query.get(entity);
      if let Ok(c) = ctrl {
        return c.effective_translation.length() > 0.001;
      };

      false
    };

  commands
    .spawn((
      StateDespawnMarker,
      StateMachine::default()
        .trans::<Idle, _>(has_moved, Move)
        .trans::<Move, _>(has_moved.not(), Idle)
        .set_trans_logging(true),
      Collider::cuboid(8., 8.),
      CollisionGroups::new(PLAYER_GROUP, ATTACK_GROUP),
      (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC),
      ActiveEvents::COLLISION_EVENTS,
      RigidBody::KinematicVelocityBased,
      Velocity::zero(),
      SpriteBundle {
        sprite: Sprite {
          color: Color::srgb(0., 0., 0.),
          custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
          ..default()
        },
        transform: Transform::from_xyz(320., 320., PLAYER_Z_INDEX),
        ..Default::default()
      },
      Player {
        speed: PLAYER_SPEED,
      },
      GravityScale(0.),
      Idle,
    ))
    .with_children(|parent| {
      parent.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., CAMERA_Z_INDEX),
        projection: OrthographicProjection {
          scale: 0.4,
          ..Default::default()
        },
        ..Default::default()
      });
    });
}

fn move_player(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut player_info: Query<(&Player, &mut Velocity)>,
) {
  for (player, mut rb_vels) in &mut player_info {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    let move_delta = Vec2::new(x_axis as f32, y_axis as f32).normalize_or_zero();

    // Update the velocity on the rigid_body_component,
    // the bevy_rapier plugin will update the Sprite transform.
    rb_vels.linvel = move_delta * player.speed;
  }
}
