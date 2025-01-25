use ready_paint::scene::{get_res, Paint as PaintTrait, Pass, Update};

use super::{
    boid::Boid,
    entity::{instance::_CircleInstance, share::Share, Entity},
    space::draw::SpaceDraw,
    uniforms::Uniforms,
};

pub struct Paint;
impl PaintTrait for Paint {
    fn paint(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        let frame = gfx.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let _ = Uniforms::update(data, gfx);
        let _ = Boid::update(data, gfx);
        // let _ = SpaceDraw::update(data, gfx);
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let mut render_pass = Uniforms::pass(data, &mut render_pass);
            let mut render_pass = Share::pass(data, &mut render_pass);
            let mut render_pass = Entity::pass(data, &mut render_pass);
            let mut render_pass = SpaceDraw::pass(data, &mut render_pass);
        }
        gfx.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
