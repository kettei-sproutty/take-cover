use bevy::prelude::*;

use crate::{assets::UiAssets, game::common::animations::AnimationTimer};

use super::{ANIMATION_RATE, PLAYER_DEFAULT_SPRITE_INDEX};

pub fn get_main_animation(
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
  image_handle: Res<UiAssets>,
) -> (Handle<Image>, TextureAtlas, AnimationTimer) {
  let texture = image_handle.player_spritesheet.clone();
  let layout = TextureAtlasLayout::from_grid(
    UVec2::new(28, 22),
    12,
    11,
    Some(UVec2::new(112, 23)),
    Some(UVec2::new(47, 23)),
  );
  let texture_atlas_layouts = texture_atlas_layouts.add(layout);

  (
    texture,
    TextureAtlas {
      layout: texture_atlas_layouts,
      index: PLAYER_DEFAULT_SPRITE_INDEX,
    },
    AnimationTimer(Timer::from_seconds(ANIMATION_RATE, TimerMode::Repeating)),
  )
}
