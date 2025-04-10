use std::any::Any;

use glam::Vec2;
use ready_paint::multi::{refs_muts, Mut, Ref};
use wgpu::{SurfaceConfiguration, TextureFormat};

use crate::scene::config::Config;

use super::{
    super::{config::BoidConfig, entity::Entity, space::Space},
    Boid,
};
