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
}

pub const ANIMATION_RATE: f32 = 0.1;
pub const BASE_ENEMIES: f32 = 5.0;
pub const ENEMY_CHARGING_RANGE: f32 = SPRITE_SIZE * 3.0;
pub const ENEMY_CHARGING_TIME: f32 = 2.0;
pub const ENEMY_DEFAULT_SPRITE_INDEX: usize = 0;
pub const ENEMY_READY_TIME: f32 = 0.5;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const SPRITE_SIZE: f32 = 16.0;
pub const PLAYER_SPEED: f32 = SPRITE_SIZE * 3.0;

// z-indexes
pub const PLAYER_Z_INDEX: f32 = 4.0;
pub const CAMERA_Z_INDEX: f32 = 10.0;
pub const ENEMY_Z_INDEX: f32 = 5.0;
pub const ENEMY_ATTACK_GIZMO_Z_INDEX: f32 = 2.0;
