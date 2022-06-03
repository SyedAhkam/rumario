use bevy::{prelude::*, asset};
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct AudioAssets {
    // #[asset(path = "audio/background.ogg")]
    // background: Handle<AudioSource>,
    // #[asset(path = "audio/plop.ogg")]
    // plop: Handle<AudioSource>
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    // #[asset(path = "images/player.png")]
    // pub player: Handle<Image>,
    // #[asset(path = "images/tree.png")]
    // pub tree: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/emulogic/emulogic.ttf")]
    pub emulogic: Handle<Font>
}