use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
//use bevy_hanabi::prelude::*;

pub mod constants;
pub mod utils;
pub mod game;

use bevy_web_fullscreen::FullViewportPlugin;
use constants::*;
use game::*;


struct ViewportPlugin {}

#[cfg(target_arch="wasm32")]
impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FullViewportPlugin);
    }
}
#[cfg(not(target_arch="wasm32"))]
impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {

    }
}

fn main() {
    App::new()
        .add_state(AppState::Chellenge_3)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.27)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "StarRust".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(game::Game{})
        .add_plugin(ViewportPlugin{})
        .run();
}
