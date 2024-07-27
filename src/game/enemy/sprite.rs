use bevy::prelude::*;

use crate::game::common::{AnimationIndices, AnimationTimer};

use super::{EnemyVariant, ANIMATION_RATE};

pub fn get_idle_animation(
  variant: &EnemyVariant,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> (
  Handle<Image>,
  TextureAtlas,
  AnimationIndices,
  AnimationTimer,
) {
  let path = match variant {
    EnemyVariant::Red => "aqua",
    EnemyVariant::Purple => "aqua",
    EnemyVariant::Gray => "aqua",
  };
  let texture: Handle<Image> =
    asset_server.load(format!("textures/worm/sheet/worm_{path}_norm.png"));
  let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 7, 6, None, None);
  let texture_atlas_layouts = texture_atlas_layouts.add(layout);

  let animation_indices = AnimationIndices { first: 0, last: 3 };
  (
    texture,
    TextureAtlas {
      layout: texture_atlas_layouts,
      index: animation_indices.first,
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(ANIMATION_RATE, TimerMode::Repeating)),
  )
}
