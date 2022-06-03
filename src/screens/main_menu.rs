use bevy::prelude::*;
use bevy_ui_navigation::NavigationPlugin;

use crate::{app_state::AppState, assets::FontAssets};

#[derive(Component)]
struct ColorText;

pub struct MainMenuScreen;

impl Plugin for MainMenuScreen {
    fn name(&self) -> &str {
        "MainMenuScreen"
    }

    fn build(&self, app: &mut App) {
        app
            .add_plugin(NavigationPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu).with_system(on_enter)
            );
    }
}


fn on_enter(mut commands: Commands, font_assets: Res<FontAssets>) {
    info!("on_enter main menu");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(250.0),
                    right: Val::Px(350.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: font_assets.emulogic.clone(),
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ColorText);
}
