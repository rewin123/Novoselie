
use bevy::prelude::*;
use bevy_egui::*;

pub fn make_numpad(dst : &mut String, ui : &mut egui::Ui) {
    egui::Grid::new("Numpad")
        .show(ui, |ui| {

            if ui.button("1").clicked() {
                dst.push('1');
            }
            if ui.button("2").clicked() {
                dst.push('2');
            }
            if ui.button("3").clicked() {
                dst.push('3');
            }
            ui.end_row();

            if ui.button("4").clicked() {
                dst.push('4');
            }
            if ui.button("5").clicked() {
                dst.push('5');
            }
            if ui.button("6").clicked() {
                dst.push('6');
            }
            ui.end_row();
            if ui.button("7").clicked() {
                dst.push('7');
            }
            if ui.button("8").clicked() {
                dst.push('8');
            }
            if ui.button("9").clicked() {
                dst.push('9');
            }
            ui.end_row();
        });

        ui.horizontal(|ui| {
            if ui.button("0").clicked() {
                dst.push('0');
            }

            if ui.button("Del").clicked() {
                if dst.len() > 0 {
                    dst.remove(dst.len() - 1);
                }
            }
        });
}