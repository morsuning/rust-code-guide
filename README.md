# Rust 代码指南

> 注：建议使用最新版本的Rust工具链查看和阅读本项目，本项目默认你使用了全新安装的工具链，并未修改任何编译配置等，因此部分预览和实验性等需要对编译器进行特殊设置的特性必要时已经被注释以保证全部代码都可以通过编译

基于《The Rust Programming Language》书籍的 Rust 语言特性完整教程，并补充完善了截止 Rust 1.97 版本的所有新增语言特性，包含所有 Rust 版本特性的代码示例和详细注释。

建议有一定编程经验的读者阅读本指南，本指南不适合编程初学者（即Rust为打算学习的第一门编程语言）。

## 项目结构

```
rust-code-guide/
├── src/
│   ├── mod_01_basics.rs         # 基础语法
│   ├── mod_02_ownership.rs      # 所有权和借用
│   ├── mod_03_structs.rs        # 结构体
│   ├── mod_04_enums.rs          # 枚举
│   ├── mod_05_pattern_matching.rs  # 模式匹配
│   ├── mod_06_error_handling.rs  # 错误处理
│   ├── mod_07_generics.rs       # 泛型
│   ├── mod_08_traits.rs         # 特征
│   ├── mod_09_collections.rs    # 集合类型
│   ├── mod_10_closures.rs       # 闭包
│   ├── mod_11_iterators.rs      # 迭代器
│   ├── mod_12_concurrency.rs    # 并发编程
│   ├── mod_13_macros.rs         # 宏系统
│   ├── mod_14_advanced.rs       # 高级特性
│   ├── mod_15_ffi.rs           # 外部函数接口
│   ├── mod_16_smart_pointers.rs # 智能指针
│   ├── mod_17_async_await.rs   # 异步编程
│   ├── mod_18_oop_features.rs  # 面向对象特性
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
- [RwLock 降级与原子更新 API](src/mod_12_concurrency.rs)

### 宏系统

- [声明式宏](src/mod_13_macros.rs)
- [过程宏](src/mod_13_macros.rs)

### 高级特性

- [不安全 Rust](src/mod_14_advanced.rs)
- [生命周期](src/mod_14_advanced.rs)
- [外部函数接口](src/mod_15_ffi.rs)
- [底层能力补充](src/mod_14_advanced.rs)

### 智能指针

- [Box 智能指针](src/mod_16_smart_pointers.rs)
- [Rc 和 Arc 引用计数](src/mod_16_smart_pointers.rs)
- [RefCell 和内部可变性](src/mod_16_smart_pointers.rs)
- [Weak 指针和循环引用](src/mod_16_smart_pointers.rs)
- [零初始化智能指针分配](src/mod_16_smart_pointers.rs)

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

> 以下内容均依据官方发布说明（blog.rust-lang.org）核实整理，只收录真实稳定特性。

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

- `const fn` 能力增强

### Rust 1.47 (2020-10-08)

- `#[track_caller]` 属性

### Rust 1.51 (2021-03-25)

- `const` 泛型参数

### Rust 1.52 (2021-05-06)

- `str::split_ascii_whitespace`

### Rust 1.53 (2021-06-17)

- 数组实现 `IntoIterator`
- Unicode 标识符
- 模式中的 or-patterns（`pat1 | pat2`）

### Rust 1.54 (2021-07-29)

- 内置属性（如 `#[cfg]`）的值支持宏
- `#[target_feature]` 允许用于部分平台的安全函数
- `wasm` 成为 `target_family`

### Rust 1.55 (2021-09-09)

- 开放范围模式（`X..`）
- Eisel-Lemire 浮点解析算法

### Rust 1.56 (2021-10-21)

- Rust 2021 Edition
- Cargo `rust-version` 字段（MSRV 校验）
- `binding @ pattern` 中允许新增绑定

### Rust 1.57 (2021-12-02)

- `panic!` 可在 const 上下文使用
- Cargo 自定义 profile

### Rust 1.58 (2022-01-13)

- 格式化字符串捕获（`{ident}`）
- 标准库全面补充 `#[must_use]` 注解

### Rust 1.59 (2022-02-24)

- 内联汇编（inline assembly）
- 解构赋值（destructuring assignments）
- 泛型 const 参数默认值

### Rust 1.60 (2022-04-07)

- 基于源码的代码覆盖率（`-C instrument-coverage`）
- Cargo feature 新语法（`dep:` 前缀、弱依赖特性）
- `Instant` 单调性保证（饱和到 0 而非 panic）

### Rust 1.61 (2022-05-19)

- `main` 支持自定义退出码（`ExitCode`）
- const fn 能力扩展

