use bevy::{app::AppExit, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_prototype_lyon::prelude::*;

mod ball;
mod components;
mod storyline;
mod thingy;
mod trophy;

use ball::BallPlugin;
use components::{Ball, ScoreText, StoryLines, TextLine, Thingy, UIElement};
use storyline::StoryLinePlugin;
use thingy::ThingyPlugin;
use trophy::TrophyPlugin;

const DARK_GRAY: Color = Color::rgb(0.31, 0.31, 0.31);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    InGame,
    GameOver,
}

fn main() {
    let window = WindowDescriptor {
        title: "re-action bevy".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..default()
    };

    App::new()
        // resources
        .insert_resource(ClearColor(DARK_GRAY))
        .insert_resource(window)
        .insert_resource(Score(0))
        .insert_resource(GameOverScore(201))
        .insert_resource(ThingyAlpha(1.0))
        // events
        .add_event::<CollisionEvent>()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<StoryLines>::new(&["json"]))
        .add_plugin(ShapePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ThingyPlugin)
        .add_plugin(TrophyPlugin)
        .add_plugin(StoryLinePlugin)
        // state
        .add_state(AppState::InGame)
        // start up systems (run only once)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui_texts)
        .add_startup_system(draw_static_ui)
        // systems (these run on every frame)
        .add_system(exit_system)
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(scoretext_update_system))
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(gameover_system))
        // run
        .run();
}

#[derive(Debug)]
struct Score(i32);

#[derive(Debug)]
struct ThingyAlpha(f32);

#[derive(Debug)]
struct GameOverScore(i32);

struct CollisionEvent(Entity);

fn gameover_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ball: Query<Entity, With<Ball>>,
    thingy: Query<Entity, With<Thingy>>,
    textline: Query<Entity, With<TextLine>>,
    mut scoretext: Query<&mut Text, With<ScoreText>>,
) {
    println!("GAME OVER");

    let ball = ball.single();
    commands.entity(ball).despawn();

    let thingy = thingy.single();
    commands.entity(thingy).despawn();

    let textline = textline.single();
    commands.entity(textline).despawn();

    for mut text in &mut scoretext {
        text.sections[0].value = "Kills:".to_string();
    }

    commands.spawn_bundle(
        TextBundle::from_section(
            "EVEN THE SMALLEST action",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(100.0),
                left: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(
        TextBundle::from_section(
            "HAS A reaction.",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(150.0),
                left: Val::Px(270.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(
        TextBundle::from_section(
            "SOMETIMES IT'S GOOD TO JUST",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(250.0),
                left: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(
        TextBundle::from_section(
            "STOP AND THINK WHAT EFFECT",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(300.0),
                left: Val::Px(80.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(
        TextBundle::from_section(
            "YOUR CURRENT ACTIONS HAVE.",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(350.0),
                left: Val::Px(100.0),
                ..default()
            },
            ..default()
        }),
    );
    commands.spawn_bundle(
        TextBundle::from_section(
            "THANK YOU FOR PLAYING.",
            TextStyle {
                font: asset_server.load("ProggyClean.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(500.0),
                left: Val::Px(50.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn scoretext_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
    gameoverscore: Res<GameOverScore>,
    mut app_state: ResMut<State<AppState>>,
) {
    for mut text in &mut query {
        text.sections[1].value = score.0.to_string();
    }

    if score.0 == gameoverscore.0 {
        app_state.set(AppState::GameOver).unwrap();
    }
}

fn setup_ui_texts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("ProggyClean.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("ProggyClean.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText)
        .insert(UIElement);
}

fn draw_static_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // [q] quit
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "[q] Quit",
                TextStyle {
                    font: asset_server.load("ProggyClean.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(UIElement);

    // Lines
    let line_width = 2.0;
    let upper_line = shapes::Line(
        Vec2::new(-(WINDOW_WIDTH / 2.0) + 20., (WINDOW_HEIGHT / 2.) - 40.),
        Vec2::new((WINDOW_WIDTH / 2.0) - 20., (WINDOW_HEIGHT / 2.) - 40.),
    );

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &upper_line,
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, line_width)),
            Transform::default(),
        ))
        .insert(UIElement);

    let lower_line = shapes::Line(
        Vec2::new(-(WINDOW_WIDTH / 2.0) + 20., -(WINDOW_HEIGHT / 2.0) + 40.),
        Vec2::new((WINDOW_WIDTH / 2.0) - 20., -(WINDOW_HEIGHT / 2.) + 40.),
    );

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &lower_line,
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, line_width)),
            Transform::default(),
        ))
        .insert(UIElement);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn exit_system(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
