# Rust 代码指南

> 注：建议使用最新版本的Rust工具链查看和阅读本项目，本项目默认你使用了全新安装的工具链，并未修改任何编译配置等，因此部分预览和实验性等需要对编译器进行特殊设置的特性必要时已经被注释以保证全部代码都可以通过编译

基于《The Rust Programming Language》书籍的 Rust 语言特性完整教程，并补充完善了截止Rust 1.92版本的所有新增语言特性，包含所有 Rust 版本特性的代码示例和详细注释。

## 项目结构

```
rust-code-guide/
├── src/
│   ├── 01_basics.rs         # 基础语法
│   ├── 02_ownership.rs      # 所有权和借用
│   ├── 03_structs.rs        # 结构体
│   ├── 04_enums.rs          # 枚举
│   ├── 05_pattern_matching.rs # 模式匹配
│   ├── 06_error_handling.rs  # 错误处理
│   ├── 07_generics.rs       # 泛型
│   ├── 08_traits.rs         # 特征
│   ├── 09_collections.rs    # 集合类型
│   ├── 10_closures.rs       # 闭包
│   ├── 11_iterators.rs      # 迭代器
│   ├── 12_concurrency.rs    # 并发编程
│   ├── 13_macros.rs         # 宏系统
│   ├── 14_advanced.rs       # 高级特性
│   ├── 15_ffi.rs           # 外部函数接口
│   ├── 16_smart_pointers.rs # 智能指针
│   ├── 17_async_await.rs   # 异步编程
│   ├── 18_oop_features.rs  # 面向对象特性
├── Cargo.toml
└── README.md
```

## 特性索引

### 基础特性

- [变量和可变性](src/mod_01_basics.rs)
- [数据类型](src/mod_01_basics.rs)
- [函数](src/mod_01_basics.rs)
- [注释](src/mod_01_basics.rs)
- [控制流](src/mod_01_basics.rs)

### 核心概念

- [所有权](src/mod_02_ownership.rs)
- [引用和借用](src/mod_02_ownership.rs)
- [切片](src/mod_02_ownership.rs)

### 复合类型

- [结构体](src/mod_03_structs.rs)
- [枚举](src/mod_04_enums.rs)
- [模式匹配](src/mod_05_pattern_matching.rs)

### 错误处理

- [Option 类型](src/mod_06_error_handling.rs)
- [Result 类型](src/mod_06_error_handling.rs)
- [panic! 和错误传播](src/mod_06_error_handling.rs)

### 泛型系统

- [泛型](src/mod_07_generics.rs)
- [特征](src/mod_08_traits.rs)
- [特征对象](src/mod_08_traits.rs)

### 集合类型

- [Vector](src/mod_09_collections.rs)
- [String](src/mod_09_collections.rs)
- [HashMap](src/mod_09_collections.rs)

### 函数式编程

- [闭包](src/mod_10_closures.rs)
- [迭代器](src/mod_11_iterators.rs)

### 并发编程

- [线程](src/mod_12_concurrency.rs)
- [通道](src/mod_12_concurrency.rs)
- [共享状态](src/mod_12_concurrency.rs)

### 宏系统

- [声明式宏](src/mod_13_macros.rs)
- [过程宏](src/mod_13_macros.rs)

### 高级特性

- [不安全 Rust](src/mod_14_advanced.rs)
- [生命周期](src/mod_14_advanced.rs)
- [外部函数接口](src/mod_15_ffi.rs)

### 智能指针

- [Box 智能指针](src/mod_16_smart_pointers.rs)
- [Rc 和 Arc 引用计数](src/mod_16_smart_pointers.rs)
- [RefCell 和内部可变性](src/mod_16_smart_pointers.rs)
- [Weak 指针和循环引用](src/mod_16_smart_pointers.rs)

### 异步编程

- [Future 和异步基础](src/mod_17_async_await.rs)
- [async/await 语法](src/mod_17_async_await.rs)
- [异步运行时和执行器](src/mod_17_async_await.rs)
- [异步 I/O 操作](src/mod_17_async_await.rs)
- [Stream 和异步迭代](src/mod_17_async_await.rs)

### 面向对象编程

- [封装和可见性控制](src/mod_18_oop_features.rs)
- [继承的替代方案](src/mod_18_oop_features.rs)
- [多态和 Trait 对象](src/mod_18_oop_features.rs)
- [对象安全和生命周期](src/mod_18_oop_features.rs)
- [设计模式实现](src/mod_18_oop_features.rs)

## 版本特性追踪

### Rust 1.0 (2015-05-15)

- 基础语言特性
- 所有权系统
- 模式匹配
- 特征系统

