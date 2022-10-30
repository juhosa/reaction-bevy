use crate::components::Thingy;
use crate::{CollisionEvent, Score, ThingyAlpha, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
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
    let t = create_thingy(1.);
    commands.spawn_bundle(t).insert(Thingy);
}

fn create_thingy(alpha: f32) -> ShapeBundle {
    let y_upper = (WINDOW_HEIGHT / 2.) - 110.;
    let y_lower = -(WINDOW_HEIGHT / 2.) + 60.;

    let x_upper = -(WINDOW_WIDTH / 2.0) + 80.;
    let x_lower = (WINDOW_WIDTH / 2.0) - 80.;
    let t = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2 { x: 30.0, y: 30.0 },
    };

    let mut rng = rand::thread_rng();
    let color: Color = Color::rgba(0.0, 0.0, 0.0, alpha);

    GeometryBuilder::build_as(
        &t,
        DrawMode::Fill(FillMode::color(color)),
        Transform {
            translation: Vec3 {
                x: rng.gen_range(x_upper..x_lower) as f32,
                y: rng.gen_range(y_lower..y_upper) as f32,
                z: 10.,
            },
            ..default()
        },
    )
}

fn collision_spawn(
    mut commands: Commands,
    mut ev: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
    mut alpha: ResMut<ThingyAlpha>,
) {
    if ev.iter().next().is_some() {
        if score.0 > 100 {
            alpha.0 -= 0.01;
        }
        let t = create_thingy(alpha.0);
        commands.spawn_bundle(t).insert(Thingy);
        score.0 += 1;
    }
}
