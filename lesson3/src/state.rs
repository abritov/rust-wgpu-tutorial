use winit::window::Window;
use winit::{
    event::*,
};
use wgpu::util::DeviceExt;

use std::ops::Div;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const CUBE_VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0,  1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-1.0,  1.0,  1.0], color: [1.0, 0.0, 0.0] },

    // Back face
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },

    // Top face
    Vertex { position: [-1.0,  1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [1.0,  1.0, -1.0], color: [1.0, 0.0, 0.0] },

    // Bottom face
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, -1.0,  1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },

    // Right face
    Vertex { position: [1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0,  1.0, -1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [1.0, -1.0,  1.0], color: [1.0, 0.0, 0.0] },

    // Left face
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-1.0,  1.0, -1.0], color: [1.0, 0.0, 0.0] },
];

const CUBE_INDICES: &[u16] = &[
    0,  1,  2,      0,  2,  3,    // front
    4,  5,  6,      4,  6,  7,    // back
    8,  9,  10,     8,  10, 11,   // top
    12, 13, 14,     12, 14, 15,   // bottom
    16, 17, 18,     16, 18, 19,   // right
    20, 21, 22,     20, 22, 23,   // left
];

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    mouse_x: Option<f64>,
    mouse_y: Option<f64>,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    // NEW!
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    render_cube: bool,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // instance - объект для связи/управления видеокартой
        // Backends::all => OpenGL + Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // new()
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        // NEW!
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = INDICES.len() as u32;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::description()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Для установки других свойств требуется поддержка Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Для установки значения true требуется Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Для установки значения true требуется Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            mouse_x: None,
            mouse_y: None,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            render_cube: false
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = Some(position.x.div(self.size.width as f64).clamp(0.0, 1.0));
                self.mouse_y = Some(position.y.div(self.size.height as f64).clamp(0.0, 1.0));
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                self.render_cube = !self.render_cube;
                let vertices = if self.render_cube {
                    self.index_buffer = self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(CUBE_INDICES),
                            usage: wgpu::BufferUsages::INDEX,
                        }
                    );
                    self.num_indices = CUBE_INDICES.len() as u32;
                    CUBE_VERTICES
                        .into_iter()
                        .map(|v| Vertex {
                            position: [v.position[0] / 2.0, v.position[1] / 2.0, v.position[2] / 2.0],
                            ..*v
                        })
                        .collect::<Vec<Vertex>>()
                } else {
                    self.index_buffer = self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(INDICES),
                            usage: wgpu::BufferUsages::INDEX,
                        }
                    );
                    self.num_indices = INDICES.len() as u32;
                    VERTICES.to_vec()
                };
                self.vertex_buffer = self.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }
                );
            }
            _ => {}
        }
        false
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.mouse_y.unwrap_or(0.1),
                            g: 0.2,
                            b: self.mouse_x.unwrap_or(0.3),
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}