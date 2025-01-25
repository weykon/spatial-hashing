use boid::Boid;
use config::{BoidConfig, Config};
use entity::{share::Share, Entity};
use paint::Paint;
use ready_paint::scene::Queue;
use space::{draw::SpaceDraw, Space};
use uniforms::Uniforms;

pub struct BoidScene;

impl Queue for BoidScene {
    fn introduce(scene: &mut ready_paint::scene::Scene) {
        scene
            .add_ready(Config::default())
            .add_ready(BoidConfig::default())
            .add_ready(Entity::default())
            .add_ready(Uniforms::default())
            .add_ready(Share::default())
            .add_ready(Space::default())
            .add_ready(SpaceDraw::default())
            .add_ready::<Boid>(Boid::default());
        scene.add_paint::<Paint>();
    }
}
mod boid;
mod config;
mod entity;
mod paint;
mod space;
mod uniforms;
