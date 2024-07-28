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

#[allow(dead_code)]
#[derive(Component)]
struct Dash(Timer);

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

  let move_state_machine = StateMachine::default()
    .trans::<Idle, _>(has_moved, Move)
    .trans::<Move, _>(has_moved.not(), Idle);

  #[cfg(feature = "dev")]
  let move_state_machine = move_state_machine.set_trans_logging(true);

  commands
    .spawn((
      StateDespawnMarker,
      move_state_machine,
      Collider::cuboid(8., 8.),
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
      Dash(Timer::from_seconds(1., TimerMode::Once)),
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
  mut player_info: Query<(&Player, &mut Velocity, &mut Dash)>,
) {
  // TODO: Implement dash
  for (player, mut rb_vels, mut _dash) in &mut player_info {
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

    // TODO: Implement dash
  }
}

// fn attack(mouse_input: Res<MouseButtonInput)
