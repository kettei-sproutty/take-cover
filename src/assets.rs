use crate::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use iyes_progress::prelude::*;

pub struct AssetsLoadingPlugin;

#[derive(Resource)]
/// Resources for the UI assets
pub struct UiAssets {
  pub font_sans: Handle<Font>,
  #[allow(unused)]
  pub font_mono: Handle<Font>,
  #[allow(unused)]
  pub atlas: Handle<Image>,
  pub planet: Handle<LdtkProject>,
}

#[derive(Resource)]
/// Audio Resources for main menu
pub struct MainMenuAssets {
  pub music: Handle<AudioSource>,
  pub volume_waves_icon: Handle<Image>,
  pub volume_stopped_icon: Handle<Image>,
  pub shift_icon: Handle<Image>,
  pub click_icon: Handle<Image>,
  pub wasd_icon: Handle<Image>,
}

impl Plugin for AssetsLoadingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      OnEnter(AppState::AssetsLoading),
      (load_ui_assets, load_main_menu_audio_assets),
    );
  }
}

/// Loading all game assets and tracking the progress
/// using the [`AssetsLoading`] resource by `iyes_progress`.
/// The progress will be shown on the loading screen in the
/// [`crate::screens::loading::LoadscreenPlugin`]
fn load_ui_assets(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut loading: ResMut<AssetsLoading>,
) {
  // Loading all game assets
  let font_sans = asset_server.load("fonts/Exo2.ttf");
  let font_mono = asset_server.load("fonts/JetBrainsMono.ttf");
  let atlas = asset_server.load("textures/1-bit/colored.png");
  let planet = asset_server.load("levels/walls.ldtk");

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

/// Loading all audio assets for the main menu
/// and tracking the progress using the [`AssetsLoading`] resource
/// by `iyes_progress`.
pub fn load_main_menu_audio_assets(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut loading: ResMut<AssetsLoading>,
) {
  // Loading all audio assets
  let music = asset_server.load("sounds/main_menu.wav");
  let volume_waves_icon = asset_server.load("icons/volume-waves.png");
  let volume_stopped_icon = asset_server.load("icons/volume-mute.png");

  // Add keyboard and mouse legend icons
  let shift_icon: Handle<Image> = asset_server.load("icons/shift.png");
  let click_icon: Handle<Image> = asset_server.load("icons/click.png");
  let wasd_icon: Handle<Image> = asset_server.load("icons/wasd.png");

  // Connect the assets to the loading tracker by `iyes_progress`
  loading.add(&music);
  loading.add(&volume_waves_icon);
  loading.add(&volume_stopped_icon);

  loading.add(&shift_icon);
  loading.add(&click_icon);
  loading.add(&wasd_icon);

  // Insert the audio resources into the game
  commands.insert_resource(MainMenuAssets {
    music,
    volume_waves_icon,
    volume_stopped_icon,
    shift_icon,
    click_icon,
    wasd_icon,
  });
}
