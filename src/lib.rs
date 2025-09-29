// Rust 代码教程库
// 这个库包含了所有 Rust 核心特性的教程和示例

// 声明所有教程模块
pub mod mod_01_basics;
pub mod mod_02_ownership;
pub mod mod_03_structs;
pub mod mod_04_enums;
pub mod mod_05_pattern_matching;
pub mod mod_06_error_handling;
pub mod mod_07_generics;
pub mod mod_08_traits;
pub mod mod_09_collections;
pub mod mod_10_closures;
pub mod mod_11_iterators;
pub mod mod_12_concurrency;
pub mod mod_13_macros;
pub mod mod_14_advanced;
pub mod mod_15_ffi;
pub mod mod_16_smart_pointers;
pub mod mod_17_async_await;
pub mod mod_18_oop_features;

// 库级别的测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_all_modules() {
        // 这个测试确保所有模块都能正确编译
        // 实际的测试在各个模块中
        assert!(true);
    }
}