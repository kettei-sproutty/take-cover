use assets::AssetsLoadingPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use game::GamePlugin;
use iyes_progress::prelude::*;
use screens::{game_over::GameOverPlugin, loading::LoadscreenPlugin, main_menu::MainMenuPlugin};

use crate::prelude::*;

mod app_state;
mod assets;
mod constants;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod prelude;
mod screens;
mod utils;

fn main() -> AppExit {
  let mut app = App::new();
  // Add the background color to the App
  app.insert_resource(ClearColor(colors::PRIMARY_900));

  let bevy_plugins = DefaultPlugins;

  // Change the default window settings
  let bevy_plugins = bevy_plugins.set(WindowPlugin {
    primary_window: Some(Window {
      #[cfg(not(target_arch = "wasm32"))]
      title: "Take Cover".into(),
      present_mode: bevy::window::PresentMode::Fifo,
      // TODO: handle fixed resolution
      // resizable: false,
      // resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
      prevent_default_event_handling: true,
      #[cfg(target_arch = "wasm32")]
      fit_canvas_to_parent: true,
      #[cfg(target_arch = "wasm32")]
      canvas: Some("#take-cover".into()),
      ..Default::default()
    }),
    ..Default::default()
  });

  let bevy_plugins = bevy_plugins.set(ImagePlugin::default_nearest());

  #[cfg(target_arch = "wasm32")]
  // Disable assets meta check on wasm to throw 4xx errors
  let bevy_plugins = bevy_plugins.set(AssetPlugin {
    meta_check: bevy::asset::AssetMetaCheck::Never,
    ..Default::default()
  });

  app.add_plugins(bevy_plugins);

  // Handling state machines
  app.add_plugins(seldom_state::StateMachinePlugin);

  // Add the AppState to the App
  app.add_plugins(crate::app_state::AppStatePlugin);

  // Physics
  app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
    TILE_SPRITE_SIZE,
  ));

  // Particle effects
  app.add_plugins(ParticleSystemPlugin);

  // We use [`iyes_progress`](https://github.com/IyesGames/iyes_progress) to track when we are done loading assets and transition to the main menu
  app.add_plugins(
    ProgressPlugin::new(AppState::AssetsLoading)
      .continue_to(AppState::MainMenu)
      .track_assets(),
  );

  app.add_plugins((
    LdtkPlugin,
    LoadscreenPlugin {
      state: AppState::AssetsLoading,
    },
    MainMenuPlugin {
      state: AppState::MainMenu,
    },
    GamePlugin {
      state: AppState::InGame,
    },
    GameOverPlugin {
      state: AppState::GameOver,
    },
    AssetsLoadingPlugin,
  ));

  #[cfg(feature = "dev")]
  app.add_plugins(dev_tools::DevToolsPlugin);

  app.run()
}
