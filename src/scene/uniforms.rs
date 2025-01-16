use bytemuck::{bytes_of, Pod, Zeroable};
use ready_paint::scene::{get_res_mut, return_res, Pass, Ready, Update};
use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct _StaticUniforms {
    window_size: [f32; 2], // 4*2 bytes
    _padding: [f32; 2],    // 4*2 bytes
}

#[repr(C)]
#[derive(Pod, Zeroable, Copy, Clone)]
pub struct _DynUniforms {
    delta_time: f32,
    _padding: [f32; 3],
}

#[derive(Default)]
pub struct Uniforms {
    static_uniforms: Option<_StaticUniforms>,
    dyn_uniform: Option<_DynUniforms>,
    static_uniforms_buffer: Option<wgpu::Buffer>,
    dyn_uniforms_buffer: Option<wgpu::Buffer>,
    pub static_uniforms_bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub dyn_uniforms_bind_group_layout: Option<wgpu::BindGroupLayout>,
    dyn_uniform_bind_group: Option<wgpu::BindGroup>,
    static_uniform_bind_group: Option<wgpu::BindGroup>,
    offset: u64,
    alignment: u32,
}
impl Ready for Uniforms {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
        let uniforms_limits = gfx.device.limits();
        let min_uniform_buffer_offset_alignment =
            uniforms_limits.min_uniform_buffer_offset_alignment;
        println!(
            "min_uniform_buffer_offset_alignment is {}",
            min_uniform_buffer_offset_alignment
        );
        // let aligned_size = wgpu::util::align_to(
        //     std::mem::size_of::<Uniforms>() as u32,
        //     limits.min_uniform_buffer_offset_alignment
        // ) as usize;  // 或者辅助函数获取
        let config = gfx.surface_config.as_ref().unwrap();
        let window_size = [config.width as f32, config.height as f32];
        println!("update window size from config, {:?}", window_size);
        let static_uniforms = _StaticUniforms {
            window_size,
            _padding: [0., 0.],
        };
        let dyn_uniform = _DynUniforms {
            delta_time: gfx.delta_time,
            _padding: [0., 0., 0.],
        };

        let static_uniforms_buffer =
            gfx.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("uniforms buffer"),
                    contents: bytemuck::cast_slice(&[static_uniforms]),
                    usage: wgpu::BufferUsages::UNIFORM, // 这里没用COPY_DST应该为resize的时候已经重新执行了ready。
                });
        let dyn_uniforms_buffer = //（环形缓冲区）
            gfx.device
                .create_buffer(
                    &wgpu::BufferDescriptor {
                        label: Some("dyn uniform buffer"),
                        size:  min_uniform_buffer_offset_alignment as u64 * 3  , //（环形缓冲区, 3帧，放三个）
                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation:false,
                    }
                );
        let static_uniforms_bind_group_layout =
            gfx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("static uniforms bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let static_uniform_bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("static uniform bind group"),
            layout: &static_uniforms_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &static_uniforms_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        let dyn_uniforms_bind_group_layout =
            gfx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("dyn uniforms bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let dyn_uniform_bind_group = gfx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("dyn uniform bind group"),
            layout: &dyn_uniforms_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &dyn_uniforms_buffer,
                    offset: 0,
                    size: Some(std::num::NonZeroU64::new(16).unwrap()),
                }),
            }],
        });
        return_res(
            data,
            Uniforms {
                static_uniforms: Some(static_uniforms),
                dyn_uniform: Some(dyn_uniform),
                static_uniforms_buffer: Some(static_uniforms_buffer),
                dyn_uniforms_buffer: Some(dyn_uniforms_buffer),
                static_uniforms_bind_group_layout: Some(static_uniforms_bind_group_layout),
                dyn_uniforms_bind_group_layout: Some(dyn_uniforms_bind_group_layout),
                dyn_uniform_bind_group: Some(dyn_uniform_bind_group),
                static_uniform_bind_group: Some(static_uniform_bind_group),
                offset: 0,
                alignment: min_uniform_buffer_offset_alignment,
            },
        );
    }
}

impl Update for Uniforms {
    fn update(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        // 使用动态偏移的方案来练习，尽管这里恰好一个f32做三缓冲区环的带宽和性能几乎一样
        let uniforms = get_res_mut::<Self>(data);
        // 这个buffer有三个16字节的数据，每次更新一个
        let dyn_uniform_buffer = uniforms.dyn_uniforms_buffer.as_ref().unwrap();
        gfx.queue.write_buffer(
            dyn_uniform_buffer,
            uniforms.offset * uniforms.alignment as u64,
            bytemuck::cast_slice(&[_DynUniforms {
                delta_time: gfx.delta_time,
                _padding: [0.; 3],
            }]),
        );
        uniforms.offset = (uniforms.offset + 1) % 3;
    }
}
impl<'a> Pass<'a> for Uniforms {
    fn pass(
        data: &mut ready_paint::scene::HashTypeId2Data,
        render_pass: &'a mut wgpu::RenderPass<'a>,
    ) -> &'a mut wgpu::RenderPass<'a> {
        let uniforms = get_res_mut::<Self>(data);
        render_pass.set_bind_group(0, uniforms.static_uniform_bind_group.as_ref().unwrap(), &[]);
        let dyn_bind_group = uniforms.dyn_uniform_bind_group.as_ref().unwrap();
        render_pass.set_bind_group(1, dyn_bind_group, &[uniforms.offset as u32 * uniforms.alignment]);
        render_pass
    }
}