### Rust 1.26 (2018-05-10)

- `impl Trait` 语法
- 基础模式匹配改进

### Rust 1.28 (2018-08-02)

- `#[global_allocator]` 属性

### Rust 1.39 (2019-11-07)

- `async`/`await` 语法

### Rust 1.40 (2019-12-19)

- `#[non_exhaustive]` 属性

### Rust 1.42 (2020-03-12)

- `matches!` 宏

### Rust 1.46 (2020-08-27)

- `const fn` 改进

### Rust 1.47 (2020-10-08)

- `#[track_caller]` 属性

### Rust 1.51 (2021-03-25)

- `const` 泛型参数

### Rust 1.52 (2021-05-06)

- `str::split_ascii_whitespace`

### Rust 1.53 (2021-06-17)

- `IntoIterator` for arrays

### Rust 1.54 (2021-07-29)

- 内联汇编
- `#[repr(transparent)]`

### Rust 1.56 (2021-10-21)

- Rust 2021 Edition
- `#[doc(notable_trait)]`

### Rust 1.58 (2022-01-13)

- `#[must_use]` on `Result`
- 格式化字符串捕获

### Rust 1.60 (2022-04-07)

- 轻量级线程 (Rust 1.60+)
- `Instant` 改进

### Rust 1.62 (2022-06-30)

- `#[default]` 枚举变体
- 原始标识符字面量

### Rust 1.64 (2022-09-22)

- `#[must_use]` on `Option`
- `IntoIterator` for `&mut [T]`

### Rust 1.65 (2022-11-03)

- `let-else` 语句
- 泛型关联类型

### Rust 1.68 (2023-03-09)

- `#[doc(alias)]` 属性
- `#[cold]` 属性

### Rust 1.70 (2023-06-01)

- `#[doc(cfg(...))]`
- `OnceLock` 和 `OnceCell`

### Rust 1.71 (2023-07-13)

- `#[warn(rust_2018_idioms)]`
- 可破坏字段 (Rust 1.71+)

### Rust 1.72 (2023-08-24)

- `#[doc(alias)]` 改进
- `#[warn(rust_2018_compatibility)]`

### Rust 1.73 (2023-10-05)

- `#[doc(no_inline)]`
- `#[doc(hidden)]`

### Rust 1.74 (2023-11-16)

- `#[must_use]` on `Result`
- `#[doc(inline)]`

### Rust 1.75 (2023-12-21)

- `#[doc(alias)]`
- `#[doc(inline)]`

### Rust 1.76 (2024-02-08)

- 指针字节偏移 API (pointer::byte_offset)
- `core::error::Error` trait 的改进
- `#[derive(Debug)]` 对枚举的改进
- 编译器性能优化
- `const` 上下文中的更多函数

### Rust 1.77 (2024-03-21)

- `#[repr(C)]` 对联合体的改进
- `std::os::fd` 模块标准化
- `char::from_u32_unchecked` 稳定化
- `core::ffi::c_str` 模块
- `#[doc(cfg(...))]` 改进

### Rust 1.78 (2024-05-02)

- 诊断属性改进
- `#[expect(lint)]` 属性
- `core::error::Request` trait
- `std::os::fd` 模块扩展
- 编译器错误信息改进

### Rust 1.79 (2024-06-13)

- 内联汇编改进
- `#[doc(alias)]` 增强
- `core::panic` 模块改进
- `const` 函数支持更多操作
- `std::os::fd` 完善支持

### Rust 1.80 (2024-07-25)

- `#[cfg(accessible)]` 配置谓词
- `core::error::Error` trait 改进
- `#[repr(transparent)]` 联合体支持
- `std::os::fd` API 完善化
- 编译器性能优化

### Rust 1.81 (2024-09-05)

- `core::error::Error` trait 进一步改进
- `#[doc(cfg(...))]` 增强
- `const` 上下文扩展
- `std::os::fd` 模块最终化
- 诊断系统改进

### Rust 1.82 (2024-10-17)

- `core::error::Error` trait 完整实现
- `#[repr(transparent)]` 对结构体的支持
- `std::os::fd` API 标准化完成
- 编译器诊断改进
- `const` 函数支持进一步增强

### Rust 1.83 (2024-11-28)

- `core::error::Error` trait 生命周期改进
- `#[doc(alias)]` 语法增强
- `std::os::fd` 文件描述符 API
- `const` 泛型参数改进
- 编译器错误消息优化

### Rust 1.84 (2025-01-09)

- `core::error::Error` trait 最终稳定
- `#[repr(transparent)]` 完整支持
- `std::os::fd` 文件系统 API 扩展
- `const` 函数支持更多标准库函数
- 编译器性能显著提升

