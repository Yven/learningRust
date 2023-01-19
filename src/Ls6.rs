use std::{vec, collections::HashMap};

use num::ToPrimitive;


trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4 addr: {}", self.0)
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6 addr: {}", self.0)
    }
}


fn main() {
    // 可变数组
    let mut v: Vec<i32> = Vec::with_capacity(10);
    for x in 0..10 {
        v.push(x);
    }
    v.push(0);
    println!("vec: {:?}({}/{})", v, v.len(), v.capacity());
    for (size, x) in v.iter_mut().enumerate() {
        match size.pow(2).to_i32() {
            Some(idx) => *x = idx,
            None => *x = 0,
        }
    }
    println!("vec: {:?}", v);
    println!("second val form index: {}", &v[2]);
    match v.get(2) {
        Some(second) => println!("second val form get(): {}", *second),
        None => println!("out of edge"),
    }

    // 特征对象数组
    let vst: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for st in vst {
        st.display();
    }

    // 定义元组动态数组，作为HashMap的原始数据
    let code_map = vec![
        ("401001", "认证错误"),
        ("401002", "参数错误"),
        ("404000", "资源不存在"),
        ("500000", "服务错误"),
    ];

    // 使用此方法快捷的将元组数组导入到HashMpa中
    let mut code_hmap: HashMap<_,_> = code_map.into_iter().collect();
    code_hmap.insert("401000", "未知错误");
    code_hmap.insert("401002", "身份错误");
    println!("{}", match code_hmap.get("401002") {Some(x) => x, None => "获取key值错误"});
    for (k, v) in &code_hmap {
        println!("{} : {}", k, v);
    }

    // 如果不存在则插入，返回的是元素的可变引用，直接修改返回的值就可以修改HashMap中的对应数据
    let vl = code_hmap.entry("500000").or_insert("不算错误");
    *vl = "Test";
    println!("{:#?}", code_hmap);

    // 类型转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    let d: u16 = 1500;
    let d_: u8 = match d.try_into() {
        Ok(x) => x, 
        Err(err) => {
            println!("error msg: {}", err);
            0
        }
    };
    println!("force transmate: {}/{}/{}/{}", a, b, c, d_);

    // 将内存地址转换为指针操作
    let mut arr: [i32; 2] = [1, 2];
    let p1: *mut i32 = arr.as_mut_ptr();
    let first_addr = p1 as usize;
    let second_addr = first_addr + 4;
    let p2 = second_addr as *mut i32;
    unsafe {
        *p2 += 1;
    }
    println!("{:?}", arr);
}