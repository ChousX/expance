use bevy::prelude::*;

use crate::chunk::Chunk;

pub struct DomainPlugin;
impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Domain([DomainNode; Self::COUNT.width * Self::COUNT.height]);

impl Domain {
    pub const COUNT: DomainCount = DomainCount::new(10, 10);
    pub const SIZE: Vec2 = vec2(
        Chunk::SIZE.x / Self::COUNT.width as f32,
        Chunk::SIZE.y / Self::COUNT.height as f32,
    );
    pub const fn index(x: usize, y: usize) -> usize{
        y * Self::COUNT.width + x
    }
    
}

pub struct DomainCount {
    pub width: usize,
    pub height: usize,
}

impl DomainCount {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Component, Default)]
pub struct DomainNode {
    //PlayerId
    owner: Option<u8>,
}
