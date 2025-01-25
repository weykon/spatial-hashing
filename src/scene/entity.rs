use super::config::Config;
use glam::Vec2;
use instance::_CircleInstance;
use noise::NoiseFn;
use rand::Rng;
use ready_paint::scene::{get_res, return_res, Pass, Ready};
use single::_Entity;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Default)]
pub struct Entity<'a> {
    pub entity_poses: Option<Vec<Vec2>>,
    pub single_buffer: Option<wgpu::Buffer>,
    pub instance_buffer: Option<wgpu::Buffer>,
    vertex_layout: Option<wgpu::VertexBufferLayout<'a>>,
    instance_layout: Option<wgpu::VertexBufferLayout<'a>>,
    pub instance_collect: Option<Vec<_CircleInstance>>,
}

impl<'a> Ready for Entity<'a> {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
        let config = get_res::<Config>(data);
        let surface_config = gfx.surface_config.as_ref().unwrap();
        let window_size = [surface_config.width as f32, surface_config.height as f32];
        let mut entity_poses: Vec<Vec2> = Vec::new();
        let noise_xy = noise::Perlin::new(1);
        let mut rand = rand::thread_rng();
        // 添加缩放因子来调整噪声的"密度"
        let scale = 0.01; // 较小的值会产生更平滑的变化
                          // 调整阈值来控制生成概率（当前0.5是中位数）
        let threshold = 0.0; // 降低阈值会产生更多实体

        for i in 0..config.max_entities as i32 {
            let x = rand.gen_range(0.0..window_size[0] as f32);
            let y = rand.gen_range(0.0..window_size[1] as f32);
            // 使用缩放因子
            let noise = noise_xy.get([x as f64 * scale, y as f64 * scale]);
            if noise > threshold {
                entity_poses.push(Vec2::new(x, y));
            }
        }

        let single_buffer = gfx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("single_buffer"),
            contents: bytemuck::bytes_of(&[
                [-1.0f32, -1.0f32],
                [1.0f32, -1.0f32],
                [-1.0f32, 1.0f32],
                [1.0f32, 1.0f32],
                [-1.0f32, 1.0f32],
                [1.0f32, -1.0f32],
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let mut rng = rand::thread_rng();
        let max = config.entity_max_speed;
        let instance_collect = entity_poses
            .iter()
            .map(|e| _CircleInstance {
                position: e.to_array(),
                radius: 5.,
                velocity: [rng.gen_range(-max..max), rng.gen_range(-max..max)],
            })
            .collect::<Vec<_CircleInstance>>();

        let instance_buffer = gfx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("single_buffer"),
            contents: bytemuck::cast_slice(&instance_collect.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<_Entity>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        };
        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<_CircleInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 4 * 2,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: 4 * 4,
                    shader_location: 3,
                },
            ],
        };
        return_res(
            data,
            Entity {
                single_buffer: Some(single_buffer),
                instance_buffer: Some(instance_buffer),
                entity_poses: Some(entity_poses),
                vertex_layout: Some(vertex_layout),
                instance_layout: Some(instance_layout),
                instance_collect: Some(instance_collect),
            },
        );
    }
}
impl<'a> Pass<'a> for Entity<'a> {
    fn pass(
        data: &mut ready_paint::scene::HashTypeId2Data,
        render_pass: &'a mut wgpu::RenderPass<'a>,
    ) -> &'a mut wgpu::RenderPass<'a> {
        let entity = get_res::<Entity>(data);
        let instances = entity.instance_collect.as_ref().unwrap().len() as u32;
        render_pass.set_vertex_buffer(0, entity.single_buffer.as_ref().unwrap().slice(..));
        render_pass.set_vertex_buffer(1, entity.instance_buffer.as_ref().unwrap().slice(..));
        render_pass.draw(0..6, 0..instances);
        render_pass
    }
}
pub mod instance;
pub mod share;
pub mod single;
