use std::ops::{ Add, Mul };

const ITERATIONS: usize = 100;
const THRESHOLD: f64 = 10000000000.0;

#[derive(Clone, Copy)]
struct ImaginaryNumber {
    real: f64,
    imaginary: f64
}

impl ImaginaryNumber {
    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
    fn sq_magnitude(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }
}

impl Add for ImaginaryNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl Mul for ImaginaryNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real
        }
    }
}

pub struct MandelbrotSet {}

impl MandelbrotSet {
    pub fn new() -> Self {
        Self{}
    }

    pub fn process(&self, x: f64, y: f64) -> (u8, u8, u8) {
        let num = ImaginaryNumber::new(x, y);
        self.color_ramp(self.madelbrot_magnitude(num))
    }

    fn madelbrot_magnitude(&self, num: ImaginaryNumber) -> usize {
        let mut i = 1;
        let mut f = num;
        while i <= ITERATIONS {
            if f.sq_magnitude() > THRESHOLD {
                return i
            }
            f = f * f + num;
            i += 1;
        }
        0
    }

    fn color_ramp(&self, num: usize) -> (u8, u8, u8) {
        if num == 0 {
            (0, 0, 0)
        } else {
            let num: f64 = num as f64 / ITERATIONS as f64;
            if num > 0.9 {
                (20, 20, 150)
            } else if num > 0.8 {
                (100, 20, 100)
            } else if num > 0.6 {
                (150, 20, 20)
            } else if num > 0.4 {
                (100, 100, 20)
            } else if num > 0.2 {
                (20, 150, 20)
            } else {
                (20, 100, 100)
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn madelbrot_values() {
        let mbs = MandelbrotSet::new();
        assert!(!mbs.number_in_set(ImaginaryNumber::new(1.0, 0.0)));
        assert!(!mbs.number_in_set(ImaginaryNumber::new(0.0, 2.0)));
        assert!(mbs.number_in_set(ImaginaryNumber::new(0.0, 0.0)));
        assert!(mbs.number_in_set(ImaginaryNumber::new(-1.0, 0.0)));
        assert!(mbs.number_in_set(ImaginaryNumber::new(0.0, 1.0)));
        assert!(mbs.number_in_set(ImaginaryNumber::new(0.0, -1.0)));
    }
}