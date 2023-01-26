use std::ops::{Add, Sub, Mul, Div}; 

//Requires the implementation of the complex numbers basic arithmetic 
// c = u + i*v - can be presented as a struct or a tuple (u, v)
// u and v are real numbers

pub struct Complex<T> {
    u: T, 
    v: T, 
}

//Implementing complex numbers
impl <T> Complex<T> {
    pub fn new(u: T, v: T) -> Self{
        Complex { u: u, v: v }
    }

    //Returns the real part of the complex number
    pub fn re(self: Self) -> T {
        self.u
    }

    //Return the imaginary part of the complex number
    pub fn im(self: Self) -> T {
        self.v
    }  
}

//Implementing the f64 addition operation on complex numbers
impl Add<Complex<f64>> for Complex<f64> {
    type Output = Self; 
    
    fn add(self, rhs: Complex<f64>) -> Self::Output {
        Self {
            u: self.u + rhs.u, 
            v: self.v + rhs.v, 
        }    
    }
}    

//Implementing the f64 subtraction operation on complex numbers
impl Sub<Complex<f64>> for Complex<f64> {
    type Output = Self; 
    
    fn sub(self, rhs: Complex<f64>) -> Self::Output {
        Self {
            u: self.u - rhs.u, 
            v: self.v - rhs.v, 
        }    
    }    
}

//Implementing the f64 multiplication operation on complex numbers
impl Mul<Complex<f64>> for Complex<f64> {
    type Output = Self; 
    
    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        Self {
            u: self.u * rhs.u - self.v * rhs.v, 
            v: self.v * rhs.u + self.u * rhs.v, 
        }    
    }    
}

//Implementing the f64 division operation on complex numbers
impl Div<Complex<f64>> for Complex<f64> {
    type Output = Self; 
    
    fn div(self, rhs: Complex<f64>) -> Self::Output {
        Self {
            u: (self.u * rhs.u + self.v * rhs.v) / (rhs.u * rhs.u + rhs.v * rhs.v), 
            v: (self.v * rhs.u - self.u * rhs.v) / (rhs.u * rhs.u + rhs.v * rhs.v), 
        }    
    }    
}


//Implementing the i64 addition operation on complex numbers
impl Add<Complex<i64>> for Complex<i64> {
    type Output = Self; 
    
    fn add(self, rhs: Complex<i64>) -> Self::Output {
        Self {
            u: self.u + rhs.u, 
            v: self.v + rhs.v, 
        }    
    }
}    

//Implementing the i64 subtraction operation on complex numbers
impl Sub<Complex<i64>> for Complex<i64> {
    type Output = Self; 
    
    fn sub(self, rhs: Complex<i64>) -> Self::Output {
        Self {
            u: self.u - rhs.u, 
            v: self.v - rhs.v, 
        }    
    }    
}

//Implementing the i64 multiplication operation on complex numbers
impl Mul<Complex<i64>> for Complex<i64> {
    type Output = Self; 
    
    fn mul(self, rhs: Complex<i64>) -> Self::Output {
        Self {
            u: self.u * rhs.u - self.v * rhs.v, 
            v: self.v * rhs.u + self.u * rhs.v, 
        }    
    }    
}

//Implementing the i64 division operation on complex numbers
impl Div<Complex<i64>> for Complex<i64> {
    type Output = Complex<f64>; 
    
    fn div(self, rhs: Complex<i64>) -> Complex<f64> {
        Complex::<f64> {
            u: (self.u * rhs.u + self.v * rhs.v) as f64 / (rhs.u * rhs.u + rhs.v * rhs.v) as f64, 
            v: (self.v * rhs.u - self.u * rhs.v) as f64 / (rhs.u * rhs.u + rhs.v * rhs.v) as f64, 
        }    
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_i64_div_re_01() {
        let c1 = Complex::new(3, 4);
        let c2 = Complex::new(8, -2);
        let c3 = c1 / c2; 
        assert_eq!(4 as f64/17 as f64, c3.re());
    }

    #[test]
    fn test_complex_i64_div_im_01() {
        let c1 = Complex::new(3, 4);
        let c2 = Complex::new(8, -2);
        let c3 = c1 / c2; 
        assert_eq!(19 as f64/34 as f64, c3.im());
    }

    #[test]
    fn test_complex_f64_div_re_01() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(8.0, -2.0);
        let c3 = c1 / c2; 
        assert_eq!(4.0/17.0, c3.re());
    }

    #[test]
    fn test_complex_f64_div_im_01() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(8.0, -2.0);
        let c3 = c1 / c2; 
        assert_eq!(19.0/34.0, c3.im());
    }

    #[test]
    fn test_complex_i64_mul_re_01() {
        let c1 = Complex::new(3, 2);
        let c2 = Complex::new(1, 7);
        let c3 = c1 * c2; 
        assert_eq!(-11, c3.re());
    }

    #[test]
    fn test_complex_f64_mul_re_01() {
        let c1 = Complex::new(3.0, 2.0);
        let c2 = Complex::new(1.0, 7.0);
        let c3 = c1 * c2; 
        assert_eq!(-11.0, c3.re());
    }

    #[test]
    fn test_complex_f64_mul_im_01() {
        let c1 = Complex::new(3.0, 2.0);
        let c2 = Complex::new(1.0, 7.0);
        let c3 = c1 * c2; 
        assert_eq!(23.0, c3.im());
    }

    #[test]
    fn test_complex_f64_minus_re_01() {
        let c1 = Complex::new(-1.0, -2.0);
        let c2 = Complex::new(1.0, 3.0);
        let c3 = c1 - c2; 
        assert_eq!(-2.0, c3.re());
    }

    #[test]
    fn test_complex_f64_minus_im_01() {
        let c1 = Complex::new(-1.0, -2.0);
        let c2 = Complex::new(1.0, 3.0);
        let c3 = c1 - c2; 
        assert_eq!(-5.0, c3.im());
    }

    #[test]
    fn test_complex_f64_sum_re_01() {
        let c1 = Complex::new(1.0, -2.0);
        let c2 = Complex::new(1.0, 3.0);
        let c3 = c1 + c2; 
        assert_eq!(2.0, c3.re());
    }

    #[test]
    fn test_complex_f64_sum_im_01() {
        let c1 = Complex::new(1.0, -2.0);
        let c2 = Complex::new(1.0, 3.0);
        let c3 = c1 + c2; 
        assert_eq!(1.0, c3.im());
    }

    #[test]
    fn test_complex_f64_re_01() {
        let c = Complex::new(1.0, -2.0);
        assert_eq!(1.0, c.re());
    }

    #[test]
    fn test_complex_f64_im_01() {
        let c = Complex::new(1.0, -2.0);
        assert_eq!(-2.0, c.im());
    }
}
