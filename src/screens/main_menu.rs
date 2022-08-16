use std::ops::DerefMut;

use bevy::prelude::*;
use bevy_ui_navigation::{DefaultNavigationPlugins, NavRequestSystem, NavEvent, Focusable, FocusState, components::FocusableButtonBundle, event_helpers::NavEventQuery};

use crate::{app_state::AppState, assets::{FontAssets, ImageAssets}};


#[derive(Component)]
enum NumPlayersButton {
    One,
    Two
}

pub struct MainMenuScreen;

impl Plugin for MainMenuScreen {
    fn name(&self) -> &str {
        "MainMenuScreen"
    }

    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultNavigationPlugins)
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu).with_system(on_enter)
            )
            .add_system(nav_focus_change.after(NavRequestSystem))
            .add_system(nav_button_events.after(NavRequestSystem))
            .add_system(print_nav_events.after(NavRequestSystem));
    }
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        println!("{:?}", event);
    }
}

fn nav_focus_change(mut interaction_query: Query<(&Focusable, &mut Children, &Button), (Changed<Focusable>, With<Text>)>, mut text_query: Query<&mut Text>) {
    // for (focus, mut children) in interaction_query.iter_mut() {
    //     let new_label = match focus.state() {
    //         FocusState::Focused => String::from("->"),
    //         _ => String::from("  ")
    //     };

    //     button.
    //     // text.sections[0].value = new_label;
    // }

    for (focus, mut children, button) in interaction_query.iter_mut() {
        let new_label = match focus.state() {
            FocusState::Focused => String::from("->"),
            _ => String::from("  ")
        };

        let mut text = text_query.get_mut(children[0]).unwrap();

        println!("text: {:?}", text);

        text.sections[0].value = new_label;
    }

    // println!("{:?}", interaction_query.iter().collect::<Vec<_>>());
    // println!("text: {:?}", text_query.iter().collect::<Vec<_>>());
}

fn nav_button_events(mut buttons: NavEventQuery<&mut NumPlayersButton>) {
    match buttons.single_activated_mut().deref_mut().ignore_remaining() {
        Some(NumPlayersButton::One) => { info!("button click") },
        Some(NumPlayersButton::Two) => { unimplemented!() },
        None => {}
    };
}

fn on_enter(mut commands: Commands, font_assets: Res<FontAssets>, image_assets: Res<ImageAssets>) {
    info!("on_enter main menu");

    // Main column
    commands.spawn_bundle(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::FlexEnd,
            padding: Rect {top: Val::Px(30.0), left: Val::Px(30.0), ..Default::default()},
            min_size: Size { width: Val::Percent(100.), height: Val::Percent(100.) },
            ..Default::default()
        },
        color: UiColor::from(Color::NONE),
        ..Default::default()
    })
        // Num players selection
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                color: UiColor::from(Color::NONE),
                ..Default::default()
            })
                .with_children(|parent| {
                    // Button 1
                    parent.spawn_bundle(ButtonBundle {
                        color: UiColor::from(Color::NONE),
                        ..Default::default()
                    })
                    .with_children(|button| {
                        button.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: String::from("  "), style: TextStyle {
                                        font: font_assets.emulogic.clone(),
                                        font_size: 30.,
                                        color: Color::BLACK
                                    }
                                    },
                                    TextSection {
                                        value: String::from("2 Player Game"), style: TextStyle {
                                            font: font_assets.emulogic.clone(),
                                            font_size: 30.,
                                            color: Color::WHITE
                                    }
                                    }
                                ],
                                ..Default::default()
                            },
                            style: Style {
                                margin: Rect {top : Val::Px(20.), ..Default::default()},
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }).insert(NumPlayersButton::Two).insert(Focusable::default());
                    // Button 2
                    parent.spawn_bundle(ButtonBundle {
                        color: UiColor::from(Color::NONE),
                        ..Default::default()
                    })
                    .with_children(|button| {
                        button.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: String::from("  "), style: TextStyle {
                                        font: font_assets.emulogic.clone(),
                                        font_size: 30.,
                                        color: Color::BLACK
                                    }
                                    },
                                    TextSection {
                                        value: String::from("1 Player Game"), style: TextStyle {
                                            font: font_assets.emulogic.clone(),
                                            font_size: 30.,
                                            color: Color::WHITE
                                    }
                                    }
                                ],
                                ..Default::default()
                            },
                            style: Style {
                                margin: Rect {top : Val::Px(20.), ..Default::default()},
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }).insert(NumPlayersButton::One).insert(Focusable::default().dormant());
                });
        })
        // Nintendo copyright
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "©️1985 Nintendo",
                    TextStyle {
                        font: font_assets.emulogic.clone(),
                        font_size: 20.,
                        color: Color::BISQUE
                    },
                    TextAlignment::default()
                ),
                style: Style {
                    position: Rect { left: Val::Px(240.), ..Default::default() },
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        // Title board
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage::from(image_assets.title_board.clone()),
                style: Style {
                    // aspect_ratio: Some(16.0 / 9.0),
                    max_size: Size { width: Val::Percent(100.), height: Val::Percent(50.) },
                    margin: Rect { top: Val::Px(10.), ..Default::default()},
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        // Top row
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    min_size: Size { width: Val::Percent(100.), ..Default::default() },
                    ..Default::default()
                },
                color: UiColor::from(Color::NONE),
                ..Default::default()
            })
                // Mario + score
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection { value: String::from("Mario"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                },
                                TextSection { value: String::from("\r\n00000"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                }
                            ],
                            ..Default::default()
                        },
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                // Coins
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection { value: String::from("Coins"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                   }
                                }
                            ],
                            ..Default::default()
                        },
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                // World
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection { value: String::from("World"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                },
                                TextSection { value: String::from("\r\n1-1"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                }
                            ],
                            ..Default::default()
                        },
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                // Time
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection { value: String::from("Time"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                },
                                TextSection { value: String::from("\r\n"), style: TextStyle {
                                    font: font_assets.emulogic.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE
                                   }
                                }
                            ],
                            ..Default::default()
                        },
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}