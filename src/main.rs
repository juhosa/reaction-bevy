use bevy::{prelude::*, app::AppExit};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

const DARK_GRAY: Color = Color::rgb(0.31, 0.31, 0.31);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    let window =  WindowDescriptor {
        title: "re-action bevy".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
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
        .add_startup_system(spawn_thingy)

        // systems (these run on every frame)
        .add_system(ball_movement)
        .add_system(exit_system)

        // run 
        .run();
}

#[derive(Component)]
struct Thingy;

fn spawn_thingy(mut commands: Commands) {
    let t = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2 { x: 30.0, y: 30.0 }
    };
    let mut rng = rand::thread_rng();
    commands.spawn_bundle(GeometryBuilder::build_as(
            &t,
            DrawMode::Fill(FillMode::color(Color::BLACK)),
            // Transform::default()
            Transform {
                translation: Vec3 { 
                    x: rng.gen_range(-(WINDOW_WIDTH/2.0)..(WINDOW_WIDTH / 2.0)) as f32, 
                    y: rng.gen_range(-(WINDOW_HEIGHT/2.0)..(WINDOW_HEIGHT/ 2.0)) as f32, 
                    z: 10. },
                ..default()
            } 
        )
    )
    .insert(Thingy);
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
