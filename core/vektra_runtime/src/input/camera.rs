//! Controlador de câmera estilo FPS que reage a teclado e mouse.

use super::{keyboard::Keyboard, mouse::Mouse};
use winit::keyboard::KeyCode;
use std::time::Duration;

/* ---------- vetorzinho 3-D ---------- */
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    fn add(self, o: Vec3) -> Vec3 { Vec3(self.0 + o.0, self.1 + o.1, self.2 + o.2) }
    fn mul(self, s: f32) -> Vec3 { Vec3(self.0 * s, self.1 * s, self.2 * s) }
    fn length(self) -> f32 { (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt() }
    fn norm(self) -> Vec3 {
        let l = self.length();
        if l != 0.0 { self.mul(1.0/l) } else { self }
    }
    fn zero() -> Vec3 { Vec3(0.0, 0.0, 0.0) }
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1*b.2 - a.2*b.1,
        a.2*b.0 - a.0*b.2,
        a.0*b.1 - a.1*b.0,
    )
}
/* ----------------------------------- */

#[derive(Debug)]
pub struct CameraController {
    pub pos: Vec3,
    velocity: Vec3,
    yaw:   f32,
    pitch: f32,
    speed: f32,
    sens:  f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self { 
            pos: Vec3(0.0, 2.0, 5.0), 
            velocity: Vec3::zero(),
            yaw: -90.0, 
            pitch: 0.0, 
            speed: 15.0, // Aumentado para ser usado como aceleração
            sens: 0.1,  // Sensibilidade ajustada
        }
    }
}

impl CameraController {
    /// Processa o teclado e retorna um vetor de direção normalizado.
    fn process_keyboard(&mut self, kb: &Keyboard) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        let forward = Vec3(yaw_rad.cos(), 0.0, yaw_rad.sin()).norm();
        let right   = Vec3(-forward.2, 0.0, forward.0);
        let mut direction = Vec3::zero();

        if kb.pressed(KeyCode::KeyW)      { direction = direction.add(forward); }
        if kb.pressed(KeyCode::KeyS)      { direction = direction.add(forward.mul(-1.0)); }
        if kb.pressed(KeyCode::KeyA)      { direction = direction.add(right.mul(-1.0)); }
        if kb.pressed(KeyCode::KeyD)      { direction = direction.add(right); }
        if kb.pressed(KeyCode::Space)     { direction.1 += 1.0; }
        if kb.pressed(KeyCode::ShiftLeft) { direction.1 -= 1.0; }

        if direction.length() > 0.0 {
            direction.norm()
        } else {
            direction
        }
    }

    /// Atualiza a câmera com base no tempo e input.
    pub fn update(&mut self, dt: Duration, kb: &Keyboard) {
        let direction = self.process_keyboard(kb);
        let dt_secs = dt.as_secs_f32();

        // Aplica aceleração baseada no input
        self.velocity = self.velocity.add(direction.mul(self.speed * dt_secs));

        // Aplica amortecimento (fricção) para suavizar a parada
        let damping = 0.92;
        self.velocity = self.velocity.mul(damping);

        // Para o movimento se a velocidade for muito baixa
        if self.velocity.length() < 0.01 {
            self.velocity = Vec3::zero();
        }

        // Atualiza a posição
        self.pos = self.pos.add(self.velocity.mul(dt_secs));
    }

    pub fn update_from_mouse(&mut self, m: &Mouse) {
        self.yaw   += m.delta.0 as f32 * self.sens;
        self.pitch -= m.delta.1 as f32 * self.sens;
        self.pitch  = self.pitch.clamp(-89.0, 89.0);
    }

    /// Matriz `view` 4 × 4 (pode ser enviada a shader futuramente).
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let dir = Vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).norm();
        let up = Vec3(0.0, 1.0, 0.0);
        let z = dir.mul(-1.0);
        let x = cross(up, z).norm();
        let y = cross(z, x);
        let dot = |a: Vec3, b: Vec3| -(a.0*b.0 + a.1*b.1 + a.2*b.2);

        [
            [x.0, y.0, z.0, 0.0],
            [x.1, y.1, z.1, 0.0],
            [x.2, y.2, z.2, 0.0],
            [dot(x, self.pos), dot(y, self.pos), dot(z, self.pos), 1.0],
        ]
    }

    /// Retorna a posição da câmera como array para uniforms
    pub fn position(&self) -> [f32; 3] {
        [self.pos.0, self.pos.1, self.pos.2]
    }
}
