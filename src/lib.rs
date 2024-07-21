use bevy::{
  asset::AssetMetaCheck,
  audio::{AudioPlugin, Volume},
  prelude::*,
};
use game::GamePlugin;

#[cfg(feature = "dev")]
mod dev_tools;
mod game;
pub mod screens;

pub struct AppPlugin;

impl Plugin for AppPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_camera);

    app.add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Window {
            title: "Take Cover".to_string(),
            canvas: Some("#take-cover-canvas".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
          }
          .into(),
          ..default()
        })
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
          meta_check: AssetMetaCheck::Never,
          ..default()
        })
        .set(AudioPlugin {
          global_volume: GlobalVolume {
            volume: Volume::new(0.3),
          },
          ..default()
        }),
    );

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.add_plugins(GamePlugin);
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    Name::new("Camera"),
    Camera2dBundle::default(),
    IsDefaultUiCamera,
  ));
}
