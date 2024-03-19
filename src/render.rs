use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::DeviceExt, BufferBinding, DeviceDescriptor, Features, Limits, PowerPreference, RequestAdapterOptions, ShaderModuleDescriptor
};
use winit::dpi::PhysicalSize;

pub struct Renderer<'a> {
    pub quads_pipeline: wgpu::RenderPipeline,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'a>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub quads_buffer: wgpu::Buffer,
    pub quads_bind_group: wgpu::BindGroup,
}

impl<'a> Renderer<'a> {
    pub fn update_surface_size(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub async fn new(wgpu_instance: &wgpu::Instance, window: &'a winit::window::Window) -> Self {
        let surface = wgpu_instance.create_surface(window).unwrap();
        let adapter = wgpu_instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .unwrap();

        let bind_group = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("quads"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::all(),
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let quads_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quads Buffer"),
            contents: bytemuck::cast_slice(
                &[Quad {
                    position: [0.0, 0.0],
                    size: [0.5, 0.5],
                    color: [0.0, 0.0, 0.0],
                }; 100],
            ),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let quads_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("quads"),
            layout: &bind_group,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &quads_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let quads_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let size = window.inner_size().max(PhysicalSize {
            width: 1,
            height: 1,
        });
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        Self {
            quads_pipeline,
            adapter,
            surface,
            surface_config,
            device,
            queue,
            quads_buffer,
            quads_bind_group
        }
    }
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Quad {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 3],
}
