use glam::Vec2;
use ready_paint::scene::{get_res, get_res_mut, return_res, Pass, Ready, Update};

#[derive(Default)]
pub struct SpaceDraw {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: u32,
    pub pipeline: Option<wgpu::RenderPipeline>,
    pub vertices: Vec<_Vertex>,
    accumulated_time: f32,
}

impl Ready for SpaceDraw {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
        let (width, height) = (
            gfx.surface_config.as_ref().unwrap().width,
            gfx.surface_config.as_ref().unwrap().height,
        );

        let spaces = get_res::<super::Space>(data);

        let collision_cell = spaces.maps.0.cell_size;
        let clustering_cell = spaces.maps.1.cell_size;

        let mut vertices = Vec::new();

        let add_grid_lines = |vertices: &mut Vec<_Vertex>, cell_size: Vec2, color: [f32; 4]| {
            // 垂直线
            let num_vertical = (width as f32 / cell_size.x).ceil() as i32;
            for i in 0..=num_vertical {
                let x = i as f32 * cell_size.x;
                vertices.push(_Vertex {
                    position: [x, 0.0],
                    color,
                });
                vertices.push(_Vertex {
                    position: [x, height as f32],
                    color,
                });
            }

            // 水平线
            let num_horizontal = (height as f32 / cell_size.y.ceil()) as i32;
            for i in 0..=num_horizontal {
                let y = i as f32 * cell_size.y;
                vertices.push(_Vertex {
                    position: [0.0, y],
                    color,
                });
                vertices.push(_Vertex {
                    position: [width as f32, y],
                    color,
                });
            }
        };

        // 添加碰撞网格 (使用红色，较低透明度)
        add_grid_lines(&mut vertices, collision_cell, [1.0, 0.0, 0.0, 0.3]);

        // 添加聚类网格 (使用蓝色，较低透明度)
        add_grid_lines(&mut vertices, clustering_cell, [0.0, 0.0, 1.0, 0.3]);
        // println!("vertices: {:?}", vertices);
        // 创建索引（每两个顶点形成一条线）
        let indices: Vec<u16> = (0..vertices.len() as u16).collect();

        let vertex_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let index_buffer = gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let num_indices = indices.len() as u32;
        let shader = gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("space line shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "space_lines.wgsl"
                ))),
            });

        let config = gfx.surface_config.as_ref().unwrap();
        let share_layout = get_res::<Share>(data);
        let share_layout = share_layout.pipeline_layout.as_ref().unwrap();
        let pipeline = gfx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&share_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<_Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.view_formats[0],
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        return_res(
            data,
            SpaceDraw {
                vertex_buffer: Some(vertex_buffer),
                index_buffer: Some(index_buffer),
                num_indices,
                pipeline: Some(pipeline),
                vertices,
                accumulated_time: 0.0,
            },
        );
    }
}

impl Update for SpaceDraw {
    // 这个原本为了刷新颜色看渲染速度是否正常
    fn update(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        // let draw = get_res_mut::<SpaceDraw>(data);
        // let dt = gfx.delta_time;
        // let total = draw.vertices.len();
        // // 更新累积时间
        // draw.accumulated_time = (draw.accumulated_time + gfx.delta_time) % 1.0;

        // for (i, v) in draw.vertices.iter_mut().enumerate() {
        //     let i = i as f32;
        //     let color_value = (draw.accumulated_time + (i / total as f32)) % 1.0;
        //     v.color[0] = 0.3 + 0.7 * color_value;
        //     v.color[1] = 0.3 + 0.7 * color_value;
        //     v.color[3] = 0.3 + 0.7 * color_value;
        // }

        // // 更新顶点缓冲区
        // if let Some(vertex_buffer) = &draw.vertex_buffer {
        //     gfx.queue
        //         .write_buffer(vertex_buffer, 0, bytemuck::cast_slice(&draw.vertices));
        // }
    }
}
impl<'a> Pass<'a> for SpaceDraw {
    fn pass(
        data: &mut ready_paint::scene::HashTypeId2Data,
        render_pass: &'a mut wgpu::RenderPass<'a>,
    ) -> &'a mut wgpu::RenderPass<'a> {
        let draw = get_res::<SpaceDraw>(data);
        render_pass.set_pipeline(draw.pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, draw.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            draw.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..draw.num_indices, 0, 0..1);
        render_pass
    }
}

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::scene::entity::share::Share;
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone, Debug)]
struct _Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
