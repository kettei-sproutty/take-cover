use bevy::prelude::*;

use super::{animations::AnimationTimer, EnemyVariant, ANIMATION_RATE, ENEMY_DEFAULT_SPRITE_INDEX};

pub fn get_idle_animation(
  variant: &EnemyVariant,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> (Handle<Image>, TextureAtlas, AnimationTimer) {
  let path = match variant {
    EnemyVariant::Aqua => "aqua",
    EnemyVariant::Purple => "purple",
    EnemyVariant::Green => "green",
  };
  let texture: Handle<Image> =
    asset_server.load(format!("textures/worm/sheet/worm_{path}_norm.png"));
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
