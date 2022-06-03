use bevy::prelude::*;

use crate::app_state::AppState;

pub struct PausedScreen;

impl Plugin for PausedScreen {
    fn name(&self) -> &str {
        "PausedScreen"
    }

    fn build(&self, app: &mut App) {
       app
        .add_system_set(
            SystemSet::on_enter(AppState::Paused).with_system(on_enter)
        );
    }
}

fn on_enter() {
    info!("on_enter paused");
}