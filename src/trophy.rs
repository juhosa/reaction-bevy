use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct TrophyPlugin;

use crate::components::Trophy;
use crate::components::TrophyText;
use crate::{Score, WINDOW_HEIGHT};

impl Plugin for TrophyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_trophy)
            .add_system(update_scoretext);
    }
}

fn spawn_trophy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x = -100.;
    let y = WINDOW_HEIGHT / 2. - 20.;
    let stroke_width = 3.;

    let ball = shapes::Circle {
        radius: 15.0,
        center: Vec2 { x: 0.0, y: 0.0 },
    };

    let b = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &ball,
            DrawMode::Stroke(StrokeMode::new(Color::BLUE, stroke_width)),
            Transform {
                translation: Vec3 { x, y, z: 9. },
                ..default()
            },
        ))
        .insert(Trophy)
        .id();

    let square = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2 { x: 19.0, y: 19.0 },
    };

    let s = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &square,
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 10.,
                },
                rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.),
                ..default()
            },
        ))
        .insert(Trophy)
        .id();

    commands.entity(b).push_children(&[s]);

    let font = asset_server.load("font.ttf");

    let text_style = TextStyle {
        font,
        font_size: 20.,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::CENTER;
    let text = "".to_string();

    let t = commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(text, text_style.clone()).with_alignment(text_alignment),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 11.,
                },
                ..default()
            },
            ..default()
        })
        .insert(TrophyText { score: 10 })
        .id();

    commands.entity(b).push_children(&[t]);
}

fn update_scoretext(
    score: Res<Score>,
    mut query: Query<(&mut Text, &TrophyText), With<TrophyText>>,
) {
    for (mut stext, sc) in query.iter_mut() {
        let n = sc.score - score.0;
        stext.sections.first_mut().unwrap().value = n.to_string();
    }
}
