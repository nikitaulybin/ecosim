use std::collections::VecDeque;

use bevy::utils::HashMap;

use crate::prelude::*;

#[derive(Component, Hash, PartialEq, Eq)]
pub enum AnimalState {
    Idle,
    Moving,
    Eating,
}

#[derive(Component, Hash, PartialEq, Eq)]
pub enum AnimalDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Component, Clone)]
pub struct Pos(pub Vec2);
#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct RelativeTextureIndex(pub usize);

#[derive(Component)]
pub struct Animal;
#[derive(Component)]
pub enum AnimalType {
    Bunny,
}
#[derive(Component)]
pub struct Path(pub VecDeque<Vec2>);
