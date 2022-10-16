use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    let window =  WindowDescriptor {
        title: "re-action bevy".to_string(),
        width: 800.0,
        height: 600.0,
        ..default()
    };
    
    App::new()
        // resources
        .insert_resource(window)

        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        // start up systems
        .add_startup_system(setup_camera)

        // systems
        .add_system(spawn_ball)

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

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
