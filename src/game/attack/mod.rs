use bevy::window::PrimaryWindow;
use seldom_state::prelude::StateMachine;

use crate::prelude::*;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Idle;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
struct Attack;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
#[allow(dead_code)]
struct AttackEnd;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), init_attack);
    app.add_systems(
      Update,
      track_mouse_movement
        .run_if(in_state(AppState::InGame))
        .after(init_attack),
    );
  }
}

fn init_attack(mut commands: Commands) {
  let left_mouse_down =
    |input: Res<ButtonInput<MouseButton>>| input.just_pressed(MouseButton::Left);

  let left_mouse_up = |input: Res<ButtonInput<MouseButton>>| input.just_released(MouseButton::Left);

  let state_machine = StateMachine::default()
    .trans::<Idle, _>(left_mouse_down, Attack)
    .trans::<Attack, _>(left_mouse_up, Idle);

  #[cfg(feature = "dev")]
  let state_machine = state_machine.set_trans_logging(true);

  commands.spawn((StateDespawnMarker, state_machine, Idle));
}

fn track_mouse_movement(
  // mut commands: Commands,
  query: Query<Entity, With<Attack>>,
  q_windows: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(_entity) = query.get_single() {
    if let Some(position) = q_windows.single().cursor_position() {
      println!("Mouse position: {:?}", position);
    }
  }
}
