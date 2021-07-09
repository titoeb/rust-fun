use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}
impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Complex<T> {
        return Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        };
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        return self.re == other.re && self.im == other.im;
    }
}
fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);

    println!("{:?}, {:?}", a, b);
    println!("{:?}", a + b);
    let c = Complex::new(3.0, 4.0);
    let mut d = Complex::new(1.0, 2.0);
    d += c;
    println!("{:?}", -d);
    let e = Complex::new(1.0, 2.0);
    let f = Complex::new(3.0, 4.0);
    println!("{}", e == f);
}
