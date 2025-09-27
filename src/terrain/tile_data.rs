use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum TileType {
    Wall,
    Ground,
}

pub struct TileData();

pub enum TerrainType {
    Stone,
}

pub struct TerrainData();
