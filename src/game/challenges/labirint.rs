
const introduction : &'static str = "За троном ты находишь скрытый проход. 
Ты берешь в руки факел и начинаешь исследовать неизвестные тебе катакомбы.
Нужно найти другой выход из них.";


const congrats : &'static str = "В конце лабиринта ты нашел новую записку!
В записке указано, что ключ находится за древним музыкуальным инструментом, что дорог хозяйке замка.";

const UP : &'static str = "UP";
const DOWN : &'static str = "DOWN";
const LEFT : &'static str = "LEFT";
const RIGHT : &'static str = "RIGHT";

use bevy::prelude::*;
use bevy_egui::*;

use crate::game::{AppState, FONT_PATH, GameStyle};


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
    asset_server : Res<AssetServer>,
    game_style : Res<GameStyle>
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

    cmds.spawn(ButtonBundle {
        style: Style { 
            size : Size::new(Val::Px(50.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content : JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            position: UiRect {
                bottom: Val::Px(-300.0),
                right: Val::Px(-200.0),
                ..default()
            },
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        transform : Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            UP,
            game_style.text.clone(),
        ));
    });

    cmds.spawn(ButtonBundle {
        style: Style { 
            size : Size::new(Val::Px(50.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content : JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            position: UiRect {
                bottom: Val::Percent(-35.0),
                right: Val::Percent(-35.0),
                ..default()
            },
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        transform : Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            LEFT,
            game_style.text.clone(),
        ));
    });

    cmds.spawn(ButtonBundle {
        style: Style { 
            size : Size::new(Val::Px(50.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content : JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            position: UiRect {
                bottom: Val::Percent(-35.0),
                right: Val::Percent(35.0),
                ..default()
            },
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        transform : Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            RIGHT,
            game_style.text.clone(),
        ));
    });

    cmds.spawn(ButtonBundle {
        style: Style { 
            size : Size::new(Val::Px(50.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content : JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            position: UiRect {
                bottom: Val::Px(-350.0),
                right: Val::Px(-200.0),
                ..default()
            },
            ..default()
        },
        background_color: NORMAL_BUTTON.into(),
        transform : Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            DOWN,
            game_style.text.clone(),
        ));
    });

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
