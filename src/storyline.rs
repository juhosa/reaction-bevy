use crate::{
    components::{StoryLines, TextLine, Thingy},
    Score,
};
use bevy::prelude::*;

pub struct StoryLinePlugin;

impl Plugin for StoryLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_storylines)
            .add_system(draw_storyline);
    }
}

fn setup_storylines(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load storylines
    let handle: Handle<StoryLines> = asset_server.load("storylines.json");
    commands.insert_resource(handle);
    let font = asset_server.load("ProggyClean.ttf");
    let text_style = TextStyle {
        font,
        font_size: 15.,
        color: Color::GRAY,
    };
    let position = Vec2::new(0., 0.);
    let size = Vec2::new(150., 200.);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("", text_style).with_alignment(TextAlignment::BOTTOM_CENTER),
            transform: Transform {
                translation: Vec3 {
                    x: position.x,
                    y: position.y,
                    z: 1.,
                },
                ..default()
            },

            text_2d_bounds: bevy::text::Text2dBounds { size },
            ..default()
        })
        .insert(TextLine);
}

fn draw_storyline(
    levels: Res<Assets<StoryLines>>,
    handles: Res<Handle<StoryLines>>,
    score: Res<Score>,
    thingy: Query<&Transform, (With<Thingy>, Without<TextLine>)>,
    mut textline: Query<(&mut Text, &mut Transform), With<TextLine>>,
) {
    if let Some(stlines) = levels.get(&handles) {
        if let Some(line) = stlines.storylines.iter().find(|&l| l.appears_at == score.0) {
            // found a storyline for the current score

            for position in thingy.iter() {
                for (mut text, mut tr) in &mut textline {
                    tr.translation.x = position.translation.x;
                    tr.translation.y = position.translation.y + 15.;
                    text.sections[0].value = line.line.to_string();
                }
            }
        } else {
            for (mut text, _tr) in &mut textline {
                text.sections[0].value = "".to_string();
            }
        }
    }
}
