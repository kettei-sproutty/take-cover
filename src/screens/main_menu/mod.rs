use bevy::audio::{PlaybackMode, Volume};

use crate::{
  assets::{MainMenuAssets, UiAssets},
  prelude::*,
};

#[allow(dead_code)]
#[derive(Default, Component)]
enum AudioStatus {
  #[default]
  Stopped,
  Playing,
}

#[allow(dead_code)]
#[derive(Component, Default)]
pub struct AudioButton(AudioStatus);

#[derive(Component)]
pub struct StateOnPress<S: States> {
  pub action: S,
}

pub struct MainMenuPlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for MainMenuPlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(
      OnEnter(self.state.clone()),
      (setup_main_menu, play_main_menu_audio),
    );
    app.add_systems(Update, style_interaction);
    app.add_systems(Update, action_on_press.before(style_interaction));
    app.add_systems(Update, check_audio_playback);
    app.add_systems(OnExit(self.state.clone()), reset_mouse_icon);
  }
}

fn reset_mouse_icon(mut windows: Query<&mut Window>) {
  let mut window = windows.single_mut();
  window.cursor.icon = CursorIcon::Default;
}

fn check_audio_playback(
  mut playback_query: Query<&mut AudioSink>,
  mut audio_query: Query<&mut AudioButton, Changed<AudioButton>>,
) {
  let Ok(playback_settings) = playback_query.get_single_mut() else {
    return;
  };

  let Ok(audio_status) = audio_query.get_single_mut() else {
    return;
  };

  match audio_status.0 {
    AudioStatus::Playing => playback_settings.toggle(),
    AudioStatus::Stopped => playback_settings.toggle(),
  };
}

fn setup_main_menu(mut commands: Commands, ui: Res<UiAssets>, main_menu_ui: Res<MainMenuAssets>) {
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
          row_gap: Val::Px(32.),
          ..Default::default()
        },
        ..Default::default()
      },
    ))
    .id();

  let play_button = commands
    .spawn((
      ButtonBundle::default(),
      StateOnPress {
        action: AppState::InGame,
      },
    ))
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Play",
        TextStyle {
          font: ui.font_sans.clone(),
          color: colors::PRIMARY_100,
          font_size: 32.,
        },
      ));
    })
    .id();

  let volume_icon = commands
    .spawn((
      ButtonBundle {
        style: Style {
          width: Val::Px(32.),
          height: Val::Px(32.),
          position_type: PositionType::Absolute,
          top: Val::Px(16.),
          right: Val::Px(16.),
          ..Default::default()
        },
        image: main_menu_ui.volume_waves_icon.clone().into(),
        ..Default::default()
      },
      AudioButton(AudioStatus::Playing),
    ))
    .id();

  // Actions Legend Layout
  let legend_grid = commands
    .spawn(NodeBundle {
      style: Style {
        position_type: PositionType::Absolute,
        left: Val::Px(16.),
        bottom: Val::Px(32.),
        display: Display::Grid,
        column_gap: Val::Px(24.),
        row_gap: Val::Px(16.),
        align_items: AlignItems::Center,
        justify_items: JustifyItems::Start,
        grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.)],
        grid_template_rows: RepeatedGridTrack::flex(3, 1.),
        ..Default::default()
      },
      ..Default::default()
    })
    .with_children(|parent| {
      // Dodge
      parent.spawn(ImageBundle {
        image: main_menu_ui.shift_icon.clone().into(),
        style: Style {
          width: Val::Px(46.),
          height: Val::Px(20.),
          justify_self: JustifySelf::Center,
          ..Default::default()
        },
        ..Default::default()
      });

      parent.spawn(TextBundle::from_section(
        "Dodge",
        TextStyle {
          font: ui.font_sans.clone(),
          color: colors::PRIMARY_300,
          ..Default::default()
        },
      ));

      // Attack
      parent.spawn(ImageBundle {
        image: main_menu_ui.click_icon.clone().into(),
        style: Style {
          width: Val::Px(28.),
          height: Val::Px(44.),
          justify_self: JustifySelf::Center,
          ..Default::default()
        },
        ..Default::default()
      });

      parent.spawn(TextBundle::from_section(
        "Draw shape to attack!",
        TextStyle {
          font: ui.font_sans.clone(),
          color: colors::PRIMARY_300,
          ..Default::default()
        },
      ));

      // Move
      parent.spawn(ImageBundle {
        image: main_menu_ui.wasd_icon.clone().into(),
        style: Style {
          justify_self: JustifySelf::Center,
          width: Val::Px(72.),
          ..Default::default()
        },
        ..Default::default()
      });

      parent.spawn(TextBundle::from_section(
        "Move player",
        TextStyle {
          font: ui.font_sans.clone(),
          color: colors::PRIMARY_300,
          ..Default::default()
        },
      ));
    })
    .id();

  commands
    .entity(container)
    .push_children(&[play_button, volume_icon, legend_grid]);
}

fn play_main_menu_audio(mut commands: Commands, audio: Res<MainMenuAssets>) {
  commands.spawn((
    StateDespawnMarker,
    AudioBundle {
      source: audio.music.clone(),
      settings: PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::new(0.2),
        ..Default::default()
      },
    },
  ));
}

fn style_interaction(
  mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
  mut audio_query: Query<
    (&Interaction, &mut UiImage, &mut AudioButton),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
  mut cursor_query: Query<&mut Window>,
  main_menu_assets: Res<MainMenuAssets>,
) {
  let mut window = cursor_query.single_mut();

  for (interaction, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match interaction {
      Interaction::Pressed => {
        text.sections[0].style.color = colors::PRIMARY_100;
      }
      Interaction::Hovered => {
        text.sections[0].style.color = colors::PRIMARY_200;
        window.cursor.icon = CursorIcon::Pointer;
      }
      Interaction::None => {
        text.sections[0].style.color = colors::PRIMARY_300;
        window.cursor.icon = CursorIcon::Default;
      }
    }
  }

  for (interaction, mut image, mut audio) in &mut audio_query {
    match interaction {
      Interaction::Pressed => {
        audio.0 = match audio.0 {
          AudioStatus::Playing => {
            image.texture = main_menu_assets.volume_stopped_icon.clone();
            AudioStatus::Stopped
          }
          AudioStatus::Stopped => {
            image.texture = main_menu_assets.volume_waves_icon.clone();
            AudioStatus::Playing
          }
        };
      }
      Interaction::Hovered => {
        window.cursor.icon = CursorIcon::Pointer;
      }
      Interaction::None => {
        window.cursor.icon = CursorIcon::Default;
      }
    }
  }
}

fn action_on_press(
  mut interaction_query: Query<
    (&Interaction, &StateOnPress<AppState>),
    (Changed<Interaction>, With<Button>),
  >,
  mut next_state: ResMut<NextState<AppState>>,
) {
  for (interaction, state) in &mut interaction_query {
    if interaction == &Interaction::Pressed {
      next_state.set(state.action);
    }
  }
}
