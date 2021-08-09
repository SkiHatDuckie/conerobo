use bevy::{
    prelude::*, 
};

// Components for the game
pub struct Paddle {
    pub speed: f32,
}

pub struct Ball {
    pub velocity: Vec3,
}

pub struct Scoreboard {
    pub score: isize,
}

pub enum Collider {
    Solid,
    Bottom,
    Paddle,
}