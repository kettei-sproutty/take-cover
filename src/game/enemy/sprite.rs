use bevy::prelude::*;

use crate::{assets::UiAssets, game::common::animations::AnimationTimer};

use super::{EnemyVariant, ANIMATION_RATE, ENEMY_DEFAULT_SPRITE_INDEX};

pub fn get_idle_animation(
  variant: &EnemyVariant,
  image_handle: Res<UiAssets>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> (Handle<Image>, TextureAtlas, AnimationTimer) {
  let texture = match variant {
    EnemyVariant::Aqua => image_handle.enemy_blue_spritesheet.clone(),
    EnemyVariant::Red => image_handle.enemy_red_spritesheet.clone(),
    EnemyVariant::Green => image_handle.enemy_green_spritesheet.clone(),
  };
  let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 7, 6, None, None);
  let texture_atlas_layouts = texture_atlas_layouts.add(layout);

  (
    texture,
    TextureAtlas {
      layout: texture_atlas_layouts,
      index: ENEMY_DEFAULT_SPRITE_INDEX,
    },
    AnimationTimer(Timer::from_seconds(ANIMATION_RATE, TimerMode::Repeating)),
  )
}
