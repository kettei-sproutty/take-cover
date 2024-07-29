use bevy_rapier2d::prelude::Group;

#[allow(dead_code)]
pub mod colors {
  use bevy::color::{Color, Hsla};

  pub const PRIMARY_100: Color = Color::Hsla(Hsla::new(180., 2.2 / 100., 91.2 / 100., 1.));
  pub const PRIMARY_200: Color = Color::Hsla(Hsla::new(210., 2. / 100., 80.8 / 100., 1.));
  pub const PRIMARY_300: Color = Color::Hsla(Hsla::new(220., 1.5 / 100., 61.4 / 100., 1.));
  pub const PRIMARY_400: Color = Color::Hsla(Hsla::new(210., 2.8 / 100., 42. / 100., 1.));
  pub const PRIMARY_500: Color = Color::Hsla(Hsla::new(205.7, 6.1 / 100., 22.5 / 100., 1.));
  pub const PRIMARY_600: Color = Color::Hsla(Hsla::new(214.3, 11.9 / 100., 11.6 / 100., 1.));
  pub const PRIMARY_700: Color = Color::Hsla(Hsla::new(210., 13.6 / 100., 8.6 / 100., 1.));
  pub const PRIMARY_800: Color = Color::Hsla(Hsla::new(200., 10.3 / 100., 5.7 / 100., 1.));
  pub const PRIMARY_900: Color = Color::Hsla(Hsla::new(210., 14.3 / 100., 2.7 / 100., 1.));

  pub const RED_100: Color = Color::Hsla(Hsla::new(0., 93. / 100., 94. / 100., 1.));
  pub const RED_200: Color = Color::Hsla(Hsla::new(0., 96. / 100., 89. / 100., 1.));
  pub const RED_300: Color = Color::Hsla(Hsla::new(0., 94. / 100., 82. / 100., 1.));
  pub const RED_400: Color = Color::Hsla(Hsla::new(0., 91. / 100., 71. / 100., 1.));
  pub const RED_500: Color = Color::Hsla(Hsla::new(0., 84. / 100., 60. / 100., 1.));
  pub const RED_600: Color = Color::Hsla(Hsla::new(0., 80. / 100., 49. / 100., 1.));
  pub const RED_700: Color = Color::Hsla(Hsla::new(0., 74. / 100., 42. / 100., 1.));
  pub const RED_800: Color = Color::Hsla(Hsla::new(0., 70. / 100., 35. / 100., 1.));
  pub const RED_900: Color = Color::Hsla(Hsla::new(0., 63. / 100., 31. / 100., 1.));
}

pub const ANIMATION_RATE: f32 = 0.1;
pub const BASE_ENEMIES: f32 = 10.0;
pub const DODGING_SPEED: f32 = 8.0;
pub const DODGING_TIMER: f32 = 0.15;
pub const DODGING_COOLDOWN: f32 = 1.0;
pub const ENEMY_CHARGING_RANGE: f32 = SPRITE_SIZE * 3.0;
pub const ENEMY_CHARGING_TIME: f32 = 2.0;
pub const ENEMY_DEFAULT_SPRITE_INDEX: usize = 0;
pub const ENEMY_READY_TIME: f32 = 0.5;
pub const ENEMY_DELIVER_TIME: f32 = 1.0;
pub const READY_FLICKER_FREQUENCY: f32 = 40.0;
pub const READY_FLICKER_WAVELENGTH: f32 = 0.5;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const SPRITE_SIZE: f32 = 22.0;
pub const PLAYER_SPEED: f32 = SPRITE_SIZE * 3.0;
pub const PLAYER_DEFAULT_SPRITE_INDEX: usize = 0;
/// The duration of a cycle, expressed in seconds.
pub const CYCLE_DURATION: f32 = 5.0;
/// The weight of a cycle, expressed in arbitrary units.
/// Every unit of weight is equivalent of 1 falling meteor.
pub const CYCLE_WEIGHT: usize = 3;
#[allow(dead_code)]
/// The delay between meteor spawns, expressed in seconds.
pub const METEOR_SPAWN_DELAY: f32 = 0.25;

// z-indexes
pub const PLAYER_Z_INDEX: f32 = 4.0;
pub const CAMERA_Z_INDEX: f32 = 10.0;
pub const ENEMY_Z_INDEX: f32 = 5.0;
pub const ENEMY_ATTACK_GIZMO_Z_INDEX: f32 = 2.0;

// colliders
pub const ATTACK_GROUP: Group = Group::GROUP_1;
pub const ENEMY_GROUP: Group = Group::GROUP_2;
pub const PLAYER_GROUP: Group = Group::GROUP_3;
pub const ATTACK_TRAIL_GROUP: Group = Group::GROUP_4;

// attack
pub const MIN_ATTACK_AREA: f32 = 10_000.0;
