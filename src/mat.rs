pub struct Mat {
    x: Option<(f64, f64)>,
    y: Option<(f64, f64)>,
}

impl Mat {
    pub fn new() -> Self {
        Mat { x: None, y: None }
    }

    pub fn set(&mut self, delta: (f64, f64)) -> bool {
        match self.x {
            Some(_) => match self.y {
                Some(_) => true,
                None => {
                    self.y = Some(delta);
                    true
                }
            },
            None => {
                self.x = Some(delta);
                false
            }
        }
    }

    pub fn mult(&self) -> [f64; 4] {
        let x = self.x.unwrap();
        let y = self.y.unwrap();
        [x.0 * y.0, x.0 * y.1, x.1 * y.0, x.1 * y.1]
    }

    pub fn reset(&mut self) {
        self.x = None;
        self.y = None;
    }
}
