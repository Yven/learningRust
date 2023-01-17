#![allow(unused)] 

use core::panic;

use num::complex::Complex;

// 定义结构体，暂时不能使用引用类型，会报错
// 开启默认的输出格式可以用println输出
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    logintimes: i32,
}
// 元组结构体
struct Position(f32, f32, f32);
// 单元结构体，当结构体的内容不重要，只需要其行为时
struct AlwaysEqual;

#[derive(Debug)]
struct Admin {
    username: String,
    email: String,
    active: bool,
    logintimes: i32,
}

// 枚举类型
#[derive(Debug)]
enum Message {
    Customer,
    MsgId(u32),
    Content(String),
    Sender{uid: u32, name: String},
    Pos(f32, f32, f32),
    Status(bool),
}
// 使用枚举将两个结构体统一为一个类型
#[derive(Debug)]
enum Role {
    Client(User),
    Backend(Admin),
}


fn main() {
    // 定义
    let a = 1;
    let b: i32 = 3;
    // 变量
    let mut c = 4;
    let d = 5_i32;
    let f: u8 = 255;
    println!("{}", f);

    // 不使用的变量，编译不会报错。用_加入到数字中提高可读性类似于67,332
    let _e: u128 = 67_332;


    let str0 = String::from("A String");// 将字面量转为String
    // let str0 = "A String".to_string(); // 另一种形式
    // 以下两个均为字符串切片
    let str1 = "This is a slice";
    let str2 = &str0[0..5];// 右半开区间
    println!("str:{}, {}, {}", &str0, str1, str2);// 将String转为字面量
    let splus = String::from(", Plus other");
    // 使用format函数格式化连接字符串
    println!("format str:{}", format!("{}{}", str0, splus));
    // 使用+连接字符串相当于调用str0的add方法，第一个参数为自身，第二个参数为拼接的字面量
    let str0 = str0 + &splus;
    println!("str:{}", str0.as_str()); // 另一种String转为字面量的形式
    // 当字符串中有"时可以在开头结尾加上#标注，数量不限
    let stmp = r###"A string with "# in it. And even "##!"###;
    println!("{}", stmp);


    // 处理utf8字符的方法
    let ctt = "这里是中国";
    for cs in ctt.chars() {
        print!("{},", cs);
    }
    println!("");
    for bs in ctt.bytes() {
        print!("{},", bs);
    }
    println!("");
    println!("length:{} [{}]", utf8_slice::slice(ctt, 2, 4), utf8_slice::len(ctt));


    // 解构元组
    let (aa, mut bb) = (true, false);
    println!("{} - {}", aa, bb);
    bb = true;
    assert_eq!(aa, bb);
    println!("c before add func {}", c);

    let tup = (1, 2.11, 3, 4, 5, 6.0, 7);
    let (a1, .. , a0, _) = tup;
    println!("begin:{}, end:{}", a1, a0);
    // 使用.访问元组
    println!("tup 1 value:{}", tup.1);
    // 获取函数返回的元组
    let (len, gstr) = str_len(ctt);
    println!("tup fn return:({},{})", len, gstr);


    let username = String::from("Yven");
    let email = String::from("yvenchang@163.com");
    // 结构体定义，同名变量可以直接赋值
    let user1 = User {
        username,
        email,
        active: true,
        logintimes: 3,
    };
    // 开启Debug之后可以使用这种方式输出
    println!("{:?}", user1);
    // 将一个结构体的数据给另一个结构体中，仅限未赋值
    let mut user2 = User {
        username: String::from("raaaisnla"),
        logintimes: dbg!(18*b), // dbg详细输出变量情况同时会返回此表达式的值
        // 必须在结构体最后一行
        ..user1
    };
    // 直接修改结构体内的值，仅限被定义为可变
    user2.email = String::from("****@****,com");
    dbg!(&user2);
    // 以更美观的方式输出
    println!("{:#?}", user2);
    // 定义单元结构体行为
    // let subject = AlwaysEqual;
    // impl testTrait for AlwaysEqual {
    // }


    // 使用枚举
    let msg_cus = Message::Customer;
    let client = Role::Client(user2);
    println!("{:?}", msg_cus);
    println!("{:?}", client);


    // Option枚举用以替代null实现对空值的定义
    let some_param = Some(5);
    let none_param: Option<i32> = None;

    if some_param.is_some() {
        println!("Option has value");
    }


    // 调用函数
    c = add(add(a, b), add(c, d));
    println!("result {}", c);


    // 变量遮罩
    let space = "    ";
    let space = space.len();
    println!("space length: {}", space);

    // 浮点数
    let ff: f64 = 1.2;
    println!("float: {}", ff);

    // NaN
    let g = (-42.0_f32).sqrt();
    if g.is_nan() {
        println!("undefined! {}", g);
    }


    let float_arr: [f32; 3] = [
        42.0_f32,
        33_f32,
        83.1,
    ];
    // 批量生成值为5的元素8个
    let _dup_arr : [f32; 3]= [5.0;3];
    // 数组元素为复杂类型时这样进行批量生成
    let _dup_str_arr: [String; 5] = core::array::from_fn(|i| String::from("test"));
    // 创建一个数组的切片，与&str同理
    let arr_slice: &[f32] = &_dup_arr[1..3];
    // 二维数组
    let arrays: [[f32; 3]; 2] = [float_arr, _dup_arr];
    let mut sum: f32 = 0.0;
    for a in &arrays {
        for ax in a {
            sum += ax;
        }
    }
    println!("sum={}", sum);
    // 输出数组第2个值，并保留两位小数
    println!("{:.2}", float_arr[1]);
    println!("{:?}", _dup_str_arr);
    println!("{:?}", _dup_arr);
    println!("{:?}", arr_slice);
    println!("{:?}", arrays);


    // 位运算，加上=号的位运算
    let (mut h, i) = (3,4);
    h <<= i;
    println!("{}", h);

    // 生成序列，只可用于数字和字符
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!("");
    // a到z的序列，不加=则不包含z
    for i in 'a'..='z' {
        print!("{} ", i);
    }
    println!("");

    // 使用复数包
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    // if作为表达式可以返回一个值，作用类似三元运算符
    let z = if c % 2 == 1 { "t1" } else {"s1"};
    println!("{}", z);

    dead_end();
}

// 函数定义
fn add(a: i32, b: i32) -> i32 {
    if a > b {
        return b - a
    }

    // 返回，这是一个表达式，不同于语句，表达式是有返回值的
    a + b
}

fn str_len(take_str: &str) -> (usize, String) {
    let give_str = take_str.to_string();
    
    (give_str.len(), give_str)
}

// 发散函数，表示此函数永不返回
fn dead_end() -> ! {
    panic!("this is the end of the world");
}