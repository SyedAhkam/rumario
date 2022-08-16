use bevy::prelude::*;

use crate::app_state::AppState;

pub struct LoadingScreen;

impl Plugin for LoadingScreen {
    fn name(&self) -> &str {
        "LoadingScreen"
    }

    fn build(&self, app: &mut App) {
       app
        .add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(on_enter)
        );
    }
}

fn on_enter() {
    info!("Loading assets");
}