use std::{fs::File, io::Read, error::Error};

use num::ToPrimitive;

// main可以自定义返回值
fn main() -> Result<(), Box<dyn Error>> {
    let content = get_file_content("E:\\rust\\world_hello\\src\\test.txt".to_string())?;
    println!("content: {}", content);
    println!("enum ?: {:?}", enum_err());
    Ok(())
}

// 对返回Option的函数使用？处理
fn enum_err() -> Option<String> {
    let x = 71_i32;
    // 使用链式调用？
    // expext和unwarp功能相同，但支持自定义错误信息返回
    let y = x.to_f32()?.to_f64().expect("自定义错误").to_string();
    Some(y)
}

// 对返回Result的函数使用？处理，将错误向上层层传递
fn get_file_content(path: String) -> Result<String, std::io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}