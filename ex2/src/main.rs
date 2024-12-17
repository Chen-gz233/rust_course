
// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
fn main() {
     // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 32;
    let b:i32 = 20;// 主动指定b的类型为i32

     // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c= 34i32;
    let d = 25_i32;
    let e = add(add(a,b),add(c,d));
    println!("a+b+c+d = {}",e);
}

fn add (a:i32,b:i32)->i32{
    return a+b;
}