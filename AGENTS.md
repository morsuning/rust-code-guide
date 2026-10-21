# AGENTS.md — rust-code-guide

## 仓库定位

基于《The Rust Programming Language》的 **Rust 教程代码库**（中文），面向有编程经验但 Rust 不熟练的读者。覆盖全部语言主题，并追踪到最新 stable 版本（当前 1.99）的所有新特性。

## 结构

- `src/mod_01_basics.rs` ~ `mod_19_advanced_reference.rs`：19 个模块（前 18 个按主题，第 19 个是“进阶参考”：低频、偏底层的特性），每个模块是若干编号小节 + 大量中文注释的函数 + `main()` + `#[cfg(test)]` 测试段。
- `src/lib.rs`：声明所有模块；`src/main.rs`：依次调用各模块 `main()`。
- `README.md`：特性索引 + "版本特性追踪"（每个版本 3-5 条**真实**亮点，附发布日期）。
- `docs/project.md`：项目简介（版本号需同步更新）。

### 版本特性"就近挂靠"约定

新版本特性不按版本建文件，而是插入语义最相关的模块末尾作为新小节，重点是特性 + 标注版本号，而不是先写版本号按版本介绍特性。
判断标准：普通业务代码几乎碰不到、多数需要 `unsafe` 或只在特定平台有意义的低频特性（裸指针与布局、ABI、内联汇编、分配器、工具链配置），统一放进 `mod_19_advanced_reference`，避免让新手误以为是必修内容；其余按下表就近挂靠：

| 特性类别 | 挂靠模块 |
|---|---|
| 数值/转换/控制流 | mod_01_basics |
| 模式匹配/断言宏 | mod_05_pattern_matching |
| 集合/序列 API | mod_09_collections |
| 迭代器/范围 | mod_11_iterators |
| 并发原语/锁/原子 | mod_12_concurrency |
| 宏 | mod_13_macros |
| const/高级类型与 trait/edition | mod_14_advanced |
| FFI 日常 API（C 字符串、路径、文件描述符） | mod_15_ffi |
| 低频底层：unsafe 指针与布局/ABI/汇编/分配器/工具链 | mod_19_advanced_reference |
| 智能指针/内部可变性 | mod_16_smart_pointers |
| 异步 | mod_17_async_await |

新小节格式：`// N. <特性标题>（Rust X.YY）` 分隔横幅 + 若干"为什么"注释 + 函数；必须**在模块 `main()` 中注册调用**，并在测试段补一个 `#[test]`。

## 硬性约定

1. **面向新手**：所有代码必须有详尽中文注释，解释"为什么"而非复述"是什么"，在最典型的使用场景下，给出具体的例子说明和最佳实践。
2. **版本标注必须与官方发布说明一致**——禁止凭记忆或推测写版本号和 API 名。历史教训：README 曾出现整段虚构的版本条目（如 1.76-1.90 的"core::error::Error 持续改进"、`cfg(accessible)` 已稳定、内联汇编 1.54 等），均已修正；再犯视为缺陷。
3. **未稳定特性不得写成已稳定**：如 `cfg(accessible)`（RFC 2523，谓词写法未实现，nightly 仅有不完整的 `#[cfg_accessible]` 雏形）、泛型 `Atomic<T>`、BTreeMap/LinkedList 游标等。需要演示时以注释形式保留并明确标注"需要 nightly"。
4. 每个小节必须有测试，交付前 `cargo test` 全绿。
5. 不创建仓库中原来不存在的文档。

## 新增版本特性流程

1. **取官方发布说明**：`blog.rust-lang.org/YYYY/MM/DD/Rust-X.YY.0`，用 Jina 抓正文（`curl -sL "https://r.jina.ai/blog.rust-lang.org/..."`，比 WebFetch 稳定）。博客只列亮点，完整清单（Language/Libraries/Stabilized APIs/Compatibility）看 rust-lang/rust 的 `RELEASES.md`；说明偶有遗漏，以本地 rust-src 里的 `#[stable(since = "X.YY.0")]` 为准。
2. **先探针后写作**：新 API 先在 `/tmp` 建最小 cargo 工程编译验证真实签名（方法、参数、返回值、是否 Copy/const、哪个版本 stable），**确认后再写进教程**。可直接读本地工具链源码核对：`$(rustc --print sysroot)/lib/rustlib/src/rust/library/...`（需 rust-src 组件）。
3. 按上表挂靠新小节 → 注册到模块 `main()` → 补测试。
4. 更新 `README.md` 版本特性追踪段（从官方说明提取真实亮点，含发布日期）。
5. 验证：`cargo build && cargo test && cargo clippy --all-targets`（clippy 必须 0 error）；含 `cfg` 分支的代码再用 `cargo check --lib --tests --target x86_64-unknown-linux-gnu` 与 `--target x86_64-pc-windows-gnu` 交叉检查（本机架构之外的分支不会被默认构建编译到），并确认除 `dead_code` 外没有任何告警（模块顶部只放行 `dead_code`，其余告警会直接显示）。
