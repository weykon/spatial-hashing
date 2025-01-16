use ready_paint::scene::{get_res, get_res_mut, return_res, Pass, Ready};

use crate::scene::uniforms::Uniforms;

use super::Entity;
#[derive(Default)]
pub struct Share {
    pipeline_layout: Option<wgpu::PipelineLayout>,
    pipeline: Option<wgpu::RenderPipeline>,
}

impl Ready for Share {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
        let config = gfx.surface_config.as_ref().unwrap();
        let uniforms = get_res::<Uniforms>(data);
        let pipeline_layout = gfx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("pipeline layout"),
                bind_group_layouts: &[
                    uniforms.static_uniforms_bind_group_layout.as_ref().unwrap(),
                    uniforms.dyn_uniforms_bind_group_layout.as_ref().unwrap(),
                ],
                push_constant_ranges: &[],
            });

        let shader = gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("instance_frag"),
                source: wgpu::ShaderSource::Wgsl(include_str!("instance_frag.wgsl").into()),
            });
        let entity = get_res_mut::<Entity>(data);
        let pipeline = gfx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        entity.vertex_layout.take().unwrap(),
                        entity.instance_layout.take().unwrap(),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(config.view_formats[0].into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });
        return_res(
            data,
            Share {
                pipeline_layout: Some(pipeline_layout),
                pipeline: Some(pipeline),
            },
        );
    }
}

impl<'a> Pass<'a> for Share {
    fn pass(
        data: &mut ready_paint::scene::HashTypeId2Data,
        render_pass: &'a mut wgpu::RenderPass<'a>,
    ) -> &'a mut wgpu::RenderPass<'a> {
        let share = get_res::<Share>(data);
        render_pass.set_pipeline(share.pipeline.as_ref().unwrap());

        render_pass
    }
}
