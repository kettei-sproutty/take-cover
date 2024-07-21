use bevy::prelude::*;
use take_cover::AppPlugin;

fn main() -> AppExit {
  App::new().add_plugins(AppPlugin).run()
}
