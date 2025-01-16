use glam::Vec2;
use ready_paint::scene::Ready;

pub struct Config {
   pub max_entities: u32,
   pub cell_size: Vec2,
   pub width: i32,
   pub height: i32,
   pub entity_max_speed: f32,
}

impl Ready for Config {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_entities: 100,
            cell_size: Vec2::new(10.0, 10.0),
            width: 600,
            height: 400,
            entity_max_speed: 10.
        }
    }
}
