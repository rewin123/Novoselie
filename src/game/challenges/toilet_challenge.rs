use bevy::prelude::*;
use bevy_egui::*;
use super::super::AppState;

const question : &'static str = "Трон каждого уважающего себя милорда. 

Найди номер рядом с ним!
";


const answer : &'static str = "6565";

fn toilet_on_update(
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<AppState>>,
    mut state : ResMut<ToiletState>,
    mut challenge_state : ResMut<super::ChallengeStates>
) {
    egui::CentralPanel::default()
        .show(ctx.ctx_mut(), |ui| {
            ui.label(question);

            ui.add(egui::TextEdit::singleline(&mut state.answer));

            if state.show_err {
                ui.label(super::constants::wrong_answer);
            }

            super::numpad::make_numpad(&mut state.answer, ui);

            if ui.button("Попытка").clicked() {
                if state.answer != answer {
                    state.show_err = true;
                    state.answer = "".to_string();
                } else {
                    if challenge_state.finished_chellenges == 0 {
                        challenge_state.finished_chellenges = 1;
                    }
                    app_state.set(AppState::Chellenge_3);
                }
            }
        });
}

#[derive(Resource, Default)]
struct ToiletState {
    pub answer : String,
    pub show_err : bool
}

#[derive(Resource, Default)]
pub struct ToiletChallenge {}

impl Plugin for ToiletChallenge {
    fn build(&self, app: &mut App) {

        app.insert_resource(ToiletState::default());

        app.add_system_set(SystemSet::on_update(AppState::Chellenge_2)
            .with_system(toilet_on_update));
    }
}