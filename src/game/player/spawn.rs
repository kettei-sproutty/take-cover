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
  pub walking_left: Vec<usize>,
  pub walking_down: Vec<usize>,
  pub walking_up: Vec<usize>,
  pub walking_right: Vec<usize>,
  pub direction: String,
  pub index: usize,
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
