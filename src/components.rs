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
