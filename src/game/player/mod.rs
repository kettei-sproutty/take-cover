use bevy::prelude::*;
use movement::player_movement;
use spawn::spawn_player;

mod movement;
mod spawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_player)
      .add_systems(Update, player_movement);
  }
}
