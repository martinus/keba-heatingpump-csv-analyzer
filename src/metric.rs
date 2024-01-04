use std::str::FromStr;

pub struct Metric {
    min: f64,
    max: f64,
    count: u64,
    sum: f64,
}

impl Metric {
    pub fn new() -> Metric {
        Metric {
            min: std::f64::MAX,
            max: std::f64::MIN,
            count: 0,
            sum: 0.0,
        }
    }

    pub fn add(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;

        if val < self.min {
            self.min = val;
        }
        if (val > self.max) {
            self.max = val;
        }
    }

    pub fn add_str(&mut self, val: &str) {
        self.add(f64::from_str(val).unwrap());
    }

    pub fn avg(&self) -> f64 {
        return self.sum / self.count as f64;
    }

    pub fn min(&self) -> f64 {
        return self.min;
    }

    pub fn max(&self) -> f64 {
        return self.max;
    }

    pub fn count(&self) -> u64 {
        return self.count;
    }
}
