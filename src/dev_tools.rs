use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
  app.add_systems(Update, log_transitions::<Screen>);
}
