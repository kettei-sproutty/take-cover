use bevy::{
  audio::{PlaybackMode, Volume},
  prelude::*,
};
use bevy_rapier2d::prelude::*;
use seldom_state::{
  prelude::{AnyState, StateMachine},
  trigger::IntoTrigger,
};
use sprite::get_main_animation;

use crate::{assets::UiAssets, prelude::*};

use super::common::animations::AnimationIndices;

mod sprite;

#[derive(Component)]
pub struct FootstepsIndices(pub Option<usize>);

#[derive(Component)]
pub struct Player {
  dodge_cooldown: Timer,
  has_ever_dodged: bool,
  last_direction: Vec2,
  speed: f32,
}

impl Default for Player {
  fn default() -> Self {
    Self {
      dodge_cooldown: Timer::from_seconds(DODGING_COOLDOWN, TimerMode::Once),
      has_ever_dodged: false,
      last_direction: Vec2::ZERO,
      speed: PLAYER_SPEED,
    }
  }
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Dodge {
  is_dodging: bool,
  timer: Timer,
}

impl Default for Dodge {
  fn default() -> Self {
    Self {
      is_dodging: false,
      timer: Timer::from_seconds(DODGING_TIMER, TimerMode::Once),
    }
  }
}

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Move;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), init_player);
    app.add_systems(
      Update,
      (
        move_player,
        dodge,
        tick_decelerate_timer,
        tick_dodge_cooldown_timer,
        play_footsteps,
      )
        .run_if(in_state(AppState::InGame))
        .after(init_player),
    );
  }
}

pub fn init_player(
  mut commands: Commands,
  texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
  ui_assets: Res<UiAssets>,
) {
  let has_moved = move |In(entity): In<Entity>, query: Query<&Player>| {
    let ctrl = query.get(entity);
    if let Ok(c) = ctrl {
      return c.last_direction.length() > 0.1;
    };

    false
  };

  let has_dodged = move |In(entity): In<Entity>,
                         query: Query<&Player, Without<Dodge>>,
                         keyboard: Res<ButtonInput<KeyCode>>| {
    let player_result = query.get(entity);
    match player_result {
      Ok(player) => {
        (player.dodge_cooldown.finished() || !player.has_ever_dodged)
          && keyboard.any_just_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
      }
      Err(_) => false,
    }
  };

  let is_dodge_done = move |In(entity): In<Entity>, query: Query<&Dodge, With<Player>>| {
    let player = query.get(entity);
    if let Ok(dodge) = player {
      return dodge.timer.finished();
    };

    false
  };

  let (texture, atlas, animation_timer) = get_main_animation(texture_atlas_layouts, &ui_assets);

  commands
    .spawn((
      StateDespawnMarker,
      StateMachine::default()
        .trans::<Idle, _>(has_moved, Move)
        .trans::<Move, _>(has_moved.not(), Idle)
        .trans::<Dodge, _>(is_dodge_done, Idle)
        .trans::<AnyState, _>(has_dodged, Dodge::default())
        .on_enter::<Idle>(|entity| {
          entity.insert((
            FootstepsIndices(None),
            AnimationIndices { first: 0, last: 10 },
          ));
        })
        .on_enter::<Move>(|entity| {
          entity.insert((
            FootstepsIndices(Some(0)),
            AnimationIndices {
              first: 24,
              last: 35,
            },
          ));
        })
        .on_enter::<Dodge>(|entity| {
          entity.insert((
            FootstepsIndices(None),
            AnimationIndices {
              first: 72,
              last: 75,
            },
          ));
        })
        .set_trans_logging(true),
      Collider::cuboid(8., 8.),
      CollisionGroups::new(PLAYER_GROUP, ATTACK_GROUP),
      ActiveCollisionTypes::all(),
      SpatialBundle::from_transform(Transform::from_xyz(320.0, 320.0, PLAYER_Z_INDEX)),
      ActiveEvents::COLLISION_EVENTS,
      RigidBody::Dynamic,
      LockedAxes::ROTATION_LOCKED,
      Velocity::zero(),
      Player::default(),
      GravityScale(0.),
      AnimationIndices { first: 0, last: 11 },
      AudioBundle {
        source: ui_assets.footsteps[0].clone(),
        settings: PlaybackSettings {
          mode: PlaybackMode::Remove,
          volume: Volume::new(0.2),
          ..Default::default()
        },
      },
      Idle,
    ))
    .with_children(|parent| {
      parent.spawn((
        SpriteBundle {
          sprite: Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
            ..default()
          },
          texture,
          transform: Transform::from_xyz(-0.5, -0.5, PLAYER_Z_INDEX),
          ..Default::default()
        },
        atlas,
        animation_timer,
      ));
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
  mut player_info: Query<(&mut Player, &mut Velocity), Without<Dodge>>,
  time: Res<Time>,
) {
  for (mut player, mut rb_vels) in &mut player_info {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    let move_delta = Vec2::new(x_axis as f32, y_axis as f32).normalize_or_zero();

    // Update the velocity on the rigid_body_component,
    // the bevy_rapier plugin will update the Sprite transform.
    rb_vels.linvel = move_delta * player.speed * time.delta_seconds();
    player.last_direction = rb_vels.linvel;
  }
}

fn dodge(mut player_query: Query<(&mut Dodge, &mut Player, &mut Velocity)>) {
  let player = player_query.get_single_mut();
  #[allow(clippy::single_match)]
  match player {
    Ok((mut dodge, mut usable_player, mut velocity)) => {
      if dodge.is_dodging {
        return;
      }

      usable_player.dodge_cooldown.reset();
      usable_player.has_ever_dodged = true;
      dodge.is_dodging = true;
      velocity.linvel = usable_player.last_direction * DODGING_SPEED;
    }
    Err(_) => (),
  };
}

fn tick_decelerate_timer(mut timer_query: Query<&mut Dodge, With<Player>>, time: Res<Time>) {
  for mut dodge in &mut timer_query {
    dodge.timer.tick(time.delta());
  }
}

fn tick_dodge_cooldown_timer(mut query: Query<&mut Player>, time: Res<Time>) {
  for mut player in &mut query {
    player.dodge_cooldown.tick(time.delta());
  }
}

fn play_footsteps(
  mut commands: Commands,
  mut query: Query<(Entity, &mut FootstepsIndices), (With<Move>, Without<AudioSink>)>,
  ui_assets: Res<UiAssets>,
) {
  for (entity, mut footsteps) in &mut query {
    if footsteps.0.is_none() {
      continue;
    };

    let index = footsteps.0.unwrap();

    let audio_source = AudioBundle {
      source: ui_assets.footsteps[index].clone(),
      settings: PlaybackSettings {
        mode: PlaybackMode::Remove,
        volume: Volume::new(0.2),
        ..Default::default()
      },
    };

    commands.entity(entity).insert(audio_source);

    let new_index = (index + 1) % ui_assets.footsteps.len();
    footsteps.0 = Some(new_index);
  }
}
