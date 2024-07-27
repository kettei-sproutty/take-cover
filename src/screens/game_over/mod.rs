use crate::{assets::UiAssets, game::Score, prelude::*};

use super::main_menu::StateOnPress;

pub struct GameOverPlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for GameOverPlugin<S> {
  fn build(&self, #[allow(unused_variables)] app: &mut App) {
    app.add_systems(OnEnter(self.state.clone()), setup_game_over);
  }
}

fn setup_game_over(mut commands: Commands, ui: Res<UiAssets>, score: Res<Score>) {
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

  let play_again_button = commands
    .spawn((
      ButtonBundle::default(),
      StateOnPress {
        action: AppState::InGame,
      },
    ))
    .with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Play Again",
        TextStyle {
          font: ui.font_sans.clone(),
          color: colors::PRIMARY_100,
          font_size: 32.,
        },
      ));
    })
    .id();

  let game_over_text = commands
    .spawn(TextBundle::from_section(
      "Game Over",
      TextStyle {
        font: ui.font_sans.clone(),
        color: colors::PRIMARY_100,
        font_size: 48.,
      },
    ))
    .id();

  let score_text = commands
    .spawn(TextBundle::from_section(
      format!("Score: {:08}", score.0),
      TextStyle {
        font: ui.font_sans.clone(),
        color: colors::PRIMARY_100,
        font_size: 32.,
      },
    ))
    .id();

  commands
    .entity(container)
    .push_children(&[game_over_text, score_text, play_again_button]);
}
