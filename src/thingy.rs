use crate::components::Thingy;
use crate::{CollisionEvent, Score, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub struct ThingyPlugin;

impl Plugin for ThingyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_thingy)
            .add_system(collision_spawn);
    }
}

pub fn spawn_thingy(mut commands: Commands) {
    let y_upper = (WINDOW_HEIGHT / 2.) - 110.;
    let y_lower = -(WINDOW_HEIGHT / 2.) + 60.;

    let x_upper = -(WINDOW_WIDTH / 2.0) + 80.;
    let x_lower = (WINDOW_WIDTH / 2.0) - 80.;
    let t = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2 { x: 30.0, y: 30.0 },
    };
    let mut rng = rand::thread_rng();
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &t,
            DrawMode::Fill(FillMode::color(Color::BLACK)),
            Transform {
                translation: Vec3 {
                    x: rng.gen_range(x_upper..x_lower) as f32,
                    y: rng.gen_range(y_lower..y_upper) as f32,
                    z: 10.,
                },
                ..default()
            },
        ))
        .insert(Thingy);
}

fn collision_spawn(
    commands: Commands,
    mut ev: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    if ev.iter().next().is_some() {
        spawn_thingy(commands);
        score.0 += 1;
        // println!("score: {:?}", score);
    }
}
