# Rust Code Guide 项目文档

## 项目简介
Rust Code Guide 是一个全面的 Rust 语言学习资源，基于《The Rust Programming Language》书籍，并补充了截止 Rust 1.92 版本的所有新特性。本项目通过详细的代码示例和注释，帮助开发者深入理解 Rust 的核心概念和高级特性。

## 功能特性
- **全版本覆盖**：从 Rust 1.0 到 1.92 的核心特性。
- **模块化设计**：每个主题独立成章，便于查阅和学习。
- **详细注释**：代码中包含大量教程级中文注释，并精确标注了新特性的引入版本。
- **实战导向**：包含并发、异步、FFI 等高级主题的实战示例。

## 项目结构
```
src/
├── mod_01_basics.rs         # 基础语法
├── mod_02_ownership.rs      # 所有权和借用
├── ...
├── mod_17_async_await.rs   # 异步编程
└── mod_18_oop_features.rs  # 面向对象特性
```

## 使用指南
1. 克隆项目
2. 浏览 `src/` 目录下的对应模块
3. 运行 `cargo test` 验证代码
4. 生成文档: `cargo doc --open`
