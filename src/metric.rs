use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Metric {
    min: f64,
    max: f64,
    count: u64,
    sum: f64,
}

impl Metric {
    fn update_min_max(&mut self, val: f64) {
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }

    pub fn new() -> Metric {
        Metric {
            min: std::f64::MAX,
            max: std::f64::MIN,
            count: 0,
            sum: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.min = std::f64::MAX;
        self.max = std::f64::MIN;
        self.count = 0;
        self.sum = 0.0;
    }

    pub fn add(&mut self, val: &str) {
        self.add_times(val, 1);
    }

    pub fn add_times(&mut self, val: &str, count: u64) {
        let v = f64::from_str(val).unwrap();
        self.count += count;
        self.sum += (count as f64) * v;
        self.update_min_max(v);
    }

    pub fn avg(&self) -> f64 {
        return self.sum / self.count as f64;
    }

    pub fn sum(&self) -> f64 {
        return self.sum;
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
