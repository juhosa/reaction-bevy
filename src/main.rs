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
        .insert_resource(Score(0))

        // events
        .add_event::<CollisionEvent>()

        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        // start up systems (run only once)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_ball)
        .add_startup_system(spawn_thingy)
        .add_startup_system(setup_ui_texts)

        // systems (these run on every frame)
        .add_system(ball_movement)
        .add_system(exit_system)
        .add_system(ball_collide)
        .add_system(collision_spawn)
        .add_system(scoretext_update_system)
        .add_system(draw_static_ui)

        // run 
        .run();
}

#[derive(Debug)]
struct Score(i32);

struct CollisionEvent(Entity);

#[derive(Component)]
struct Thingy;

#[derive(Component)]
struct ScoreText;

fn scoretext_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>
    ) {
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
                        font: asset_server.load("font.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("font.ttf"),
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
        .insert(ScoreText);
}

fn draw_static_ui(mut commands: Commands) {
    let line_width = 2.0;
    let upper_line = shapes::Line(
            Vec2::new(-(WINDOW_WIDTH / 2.0) + 20., (WINDOW_HEIGHT / 2.) - 40.), 
            Vec2::new((WINDOW_WIDTH / 2.0) - 20., (WINDOW_HEIGHT / 2.) - 40.), 
        );

    commands.spawn_bundle(GeometryBuilder::build_as(
            &upper_line,
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, line_width)),
            Transform::default()
            ));

    let lower_line = shapes::Line(
            Vec2::new(-(WINDOW_WIDTH / 2.0) + 20., -(WINDOW_HEIGHT / 2.0) + 40.), 
            Vec2::new((WINDOW_WIDTH / 2.0) - 20., -(WINDOW_HEIGHT / 2.) + 40.), 
        );

    commands.spawn_bundle(GeometryBuilder::build_as(
            &lower_line,
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, line_width)),
            Transform::default()
            ));

}

fn spawn_thingy(mut commands: Commands) {
    let t = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2 { x: 30.0, y: 30.0 }
    };
    let mut rng = rand::thread_rng();
    commands.spawn_bundle(GeometryBuilder::build_as(
            &t,
            DrawMode::Fill(FillMode::color(Color::BLACK)),
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

fn ball_collide(
    mut commands: Commands,
    ball_positions: Query<&Transform, With<Ball>>,
    thingy_positions: Query<(Entity, &Transform), With<Thingy>>,
    mut ev_collision: EventWriter<CollisionEvent>) 
{
    for ball in ball_positions.iter() {
        for (ent, t) in thingy_positions.iter() {
           if collision(ball.translation, t.translation) {
               commands.entity(ent).despawn();
               ev_collision.send(CollisionEvent(ent));
           }
        }
    }
}

fn collision_spawn(
    commands: Commands, 
    mut ev: EventReader<CollisionEvent>,
    mut score: ResMut<Score>
    ) {
    if ev.iter().next().is_some() {
        spawn_thingy(commands);
        score.0 += 1;
        // println!("score: {:?}", score);
    }
}

fn collision(a: Vec3, b: Vec3) -> bool {
    let radius = 15.0;
    let thingy_side = 30.0;
    if a.x - radius < (b.x + thingy_side) &&
        a.x + radius > b.x && 
        a.y - radius < (b.y + thingy_side) &&
        a.y + radius > b.y 
    {
        return true
    }
    false
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
