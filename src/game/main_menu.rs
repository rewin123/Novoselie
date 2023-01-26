use bevy::prelude::*;
use bevy_egui::*;

fn startup_menu(
    mut set : ResMut<EguiSettings>,
    mut windows : ResMut<Windows>) {
    set.scale_factor = 2000.0 / windows.get_primary().unwrap().width() as f64;
}

fn main_menu_on_update(mut ctx: ResMut<EguiContext>) {
    
    bevy_egui::egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.button("Испытание 1");
            ui.button("Испытание 2");
            ui.button("Испытание 3");
            ui.button("Испытание 4");
            ui.button("Испытание 5");
            ui.button("Испытание 6");
            ui.button("Испытание 7");
        });
    });
}

pub struct MainMenu {}

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {

        app
            .add_startup_system(startup_menu);

        app.add_system_set(SystemSet::on_update(super::AppState::Menu)
            .with_system(main_menu_on_update));
    }
}