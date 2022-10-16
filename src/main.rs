use bevy::{prelude::*, app::AppExit};
use bevy_prototype_lyon::prelude::*;

const DARK_GRAY: Color = Color::rgb(0.31, 0.31, 0.31);

fn main() {
    let window =  WindowDescriptor {
        title: "re-action bevy".to_string(),
        width: 800.0,
        height: 600.0,
        ..default()
    };
    
    App::new()
        // resources
        .insert_resource(ClearColor(DARK_GRAY))
        .insert_resource(window)

        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        // start up systems (run only once)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_ball)

        // systems (these run on every frame)
        .add_system(ball_movement)
        .add_system(exit_system)

        // run 
        .run();
}

#[derive(Component)]
struct Ball;

fn spawn_ball(mut commands: Commands) {
    let ball = shapes::Circle {
        radius: 15.0,
        center: Vec2 { x: 0.0, y: 0.0 }
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
            &ball,
            DrawMode::Fill(FillMode::color(Color::YELLOW)),
            Transform::default()
    ))
    .insert(Ball);
}

fn ball_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ball_positions: Query<&mut Transform, With<Ball>>
    ) {
    
    for mut ball in ball_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            ball.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            ball.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            ball.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            ball.translation.y += 2.;
        }
    }

}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn exit_system(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}
