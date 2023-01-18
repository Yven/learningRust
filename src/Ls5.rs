use std::ops::{Add, Mul};
use std::{fmt, vec};
use std::fmt::{Display, Debug};

use num::integer::Roots;

// 定义具有泛型的特征
pub trait Distence {
    // 使用关联类型简化特征声明
    type Obj;
    fn dim(&self) -> Self::Obj;
}

pub trait Draw {
    fn draw(&self) -> String;
}


// 将外部类型使用元组结构体包裹（newtype模式），就可以绕过孤儿规则，在外部类型上实现外部特征
struct Matrix(Vec<i32>);
impl Distence for Matrix {
    type Obj = i32;
    fn dim(&self) -> Self::Obj {
        let mut sum = 0;
        for x in self.0.iter() {
            sum += x*x;
        }

        sum.sqrt()
    }
}


#[derive(Debug,Clone,Copy)]
struct Point<T> where T: Mul {
    x: T,
    y: T,
}

impl<T: Mul + Debug> Draw for Point<T> {
    fn draw(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: Mul + Debug> Point<T> {
    fn draw(&self) -> String {
        format!("Nothing happen")
    }
}

// 为Point<T>实现特征
impl<T> Distence for Point<T>
where T: Copy+Mul<Output = T>+Add<Output = T>
{
    type Obj = T;
    fn dim(&self) -> T {
        (self.x*self.x) + (self.y*self.y)
    }
}

// 计算距离函数，返回ApiResult结果
impl<T: Copy+Mul<Output = T>+Add<Output = T>> Point<T> {
    fn distence(&self, size: usize) -> ApiResult<T, String> {
        if core::mem::size_of::<T>() < size{
            return ApiResult::Ok(self.dim());
        }

        ApiResult::Err(format!("{}{}", "数据过大".to_string(), core::mem::size_of::<T>()))
    }
}

// 为f32类型单独实现计算距离函数
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

impl<T: Mul + Debug> Draw for Point3D<T> {
    fn draw(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> Distence for Point3D<T> 
where T: Copy+Mul<Output = T>+Add<Output = T>
{
    type Obj = T;
    fn dim(&self) -> T {
        (self.x*self.x) + (self.y*self.y) + (self.z*self.z)
    }
}


// 具有泛型的枚举
#[derive(Debug,Clone,Copy)]
enum ApiResult<T, E> {
    Ok(T),
    Err(E),
}


// 为枚举实现系统的Display特征用以输出
impl<T: Display, E: Display> Display for ApiResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiResult::Ok(t) => write!(f, "success! data: {}", t),
            ApiResult::Err(err) => write!(f, "Error! message: {}", err),
        }
    }
}

fn main() {
    println!("newtype: {}", Matrix(vec![1, 3, 5, 7]).dim());

    let p = Point{x: 83_f32, y: 25_f32};
    println!("impl be func: {}", point_dim(&p));
    println!("{}", p.distence(4));
    println!("{}", p.distence(5));
    println!("sqrt distence: {}", p.distence_sqrt());

    vec_add([4; 10]);
    vec_add([5; 5]);

    // 正常调用同名方法
    println!("Point fn: {}", p.draw());
    // 调用特征中的同名方法
    println!("Direct fn: {}", Draw::draw(&p));
    // 强制调用同名方法（在调用关联函数时）
    println!("Force fn: {}", <Point<_> as Draw>::draw(&p));
    print_dim1(Box::new(Point3D { x: 13, y: 7, z: 37}));
    print_dim2(&p);
}

// 常量泛型，对数组长度使用泛型
fn vec_add<T: std::fmt::Debug, const N: usize>(val: [T; N]) {
    println!("arr:{:?}", val)
}

// 使用特征作为函数参数，接受所有实现此特征的参数
fn point_dim<T>(p: &impl Distence<Obj = T>) -> T {
    p.dim()
}

// 传入特征对象类型的参数
fn print_dim1(p: Box<dyn Draw>) {
    println!("{}", p.draw())
}
fn print_dim2(p: &dyn Draw) {
    println!("{}", p.draw())
}