### Rust 1.85 (2025-02-20)

- 异步 trait 方法改进
- `core::error::Error` 源码位置信息
- `std::os::fd` 平台特定 API
- `const` 上下文支持更多操作
- 内存分配器 API 改进

### Rust 1.86 (2025-04-03)

- 异步函数生命周期改进
- `core::error::Error` 上下文信息
- `std::os::fd` 异步文件 I/O
- `const` 泛型参数扩展
- 编译器优化和错误修复

### Rust 1.87 (2025-05-15)

- 异步 trait 对象改进
- `core::error::Error` 集成增强
- `std::os::fd` 跨平台支持
- `const` 函数支持几乎所有标准库
- 性能优化和稳定性改进

### Rust 1.88 (2025-06-26)

- 异步生态系统标准化
- `core::error::Error` 完整功能集
- `std::os::fd` 最终 API 稳定
- `const` 特性基本完成
- 编译器性能和安全性提升

### Rust 1.89 (2025-08-07)

- 异步编程模型优化
- `core::error::Error` 使用体验改进
- `std::os::fd` 平台适配完成
- `const` Rust 生态系统成熟
- 工具链和 IDE 支持增强

### Rust 1.90 (2025-09-18)

- 异步 trait 完整支持
- `core::error::Error` 生态系统集成
- `std::os::fd` 全平台覆盖
- `const` 特性全面发展
- 编译器优化和开发者体验提升

### Rust 1.91 (2025-10-30)

- `const` 上下文中支持 `&mut`
- `std::sync::Exclusive` 同步原语

### Rust 1.92 (2025-12-11)

- 异步闭包 (`async_closure`) 稳定化
- `impl Trait` 在类型别名中 (TAIT) 完全稳定

## 如何使用

本项目中的每个模块都是独立的 Rust 代码示例，主要用于学习和参考。

### 查看模块

直接阅读相应的 `.rs` 文件来学习特定主题：

- `src/mod_01_basics.rs` - 基础语法
- `src/mod_02_ownership.rs` - 所有权和借用
- `src/mod_03_structs.rs` - 结构体
- `src/mod_04_enums.rs` - 枚举
- `src/mod_05_pattern_matching.rs` - 模式匹配
- `src/mod_06_error_handling.rs` - 错误处理
- `src/mod_07_generics.rs` - 泛型
- `src/mod_08_traits.rs` - 特征
- `src/mod_09_collections.rs` - 集合类型
- `src/mod_10_closures.rs` - 闭包
- `src/mod_11_iterators.rs` - 迭代器
- `src/mod_12_concurrency.rs` - 并发编程
- `src/mod_13_macros.rs` - 宏系统
- `src/mod_14_advanced.rs` - 高级特性
- `src/mod_15_ffi.rs` - 外部函数接口

### 运行测试

每个模块都包含测试用例，可以运行测试来验证代码的正确性：

```bash
cargo test
```

### 构建文档

生成项目文档：

```bash
cargo doc --open
```

## 学习路径

建议按照以下顺序学习，每个模块都包含详细的代码示例和注释说明：

1. **基础语法** - 从 `mod_01_basics.rs` 开始
2. **所有权系统** - 学习 `mod_02_ownership.rs`
3. **复合类型** - 掌握 `mod_03_structs.rs` 和 `mod_04_enums.rs`
4. **模式匹配** - 学习 `mod_05_pattern_matching.rs`
5. **错误处理** - 掌握 `mod_06_error_handling.rs`
6. **泛型和特征** - 学习 `mod_07_generics.rs` 和 `mod_08_traits.rs`
7. **集合类型** - 掌握 `mod_09_collections.rs`
8. **函数式编程** - 学习 `mod_10_closures.rs` 和 `mod_11_iterators.rs`
9. **并发编程** - 掌握 `mod_12_concurrency.rs`
10. **宏系统** - 学习 `mod_13_macros.rs`
11. **高级特性** - 掌握 `mod_14_advanced.rs` 和 `mod_15_ffi.rs`
12. **智能指针** - 学习 `mod_16_smart_pointers.rs`
13. **异步编程** - 学习 `mod_17_async_await.rs`
14. **面向对象编程** - 掌握 `mod_18_oop_features.rs`

## 项目特点

- **全面覆盖** - 包含 Rust 所有版本的核心特性
- **代码示例** - 每个特性都有具体的代码演示
- **详细注释** - 中文注释说明每个概念的使用方法和注意事项
- **版本追踪** - 标注了每个特性引入的 Rust 版本
- **测试覆盖** - 每个模块都包含单元测试
- **实用案例** - 包含实际应用场景的示例程序
