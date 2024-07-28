pub mod attack;
pub mod cycle;
pub mod enemy;
pub mod player;

use attack::AttackPlugin;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
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

#[derive(Component)]
struct CycleComponent;

impl<S: States> Plugin for GamePlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(self.state.clone()), setup_game);
    app.insert_resource(LevelSelection::index(0));
    app.insert_resource(Score(0));

    app.add_plugins((PlayerPlugin, EnemyPlugin, CyclePlugin, AttackPlugin));
    app.add_systems(Update, (update_score, update_state));
  }
}

fn setup_game(
  mut commands: Commands,
  ui: Res<UiAssets>,
  mut score: ResMut<Score>,
  state: Res<State<CycleState>>,
) {
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

  commands
    .spawn((
      StateDespawnMarker,
      NodeBundle {
        style: Style {
          height: Val::Px(32.),
          position_type: PositionType::Absolute,
          top: Val::Px(46.),
          right: Val::Px(16.),
          ..Default::default()
        },
        ..Default::default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        CycleComponent,
        TextBundle::from_section(
          match state.get() {
            CycleState::Standard => "Standard".to_string(),
            CycleState::Meteors => "Meteors".to_string(),
          },
          TextStyle {
            font: ui.font_mono.clone(),
            font_size: 24.,
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

fn update_state(state: Res<State<CycleState>>, mut query: Query<&mut Text, With<CycleComponent>>) {
  for mut text in query.iter_mut() {
    text.sections[0].value = match state.get() {
      CycleState::Standard => "Standard".to_string(),
      CycleState::Meteors => "Meteors".to_string(),
    };
  }
}
