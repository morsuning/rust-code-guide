# 变更记录

## [2025-12-13] 更新至 Rust 1.92

### 新增
- **Rust 1.92**: 
  - 异步闭包 (`async || {}`) 示例 (`src/mod_17_async_await.rs`)
  - Type Alias Impl Trait (TAIT) 示例 (`src/mod_08_traits.rs`)
- **Rust 1.91**:
  - Const 上下文可变引用示例 (`src/mod_14_advanced.rs`)
  - `std::sync::Exclusive` 示例 (`src/mod_12_concurrency.rs`)
- **文档**:
  - 创建 `docs/project.md`
  - 创建 `docs/changelog.md`

### 修复
- 修复 `README.md` 中指向源代码的链接（统一使用 `src/mod_XX.rs` 格式）
- 更新 `README.md` 版本特性索引
- **代码质量**:
  - 全面修复 `mod_15_ffi.rs` 中的 unsafe 块合规性问题
  - 移除了 `mod_17_async_await.rs` 中的无效引用
  - 全局统一添加 allow 属性抑制教学代码中的未使用告警，实现 Clean Build
