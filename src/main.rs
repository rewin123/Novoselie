use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
//use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

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
        println!("Hello wasm");
    }
}
#[cfg(not(target_arch="wasm32"))]
impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {

    }
}

fn main() {
    App::new()
        .add_state(AppState::Chellenge_5)
        .insert_resource(ClearColor(Color::rgb(74.0 / 255.0, 52.0 / 255.0, 27.0 / 255.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "StarRust".to_string(),
                fit_canvas_to_parent : true,
                ..default()
            },
            ..default()
        })
        .set(ImagePlugin::default_nearest()))
        .add_plugin(EguiPlugin)
        .add_plugin(game::Game{})
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}
