use bevy::audio::{PlaybackMode, Volume};

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
  }
}

fn setup_main_menu(mut commands: Commands, ui: Res<UiAssets>) {
  // spawn a camera that despawn when `MainMenuState` is left
  commands.spawn((
    StateDespawnMarker,
    Camera2dBundle {
      camera: Camera {
        clear_color: ClearColorConfig::Custom(colors::PRIMARY_800),
        ..Default::default()
      },
      ..Default::default()
    },
  ));

  let container = commands
    .spawn((
      StateDespawnMarker,
      NodeBundle {
        style: Style {
          width: Val::Percent(100.),
          height: Val::Percent(100.),
          position_type: PositionType::Relative,
          flex_direction: FlexDirection::Column,
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          row_gap: Val::Px(16.),
          ..Default::default()
        },
        ..Default::default()
      },
    ))
    .id();

  let title = commands
    .spawn(NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        top: Val::Px(8.),
        right: Val::Px(8.),
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  let title_text = commands
    .spawn(TextBundle {
      text: Text {
        sections: vec![TextSection {
          value: "Take Cover | Bevy Jam 5".to_string(),
          style: TextStyle {
            font: ui.font_mono.clone(),
            color: colors::PRIMARY_100,
            font_size: 20.,
          },
        }],
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  commands.entity(title).push_children(&[title_text]);

  let play_button = commands
    .spawn(ButtonBundle {
      ..Default::default()
    })
    .id();

  let play_button_text = commands
    .spawn(TextBundle {
      text: Text {
        sections: vec![TextSection {
          value: "Play".to_string(),
          style: TextStyle {
            font: ui.font_sans.clone(),
            color: colors::PRIMARY_100,
            font_size: 32.,
          },
        }],
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  commands
    .entity(play_button)
    .push_children(&[play_button_text]);

  let footer = commands
    .spawn(NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(16.),
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  let footer_text = commands
    .spawn(TextBundle {
      text: Text {
        sections: vec![TextSection {
          value: "Made with <3 by Alessio Marchi and Mauro Bellinzona".to_string(),
          style: TextStyle {
            font: ui.font_mono.clone(),
            color: colors::PRIMARY_100,
            font_size: 16.,
          },
        }],
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  commands.entity(footer).push_children(&[footer_text]);

  commands
    .entity(container)
    .push_children(&[title, play_button, footer]);
}

fn play_main_menu_audio(mut commands: Commands, audio: Res<MainMenuAudio>) {
  commands.spawn((
    StateDespawnMarker,
    AudioBundle {
      source: audio.music.clone(),
      settings: PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::new(0.5),
        ..Default::default()
      },
    },
  ));
}
