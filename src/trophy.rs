use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct TrophyPlugin;

use crate::components::Trophy;
use crate::components::TrophyText;
use crate::{Score, WINDOW_HEIGHT};

#[derive(Component, Debug)]
struct LocalTrophy {
    score: i32,
    square_color: Color,
    circle_color: Color,
    achieved: bool,
}

#[derive(Component, Debug)]
struct TrophySquare;

impl Plugin for TrophyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_trophy)
            .add_system(update_scoretext)
            .add_system(update_trophy_colors);
    }
}

fn spawn_trophy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let trophies: Vec<LocalTrophy> = vec![
        LocalTrophy {
            score: 10,
            square_color: Color::RED,
            circle_color: Color::BLUE,
            achieved: false,
        },
        LocalTrophy {
            score: 50,
            square_color: Color::YELLOW,
            circle_color: Color::PINK,
            achieved: false,
        },
        LocalTrophy {
            score: 110,
            square_color: Color::BLACK,
            circle_color: Color::BLACK,
            achieved: false,
        },
    ];

    let mut x = -100.;
    let y = WINDOW_HEIGHT / 2. - 20.;
    let stroke_width = 3.;
    let x_offset = 50.;

    for lt in trophies {
        let ball = shapes::Circle {
            radius: 15.0,
            center: Vec2 { x: 0.0, y: 0.0 },
        };

        let b = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball,
                DrawMode::Stroke(StrokeMode::new(Color::GRAY, stroke_width)),
                Transform {
                    translation: Vec3 { x, y, z: 9. },
                    ..default()
                },
            ))
            .insert(Trophy)
            .insert(LocalTrophy {
                score: lt.score,
                square_color: lt.square_color,
                circle_color: lt.circle_color,
                achieved: lt.achieved,
            })
            .id();

        let square = shapes::Rectangle {
            origin: RectangleOrigin::Center,
            extents: Vec2 { x: 19.0, y: 19.0 },
        };

        let s = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &square,
                DrawMode::Fill(FillMode::color(Color::GRAY)),
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
            .insert(TrophySquare)
            .id();

        commands.entity(b).push_children(&[s]);

        let font = asset_server.load("font.ttf");

        let text_style = TextStyle {
            font,
            font_size: 20.,
            color: Color::BLACK,
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
            .insert(TrophyText { score: lt.score })
            .id();

        commands.entity(b).push_children(&[t]);
        x += x_offset;
    }
}

fn update_scoretext(
    score: Res<Score>,
    mut query: Query<(&mut Text, &TrophyText), With<TrophyText>>,
) {
    for (mut stext, sc) in query.iter_mut() {
        let n = sc.score - score.0;
        if n <= 0 {
            stext.sections.first_mut().unwrap().value = "".to_string();
        } else {
            stext.sections.first_mut().unwrap().value = n.to_string();
        }
    }
}

fn update_trophy_colors(
    score: Res<Score>,
    mut trophies: Query<(&Trophy, &mut DrawMode, &mut LocalTrophy, &Children)>,
    mut q_child: Query<(&TrophySquare, &mut DrawMode), Without<Trophy>>,
) {
    for (_t, mut circle_draw, mut lt, children) in trophies.iter_mut() {
        if lt.achieved {
            continue;
        }
        if score.0 >= lt.score {
            lt.achieved = true;
            *circle_draw = DrawMode::Stroke(StrokeMode::new(lt.circle_color, 3.));
            for &child in children.iter() {
                match q_child.get_mut(child) {
                    Ok((_c, mut dm)) => {
                        *dm = DrawMode::Stroke(StrokeMode::new(lt.square_color, 3.));
                    }
                    Err(_) => continue,
                }
            }
        }
    }
}
