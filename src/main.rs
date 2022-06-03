use bevy::prelude::*;

pub mod app_state;
pub mod character;

const WINDOW_TITLE: &'static str = "Rumario";
const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;

const BACKGROUND_COLOR: &'static str = "9290ff";


fn main() {
    App::new()

        // Set window properties
        .insert_resource(WindowDescriptor {
            title: WINDOW_TITLE.to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })

        // Clear the screen with a background color
        .insert_resource(ClearColor(Color::hex(BACKGROUND_COLOR).unwrap()))

        // Add default bevy plugins
        .add_plugins(DefaultPlugins)

        // Set initial app state
        .add_state(app_state::AppState::MainMenu)

        .add_startup_system(startup)

        .run();
}

fn startup() {}