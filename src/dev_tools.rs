use bevy::{dev_tools::states::log_transitions, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::prelude::*;

/// A Bevy plugin that runs only when the `dev` feature is enabled.
pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
  fn build(&self, app: &mut App) {
    // Track all [`AppState`] transitions
    app.add_systems(Update, log_transitions::<AppState>);

    app.add_plugins(RapierDebugRenderPlugin::default());

    // Beware that when these plugins are active, cursor icon won't change
    // Add the World and State inspector plugins by `bevy-inspector-egui`
    // The interface can be toggled by pressing the `Escape` key
    app.add_plugins((
      WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
      StateInspectorPlugin::<AppState>::default()
        .run_if(input_toggle_active(true, KeyCode::Escape)),
      StateInspectorPlugin::<CycleState>::default()
        .run_if(input_toggle_active(true, KeyCode::Escape)),
    ));
  }
}
