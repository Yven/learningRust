#![allow(unused)]

use num::ToPrimitive; 

fn main() {
    let mut arr: [i32; 10] = [0; 10];
    // 获取arr的可变引用和key值
    // while同理
    for (i, a) in arr.iter_mut().enumerate() {
        *a = if i % 2 == 0 {
            i.to_i32().unwrap()
        } else {
            (i%2).to_i32().unwrap()
        }
    }

    println!("for:{:?}", arr);

    // loop循环是一个表达式，可以使用break返回值
    let mut counter = 0;
    let res = loop {
        counter += 1;

        if counter == 5 {
            break counter;
        }
    };
    println!("loop:{}", res);


    let x = -10;
    let var_op = 100_f32.to_i32();
    // 使用match处理Option枚举的情况
    let var_real = match var_op {
        Some(1) | Some(2) => 1,
        Some(x) => x,
        _ => 0,
    };
    println!("match var real:{}", var_real);
    let var_op: Option<i32> = None;
    // 使用if let处理，可以省略match中其他情况的语句
    if let Some(x) = var_op {
        println!("if let var real:{}", x);
    }
    // 使用matches!宏处理，返回true/false
    println!("matches!:{}", matches!(var_op, Some(x) if x == 100));
    // 上面匹配模式种的同名x会覆盖x的值，但是当作用域结束时x会回到原来的值
    println!("x val:{}", x);

    match x {
        1 ..= 5 => println!("one to five"),
        _ => println!("other"),
    }
}