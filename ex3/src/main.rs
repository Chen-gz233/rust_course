fn mian() {
    let mut x = 34;     //可变变量
    println!("x = {}",x);
    x= 45;
    println!("x = {}",x);
    let _y = 56;        //你希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头：
}


fn main(){
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    // println!("a = {:?}, b = {:?}", a, b);
    println!("a = {}, b = {}", a, b);

    b = true;
    assert_eq!(a, b);
    mian();
}