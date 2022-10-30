use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Thingy;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct UIElement;

#[derive(Component)]
pub struct Trophy;

#[derive(Component, Debug)]
pub struct TrophyText {
    pub score: i32,
}

#[derive(Debug, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b38022c46"]
pub struct StoryLines {
    pub storylines: Vec<StoryLine>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StoryLine {
    pub line: String,
    pub appears_at: i32,
}

#[derive(Component)]
pub struct TextLine;
