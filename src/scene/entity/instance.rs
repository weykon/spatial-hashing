use super::Entity;
use ready_paint::scene::{get_res_mut, Update};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
pub struct _CircleInstance {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Update for _CircleInstance {
    fn update(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        let entity = get_res_mut::<Entity>(data);
        let instance_buffer = entity.instance_buffer.as_ref().unwrap();
        let dt = gfx.delta_time;
        if let Some(instances) = &mut entity.instance_collect {
            for instance in instances.iter_mut() {
                for pos in instance.position.iter_mut() {
                    *pos += 10.0 * dt;
                }
            }
            let data_bytes = bytemuck::cast_slice(instances.as_slice());
            gfx.queue.write_buffer(instance_buffer, 0, data_bytes);
        }
    }
}