### Rust 1.62 (2022-06-30)

- `cargo add` 命令
- `#[default]` 枚举变体
- Linux 上更小更快的 Mutex（futex 实现）

### Rust 1.63 (2022-08-11)

- 作用域线程（scoped threads，`std::thread::scope`）
- I/O 安全（原始文件描述符所有权）
- const 上下文中的 `Mutex` / `RwLock` / `Condvar` 初始化

### Rust 1.64 (2022-09-22)

- `IntoFuture` trait（`.await` 可作用于任意可转换类型）
- `core::ffi` / `alloc::ffi` 提供 C 兼容类型
- Cargo 工作区字段继承

### Rust 1.65 (2022-11-03)

- 泛型关联类型（GATs）
- `let-else` 语句
- `break` 跳出带标签的块

### Rust 1.66 (2022-12-15)

- 带字段枚举的显式判别值
- `core::hint::black_box`
- `cargo remove`

### Rust 1.67 (2023-01-26)

- `#[must_use]` 对 `async fn` 生效
- `std::sync::mpsc` 实现更新

### Rust 1.68 (2023-03-09)

- Cargo 稀疏注册表协议（sparse protocol）
- `pin!` 宏（局部 Pin 构造）
- 默认分配错误处理（`default_alloc_error_handler`）

### Rust 1.69 (2023-04-20)

- Cargo 自动修复部分警告
- 构建脚本默认不再包含调试信息

### Rust 1.70 (2023-06-01)

- `OnceCell` 和 `OnceLock`
- `IsTerminal` trait
- crates.io 默认使用稀疏协议

### Rust 1.71 (2023-07-13)

- C-unwind ABI
- `#[debug_visualizer]` 调试器可视化属性
- Windows raw-dylib 链接

### Rust 1.72 (2023-08-24)

- 错误中报告可能有用的 `cfg` 禁用项
- const 求值时间不再受限
- 从 Clippy 提升的 lints

### Rust 1.73 (2023-10-05)

- thread_local 直接操作 API（`LocalKey<Cell<T>>::get/set` 等）
- panic 消息输出改进

### Rust 1.74 (2023-11-16)

- Cargo `[lints]` 配置表（RFC 3389）
- Cargo registry 认证
- 不透明返回类型中的投影（`Self` 与关联类型）

### Rust 1.75 (2023-12-28)

- `async fn` 与 return-position `impl Trait` 在 trait 中稳定（RPITIT）
- 原始指针字节偏移 API（`byte_add` / `byte_offset` 等）
- `Atomic*::from_ptr`

### Rust 1.76 (2024-02-08)

- `std::any::type_name_of_val`
- 官方保证 `char` 与 `u32` ABI 兼容
- `Arc::unwrap_or_clone` / `Rc::unwrap_or_clone`
- `Result::inspect` / `Option::inspect`

### Rust 1.77 (2024-03-21)

- C 字符串字面量（`c"..."`）
- `offset_of!` 宏
- `async fn` 递归调用（需间接方式）
- `File::create_new`（原子创建新文件）

### Rust 1.78 (2024-05-02)

- `#[diagnostic]` 命名空间与 `#[diagnostic::on_unimplemented]`
- `#[cfg(target_abi = ...)]`
- unsafe 前置条件断言改进

### Rust 1.79 (2024-06-13)

- 内联 `const {}` 表达式
- 关联类型位置上的约束（`T: Trait<Assoc: Bound>`）
- `num::NonZero<T>` 类型别名

### Rust 1.80 (2024-07-25)

- `LazyCell` 和 `LazyLock`
- 受检的 `cfg` 名称和值（`unexpected_cfgs` lint）
- 模式中的排他范围（exclusive ranges）

### Rust 1.81 (2024-09-05)

- `core::error::Error`（no_std 可用的错误抽象）
- `#[expect(lint)]` 属性与 lint reasons
- 标准库排序算法重写
- `extern "C"` 函数中未捕获 panic 改为 abort

### Rust 1.82 (2024-10-17)

- `+ use<..>` 精确捕获（precise capturing）
- `&raw const` / `&raw mut` 原生裸指针语法
- `unsafe extern` 块与 unsafe 属性
- `cargo info`

### Rust 1.83 (2024-11-28)

- const 上下文中的 `&mut` / `*mut` / `&Cell` / `*const Cell`
- 原始生命周期和标签（`'r#ident`）
- `Waker::new`（自定义 Waker 构造）

### Rust 1.84 (2025-01-09)

- 下一代 trait solver 用于一致性检查
- 严格来源（strict provenance）API
- `<int>::isqrt` / `checked_isqrt`（整数平方根）

