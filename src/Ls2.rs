fn main() {
    // 在函数中移动所有权的例子

    let s1 = give_owner();

    let mut s2 = String::from("old one");

    // 使用引用借出s2，可以声明多个不可变引用，可以查看其值，但不可修改，且在其最后一次使用之前无法再借出可变引用
    let s2imut1: &String = &s2;
    let s2imut2: &String = &s2;
    let len = get_len(&s2);
    println!("s2:{},{}", *s2imut1, *s2imut2);
    // 这里之后可以借出可变引用了
    let s2mut: &mut String = &mut s2;
    change(s2mut);
    println!("s2:{}", s2);
    println!("s2 lenght:{}", len);

    // 在这里s2的所有权已被移走，s2已经失效，函数的返回值所有权被转移给s3
    let s3 = take_and_give(s2);

    println!("s1:{}", s1);
    println!("s3:{}", s3);
}

fn give_owner()-> String {
    String::from("the new string")
}

fn take_and_give(other_str: String)-> String {
    other_str
}

fn get_len(str: &String)-> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str(", but this is new");
}