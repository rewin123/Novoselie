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
        .add_plugin(game::main_menu::MainMenu{})
        .add_plugin(game::introduction::Introduction{})
        .add_plugin(game::challenges::Challenges{})
        .add_startup_system(setup_camera)
        .add_plugin(ViewportPlugin{})
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    /*commands.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        camera: Camera {priority: 1, ..default()},
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
    ); // FIXME: this does not render at all*/
    // Bevy 2d camera is at Z=999.9
    // commands
    //     .spawn(Camera3dBundle {
    //         camera_3d: Camera3d { ..default() },
    //         camera: Camera {
    //             priority: 0,
    //             ..default()
    //         },
    //         projection: Projection::Orthographic(OrthographicProjection {
    //             scale: 1.0,
    //             ..default()
    //         }),
    //         transform: Transform::from_xyz(0.0, 0.0, CAMERA_FAR - 0.1)
    //             .looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     });
}