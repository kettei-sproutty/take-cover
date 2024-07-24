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
    app.add_systems(Update, action_on_press.after(style_interaction));
  }
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
      StateDespawnMarker,
      ImageBundle {
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
    ))
    .id();

  commands
    .entity(container)
    .push_children(&[play_button, volume_icon]);
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
  mut text_query: Query<&mut Text>,
) {
  for (interaction, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match interaction {
      Interaction::Pressed => {
        text.sections[0].style.color = colors::PRIMARY_100;
      }
      Interaction::Hovered => {
        text.sections[0].style.color = colors::PRIMARY_200;
      }
      Interaction::None => {
        text.sections[0].style.color = colors::PRIMARY_300;
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
