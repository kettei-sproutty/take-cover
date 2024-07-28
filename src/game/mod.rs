pub mod common;
pub mod cycle;
pub mod enemy;
pub mod player;

use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use common::flip;
use cycle::CyclePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

use crate::{assets::UiAssets, prelude::*};

pub struct GamePlugin<S: States> {
  pub state: S,
}

#[derive(Resource)]
pub struct Score(pub usize);

#[derive(Component)]
struct ScoreComponent;

impl<S: States> Plugin for GamePlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(self.state.clone()), setup_game);
    app.insert_resource(LevelSelection::index(0));
    app.insert_resource(Score(0));

    app.add_plugins((PlayerPlugin, EnemyPlugin, CyclePlugin));
    app.add_systems(Update, update_score);
    app.add_systems(Update, flip.run_if(in_state(AppState::InGame)));
  }
}

fn setup_game(mut commands: Commands, ui: Res<UiAssets>, mut score: ResMut<Score>) {
  score.0 = 0;

  commands.spawn((
    StateDespawnMarker,
    LdtkWorldBundle {
      ldtk_handle: ui.planet.clone(),
      ..Default::default()
    },
  ));

  commands
    .spawn((
      StateDespawnMarker,
      NodeBundle {
        style: Style {
          height: Val::Px(32.),
          position_type: PositionType::Absolute,
          top: Val::Px(16.),
          right: Val::Px(16.),
          ..Default::default()
        },
        ..Default::default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        ScoreComponent,
        TextBundle::from_section(
          format!("{:08}", score.0),
          TextStyle {
            font: ui.font_mono.clone(),
            font_size: 32.,
            color: Color::WHITE,
          },
        ),
      ));
    });
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreComponent>>) {
  for mut text in query.iter_mut() {
    text.sections[0].value = format!("{:08}", score.0);
  }
}
