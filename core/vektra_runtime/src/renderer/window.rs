use std::{error::Error, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent, StartCause},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{CursorGrabMode, Window, WindowAttributes, WindowId},
};

use crate::{
    input::{camera::CameraController, keyboard::Keyboard, mouse::Mouse},
    renderer::viewport::Viewport,
};

pub struct App {
    window: Option<Window>,
    viewport: Option<Viewport>,
    keyboard: Keyboard,
    mouse: Mouse,
    cam_controller: CameraController,
    right_mouse_pressed: bool,
    last_frame: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            viewport: None,
            keyboard: Keyboard::default(),
            mouse: Mouse::default(),
            cam_controller: CameraController::default(),
            right_mouse_pressed: false,
            last_frame: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _el: &ActiveEventLoop) {
        // A criação da janela foi movida para `new_events` para garantir a ordem correta.
    }

    fn new_events(&mut self, el: &ActiveEventLoop, cause: StartCause) {
        if cause == StartCause::Init {
            // 1. Criar a janela primeiro.
            let window = match el.create_window(WindowAttributes::default().with_title("Vektra Viewport")) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Falha ao criar janela: {e}");
                    el.exit();
                    return;
                }
            };
            
            // 2. Criar a viewport usando a referência da janela recém-criada.
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
            let viewport = Viewport::new(&window, &instance);
            
            // 3. Armazenar tudo no estado da App.
            self.window = Some(window);
            self.viewport = Some(viewport);

            println!("=== Vektra 3D Viewport ===");
            println!("Controles:");
            println!("  WASD - Mover câmera");
            println!("  Espaço - Subir");
            println!("  Shift - Descer");
            println!("  Botão direito (segurar) - Capturar mouse e olhar ao redor");
            println!("  Esc - Fechar");

            self.last_frame = Instant::now();
        }
    }
    
    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        if self.window.is_none() || self.viewport.is_none() { return; }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame);
        self.last_frame = now;

        self.cam_controller.update(dt, &self.keyboard);
        self.cam_controller.update_from_mouse(&self.mouse);
        
        self.mouse.reset_delta();
        
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if self.right_mouse_pressed {
                self.mouse.process_delta(delta);
            }
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // Agora podemos verificar window e viewport juntos.
        let (Some(window), Some(_viewport)) = (&self.window, &self.viewport) else { return; };
        if window.id() != id { return; }

        match event {
            WindowEvent::CloseRequested => el.exit(),
            
            WindowEvent::KeyboardInput { event, .. } => {
                if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) {
                    el.exit();
                }
                let raw_key = winit::event::RawKeyEvent {
                    physical_key: event.physical_key,
                    state: event.state,
                };
                self.keyboard.process_input(&raw_key);
            }

            WindowEvent::Resized(sz) => {
                self.viewport.as_mut().unwrap().resize(sz.width, sz.height);
            }

            WindowEvent::RedrawRequested => {
                // Usamos a referência segura que já verificamos.
                self.viewport.as_mut().unwrap().render(&self.cam_controller);
            }

            WindowEvent::MouseInput { button, state, .. } => {
                if button == winit::event::MouseButton::Right {
                    let is_pressed = state == winit::event::ElementState::Pressed;
                    self.right_mouse_pressed = is_pressed;
                    if is_pressed {
                        self.capture_cursor(window);
                    } else {
                        self.release_cursor(window);
                    }
                }
            }
            
            _ => {}
        }
    }
}

impl App {
    fn capture_cursor(&self, window: &Window) {
        if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined) {
            if let Err(e2) = window.set_cursor_grab(CursorGrabMode::Locked) {
                eprintln!("Falha ao capturar cursor: Confined({e}), Locked({e2})");
            }
        }
        window.set_cursor_visible(false);
    }

    fn release_cursor(&self, window: &Window) {
        if let Err(e) = window.set_cursor_grab(CursorGrabMode::None) {
            eprintln!("Falha ao liberar cursor: {e}");
        }
        window.set_cursor_visible(true);
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let el = EventLoop::new()?;
    let mut app = App::default();
    el.run_app(&mut app)?;
    Ok(())
}
