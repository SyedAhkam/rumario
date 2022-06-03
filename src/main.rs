use bevy::prelude::*;

use bevy_asset_loader::AssetLoader;

pub mod app_state;
pub mod character;
pub mod assets;

use app_state::AppState;
use assets::{AudioAssets, ImageAssets};

const APP_NAME: &'static str = "Rumario";
const VERSION: &'static str = "v0.1";

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGHT: f32 = 500.;

const BACKGROUND_COLOR: &'static str = "9290ff";

fn main() {
    // Create an app with default plugins
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Configure the asset loader
    AssetLoader::new(AppState::Loading)
        .continue_to_state(AppState::MainMenu)
        .with_collection::<AudioAssets>()
        .with_collection::<ImageAssets>()
        .build(&mut app);

    app    
        // Set window properties
        .insert_resource(WindowDescriptor {
            title: APP_NAME.to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })

        // Clear the screen with a background color
        .insert_resource(ClearColor(Color::hex(BACKGROUND_COLOR).unwrap()))

        // Add a startup system
        .add_startup_system(startup)

        // Add initial app state
        .add_state(AppState::Loading)

        .add_system(on_tick)

        .run();

}

fn startup(mut commands: Commands) {
    info!("Starting {} {}.", APP_NAME, VERSION);

    // Create a UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Create a 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn on_tick(app_state: Res<State<AppState>>) {
    info!("Ticking: {:?}", app_state);
}