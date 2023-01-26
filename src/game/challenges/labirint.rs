
const introduction : &'static str = "За троном ты находишь скрытый проход. 
Ты берешь в руки факел и начинаешь исследовать неизвестные тебе катакомбы.
Нужно найти другой выход из них.";


const congrats : &'static str = "В конце лабиринта ты нашел новую записку!
В записке указано, что ключ находится за древним музыкуальным инструментом, что дорог хозяйке замка.";

const UP : &'static str = "UP";
const DOWN : &'static str = "DOWN";
const LEFT : &'static str = "LEFT";
const RIGHT : &'static str = "RIGHT";

use std::hash::Hash;

use bevy::{prelude::*, utils::HashSet};
use bevy_egui::*;
use bevy_rapier2d::prelude::*;
use maze_generator::prelude::*;
use maze_generator::recursive_backtracking::RbGenerator;

use crate::game::{AppState, FONT_PATH, GameStyle};


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Default)]
struct LabirintPlayer {}

#[derive(Resource, Default)]
struct PlayerMoveInfo {
    pub dx : f32,
    pub dy : f32
}

fn player_buttons(
        mut state : ResMut<LabitintState>,
        mut ctx : ResMut<EguiContext>,
        mut settings : ResMut<EguiSettings>,
        mut move_info : ResMut<PlayerMoveInfo>,
        mut windows : ResMut<Windows>) {

    if state.stage != LabirintStage::Game {
        return;
    }

    let height = windows.get_primary().unwrap().height();
    let width = windows.get_primary().unwrap().width();

    let scale = settings.scale_factor as f32;

    let pos_x = width / 2.0 / scale;
    let pos_y = 3.0 * height / 4.0 / scale;

    move_info.dx = 0.0;
    move_info.dy = 0.0;

    egui::Window::new("Управление")
    .pivot(egui::Align2::CENTER_CENTER)
    .fixed_pos([pos_x, pos_y])
    .collapsible(false)
        .show(ctx.ctx_mut(), |ui| {
            egui::Grid::new("joystick grid").show(ui, |ui| {
                ui.label("");
                if ui.button("Up").hovered() {
                    move_info.dy += 1.0;
                }
                ui.label("");
                ui.end_row();

                if ui.button("LEFT").hovered() {
                    move_info.dx += -1.0;
                }
                ui.label("");
                if ui.button("RIGHT").hovered() {
                    move_info.dx += 1.0;
                }
                ui.end_row();

                ui.label("");
                if ui.button("DOWN").hovered() {
                    move_info.dy += -1.0;
                }
                ui.label("");
            });
        });
    
}

fn labirint_on_update(
    mut state : ResMut<LabitintState>,
    mut ctx : ResMut<EguiContext>,
    mut app_state : ResMut<State<AppState>>
) {
    match &state.stage {
        LabirintStage::Introduction => {
            egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
                ui.label(introduction);
                if ui.button("Далее").clicked() {
                    state.stage = LabirintStage::Game;
                }
            });
        },
        LabirintStage::Game => {
            
        },
        LabirintStage::Finish => {
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
    mut move_info : ResMut<PlayerMoveInfo>,
    mut queue : Query<(&mut KinematicCharacterController, &mut Transform), Without<Camera>>,
    mut cams : Query<(&Camera, &mut Transform)>
) {
    let speed = 1000.0;
    let mut pos = Vec3::default();
    for (mut controller, transform) in &mut queue {
        controller.translation = Some(Vec2::new(time.delta_seconds() * move_info.dx * speed, time.delta_seconds() * move_info.dy * speed));
        pos = transform.translation;
        break;
    }

    let dt = time.delta_seconds();
    let k = 0.3;
    for (cam, mut transform) in &mut cams {
        let cam_pos = transform.translation;
        let smooted_pos = cam_pos * (1.0 - k) + k * pos;
        transform.translation.x = smooted_pos.x;
        transform.translation.y = smooted_pos.y;
    }
}

#[derive(PartialEq)]
struct HashedVec {
    pub x : f32,
    pub y : f32
}

impl Hash for HashedVec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Eq for HashedVec {

}

fn collision_events(
    mut portals : Query<(&Transform), With<Portal>>,
    mut players : Query<(&Transform), With<KinematicCharacterController>>,
    mut state : ResMut<LabitintState>,
) {
    if !portals.is_empty() && !players.is_empty() {
        let portal = portals.iter().next().unwrap();
        let player = players.iter().next().unwrap();

        let dist = (portal.translation - player.translation).distance(Vec3::ZERO);

        if dist < 100.0 {
            state.stage = LabirintStage::Finish;
        }
    }
}

#[derive(Component)]
struct Portal {}

