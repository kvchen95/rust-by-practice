// 字符
// 1. 
// 修改2处 `assert_eq!` 让代码工作

use std::mem::size_of_val;
// fn test_1() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),1); 

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),3); 

//     println!("Success!")
// }
fn test_1() {
    
    // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节
    let c1 = 'a';
    // assert_eq!(size_of_val(&c1),1); 
    assert_eq!(size_of_val(&c1), 4); 

    let c2 = '中';
    // assert_eq!(size_of_val(&c2),3);
    assert_eq!(size_of_val(&c2), 4); 

    println!("Success!")
} 


// 修改一行让代码正常打印
// fn test_2() {
//     let c1 = "中";
//     print_char(c1);
// } 

// fn print_char(c : char) {
//     println!("{}", c);
// }


fn test_2() {
    // let c1 = "中";
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}



// 使成功打印
// fn test_3() {
//     let _f: bool = false;

//     let t = true;
//     if !t {
//         println!("Success!")
//     }
// } 

fn test_3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
} 



// fn test_4() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, f);

//     println!("Success!")
// }


fn test_4() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!")
}


// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn test_5() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }

// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

fn test_5() {
    // let _v: () = ();
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

// 6
// 让代码工作：修改 `assert!` 中的 `4` 
// use std::mem::size_of_val;
// fn test_6() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 4);

//     println!("Success!")
// }
fn test_6() {
    // 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
    let unit: () = ();
    // assert!(size_of_val(&unit) == 4);
    assert!(size_of_val(&unit) == 0);
    println!("Success!")
}

fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
    test_5();
    test_6();
}