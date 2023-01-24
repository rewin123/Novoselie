pub mod main_menu;
pub mod introduction;
pub mod challenges;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Introduction,
    Menu,
    Chellenge_1,
    Chellenge_2,
    Chellenge_3,
    Chellenge_4,
    Chellenge_5,
    Chellenge_6,
    Chellenge_7,
}

pub const FONT_PATH : &'static str = "fonts/diary/font.ttf";

#[derive(Resource, Default)]
pub struct GameStyle {
    pub text : TextStyle
}

fn startup_style(
    mut style : ResMut<GameStyle>,
    asset_server : Res<AssetServer>
) {
    style.text = TextStyle {
        font : asset_server.load(FONT_PATH),
        font_size : 40.0,
        color: Color::rgb(0.9,0.9,0.9)
    };
}

pub struct Game{}

impl Plugin for Game {
    fn build(&self, app: &mut App) {

        app.insert_resource(GameStyle::default());
        
        app.add_startup_system(startup_style);

        app
        .add_plugin(main_menu::MainMenu{})
        .add_plugin(introduction::Introduction{})
        .add_plugin(challenges::Challenges{});
    }
}