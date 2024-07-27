use crate::prelude::*;
use enum_iterator::{all, Sequence};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
  fn build(&self, app: &mut App) {
    // Add the AppState resource to the App
    app
      .init_state::<AppState>()
      .add_sub_state::<MainMenuSubState>()
      .add_sub_state::<InGameSubState>();

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
  /// The `game_over` state of the application.
  /// In this state the application is displaying the game over screen.
  /// The user can restart the game or return to the main menu.
  GameOver,
}

#[allow(dead_code)]
/// [`SubStates`] for the [`AppState::MainMenu`] state.
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuSubState {
  #[default]
  /// The `default` [`SubStates`] of the main menu.
  /// In this sub-state the application is displaying the main menu.
  None,
  /// The `settings` [`SubStates`] of the application.
  /// In this state the application is displaying the settings menu.
  /// The user can change the settings of the application.
  /// The user can return to the main menu [`MainMenuSubState::None`].
  Settings,
  /// The `credits` [`SubStates`] of the application.
  /// In this state the application is displaying the credits.
  /// The user can see the credits of the application.
  /// The user can return to the main menu [`MainMenuSubState::None`].
  Credits,
}

#[allow(dead_code)]
/// [`SubStates`] for the [`AppState::InGame`] state.
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum InGameSubState {
  #[default]
  /// The `default` [`SubStates`] of the game.
  /// In this sub-state the application is running the game.
  /// The user can play the game.
  None,
  /// The `paused` [`SubStates`] of the game.
  /// In this state the application is paused.
  /// The user can resume the game [`InGameSubState::None`],
  /// or return to the main menu [`AppState::MainMenu`].
  Paused,
}

/// Marker component that is used to despawn all entities that are in a specific state.
///
/// When spawning an entity that is stricly related to a specific state, you can add this component
/// to the entity to make sure that the entity is despawned when the state changes.
#[derive(Component)]
pub struct StateDespawnMarker;
