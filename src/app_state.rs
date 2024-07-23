use crate::prelude::*;
use enum_iterator::{all, Sequence};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
  fn build(&self, app: &mut App) {
    // Add the AppState resource to the App
    app.init_state::<AppState>();

    for state in all::<AppState>() {
      // When a change of state occurs, despawn all entities that have the StateDespawnMarker component
      app.add_systems(OnExit(state), despawn_all_recursive::<StateDespawnMarker>);
    }
  }
}

/// State that describes the current state of the application,
/// that defines in which state the application is.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default, States, Reflect, Sequence)]
pub enum AppState {
  /// The `default` state of the application.
  /// In this state the application is loading assets.
  #[default]
  AssetsLoading,
  /// The `main_menu` state of the application.
  /// In this state the application is displaying the main menu.
  /// The user can start a new game or quit the application.
  MainMenu,
  /// The `game` state of the application.
  /// In this state the application is running the game.
  InGame,
}

/// Marker component that is used to despawn all entities that are in a specific state.
///
/// When spawning an entity that is stricly related to a specific state, you can add this component
/// to the entity to make sure that the entity is despawned when the state changes.
#[derive(Component)]
pub struct StateDespawnMarker;
