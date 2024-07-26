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

pub const BASE_ENEMIES: f32 = 15.0;
pub const SPRITE_SIZE: f32 = 16.0;
pub const PLAYER_SPEED: f32 = SPRITE_SIZE * 3.0;
