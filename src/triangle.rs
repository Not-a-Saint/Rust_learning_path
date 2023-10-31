#[derive(Debug)]
pub struct Triangle {
    pub(crate) cat1: f64,
    pub(crate) cat2: f64,
}

impl Triangle {
    pub fn find_hyp(&self) -> f64 {
        return (self.cat1.powf(2.0) + self.cat2.powf(2.0)).sqrt();
    }

    pub fn find_area(&self) -> f64 {
        return (self.cat1 * self.cat2) / 2.0;
    }

    pub fn create_isc(cat: f64) -> Triangle {
        Triangle {
            cat1: cat,
            cat2: cat,
        }
    }
}