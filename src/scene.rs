use wgpu::util::RenderEncoder;

use crate::render::{Quad, Renderer};

pub struct Scene {
    pub quads: Vec<Quad>,
}

impl Scene {
    pub fn draw(&self, renderer: &Renderer) {
        let surface = &renderer.surface;
        let device = &renderer.device;
        let render_pipeline = &renderer.quads_pipeline;
        let queue = &renderer.queue;

        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        queue.write_buffer(&renderer.quads_buffer, 0, bytemuck::cast_slice(&self.quads));

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_bind_group(0, &renderer.quads_bind_group, &[]);
            rpass.set_pipeline(render_pipeline);
            rpass.draw(0..4, 0..self.quads.len() as u32);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
