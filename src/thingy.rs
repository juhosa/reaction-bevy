use crate::components::Thingy;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub struct ThingyPlugin;

impl Plugin for ThingyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_thingy);
    }
}

pub fn spawn_thingy(mut commands: Commands) {
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
                    x: rng.gen_range(-(WINDOW_WIDTH / 2.0)..(WINDOW_WIDTH / 2.0)) as f32,
                    y: rng.gen_range(-(WINDOW_HEIGHT / 2.0)..(WINDOW_HEIGHT / 2.0)) as f32,
                    z: 10.,
                },
                ..default()
            },
        ))
        .insert(Thingy);
}
