// 数值类型
// 整数
// 🌟
// Tips: 如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型
// 移除某个部分让代码工作
// fn test_1() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;
//     y = x;
//     let z = 10; // 这里 z 的类型是?
// }
fn test_1() {
    let x: i32 = 5;
    // let mut y: u32 = 5;
    let mut y = 5;

    y = x;
    
    // Rust 整型默认使用 i32
    let _z = 10; // 这里 z 的类型是? i32
}


// 填空
fn test_2() {
    let v: u16 = 38_u8 as u16;
}

// 🌟🌟🌟
// Tips: 如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型

// 修改 `assert_eq!` 让代码工作
// fn test_3() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));
// }
fn test_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// 填空，让代码工作
// fn test_4() {
//     assert_eq!(i8::MAX, __); 
//     assert_eq!(u8::MAX, __); 
// }
fn test_4() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
}


// 解决代码中的错误和 `panic`
// fn test_5() {
//     let v1 = 251_u8 + 8;
//     let v2 = i8::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }

fn test_5() {
    // i8: -128 ~ 127
    // u8: 0 ~ 255
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("test_5: {},{}",v1,v2);
}

// fn test_6() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
// }
fn test_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    let a = 1_024;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_1111;
    println!("test_6: v: {}, a,b,c,d: {}, {}, {}, {}", v, a,b,c,d);
    // test_6: v: 1597, a,b,c,d: 1024, 255, 63, 255
    assert!(v == 1597);
}

// fn test_7() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }
fn test_7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("test_7 x, y, z: {}, {}, {}", x, y, z);
    // test_7 x, y, z: 1000.0001, 0.12, 0.01
}

fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();
    test_7();
}
