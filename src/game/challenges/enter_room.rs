const question : &'static str = "Я стена между светом и тенью,
За мной заходя, защиту и безопасность находишь.
Кто я такой? Загадку разгадай.

Найди номер рядом с ними";

const answer : &'static str = "6565";

use bevy::prelude::*;
use bevy_egui::*;
use super::super::AppState;


fn enter_room_on_update(
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<AppState>>,
    mut state : ResMut<EnterRoomState>,
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
                    app_state.set(AppState::Chellenge_7);
                }
            }
        });
}

#[derive(Resource, Default)]
struct EnterRoomState {
    pub answer : String,
    pub show_err : bool
}

pub struct EnterRoomChallenge {}

impl Plugin for EnterRoomChallenge {
    fn build(&self, app: &mut App) {

        app.insert_resource(EnterRoomState::default());

        app.add_system_set(SystemSet::on_update(AppState::Chellenge_6)
            .with_system(enter_room_on_update));
    }
}