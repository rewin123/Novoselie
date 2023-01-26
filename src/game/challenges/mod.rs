use bevy::prelude::*;

pub mod kitchen_table;
pub mod numpad;
pub mod toilet_challenge;
pub mod constants;
pub mod labirint;
pub mod book_shelf;
pub mod music;
pub mod enter_room;
pub mod flappy_bird;

#[derive(Resource, Default)]
pub struct ChallengeStates {
    pub finished_chellenges : i32
}

pub struct Challenges {

}

impl Plugin for Challenges {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChallengeStates::default());

        app.add_plugin(kitchen_table::KitchenTableChallenge{});
        app.add_plugin(toilet_challenge::ToiletChallenge{});
        app.add_plugin(labirint::LabirintChallenge{});
        app.add_plugin(book_shelf::BookShelfChallenge{});
        app.add_plugin(music::MusicChallenge{});
        app.add_plugin(enter_room::EnterRoomChallenge{});
    }
}