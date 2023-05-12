#![allow(unused)]

// FnOnce声明的闭包只能调用一次，因为闭包内捕获的变量所有权将被转移
// 拥有Copy声明，多次使用将使用被捕获变量的拷贝
struct Cacher<T,E>
where
    T: FnOnce(E) -> E + Copy,
{
    query: T,
    value: Option<E>
}

impl<T,E> Cacher<T,E>
where
    T: FnOnce(E) -> E + Copy,
    E: Clone
{
    fn new(query: T) -> Cacher<T,E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match &self.value {
            Some(v) => v.clone(),
            None => {
                let v = (self.query)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }

    fn clear(&mut self) {
        self.value = None;
    }
}

fn main() {
    let x = 2;
    // 闭包的简略定义，会自动根据上下文获取类型
    // 可以获取到作用域内的变量
    let sum = |y| x+y;

    println!("{}", sum(2));

    let mut cc1 = Cacher::new(|a: &str| a.clone());
    // 如果闭包的生命周期可能大于被捕获变量的生命周期，可以使用move强制获取被捕获变量的所有权
    let mut cc2 = Cacher::new(move |a: u32| a + x);

    println!("{}", cc1.value("use case"));
    println!("{}", cc2.value(88));
    cc2.clear();
    // 由于闭包实现了Copy特性，因此虽然被声明为FnOnce，这里还是可以再次调用
    println!("{}", cc2.value(38));

    let mut s = String::new();
    // 捕获的是环境中的可变借用变量
    let mut try_mut_fn = |str| s.push_str(str);
    try_mut_fn("Hello ");
    update_str(try_mut_fn);

    println!("{:?}", s);

    let s2 = "imut word".to_string();
    // 捕获的是不可变借用变量，声明形式为Fn
    let try_fn = |str: &str| println!("{:?}{:?}", str, s2);
    try_fn("origin fn");

    let s3 = "Hello,Word".to_string();
    // 在这个闭包中使用了s3，但是仅作为不可变借用来使用
    // Fn <- FnMut <- FnOnce 三种闭包类型的层级关系层层递进，Fn类型的闭包同时也实现了前两种类型
    // 因此下面三个类型的闭包声明都符合此闭包的条件
    // 闭包的类型仅仅与如何使用捕获变量有关，与捕获变量的传递方式无关
    let test_fn = || println!("{:?}", s3);
    exec(test_fn);
    exec2(test_fn);
    exec3(test_fn);

    factory(1);
    factory(7);
}

// 声明捕获了可变借用变量作为参数
fn update_str<'a, F: FnMut(&'a str)>(mut f: F) {
    f("add new word")
}

fn exec3<F: FnOnce()>(mut f: F) { f() }
fn exec2<F: FnMut()>(mut f: F) { f() }
fn exec<F: Fn()>(mut f: F) { f() }

// 闭包作为函数返回值
// 闭包声明的大小是不固定的，函数的参数和返回值必须有固定的大小
// 闭包之间即使签名相同，也属于不同的类型
// 使用特征对象返回此类值
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    if x > 1 {
        Box::new(move |x| x + 5)
    } else {
        Box::new(move |x| x - 5)
    }
}