#[allow(unused_variables)]

fn main() {
    /**
     * 绑定和可变性
     */
    // 1. 🌟 变量只有在初始化后才能被使用
    
    // 修复下面代码的错误并尽可能少的修改
    // fn test_1() {
    //     let x: i32; // 未初始化，但被使用
    //     let y: i32; // 未初始化，也未被使用
    //     println!("x is equal to {}", x); 
    // }
    fn test_1() {
        let x: i32 = 5;
        let y: i32;
        println!("{} is equal to 5", x);
    }

    // 2. 🌟🌟 可以使用 mut 将变量标记为可变
    // fn test_2() {
        // let __ = 1;
        // __ += 2; 
        
        // println!("x = {}", x); 
    // }
    fn test_2() {
        let mut x = 1;
        x += 2;
        println!("x = {}", x)
    }

    /** 
     * 变量作用域
    */

    // 3. 🌟 作用域是一个变量在程序中能够保持合法的范围
    // 修复下面代码的错误并使用尽可能少的改变
    // fn test_3 () {
    //     let x: i32 = 10;
    //     {
    //         let y: i32 = 5;
    //         println!("x 的值是 {}, y 的值是 {}", x, y);
    //     }
    //     println!("x 的值是 {}, y 的值是 {}", x, y); 
    // }
    fn test_3() {
        let x: i32 = 10;
        let y: i32 = 0;
        {
            let y: i32 = 5;
            println!("x 的值是 {}, y 的值是 {}", x, y);
            
        }
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }


    // 修复错误
    // fn test_4() {
    //     println!("{}, world", x); 
    // }

    // fn define_x() {
    //     let x = "hello";
    // }
    // 修复错误
    fn test_4() {
        let x = define_x();
        println!("{}, world", x); 
    }

    fn define_x() -> String {
        let x = "hello".to_string();
        x
    }

    /** 
     * 变量遮蔽( Shadowing )
     */
    // 5. 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
    // // 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
    // fn test_5() {
    //     let x: i32 = 5;
    //     {
    //         let x = 12;
    //         assert_eq!(x, 5);
    //     }

    //     assert_eq!(x, 12);

    //     let x = 42;
    //     println!("{}", x); // 输出 "42".
    // }

    // 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
    fn test_5() {
        let x: i32 = 5;
        {
            let x = 12;
            // assert_eq!(x, 5);
            assert_eq!(x, 12);
        }
        // assert_eq!(x, 12);
        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // 输出 "42".
    }

    // 🌟🌟 修改一行代码以通过编译
    fn test_6() {
        let mut x: i32 = 1;
        x = 7;
        // 遮蔽且再次绑定
        let mut x = x;
        x += 3;


        let y = 4;
        // 遮蔽
        let y = "I can also be bound to text!"; 
    }

    /**
     * 未使用的变量
     */
    // 使用以下方法来修复编译器输出的 warning :
    // 🌟 一种方法
    // 🌟🌟 两种方法

    fn test_7() {
        let _x = 1;
    }
    // file head add:
    // #[allow(unused_variables)
    fn test_7_2() {
        let x = 1;
    }

    /**
     * 变量解构
     */
    // 🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
    // 提示: 可以使用变量遮蔽或可变性

    // 修复下面代码的错误并尽可能少的修改
    // fn test_8() {
    //     let (x, y) = (1, 2);
    //     x += 2;

    //     assert_eq!(x, 3);
    //     assert_eq!(y, 2);
    // }
    fn test_8() {
        let (mut x, y) = (1, 2);
        x += 2;
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }

    // 解构式赋值
    // 该功能于 Rust 1.59 版本引入：你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值。
    // 🌟🌟
    // Note: 解构式赋值只能在 Rust 1.59 或者更高版本中使用
    // fn test_9() {
    //     let (x, y);
    //     (x,..) = (3, 4);
    //     [.., y] = [1, 2];
    //     // 填空，让代码工作
    //     assert_eq!([x,y], __);
    // } 
    fn test_9() {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];
        // 填空，让代码工作
        assert_eq!([x,y], [3,2]);
    } 


    



 
    fn main() {
        test_1();
        // 5 is equal to 5
        test_2();
        // x = 3
        test_3();
        // x 的值是 10, y 的值是 5
        //x 的值是 10, y 的值是 0
        test_4();
        // hello, world
        test_5();
        // 42
        test_6();
        test_7();
        test_7_2();
        test_8();
        test_9();
    }



    main()
}
