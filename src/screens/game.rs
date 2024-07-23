use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

use crate::{
  assets::{MainMenuAudio, UiAssets},
  prelude::*,
};

pub struct MainMenuPlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for MainMenuPlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(
      OnEnter(self.state.clone()),
      (setup_main_menu, play_main_menu_audio),
    );
    app.insert_resource(LevelSelection::index(0));
  }
}

fn setup_main_menu(mut commands: Commands, ui: Res<UiAssets>) {
  let mut camera = Camera2dBundle::default();
  camera.projection.scale = 0.5;
  camera.transform.translation.x += 1280.0 / 4.0;
  camera.transform.translation.y += 720.0 / 4.0;
  commands.spawn(camera);

  commands.spawn(LdtkWorldBundle {
    ldtk_handle: ui.planet.clone(),
    ..Default::default()
  });
}

fn play_main_menu_audio(mut commands: Commands, audio: Res<MainMenuAudio>) {
  commands.spawn((
    StateDespawnMarker,
    AudioBundle {
      source: audio.music.clone(),
      settings: PlaybackSettings::LOOP,
    },
  ));
}