### Rust 1.85 (2025-02-20)

- Rust 2024 Edition
- 异步闭包（async closures，`AsyncFn` trait 族）
- `#[diagnostic::do_not_recommend]`

### Rust 1.86 (2025-04-03)

- trait 对象向上转型（upcasting）
- 安全函数标记 `#[target_feature]`
- `slice::get_disjoint_mut` / `HashMap::get_disjoint_mut`
- `missing_abi` lint 默认警告

### Rust 1.87 (2025-05-15)

- 匿名管道（`std::io::pipe`）
- `asm!` 跳转到 Rust 代码（`asm_goto`）
- trait 定义中的精确捕获（`+ use<...>`）

### Rust 1.88 (2025-06-26)

- Let chains（`if let a && let b`）
- 裸函数（naked functions）
- 布尔 cfg 字面量（`cfg(true)`）
- Cargo 自动清理缓存

### Rust 1.89 (2025-08-07)

- 显式推断的 const 泛型参数（`[false; _]`）
- `i128` / `u128` 可用于 `extern "C"` 函数
- `File::lock` 等文件锁 API

### Rust 1.90 (2025-09-18)

- Linux 默认使用 LLD 链接器
- Cargo 原生工作区发布（`cargo publish --workspace`）
- 有符号整数饱和减法系列

### Rust 1.91 (2025-10-30)

- Cargo `build.build-dir` 稳定
- `sysv64` / `win64` / `efiapi` / `aapcs` ABI 变参函数
- `Path::file_prefix`、`AtomicPtr` 指针运算
- `dangling_pointers_from_locals` lint

### Rust 1.92 (2025-12-11)

- 安全代码中允许对 `union` 字段取原始指针（`&raw const`）
- `RwLockWriteGuard::downgrade`
- `BTreeMap::Entry::insert_entry`
- `Box` / `Rc` / `Arc::new_zeroed`
- `TokenStream` 支持 `Extend<Ident/Literal/Punct/Group>`

### Rust 1.93 (2026-01-22)

- `asm!` 内部支持 `#[cfg]`（`asm_cfg`）
- `extern "system"` 支持 C 风格变参
- `Vec::into_raw_parts` / `String::into_raw_parts`
- `VecDeque::pop_front_if` / `pop_back_if`
- `fmt::from_fn`

### Rust 1.94 (2026-03-05)

- `slice::array_windows`
- `Peekable::next_if_map` / `next_if_map_mut`
- `LazyCell` / `LazyLock` 新增 `get` / `get_mut` / `force_mut`
- Unicode 17 更新

### Rust 1.95 (2026-04-16)

- `if let` guards（match 分支守卫中的模式匹配）
- `cfg_select!` 宏
- `Vec::push_mut` / `insert_mut`、`VecDeque` 与 `LinkedList` 的 `*_mut` API
- 原子类型的 `update` / `try_update`
- `std::hint::cold_path`
- `MaybeUninit<[T; N]>` 与 `[MaybeUninit<T>; N]` 互转、`Cell<[T; N]>` 的 `AsRef`
- `bool: TryFrom<{integer}>`
- 裸指针 `as_ref_unchecked` / `as_mut_unchecked`
- `Layout::repeat` / `repeat_packed` / `extend_packed` / `dangling_ptr`
- `core::range::RangeInclusive`（可 Copy 范围类型的首个成员）
- `use` 中重命名路径段关键字

### Rust 1.96 (2026-05-28)

- `core::range` 可 Copy 范围类型补齐：`Range` / `RangeFrom` / `RangeToInclusive`
- `assert_matches!` / `debug_assert_matches!`（需手动引入，不在 prelude）
- `From<T> for AssertUnwindSafe<T>`、`From<T> for LazyCell/LazyLock<T, F>`
- WebAssembly 目标默认不再传 `--allow-undefined`
- Cargo 修复 CVE-2026-5222/5223（第三方 registry 场景）

### Rust 1.97 (2026-07-09)

- 符号修饰（symbol mangling）v0 方案默认启用
- Cargo `build.warnings` 配置（`CARGO_BUILD_WARNINGS=deny`）
- 链接器输出默认不再隐藏（`linker_messages` lint）
- 整数位操作：`highest_one` / `lowest_one` / `isolate_highest_one` / `isolate_lowest_one` / `bit_width`（含 NonZero 版本）
- `Default for RepeatN`、`Copy for FromBytesUntilNulError`
- `char::is_control` 可在 const 中使用

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
- `src/mod_16_smart_pointers.rs` - 智能指针
- `src/mod_17_async_await.rs` - 异步编程
- `src/mod_18_oop_features.rs` - 面向对象特性

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
