# AGENTS.md — rust-code-guide

## 仓库定位

基于《The Rust Programming Language》的 **Rust 教程代码库**（中文），面向有编程经验但 Rust 不熟练的读者。覆盖全部语言主题，并追踪到最新 stable 版本（当前 1.97）的所有新特性。

## 结构

- `src/mod_01_basics.rs` ~ `mod_18_oop_features.rs`：18 个主题模块，每个模块是若干编号小节 + 大量中文注释的函数 + `main()` + `#[cfg(test)]` 测试段。
- `src/lib.rs`：声明所有模块；`src/main.rs`：依次调用各模块 `main()`。
- `README.md`：特性索引 + "版本特性追踪"（每个版本 3-5 条**真实**亮点，附发布日期）。
- `docs/project.md`：项目简介（版本号需同步更新）。

### 版本特性"就近挂靠"约定

新版本特性不单独建文件，而是插入语义最相关的主题模块末尾作为新小节：

| 特性类别 | 挂靠模块 |
|---|---|
| 数值/转换/控制流 | mod_01_basics |
| 模式匹配/断言宏 | mod_05_pattern_matching |
| 集合/序列 API | mod_09_collections |
| 迭代器/范围 | mod_11_iterators |
| 并发原语/锁/原子 | mod_12_concurrency |
| 宏 | mod_13_macros |
| 底层/unsafe/布局/const | mod_14_advanced |
| FFI/C 互操作 | mod_15_ffi |
| 智能指针/内部可变性 | mod_16_smart_pointers |
| 异步 | mod_17_async_await |

新小节格式：`// N. Rust X.YY <标题>` 分隔横幅 + 若干"为什么"注释 + 函数；必须**在模块 `main()` 中注册调用**，并在测试段补一个 `#[test]`。

## 硬性约定

1. **面向新手**：所有代码必须有详尽中文注释，解释"为什么"而非复述"是什么"。
2. **版本标注必须与官方发布说明一致**——禁止凭记忆或推测写版本号和 API 名。历史教训：README 曾出现整段虚构的版本条目（如 1.76-1.90 的"core::error::Error 持续改进"、`cfg(accessible)` 已稳定、内联汇编 1.54 等），均已修正；再犯视为缺陷。
3. **未稳定特性不得写成已稳定**：如 `#[cfg(accessible)]`（至今 nightly-only）、BTreeMap/LinkedList 游标等。需要演示时以注释形式保留并明确标注"需要 nightly"。
4. 每个小节必须有测试，交付前 `cargo test` 全绿。
5. 不创建仓库中原来不存在的文档。

## 新增版本特性流程

1. **取官方发布说明**：`blog.rust-lang.org/YYYY/MM/DD/Rust-X.YY.0`，用 Jina 抓正文（`curl -sL "https://r.jina.ai/blog.rust-lang.org/..."`，比 WebFetch 稳定）。
2. **先探针后写作**：新 API 先在 `/tmp` 建最小 cargo 工程编译验证真实签名（方法、参数、返回值、是否 Copy/const、哪个版本 stable），**确认后再写进教程**。可直接读本地工具链源码核对：`$(rustc --print sysroot)/lib/rustlib/src/rust/library/...`（需 rust-src 组件）。
3. 按上表挂靠新小节 → 注册到模块 `main()` → 补测试。
4. 更新 `README.md` 版本特性追踪段（从官方说明提取真实亮点，含发布日期）。
5. 验证：`cargo build && cargo test && cargo clippy --all-targets`（clippy 必须 0 error）。

## 已知坑（写代码前先看）

- clippy 的 `approx_constant` 是 **deny 级别**：教程里不要手写 3.14/2.71/1.414 之类字面量，改用 `std::f64::consts::*` 或普通小数。
- `core::range` 新范围类型用 `From<legacy::Range<T>>`（`(1..3).into()`）构造，**没有 `new()`**；`RangeInclusive` 的 `start`/`last` 是公开字段。
- `catch_unwind` 只接受 1 个参数；`AssertUnwindSafe::from` 要求 `T: UnwindSafe`（`&mut` 不行，需手动包闭包）。
- `Cell<[T; N]>` 同时实现两个 `AsRef`，调用需完整限定语法 `<Cell<[T; N]> as AsRef<[Cell<T>]>>::as_ref(&x)`。
- `NonZeroU32::bit_width()` 返回 `NonZeroU8`，普通 `u32::bit_width()` 返回 `u32`；`highest_one()`/`lowest_one()` 返回 `Option`，NonZero 版本直接返回 `usize`。
- `assert_matches!` **不在 prelude**，需要 `use core::assert_matches;`。
- `LazyLock` 在 `std::sync`，`LazyCell` 在 `std::cell`；`From<T>` 构造需要标注闭包类型 `fn() -> T`。
- 模块文件顶部有 `#![allow(...)]` 集合，新代码尽量不要新增 suppress。
