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
  pub player_spritesheet: Handle<Image>,
  pub enemy_red_spritesheet: Handle<Image>,
  pub enemy_blue_spritesheet: Handle<Image>,
  pub enemy_green_spritesheet: Handle<Image>,
  pub footsteps: [Handle<AudioSource>; 5],
  pub game_soundtrack: Handle<AudioSource>,
  pub enemy_dirt_sprite: Handle<Image>,
  pub dead_enemy_sprite: Handle<Image>,
  pub dead_enemy_red_sprite: Handle<Image>,
  pub dead_enemy_green_sprite: Handle<Image>,
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

  // player spritesheet
  let planet = asset_server.load("levels/walls.ldtk");
  let player_spritesheet: Handle<Image> = asset_server.load("textures/player/player_norm.png");

  // enemy spritesheets
  let dead_enemy_sprite: Handle<Image> = asset_server.load("textures/worm/thorax.png");
  let dead_enemy_red_sprite: Handle<Image> = asset_server.load("textures/worm/red_thorax.png");
  let dead_enemy_green_sprite: Handle<Image> = asset_server.load("textures/worm/green_thorax.png");
  let enemy_dirt_sprite: Handle<Image> = asset_server.load("textures/worm/dirt.png");
  let enemy_blue_spritesheet: Handle<Image> =
    asset_server.load("textures/worm/sheet/worm_blue_norm.png");
  let enemy_red_spritesheet: Handle<Image> =
    asset_server.load("textures/worm/sheet/worm_red_norm.png");
  let enemy_green_spritesheet: Handle<Image> =
    asset_server.load("textures/worm/sheet/worm_green_norm.png");

  // footsteps
  let footsteps: [Handle<AudioSource>; 5] = [
    asset_server.load("sounds/impact-sounds/footstep_wood_000.ogg"),
    asset_server.load("sounds/impact-sounds/footstep_wood_001.ogg"),
    asset_server.load("sounds/impact-sounds/footstep_wood_002.ogg"),
    asset_server.load("sounds/impact-sounds/footstep_wood_003.ogg"),
    asset_server.load("sounds/impact-sounds/footstep_wood_004.ogg"),
  ];

  // game soundtrack
  let game_soundtrack: Handle<AudioSource> = asset_server.load("sounds/stellar-drift.ogg");

  // Connect the assets to the loading tracker by `iyes_progress`
  loading.add(&font_sans);
  loading.add(&font_mono);
  loading.add(&atlas);
  loading.add(&planet);
  loading.add(&player_spritesheet);
  loading.add(&dead_enemy_sprite);
  loading.add(&dead_enemy_red_sprite);
  loading.add(&dead_enemy_green_sprite);
  loading.add(&enemy_blue_spritesheet);
  loading.add(&enemy_green_spritesheet);
  loading.add(&enemy_red_spritesheet);
  loading.add(&game_soundtrack);

  for footstep in footsteps.iter() {
    loading.add(footstep);
  }
  loading.add(&enemy_dirt_sprite);

  // Insert the assets resources into the game
  commands.insert_resource(UiAssets {
    dead_enemy_sprite,
    dead_enemy_green_sprite,
    dead_enemy_red_sprite,
    enemy_dirt_sprite,
    font_sans,
    font_mono,
    atlas,
    planet,
    player_spritesheet,
    enemy_blue_spritesheet,
    enemy_green_spritesheet,
    enemy_red_spritesheet,
    footsteps,
    game_soundtrack,
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
