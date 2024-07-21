use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
  Splash,
  Loading,
  MainMenu,
  Credits,
  #[default]
  Playing,
}
