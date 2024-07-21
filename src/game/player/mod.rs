use bevy::prelude::*;
use spawn::{player_movement, spawn_player};

mod spawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_player)
      .add_systems(Update, player_movement);
  }
}
