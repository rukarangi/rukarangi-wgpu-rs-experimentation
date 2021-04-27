use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::window::Window;

use wgpu::util::DeviceExt;
use std::time::{Instant, Duration};


struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipline: wgpu::RenderPipeline,
    instant: Instant,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    resolution: [f32; 2],
    // vertex_buffer: wgpu::Buffer,
    // num_vertices: u32,
}

impl State {
    async fn new(window: &Window) -> State {
        let size = window.inner_size();
        // instance is what talks to gpu
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();


        // swap chain shindig
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // get shaders
        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        // create uniform for time

        let resolution: [f32; 2] = [window.inner_size().height as f32, window.inner_size().width as f32];

        println!("{:?}", resolution);

        let mut uniforms = Uniforms::new(resolution);

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        // pipeline creation
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        
        let render_pipline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[
                    //Vertex::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main", 
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let mut instant = Instant::now();

        


        // create vertex buffer
        // let vertex_buffer = device.create_buffer_init(
        //     &wgpu::util::BufferInitDescriptor {
        //         label: Some("Vertex Buffer"),
        //         contents: bytemuck::cast_slice(VERTICES),
        //         usage: wgpu::BufferUsage::VERTEX,
        //     }
        // );

        // let num_vertices = VERTICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipline,
            instant,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            resolution,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        
        let resolution: [f32; 2] = [new_size.height as f32, new_size.width as f32];
        self.uniforms.update_resolution(resolution);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.uniforms.update_time(&self.instant);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self
            .swap_chain
            .get_current_frame()?
            .output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    time: f32,
    resolution: [f32; 2],
}

impl Uniforms {
    fn new(resolution: [f32; 2]) -> Self {
        Self {
            time: 0.0,
            resolution: resolution,
        }
    }

    fn update_time(&mut self, instant: &Instant) {
        let elapsed = instant.elapsed();
        let float_elapsed = elapsed.as_secs_f32();

        self.time = float_elapsed;
    }

    fn update_resolution(&mut self, resolution: [f32; 2]) {
        let new_resolution = resolution;

        self.resolution = new_resolution;
    }
}

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//     position: [f32; 3],
//     color: [f32; 3],
// }

// const VERTICES: &[Vertex] = &[
//     Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
//     Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
// ];

// impl Vertex {
//     fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         let attributes: &[wgpu::VertexAttribute]  = &[
//             wgpu::VertexAttribute {
//                 offset: 0,
//                 shader_location: 0,
//                 format: wgpu::VertexFormat::Float3,
//             },
//             wgpu::VertexAttribute {
//                 offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                 shader_location: 1,
//                 format: wgpu::VertexFormat::Float3,
//             },
//         ];

//         return wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::InputStepMode::Vertex,
//             attributes,
//         };
//     }
// }

fn main() {
    use futures::executor::block_on;

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            // window changes, and inputs
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { 
                            input,
                            .. 
                        } => {
                            match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {},
                            }
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        },
                        _ => {},
                    }
                }
            },
            // update screen
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {},
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {},
        }
    });
}