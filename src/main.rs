use rust_code_guide::*;

fn main() {
    println!("Rust 代码教程库 - 主程序");
    println!("=======================");

    println!("运行各个教程模块的演示：");
    println!();

    // 运行基础教程
    println!("1. 基础语法教程：");
    mod_01_basics::main();
    println!();

    // 运行所有权教程
    println!("2. 所有权系统教程：");
    mod_02_ownership::main();
    println!();

    // 运行结构体教程
    println!("3. 结构体教程：");
    mod_03_structs::main();
    println!();

    // 运行枚举教程
    println!("4. 枚举教程：");
    mod_04_enums::main();
    println!();

    // 运行模式匹配教程
    println!("5. 模式匹配教程：");
    mod_05_pattern_matching::main();
    println!();

    // 运行错误处理教程
    println!("6. 错误处理教程：");
    mod_06_error_handling::main();
    println!();

    // 运行泛型教程
    println!("7. 泛型教程：");
    mod_07_generics::main();
    println!();

    // 运行特征教程
    println!("8. 特征教程：");
    mod_08_traits::main();
    println!();

    // 运行集合教程
    println!("9. 集合教程：");
    mod_09_collections::main();
    println!();

    // 运行闭包教程
    println!("10. 闭包教程：");
    mod_10_closures::main();
    println!();

    // 运行迭代器教程
    println!("11. 迭代器教程：");
    mod_11_iterators::main();
    println!();

    // 运行并发教程
    println!("12. 并发教程：");
    mod_12_concurrency::main();
    println!();

    // 运行宏教程
    println!("13. 宏教程：");
    mod_13_macros::main();
    println!();

    // 运行高级特性教程
    println!("14. 高级特性教程：");
    mod_14_advanced::main();
    println!();

    // 运行FFI教程
    println!("15. FFI教程：");
    mod_15_ffi::main();
    println!();

    // 运行智能指针教程
    println!("16. 智能指针教程：");
    mod_16_smart_pointers::main();
    println!();

    // 运行异步教程
    println!("17. 异步教程：");
    mod_17_async_await::main();
    println!();

    // 运行面向对象特性教程
    println!("18. 面向对象特性教程：");
    mod_18_oop_features::main();
    println!();

    println!("所有教程演示完成！");
}
