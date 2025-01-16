use config::Config;
use entity::{share::Share, Entity};
use paint::Paint;
use ready_paint::scene::Queue;
use uniforms::Uniforms;

pub struct BoidScene;

impl Queue for BoidScene {
    fn introduce(scene: &mut ready_paint::scene::Scene) {
        scene
            .add_ready(Config::default())
            .add_ready(Entity::default())
            .add_ready(Uniforms::default())
            .add_ready(Share::default());
        scene.add_paint::<Paint>();
    }
}
mod config;
mod entity;
mod paint;
mod uniforms;
