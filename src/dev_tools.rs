use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::AppState;

/// A Bevy plugin that runs only when the `dev` feature is enabled.
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
  fn build(&self, app: &mut App) {
    // Track all [`AppState`] transitions
    app.add_systems(Update, log_transitions::<AppState>);
  }
}