fn labirint_setup(
    mut cmds : Commands,
    asset_server : Res<AssetServer>,
    game_style : Res<GameStyle>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let camera = cmds.spawn(Camera2dBundle{
        projection: OrthographicProjection {
            scale: 0.6,
            ..default()
        },
        camera: Camera {priority: 1, ..default()},
        transform: Transform::from_xyz(0.0, 0.0, 100.0-0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(LabirintPlayer::default()).id();

    {
        let walk_handle = asset_server.load("Knight Pixel Art/Spritesheet/Hero-walk-Sheet.png");
        let texture_atlas = TextureAtlas::from_grid(
            walk_handle,
            Vec2::new(48.0, 24.0), 
            6, 
            1, 
            None, 
            None);
        let texture_atlas_hande = texture_atlases.add(texture_atlas);

        cmds.spawn(SpriteSheetBundle {
            texture_atlas : texture_atlas_hande,
            transform : Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        }).insert(LabirintPlayer::default())
        .insert(Collider::capsule_y(8.0, 4.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default());
    }

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

    //generate labirint
    let wall_tex = asset_server.load("cobblestone.png");

    let mut generator = RbGenerator::new(Some([42; 32]));
    let maze_size = 3;
    let maze = generator.generate(maze_size, maze_size).unwrap();
    
    let mut maze_pos_sets : HashSet<HashedVec> = HashSet::new();

    let mut maze_to_world = |x : i32, y : i32| {
        let world_x = x as f32 * 2.0;
        let world_y = -y as f32 * 2.0;

        (world_x, world_y)
    };

    let world_scale = 160.0;
    let mut spawn = |x : f32, y : f32| {
        if !maze_pos_sets.contains(&HashedVec {x : x, y : y}) {
            cmds.spawn(SpriteBundle {
                texture : wall_tex.clone(),
                transform : Transform::from_xyz(x * world_scale, y * world_scale, 0.5),
                ..default()
            })
            .insert(Collider::cuboid(80.0, 80.0))
            .insert(RigidBody::Fixed);
            maze_pos_sets.insert(HashedVec {x : x, y : y});
        }
    };

    for y in 0..maze_size {
        for x in 0..maze_size {
            let (world_x, world_y) = maze_to_world(x, y);

            let field = maze.get_field(&Coordinates::new(x, y)).unwrap();
            if !field.has_passage(&maze_generator::prelude::Direction::West) {
                spawn(world_x - 1.0, world_y);
                
            }

            if !field.has_passage(&maze_generator::prelude::Direction::East) {
                spawn(world_x + 1.0, world_y);
            }

            if !field.has_passage(&maze_generator::prelude::Direction::North ) {
                spawn(world_x, world_y + 1.0);
            }

            if !field.has_passage(&maze_generator::prelude::Direction::South) {
                spawn(world_x, world_y - 1.0);
            }
            spawn(world_x - 1.0, world_y - 1.0);
            spawn(world_x + 1.0, world_y - 1.0);
            spawn(world_x - 1.0, world_y + 1.0);
            spawn(world_x + 1.0, world_y + 1.0);
        }
        
        
    }

    println!("{:?}", maze);

    //spawn portal
    {
        let atlas = asset_server.load("labirint/portal.png");
        let texture_atlas = TextureAtlas::from_grid(
            atlas,
            Vec2::new(64.0, 64.0), 
            8, 
            3, 
            None, 
            None);
        let texture_atlas_hande = texture_atlases.add(texture_atlas);

        let (world_x, world_y) = maze_to_world(maze.goal.x, maze.goal.y);

        cmds.spawn(SpriteSheetBundle {
            texture_atlas : texture_atlas_hande,
            transform : Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(world_x * world_scale, world_y * world_scale, 1.0)),
            ..default()
        })
        .insert(Portal{});
    }

}

#[derive(PartialEq, Eq)]
enum LabirintStage {
    Introduction,
    Game,
    Finish
}

impl Default for LabirintStage {
    fn default() -> Self {
        LabirintStage::Introduction
    }
}

#[derive(Default, Resource)]
struct LabitintState {
    pub stage : LabirintStage
}

pub struct LabirintChallenge {}


impl Plugin for LabirintChallenge {
    fn build(&self, app: &mut App) {

        app.insert_resource(LabitintState::default());
        app.insert_resource(PlayerMoveInfo::default());

        app.add_system_set(SystemSet::on_enter(AppState::Chellenge_3)
            .with_system(labirint_setup));

        app.add_system_set(SystemSet::on_update(AppState::Chellenge_3)
            .with_system(labirint_on_update)
            .with_system(player_buttons)
            .with_system(player_move)
            .with_system(collision_events));
    }
}
