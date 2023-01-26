use bevy::prelude::*;
use bevy_egui::*;
use super::super::AppState;

const question : &'static str = "Вокруг древнего него собрались в доме друзья,
За ним блюда накрыты и кувшины наполнены.
Он стал объектом нашей почитания,
И когда мы вместе едим, к нему мы стремимся.
Вокруг него мы сидим, как вокруг костра,
И в его присутствии душа наша наполняется.
Но где же мы сидим, когда он перед нами?

Найди номер рядом с ним!
";


const answer : &'static str = "4567";

fn kitchen_table_on_update(
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<AppState>>,
    mut state : ResMut<KitchenTableState>,
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
                    app_state.set(AppState::Chellenge_2);
                }
            }
        });
}

#[derive(Resource, Default)]
struct KitchenTableState {
    pub answer : String,
    pub show_err : bool,
    pub texture : egui::TextureId
}

fn kitchen_setup_state(
    mut state : ResMut<KitchenTableState>,
    mut asset_server : Res<AssetServer>,
    mut ctx : ResMut<EguiContext>
) {
    state.texture = ctx.add_image(asset_server.load("chs/1/background.png"));
}

pub struct KitchenTableChallenge {}

impl Plugin for KitchenTableChallenge {
    fn build(&self, app: &mut App) {

        app.insert_resource(KitchenTableState::default());

        app.add_system_set(SystemSet::on_enter(AppState::Chellenge_1)
            .with_system(kitchen_setup_state));
        app.add_system_set(SystemSet::on_update(AppState::Chellenge_1)
            .with_system(kitchen_table_on_update));
    }
}