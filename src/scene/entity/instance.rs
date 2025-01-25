use super::Entity;
use ready_paint::scene::{get_res_mut, Update};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct _CircleInstance {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
}
