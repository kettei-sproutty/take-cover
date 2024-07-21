use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;

mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    #[cfg(feature = "dev")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins(PlayerPlugin);
  }
}
