use crate::prelude::*;

pub struct GameOverPlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for GameOverPlugin<S> {
  fn build(&self, #[allow(unused_variables)] app: &mut App) {
    print!("GameOverPlugin {:?}", self.state);
  }
}
