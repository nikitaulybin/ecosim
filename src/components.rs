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

#[derive(Component)]
pub struct Pos(pub Vec2);
#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct RelativeTextureIndex(pub usize);

#[derive(Component)]
pub struct Animal{ 
    pub frame_index: usize,
}
#[derive(Component)]
pub enum AnimalType {
    Bunny,
}
