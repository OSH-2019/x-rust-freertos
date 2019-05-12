fn main() {
    let mut a = String::new();

    // NOTE: 将类似 #[cfg(include_1)] 的写法改为如下写法。
    #[cfg(feature = "include_1")]
    println!("include_1 is defined.");

    // 当#[cfg(...)]修饰一个赋值语句时，会出现error.
    #[cfg(feature = "include_1")]
    a = String::from("This will cause an error");

    // 修饰赋值语句的正确方式
    // 当#ifdef修饰了多行代码时，也应当这样处理。
    {
        // NOTE: 此时用的是#!而不再是#
        // #!修饰的是inner attribute，作用于其所在的整个块
        // #修饰的是outer attribue，仅作用于后面的语句
        #![cfg(feature = "include_1")]
        a = String::from("This is correct");
        println!("{}", a);
    }

    // NOTE: feature = "include_1" 和 feature = "include_2"可以同时成立
    #[cfg(all(feature = "include_1", feature = "include_2"))]
    println!("Both include_1 and include_2 is defined");
}
