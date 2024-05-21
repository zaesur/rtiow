pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn clamp(&self, x: f32) -> f32 {
        f32::clamp(x, self.min, self.max)
    }
}

#[allow(dead_code)]
pub const EMPTY: Interval = Interval {
    min: f32::INFINITY,
    max: -f32::INFINITY,
};

#[allow(dead_code)]
pub const INFINITY: Interval = Interval {
    min: -f32::INFINITY,
    max: f32::INFINITY,
};
