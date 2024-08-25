// fn test_1() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };
// }
fn test_1() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x
    };
}

// 使用两种方法让代码工作起来
// fn test_2() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };

//     assert_eq!(v, 3);
// }
fn test_2() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}


// fn test_3() {
//     let v = (let x = 3);
 
//     assert!(v == 3);
//  }
fn test_3() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
 }


// fn test_4() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y;
// }


fn test_4() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
}
