use std::ops::{Add, Mul};
use std::fmt;
use std::fmt::Display;

#[derive(Debug,Clone,Copy)]
struct Point<T> where T: Mul {
    x: T,
    y: T,
}

pub trait Distence<T> {
    fn dim(&self) -> T;
}

impl<T: Copy+Mul<Output = T>+Add<Output = T>> Distence<T> for Point<T> {
    fn dim(&self) -> T {
        (self.x*self.x) + (self.y*self.y)
    }
}

impl<T: Copy+Mul<Output = T>+Add<Output = T>> Point<T> {
    fn distence(&self, size: usize) -> ApiResult<T, String> {
        if core::mem::size_of::<T>() < size{
            return ApiResult::Ok(self.dim());
        }

        ApiResult::Err(format!("{}{}", "数据过大".to_string(), core::mem::size_of::<T>()))
    }
}

impl Point<f32> {
    fn distence_sqrt(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


#[derive(Debug,Clone,Copy)]
struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy+Mul<Output = T>+Add<Output = T>> Distence<T> for Point3D<T> {
    fn dim(&self) -> T {
        (self.x*self.x) + (self.y*self.y) + (self.z*self.z)
    }
}


#[derive(Debug,Clone,Copy)]
enum ApiResult<T, E> {
    Ok(T),
    Err(E),
}


impl<T: Display, E: Display> Display for ApiResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiResult::Ok(t) => write!(f, "success! data: {}", t),
            ApiResult::Err(err) => write!(f, "Error! message: {}", err),
        }
    }
}

fn main() {
    let p = Point{x: 83_f32, y: 25_f32};
    println!("impl be func: {}", point_dim(&p));
    println!("{}", p.distence(4));
    println!("{}", p.distence(5));
    println!("sqrt distence: {}", p.distence_sqrt());

    vec_add([4; 10]);
}

fn vec_add<T: std::fmt::Debug, const N: usize>(val: [T; N]) {
    println!("arr:{:?}", val)
}

fn point_dim<T>(p: &impl Distence<T>) -> T {
    p.dim()
}
