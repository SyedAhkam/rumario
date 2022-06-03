use bevy::prelude::*;

use crate::app_state::AppState;

pub struct InGameScreen;

impl Plugin for InGameScreen {
    fn name(&self) -> &str {
        "InGameScreen"
    }

    fn build(&self, app: &mut App) {
       app
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(on_enter)
        );
    }
}

fn on_enter() {
    info!("on_enter in_game");
}