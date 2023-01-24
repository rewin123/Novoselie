use bevy::prelude::*;
use bevy_egui::*;

const INTRODUCTION : &'static str = "Вы только что приехали в замок, в который вас пригласил местный владелец. Исследуя замок, вы наткнулись на старый дневник, принадлежавший прапрадедушке владельца замка. В журнале рассказывается история о сокровище, которое спрятал в стенах замка предок, мудрый и могущественный волшебник, который вынужден был в спешке покинуть замок, оставив после себя клад драгоценных камней, золота и артефактов.";

fn introduction_on_update(
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<super::AppState>>
) {
    egui::CentralPanel::default()
        .show(ctx.ctx_mut(), |ui| {
            ui.label(INTRODUCTION);

            if ui.button("Далее").clicked() {
                app_state.set(super::AppState::Chellenge_1).unwrap();
            }
        });
}

pub struct Introduction {}

impl Plugin for Introduction {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(super::AppState::Introduction)
            .with_system(introduction_on_update));
    }
}