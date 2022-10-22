use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::thingy::spawn_thingy;
use crate::{CollisionEvent, Score};

use crate::components::{Ball, Thingy};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball)
            .add_system(ball_movement)
            .add_system(collision_spawn)
            .add_system(ball_collide);
    }
}

fn spawn_ball(mut commands: Commands) {
    let ball = shapes::Circle {
        radius: 15.0,
        center: Vec2 { x: 0.0, y: 0.0 },
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &ball,
            DrawMode::Fill(FillMode::color(Color::YELLOW)),
            Transform::default(),
        ))
        .insert(Ball);
}

fn ball_collide(
    mut commands: Commands,
    ball_positions: Query<&Transform, With<Ball>>,
    thingy_positions: Query<(Entity, &Transform), With<Thingy>>,
    mut ev_collision: EventWriter<CollisionEvent>,
) {
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
    mut score: ResMut<Score>,
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
    if a.x - radius < (b.x + thingy_side)
        && a.x + radius > b.x
        && a.y - radius < (b.y + thingy_side)
        && a.y + radius > b.y
    {
        return true;
    }
    false
}

fn ball_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ball_positions: Query<&mut Transform, With<Ball>>,
) {
    for mut ball in ball_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            ball.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            ball.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            ball.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            ball.translation.y += 2.;
        }
    }
}
