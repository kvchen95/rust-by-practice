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
    let z = 10; // 这里 z 的类型是? i32
}
fn main() {
    test_1();
}
