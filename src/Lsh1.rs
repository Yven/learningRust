#[derive(Debug)]
struct SpliteString<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> SpliteString<'a> {
    // 当参数中存在&self或&mut self，则返回值的生命周期自动设置为self的生命周期
    fn splite(&self) -> &str {
        self.part.split(' ').next().expect("Could not find a ' '")
    }

    // 强制设定返回值的生命周期为另一个参数的生命周期，但由于返回的是self的引用，需要先声明'a与'b的关系才能使用，声明方式与泛型相同
    fn return_some_str(&self, anno: &'b str) -> &'b str {
        println!("{}", anno);
        self.part
    }
}

fn main() {
    let str1 = String::from("cat is cute");
    let str2 = "dog also";
    let result = longest(str1.as_str(), str2);
    println!("{}", result);
    println!("{}", new_one());
    
    let s = SpliteString {
        part: result
    };
    println!("{:#?}", s.part);
    println!("{}", s.splite());
    println!("{}", s.return_some_str("the new one"));

    // 此变量拥有静态生命周期声明，他的生命周期与程序一样长
    let sts: &'static str = "this is forever";
    println!("{}", sts);
}

// 此函数返回值的生命周期会等于a和b中生命周期小的那个
// 注意：生命周期的声明不会改变变量的生命周期，只是通过此方式人为限制编译通过的条件达成引用安全
// 如a和b中a的生命周期较小，此时返回值被声明为与a同等的生命周期，但如果此返回值在a的生命周期外被引用编译器就会报错，以此保证内存安全
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// 此时返回引用会导致悬垂指针
// fn new_one() -> &str {
//     let result = String::from("test haha");
//     result.as_str()
// }
// 直接移交所有权
fn new_one() -> String{
    String::from("test haha")
}