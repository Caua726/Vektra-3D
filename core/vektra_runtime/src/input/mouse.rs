//! Representação do estado do mouse.

#[derive(Default, Debug)]
pub struct Mouse {
    pub delta: (f64, f64),
}

impl Mouse {
    /// Acumula o delta de movimento do mouse.
    pub fn process_delta(&mut self, delta: (f64, f64)) {
        self.delta.0 += delta.0;
        self.delta.1 += delta.1;
    }

    /// Zera o delta acumulado (deve ser chamado a cada frame).
    pub fn reset_delta(&mut self) {
        self.delta = (0.0, 0.0);
    }
}
