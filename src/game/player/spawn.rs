use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player {
  /// The player's speed in pixels per second.
  pub speed: f32,
  #[allow(dead_code)]
  /// The player's owned keys.
  pub keys: u8,
}

#[derive(Component)]
pub struct AnimationIndices {
  walking_left: Vec<usize>,
  walking_down: Vec<usize>,
  walking_up: Vec<usize>,
  walking_right: Vec<usize>,
  direction: String,
  index: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(
  mut commands: Commands,
  mut rapier_config: ResMut<RapierConfiguration>,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
  rapier_config.gravity = Vec2::ZERO;

  let texture = asset_server.load("urban-pack/Tilemap/tilemap.png");
  let layout =
    TextureAtlasLayout::from_grid(UVec2::splat(16), 29, 29, UVec2::splat(1).into(), None);
  let texture_atlas_layout = texture_atlas_layouts.add(layout);

  let animation_indices = AnimationIndices {
    walking_left: vec![23, 52, 81],
    walking_down: vec![24, 53, 82],
    walking_up: vec![25, 54, 83],
    walking_right: vec![26, 55, 84],
    direction: "up".to_string(),
    index: 0,
  };

  let sprite_size = 64.0;

  commands.spawn((
    SpriteBundle {
      transform: Transform::from_scale(Vec3::splat(6.0)),
      texture,
      ..default()
    },
    TextureAtlas {
      layout: texture_atlas_layout,
      index: *animation_indices.walking_up.first().unwrap(),
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
    RigidBody::Dynamic,
    Velocity::zero(),
    Collider::ball(sprite_size / 2.0),
    Player {
      speed: 100.0,
      keys: 0,
    },
  ));
}

pub fn player_movement(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut player_info: Query<(&Player, &mut Velocity)>,
  time: Res<Time>,
  mut query: Query<(
    &mut AnimationIndices,
    &mut AnimationTimer,
    &mut TextureAtlas,
  )>,
) {
  for (player, mut rb_vels) in &mut player_info {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;

    let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
    if move_delta != Vec2::ZERO {
      move_delta /= move_delta.length();
    }

    rb_vels.linvel = move_delta * player.speed;

    for (mut indices, mut timer, mut atlas) in &mut query {
      timer.tick(time.delta() / 2);
      if !up && !down && !left && !right {
        if indices.direction == "up" {
          atlas.index = *indices.walking_up.first().unwrap();
        } else if indices.direction == "down" {
          atlas.index = *indices.walking_down.first().unwrap();
        } else if indices.direction == "left" {
          atlas.index = *indices.walking_left.first().unwrap();
        } else if indices.direction == "right" {
          atlas.index = *indices.walking_right.first().unwrap();
        }
        continue;
      }

      if up {
        if indices.direction != "up" {
          indices.direction = "up".into();
          indices.index = 0;
          atlas.index = *indices.walking_up.first().unwrap();
        } else if timer.finished() {
          if indices.index == indices.walking_up.len() - 1 {
            indices.index = 1;
            atlas.index = indices.walking_up[1];
          } else {
            indices.index += 1;
            atlas.index = indices.walking_up[indices.index];
          }
        }
      } else if down {
        if indices.direction != "down" {
          indices.direction = "down".into();
          indices.index = 0;
          atlas.index = *indices.walking_down.first().unwrap();
        } else if timer.finished() {
          if indices.index == indices.walking_down.len() - 1 {
            indices.index = 1;
            atlas.index = indices.walking_down[1];
          } else {
            indices.index += 1;
            atlas.index = indices.walking_down[indices.index];
          }
        }
      } else if left {
        if indices.direction != "left" {
          indices.direction = "left".into();
          indices.index = 0;
          atlas.index = *indices.walking_left.first().unwrap();
        } else if timer.finished() {
          if indices.index == indices.walking_left.len() - 1 {
            indices.index = 1;
            atlas.index = indices.walking_left[1];
          } else {
            indices.index += 1;
            atlas.index = indices.walking_left[indices.index];
          }
        }
      } else if right {
        if indices.direction != "right" {
          indices.direction = "right".into();
          indices.index = 0;
          atlas.index = *indices.walking_right.first().unwrap();
        } else if timer.finished() {
          if indices.index == indices.walking_right.len() - 1 {
            indices.index = 1;
            atlas.index = indices.walking_right[1];
          } else {
            indices.index += 1;
            atlas.index = indices.walking_right[indices.index];
          }
        }
      }
    }
  }
}
