
const introduction : &'static str = "За троном ты находишь скрытый проход. 
Ты берешь в руки факел и начинаешь исследовать неизвестные тебе катакомбы.
Нужно найти другой выход из них.";


const congrats : &'static str = "В конце лабиринта ты нашел новую записку!
В записке указано, что ключ находится за древним музыкуальным инструментом, что дорог хозяйке замка.";

use bevy::prelude::*;
use bevy_egui::*;

use crate::game::AppState;

#[derive(Component, Default)]
struct LabirintPlayer {}

fn labirint_on_update(
    mut state : ResMut<LabitintState>,
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<AppState>>
) {
    match &state.stage {
        LasbirintStage::Introduction => {
            egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
                ui.label(introduction);
                if ui.button("Далее").clicked() {
                    state.stage = LasbirintStage::Game;
                }
            });
        },
        LasbirintStage::Game => {},
        LasbirintStage::Finish => {
            egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
                ui.label(congrats);
                if ui.button("Далее").clicked() {
                    app_state.set(AppState::Chellenge_4);
                }
            });
        },
    }
}

fn player_move(
    time : Res<Time>,
    mut queue : Query<(&mut LabirintPlayer, &mut Transform)>
) {
    for (mut player, mut transform) in &mut queue {
        transform.translation.y += time.delta_seconds() * 10.0;
    }
}

fn labirint_setup(
    mut cmds : Commands,
    asset_server : Res<AssetServer>
) {
    cmds.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        camera: Camera {priority: 1, ..default()},
        transform: Transform::from_xyz(0.0, 0.0, 100.0-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(LabirintPlayer::default());

    cmds.spawn(SpriteBundle {
        texture : asset_server.load("Environment/Anvil.png"),
        ..default()
    }).insert(LabirintPlayer::default());

    let scale = 100.0;
    for x in -100..100 {
        for y in -100..100 {
            cmds.spawn(SpriteBundle {
                transform: Transform::from_xyz(x as f32 * scale, y as f32 * scale, -1.0),
                texture: asset_server.load("Platformer/Ground_06.png"),
                ..default()
            });
        }
    }
}

enum LasbirintStage {
    Introduction,
    Game,
    Finish
}

impl Default for LasbirintStage {
    fn default() -> Self {
        LasbirintStage::Introduction
    }
}

#[derive(Default, Resource)]
struct LabitintState {
    pub stage : LasbirintStage
}

pub struct LabirintChallenge {}


impl Plugin for LabirintChallenge {
    fn build(&self, app: &mut App) {

        app.insert_resource(LabitintState::default());

        app.add_system_set(SystemSet::on_enter(AppState::Chellenge_3)
            .with_system(labirint_setup));

        app.add_system_set(SystemSet::on_update(AppState::Chellenge_3)
            .with_system(labirint_on_update)
            .with_system(player_move));
    }
}
