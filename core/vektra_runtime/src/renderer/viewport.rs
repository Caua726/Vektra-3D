use std::{mem, pin::Pin, task::*};
use wgpu::{
    util::DeviceExt, ColorTargetState, Device, Queue, RenderPipeline, Surface,
    SurfaceConfiguration, VertexAttribute, VertexBufferLayout, VertexStepMode,
};
use winit::window::Window;
use crate::input::camera::CameraController;

/* ---------- executor síncrono mínimo (substitui pollster) ---------- */
fn block_on<F: std::future::Future>(mut fut: F) -> F::Output {
    fn no_op(_: *const ()) {}
    const V: RawWakerVTable = RawWakerVTable::new(|p| RawWaker::new(p, &V), no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &V)) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => break v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

/* --------------------------- vértices --------------------------- */
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 3],
    _padding: f32,
}

// Layout de buffer
impl Vertex {
    const ATTRS: [VertexAttribute; 1] =
        wgpu::vertex_attr_array![0 => Float32x3];

    fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as _,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

/* -------------------------- viewport -------------------------- */
pub struct Viewport {
    surface: Surface<'static>,
    cfg: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    vbuf: wgpu::Buffer,
    vert_count: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Viewport {
    pub fn new(window: &Window, instance: &wgpu::Instance) -> Self {
        /* ---- surface & device ---- */
        let surface_tmp = instance.create_surface(window).expect("surface");
        let surface = unsafe { mem::transmute::<Surface<'_>, Surface<'static>>(surface_tmp) };
        let size = window.inner_size();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("adapter");

        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];

        let cfg = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &cfg);

        /* ---- uniform buffer ---- */
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        /* ---- bind group layout ---- */
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind_group_layout"),
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

        /* ---- bind group ---- */
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        /* ---- pipeline layout ---- */
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        /* ---- shader ---- */
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("grid_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/grid.wgsl").into()),
        });

        /* ---- pipeline ---- */
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("grid_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        /* ---- gera grade (agora um plano) ---- */
        const SIZE: f32 = 250.0; // Um plano bem grande
        let verts = vec![
            Vertex { pos: [-SIZE, 0.0, -SIZE] },
            Vertex { pos: [ SIZE, 0.0, -SIZE] },
            Vertex { pos: [ SIZE, 0.0,  SIZE] },
            Vertex { pos: [ SIZE, 0.0,  SIZE] },
            Vertex { pos: [-SIZE, 0.0,  SIZE] },
            Vertex { pos: [-SIZE, 0.0, -SIZE] },
        ];

        // ---- cria vertex-buffer sem bytemuck ----
        let raw: &[u8] = unsafe {
            std::slice::from_raw_parts(
                verts.as_ptr() as *const u8,
                verts.len() * std::mem::size_of::<Vertex>(),
            )
        };
        let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("grid_vertices"),
            contents: raw,
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            cfg,
            device,
            queue,
            pipeline,
            vbuf,
            vert_count: verts.len() as u32,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.cfg.width = w;
        self.cfg.height = h;
        self.surface.configure(&self.device, &self.cfg);
    }

    pub fn render(&mut self, camera: &CameraController) {
        // Calcula matriz de projeção
        let aspect = self.cfg.width as f32 / self.cfg.height as f32;
        let proj = perspective_matrix(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        
        // Obtém matriz view da câmera
        let view = camera.view_matrix();
        
        // Multiplica view * proj
        let view_proj = multiply_matrices(view, proj);
        
        // Atualiza uniform buffer
        let uniforms = Uniforms { view_proj, camera_pos: camera.position(), _padding: 0.0 };
        let uniform_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                &uniforms as *const _ as *const u8,
                std::mem::size_of::<Uniforms>(),
            )
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, uniform_data);

        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => return,
        };
        let view = frame.texture.create_view(&Default::default());

        let mut enc =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("enc") });

        {
            let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("grid_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rp.set_pipeline(&self.pipeline);
            rp.set_bind_group(0, &self.uniform_bind_group, &[]);
            rp.set_vertex_buffer(0, self.vbuf.slice(..));
            rp.draw(0..self.vert_count, 0..1);
        }

        self.queue.submit(Some(enc.finish()));
        frame.present();
    }
}

// Função para criar matriz de perspectiva
fn perspective_matrix(fovy: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fovy / 2.0).tan();
    [
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), -1.0],
        [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
    ]
}

// Função para multiplicar duas matrizes 4x4
fn multiply_matrices(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}
