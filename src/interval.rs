use std::f64;

#[derive(Debug, Clone, Copy,Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval{
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval{
            min,
            max
        }
    }

    pub fn size(&self) -> f64{
        return self.max - self.min;
    }

    pub fn contains(&self,x: f64) -> bool{
        return self.min < x && self.max > x;
    }

    pub fn surrounds(&self,x:f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self,x: f64) -> f64{
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max ;
        }
        return x ;
    }

}
#[allow(dead_code)]
const EMPTY_INTERVAL: Interval = Interval {
    min : f64::INFINITY,
    max : f64::NEG_INFINITY,
};
#[allow(dead_code)]
const UNIVERSE_INTERVAL: Interval = Interval {
    min : f64::NEG_INFINITY,
    max : f64::INFINITY,
};