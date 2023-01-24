use bevy::prelude::*;

pub mod kitchen_table;
pub mod numpad;
pub mod toilet_challenge;
pub mod constants;
pub mod labirint;

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
    }
}