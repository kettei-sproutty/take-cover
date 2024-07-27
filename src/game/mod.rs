pub mod cycle;
pub mod enemy;
pub mod player;

use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use cycle::CyclePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

use crate::{assets::UiAssets, prelude::*};

pub struct GamePlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for GamePlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(self.state.clone()), setup_game);
    app.insert_resource(LevelSelection::index(0));

    app.add_plugins((PlayerPlugin, EnemyPlugin, CyclePlugin));
  }
}

fn setup_game(mut commands: Commands, ui: Res<UiAssets>) {
  commands.spawn((
    StateDespawnMarker,
    LdtkWorldBundle {
      ldtk_handle: ui.planet.clone(),
      ..Default::default()
    },
  ));
}
