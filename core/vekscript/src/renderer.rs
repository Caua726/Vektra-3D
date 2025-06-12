use crate::interpreter::{World, Mesh, Keyframe};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use cgmath::{prelude::*, Point3, Matrix4, Deg, Vector3 as CgVector3};
use std::time::Instant;
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: CgVector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
}

struct Instance {
    position: CgVector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)).into(),
        }
    }
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instance: Instance,
    instance_buffer: wgpu::Buffer,
    instance_bind_group: wgpu::BindGroup,
    world: World,
    start_time: Instant,
    window: Arc<Window>,
}

impl State {
    async fn new(window: Arc<Window>, world: World) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        let camera = Camera {
            eye: (0.0, 1.0, 5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: CgVector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform {
            view_proj: [[0.0; 4]; 4],
        };
        camera_uniform.view_proj = camera.build_view_projection_matrix().into();

        use wgpu::util::DeviceExt;
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        
        let instance = Instance {
            position: CgVector3::zero(),
            rotation: cgmath::Quaternion::from_axis_angle(CgVector3::unit_y(), Deg(0.0)),
        };

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&[instance.to_raw()]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        let instance_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("instance_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });
        
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &instance_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: instance_buffer.as_entire_binding(),
                }
            ],
            label: Some("instance_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &instance_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: (3 * std::mem::size_of::<f32>()) as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                    }
                ],
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
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let first_mesh: &Mesh = world.objects.values()
            .filter_map(|obj| obj.mesh.as_ref())
            .next()
            .expect("No objects with a mesh found in the world");
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&first_mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&first_mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = first_mesh.indices.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instance,
            instance_buffer,
            instance_bind_group,
            world,
            start_time: Instant::now(),
            window,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }
    
    fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        
        // Update camera matrix
        self.camera_uniform.view_proj = self.camera.build_view_projection_matrix().into();
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
        
        let object_name = self.world.objects.keys().next().unwrap();
        let mut keyframes: Vec<&Keyframe> = self.world.keyframes.iter()
            .filter(|k| &k.object_name == object_name)
            .collect();
        keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        if keyframes.len() < 2 { return; }

        let total_duration = keyframes.last().unwrap().time as f32;
        let anim_time = elapsed % total_duration;

        let current_keyframe_index = keyframes.iter().rposition(|k| k.time as f32 <= anim_time).unwrap_or(0);
        let next_keyframe_index = (current_keyframe_index + 1) % keyframes.len();
        
        let k1 = keyframes[current_keyframe_index];
        let k2 = keyframes[next_keyframe_index];

        let t = (anim_time - k1.time as f32) / (k2.time as f32 - k1.time as f32);
        
        // Interpolate position
        if let (Some(pos1), Some(pos2)) = 
            (k1.properties.get("position"), k2.properties.get("position")) {
            
            // Extract vector data using pattern matching on the full type path
            if let (crate::grammar::PropertyValue::Vector(v1), crate::grammar::PropertyValue::Vector(v2)) = (pos1, pos2) {
                let p1 = CgVector3::new(v1.x as f32, v1.y as f32, v1.z as f32);
                let p2 = CgVector3::new(v2.x as f32, v2.y as f32, v2.z as f32);
                self.instance.position = p1.lerp(p2, t.into());
            }
        }

        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&[self.instance.to_raw()]));
    }


    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.instance_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn run(world: &World) {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Vektra 3D")
        .build(&event_loop)
        .unwrap());

    let mut state = pollster::block_on(State::new(window, world.clone()));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("Error: {:?}", e),
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                // Request a redraw to trigger the next frame of the animation
                state.window.request_redraw();
            },
            _ => (),
        }
    }).unwrap();
} 