#![allow(unused)]

use world_hello::front_house::{self, host, serving};
use world_hello::backend_house::ketchen;
use std::io::Result as IoResult;
use std::fmt::Result as FmtResult;

fn main() {
    // 输出实现了std::fmt::Display特征的类型
    println!("{}", 42);
    // 输出实现了std::fmt::Debug特征的类型
    println!("{:?}", 42);
}
