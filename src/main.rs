use bevy::{app::AppExit, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_prototype_lyon::prelude::*;

mod ball;
mod components;
mod storyline;
mod thingy;
mod trophy;

use ball::BallPlugin;
use components::{ScoreText, StoryLines, UIElement};
use storyline::StoryLinePlugin;
use thingy::ThingyPlugin;
use trophy::TrophyPlugin;

const DARK_GRAY: Color = Color::rgb(0.31, 0.31, 0.31);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

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
        // start up systems (run only once)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui_texts)
        .add_startup_system(draw_static_ui)
        // systems (these run on every frame)
        .add_system(exit_system)
        .add_system(scoretext_update_system)
        // run
        .run();
}

#[derive(Debug)]
struct Score(i32);

struct CollisionEvent(Entity);

fn scoretext_update_system(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in &mut query {
        text.sections[1].value = score.0.to_string();
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
