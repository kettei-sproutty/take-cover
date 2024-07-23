use crate::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use iyes_progress::prelude::*;

pub struct AssetsLoadingPlugin;

#[derive(Resource)]
/// Resources for the UI assets
pub struct UiAssets {
  #[allow(unused)]
  font_sans: Handle<Font>,
  #[allow(unused)]
  font_mono: Handle<Font>,
  #[allow(unused)]
  pub atlas: Handle<Image>,
  pub planet: Handle<LdtkProject>,
}

impl Plugin for AssetsLoadingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::AssetsLoading), load_assets);
  }
}

/// Loading all game assets and tracking the progress
/// using the [`AssetsLoading`] resource by `iyes_progress`.
/// The progress will be shown on the loading screen in the
/// [`crate::screens::loading::LoadscreenPlugin`]
fn load_assets(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut loading: ResMut<AssetsLoading>,
) {
  // Loading all game assets
  let font_sans = asset_server.load("fonts/Exo2.ttf");
  let font_mono = asset_server.load("fonts/UbuntuMono.ttf");
  let atlas = asset_server.load("textures/1-bit/colored.png");
  let planet = asset_server.load("levels/planet.ldtk");

  // Connect the assets to the loading tracker by `iyes_progress`
  loading.add(&font_sans);
  loading.add(&font_mono);
  loading.add(&atlas);
  loading.add(&planet);

  // Insert the assets resources into the game
  commands.insert_resource(UiAssets {
    font_sans,
    font_mono,
    atlas,
    planet,
  });
}
