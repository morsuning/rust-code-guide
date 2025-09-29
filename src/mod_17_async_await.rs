// Rust 异步编程与 async/await
// 深入讲解 Rust 中的异步编程模型，包括 Future、async/await、执行器等核心概念
// 异步编程是 Rust 中处理高并发 I/O 操作的重要工具

// ===========================================
// 1. 异步编程基础概念
// ===========================================

// 异步编程允许程序在等待 I/O 操作时执行其他任务
// 避免阻塞整个线程，提高系统的并发性能

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};

fn async_concepts() {
    println!("=== 异步编程基础概念 ===");

    // 异步编程的核心概念：
    // 1. 非阻塞操作：在等待 I/O 时执行其他任务
    // 2. 事件驱动：基于事件循环的执行模型
    // 3. 并发处理：单线程处理大量并发连接
    // 4. 资源效率：减少线程创建和上下文切换开销

    // 同步 vs 异步的对比：
    // 同步：一个任务完成后才能开始下一个任务
    // 异步：可以在等待一个任务时执行其他任务

    // 异步编程的优势：
    // 1. 高并发：单线程处理数千个并发连接
    // 2. 低延迟：减少 I/O 等待时间
    // 3. 资源效率：减少内存和 CPU 使用
    // 4. 可扩展性：更容易水平扩展

    // 异步编程的挑战：
    // 1. 学习曲线：比同步编程更复杂
    // 2. 调试困难：异步调用栈难以追踪
    // 3. 生命周期：需要仔细管理异步资源的生命周期
    // 4. 错误处理：异步错误的处理更加复杂

    println!();
}

// ===========================================
// 2. Future trait 基础
// ===========================================

// Future 是 Rust 异步编程的核心抽象
// 表示一个可能在未来完成的异步操作

fn future_trait_basics() {
    println!("=== Future trait 基础 ===");

    // Future trait 的核心概念：
    // 1. 异步操作：表示一个尚未完成的操作
    // 2. Poll 轮询：通过轮询检查操作是否完成
    // 3. Waker 唤醒：在操作完成时唤醒任务
    // 4. 状态管理：管理异步操作的不同状态

    // 简单的 Future 实现
    struct SimpleFuture {
        value: i32,
        completed: bool,
    }

    impl Future for SimpleFuture {
        type Output = i32;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.completed {
                Poll::Ready(self.value)
            } else {
                self.completed = true;
                Poll::Pending
            }
        }
    }

    // Future 的生命周期：
    // 1. 创建：创建 Future 实例
    // 2. 轮询：执行器调用 poll 方法
    // 3. 等待：返回 Poll::Pending，注册 Waker
    // 4. 唤醒：操作完成时唤醒任务
    // 5. 完成：返回 Poll::Ready，提供结果

    // Future 的使用场景：
    // 1. 网络操作：HTTP 请求、数据库连接
    // 2. 文件操作：读写文件、目录操作
    // 3. 计时器：延时操作、超时控制
    // 4. 外部服务：API 调用、消息队列

    println!("Future trait 理解完成");
    println!();
}

// ===========================================
// 3. async/await 语法
// ===========================================

// async/await 是 Rust 异步编程的语法糖
// 让异步代码看起来像同步代码一样简洁

async fn async_await_syntax() {
    println!("=== async/await 语法 ===");

    // async fn 语法：标记函数为异步函数
    // 异步函数返回一个 Future，而不是直接返回结果

    // 基本的 async 函数
    async fn say_hello() -> String {
        "Hello, Async World!".to_string()
    }

    // 等待异步函数完成
    let result = say_hello().await;
    println!("异步函数结果: {}", result);

    // async 块：创建匿名异步函数
    let async_block = async {
        let a = 1;
        let b = 2;
        a + b
    };

    let sum = async_block.await;
    println!("异步块结果: {}", sum);

    // async/await 的工作原理：
    // 1. async fn 将函数转换为返回 Future 的函数
    // 2. await 暂停当前 Future，等待被唤醒
    // 3. 被唤醒后继续执行，直到完成或遇到下一个 await
    // 4. 编译器将异步代码转换为状态机

    // 异步函数的特点：
    // 1. 非阻塞：await 点不会阻塞线程
    // 2. 可组合：可以方便地组合多个异步操作
    // 3. 错误处理：支持 ? 操作符进行错误传播
    // 4. 生命周期：编译器自动管理异步资源的生命周期

    // 复杂的异步函数示例
    async fn complex_async_operation() -> Result<String, String> {
        // 模拟多个异步操作
        let step1 = async_step1().await?;
        let step2 = async_step2(step1).await?;
        let result = async_step3(step2).await?;

        Ok(result)
    }

    async fn async_step1() -> Result<i32, String> {
        // 模拟异步操作
        Ok(42)
    }

    async fn async_step2(input: i32) -> Result<i32, String> {
        // 模拟异步操作
        Ok(input * 2)
    }

    async fn async_step3(input: i32) -> Result<String, String> {
        // 模拟异步操作
        Ok(format!("最终结果: {}", input))
    }

    match complex_async_operation().await {
        Ok(result) => println!("复杂异步操作成功: {}", result),
        Err(e) => println!("复杂异步操作失败: {}", e),
    }

    println!();
}

// ===========================================
// 4. 异步执行器和运行时
// ===========================================

// 异步执行器负责运行和管理异步任务
// Tokio 是 Rust 生态系统中最流行的异步运行时

async fn async_executors() {
    println!("=== 异步执行器和运行时 ===");

    // 异步执行器的核心概念：
    // 1. 任务调度：决定何时运行哪个任务
    // 2. 事件循环：监听 I/O 事件并唤醒相应任务
    // 3. 资源管理：管理异步资源的生命周期
    // 4. 并发控制：控制并发任务的执行

    // Tokio 运行时的特性：
    // 1. 多线程：使用线程池提高并发性能
    // 2. 工作窃取：负载均衡的调度策略
    // 3. 定时器：高效的定时器实现
    // 4. I/O 驱动：跨平台的异步 I/O 支持

    // 基本的异步任务
    async fn task1() {
        println!("任务 1 开始");
        sleep(Duration::from_millis(100)).await;
        println!("任务 1 完成");
    }

    async fn task2() {
        println!("任务 2 开始");
        sleep(Duration::from_millis(50)).await;
        println!("任务 2 完成");
    }

    // 并发执行多个任务
    let handle1 = tokio::spawn(task1());
    let handle2 = tokio::spawn(task2());

    // 等待所有任务完成
    let _ = tokio::join!(handle1, handle2);

    println!("所有任务完成");

    // 异步任务的生命周期：
    // 1. 创建：通过 tokio::spawn 创建任务
    // 2. 调度：执行器决定何时运行任务
    // 3. 执行：任务执行到第一个 await 点
    // 4. 暂停：在 await 点暂停，等待唤醒
    // 5. 唤醒：I/O 事件完成，任务被唤醒
    // 6. 完成：任务执行完成，返回结果

    println!();
}

// ===========================================
// 5. 异步 I/O 操作
// ===========================================

// 异步 I/O 是异步编程的主要应用场景
// 包括网络通信、文件操作、数据库访问等

async fn async_io_operations() {
    println!("=== 异步 I/O 操作 ===");

    // 异步 I/O 的核心概念：
    // 1. 非阻塞 I/O：I/O 操作不会阻塞线程
    // 2. 事件驱动：通过事件通知 I/O 完成
    // 3. 零拷贝：减少数据在内存中的复制
    // 4. 批量操作：优化多个 I/O 操作的执行

    // 异步网络操作示例
    // 注意：reqwest 是一个外部依赖 crate，需要添加到 Cargo.toml 才能使用
    async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 实际使用时需要添加 reqwest 依赖：
        // let response = reqwest::get(url).await?;
        // let body = response.text().await?;
        // Ok(body)

        // 模拟网络延迟
        sleep(Duration::from_millis(100)).await;
        Ok(format!("从 {} 获取的数据", url))
    }

    // 异步文件操作示例
    async fn read_file_async(path: &str) -> Result<String, std::io::Error> {
        // 模拟文件读取
        // 实际使用时需要使用 tokio::fs
        // let content = tokio::fs::read_to_string(path).await?;
        // Ok(content)

        sleep(Duration::from_millis(50)).await;
        Ok(format!("文件 {} 的内容", path))
    }

    // 并发执行多个 I/O 操作
    let url = "https://example.com";
    let file_path = "/tmp/example.txt";

    let data_future = fetch_data(url);
    let file_future = read_file_async(file_path);

    let (data_result, file_result) = tokio::join!(data_future, file_future);

    match data_result {
        Ok(data) => println!("网络请求成功: {}", data),
        Err(e) => println!("网络请求失败: {}", e),
    }

    match file_result {
        Ok(content) => println!("文件读取成功: {}", content),
        Err(e) => println!("文件读取失败: {}", e),
    }

    // 异步 I/O 的优势：
    // 1. 高并发：单线程处理大量并发连接
    // 2. 低延迟：减少 I/O 等待时间
    // 3. 资源效率：减少线程和内存使用
    // 4. 可扩展性：更容易处理大量并发请求

    // 异步 I/O 的使用场景：
    // 1. Web 服务器：处理大量 HTTP 请求
    // 2. 数据库客户端：并发执行数据库查询
    // 3. 消息队列：处理大量消息
    // 4. 文件处理：异步读写大文件

    println!();
}

// ===========================================
// 6. 流处理和迭代器
// ===========================================

// 异步流 (Stream) 是异步版本的迭代器
// 允许异步地生成和处理数据序列

async fn async_streams() {
    println!("=== 流处理和迭代器 ===");

    use futures::stream::{self, StreamExt};

    // 异步流的核心概念：
    // 1. 异步迭代：按需生成数据项
    // 2. 懒加载：只在需要时生成数据
    // 3. 组合操作：支持函数式风格的链式操作
    // 4. 背压处理：处理生产者和消费者的速度差异

    // 创建异步流
    let numbers = stream::iter(0..10)
        .map(|n| async move { n * 2 })
        .buffer_unordered(5); // 并发处理最多 5 个项目

    // 处理异步流
    let results: Vec<i32> = numbers.collect().await;
    println!("异步流处理结果: {:?}", results);

    // 异步生成器
    async fn generate_numbers(count: usize) -> impl futures::Stream<Item = i32> {
        let mut counter = 0i32;
        futures::stream::unfold((), move |_| async move {
            if counter < count as i32 {
                let result = counter;
                counter += 1;
                Some((result, ()))
            } else {
                None
            }
        })
    }

    // 使用 Box::pin 替代 .boxed() 方法，需要先等待 Future 完成
    let mut number_stream = Box::pin(generate_numbers(5).await);

    while let Some(number) = number_stream.next().await {
        println!("生成的数字: {}", number);
    }

    // 流操作示例 - 使用标准迭代器
    let data_stream: Vec<i32> = (0..20)
        .filter(|&x| x % 2 == 0)  // 过滤偶数
        .map(|x| x * x)           // 平方
        .take(5)                  // 取前5个
        .collect();

    println!("流操作结果: {:?}", data_stream);

    // 异步流的应用场景：
    // 1. 数据处理：批量处理大量数据
    // 2. 网络流：处理网络数据流
    // 3. 文件流：异步读取大文件
    // 4. 事件流：处理事件序列

    println!();
}

// ===========================================
// 7. 错误处理和超时控制
// ===========================================

// 异步编程中的错误处理和超时控制
// 确保异步程序的健壮性和可靠性

async fn error_handling_and_timeouts() {
    println!("=== 错误处理和超时控制 ===");

    // 异步错误处理的核心概念：
    // 1. Result 类型：使用 Result<T, E> 表示可能失败的操作
    // 2. ? 操作符：简化错误传播
    // 3. 自定义错误类型：定义领域特定的错误类型
    // 4. 错误恢复：优雅地处理和恢复错误

    // 异步函数的错误处理
    async fn async_operation(success: bool) -> Result<String, String> {
        if success {
            Ok("操作成功".to_string())
        } else {
            Err("操作失败".to_string())
        }
    }

    // 处理异步操作结果
    match async_operation(true).await {
        Ok(result) => println!("异步操作成功: {}", result),
        Err(e) => println!("异步操作失败: {}", e),
    }

    // 超时控制
    async fn long_running_operation() -> String {
        sleep(Duration::from_secs(2)).await;
        "长时间操作完成".to_string()
    }

    // 设置超时
    match timeout(Duration::from_secs(1), long_running_operation()).await {
        Ok(result) => println!("操作在超时前完成: {}", result),
        Err(_) => println!("操作超时"),
    }

    // 重试机制
    async fn retry_operation<F, Fut, T, E>(operation: F, max_retries: usize) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let mut retries = 0;
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    retries += 1;
                    if retries >= max_retries {
                        return Err(e);
                    }
                    println!("重试 {}/{}", retries, max_retries);
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    async fn flaky_operation() -> Result<i32, String> {
        // 模拟有时失败的操作
        if rand::random::<f64>() < 0.7 {
            Ok(42)
        } else {
            Err("随机失败".to_string())
        }
    }

    match retry_operation(flaky_operation, 3).await {
        Ok(result) => println!("重试成功: {}", result),
        Err(e) => println!("重试失败: {}", e),
    }

    // 错误处理的最佳实践：
    // 1. 明确错误类型：定义清晰的错误层次
    // 2. 错误上下文：提供足够的错误信息
    // 3. 优雅降级：提供合理的默认行为
    // 4. 监控和日志：记录错误以便调试

    println!();
}

// ===========================================
// 8. 异步模式和最佳实践
// ===========================================

// 异步编程中的常见模式和最佳实践
// 提高异步代码的可维护性和性能

async fn async_patterns_and_best_practices() {
    println!("=== 异步模式和最佳实践 ===");

    // 模式 1: 批量处理
    async fn batch_processing<T, F, Fut>(items: Vec<T>, processor: F) -> Vec<Fut::Output>
    where
        F: Fn(T) -> Fut,
        Fut: Future,
    {
        let futures = items.into_iter().map(processor).collect::<Vec<_>>();
        futures::future::join_all(futures).await
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let results = batch_processing(numbers, |n| async move { n * 2 }).await;
    println!("批量处理结果: {:?}", results);

    // 模式 2: 限流和并发控制
    use futures::stream::{self, StreamExt};

    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = stream::iter(items)
        .for_each_concurrent(3, |item| async move {
            println!("处理项目: {}", item);
            sleep(Duration::from_millis(100)).await;
        })
        .await;

    // 模式 3: 缓存异步结果
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, Condvar};
    use tokio::sync::RwLock;

    #[derive(Clone)]
    struct AsyncCache<K, V> {
        cache: Arc<RwLock<HashMap<K, V>>>,
    }

    impl<K: std::hash::Hash + Eq + Clone, V: Clone> AsyncCache<K, V> {
        fn new() -> Self {
            Self {
                cache: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        async fn get_or_insert<F, Fut>(&self, key: K, factory: F) -> V
        where
            F: FnOnce() -> Fut,
            Fut: Future<Output = V>,
        {
            {
                let cache = self.cache.read().await;
                if let Some(value) = cache.get(&key) {
                    return value.clone();
                }
            }

            let value = factory().await;
            let mut cache = self.cache.write().await;
            cache.insert(key.clone(), value.clone());
            value
        }
    }

    let cache = AsyncCache::new();
    let result1 = cache.get_or_insert(
        "key1".to_string(),
        || async { "计算值 1".to_string() },
    ).await;

    let result2 = cache.get_or_insert(
        "key1".to_string(),
        || async { "不应该执行".to_string() },
    ).await;

    println!("缓存结果: {} {}", result1, result2);

    // 异步编程的最佳实践：
    // 1. 合理设置并发度：避免过度并发
    // 2. 正确处理取消：支持优雅的任务取消
    // 3. 资源清理：确保异步资源正确释放
    // 4. 性能监控：监控异步任务的性能
    // 5. 错误处理：实现完善的错误处理机制

    println!();
}

// ===========================================
// 9. 测试异步代码
// ===========================================

// 异步代码的测试策略和技巧
// 确保异步程序的正确性和可靠性

async fn testing_async_code() {
    println!("=== 测试异步代码 ===");

    // 异步测试的核心概念：
    // 1. 异步测试函数：使用 #[tokio::test] 标记
    // 2. Mock 异步依赖：模拟异步操作
    // 3. 超时控制：避免测试无限等待
    // 4. 并发测试：测试并发场景的正确性

    // 模拟异步服务
    #[derive(Clone)]
    struct MockAsyncService {
        delay: Duration,
        should_fail: bool,
    }

    impl MockAsyncService {
        async fn call(&self) -> Result<String, String> {
            sleep(self.delay).await;
            if self.should_fail {
                Err("模拟失败".to_string())
            } else {
                Ok("模拟成功".to_string())
            }
        }
    }

    // 测试异步函数
    async fn test_async_function() {
        let service = MockAsyncService {
            delay: Duration::from_millis(100),
            should_fail: false,
        };

        match service.call().await {
            Ok(result) => println!("测试通过: {}", result),
            Err(e) => println!("测试失败: {}", e),
        }
    }

    test_async_function().await;

    // 测试超时处理
    async fn test_timeout_handling() {
        let service = MockAsyncService {
            delay: Duration::from_secs(2),
            should_fail: false,
        };

        match timeout(Duration::from_secs(1), service.call()).await {
            Ok(Ok(result)) => println!("在超时内完成: {}", result),
            Ok(Err(e)) => println!("操作失败: {}", e),
            Err(_) => println!("操作超时"),
        }
    }

    test_timeout_handling().await;

    // 测试并发场景
    async fn test_concurrent_operations() {
        let service = MockAsyncService {
            delay: Duration::from_millis(50),
            should_fail: false,
        };

        let handles = (0..10)
            .map(|_| {
                let service = service.clone();
                tokio::spawn(async move { service.call().await })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::join_all(handles).await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        let success_count = results.iter().filter(|r| r.is_ok()).count();
        println!("并发测试结果: {} 成功, {} 失败",
                 success_count,
                 results.len() - success_count);
    }

    test_concurrent_operations().await;

    // 异步测试的最佳实践：
    // 1. 隔离测试：每个测试独立运行
    // 2. Mock 依赖：模拟外部异步服务
    // 3. 超时保护：避免测试无限等待
    // 4. 资源清理：确保测试资源正确释放
    // 5. 性能测试：测试异步性能特征

    println!();
}

// ===========================================
// 10. 实际应用示例
// ===========================================

// 综合示例：异步 Web 服务器
// 展示异步编程在实际项目中的应用

async fn practical_examples() {
    println!("=== 实际应用示例 ===");

    // 示例 1: 异步 HTTP 服务器框架
    #[derive(Debug)]
    struct HttpRequest {
        method: String,
        path: String,
        headers: std::collections::HashMap<String, String>,
    }

    #[derive(Debug)]
    struct HttpResponse {
        status: u16,
        headers: std::collections::HashMap<String, String>,
        body: String,
    }

    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    // 这是 Rust 的当前限制，未来版本可能会改进
    // 参见：https://github.com/rust-lang/rust/issues/91612
    trait HttpHandler {
        async fn handle(&self, request: HttpRequest) -> HttpResponse;
    }

    // 注意：以下代码目前无法编译，因为 dyn trait 不支持 async 方法
    // 这是语言的当前限制
    /*
    struct AsyncHttpServer {
        handlers: std::collections::HashMap<String, Box<dyn HttpHandler + Send + Sync>>,
    }

    impl AsyncHttpServer {
        fn new() -> Self {
            Self {
                handlers: std::collections::HashMap::new(),
            }
        }

        fn add_handler<H>(&mut self, path: String, handler: H)
        where
            H: HttpHandler + Send + Sync + 'static,
        {
            self.handlers.insert(path, Box::new(handler));
        }

        async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
            if let Some(handler) = self.handlers.get(&request.path) {
                handler.handle(request).await
            } else {
                HttpResponse {
                    status: 404,
                    headers: std::collections::HashMap::new(),
                    body: "Not Found".to_string(),
                }
            }
        }
    }
    */

    // HelloHandler 的实现也被注释掉，因为依赖 AsyncHttpServer
    /*
    struct HelloHandler;

    impl HttpHandler for HelloHandler {
        async fn handle(&self, _request: HttpRequest) -> HttpResponse {
            HttpResponse {
                status: 200,
                headers: {
                    let mut headers = std::collections::HashMap::new();
                    headers.insert("Content-Type".to_string(), "text/plain".to_string());
                    headers
                },
                body: "Hello, Async World!".to_string(),
            }
        }
    }

    let mut server = AsyncHttpServer::new();
    server.add_handler("/hello".to_string(), HelloHandler);

    let request = HttpRequest {
        method: "GET".to_string(),
        path: "/hello".to_string(),
        headers: std::collections::HashMap::new(),
    };

    let response = server.handle_request(request).await;
    println!("HTTP 响应: {:?}", response);
    */

    // 示例 2: 异步任务队列
    #[derive(Debug)]
    struct AsyncTask {
        id: String,
        data: String,
        created_at: Instant,
    }

    #[derive(Debug)]
    struct AsyncTaskQueue {
        tasks: Arc<tokio::sync::Mutex<Vec<AsyncTask>>>,
        active_tasks: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Instant>>>,
    }

    impl AsyncTaskQueue {
        fn new() -> Self {
            Self {
                tasks: Arc::new(tokio::sync::Mutex::new(Vec::new())),
                active_tasks: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            }
        }

        async fn add_task(&self, task: AsyncTask) {
            let mut tasks = self.tasks.lock().await;
            tasks.push(task);
        }

        async fn process_tasks(&self) {
            let mut tasks = self.tasks.lock().await;
            if tasks.is_empty() {
                return;
            }

            let task = tasks.remove(0);
            let task_id = task.id.clone();

            {
                let mut active_tasks = self.active_tasks.write().await;
                active_tasks.insert(task_id.clone(), Instant::now());
            }

            // 在后台处理任务
            let active_tasks = self.active_tasks.clone();
            tokio::spawn(async move {
                println!("处理任务: {}", task.data);
                sleep(Duration::from_millis(100)).await;

                let mut active_tasks = active_tasks.write().await;
                active_tasks.remove(&task_id);
                println!("任务完成: {}", task.data);
            });
        }

        async fn get_status(&self) -> (usize, usize) {
            let tasks = self.tasks.lock().await;
            let active_tasks = self.active_tasks.read().await;
            (tasks.len(), active_tasks.len())
        }
    }

    let task_queue = AsyncTaskQueue::new();

    // 添加任务
    for i in 0..5 {
        task_queue.add_task(AsyncTask {
            id: format!("task_{}", i),
            data: format!("任务数据 {}", i),
            created_at: Instant::now(),
        }).await;
    }

    // 处理任务
    for _ in 0..3 {
        task_queue.process_tasks().await;
    }

    // 等待一些任务完成
    sleep(Duration::from_millis(200)).await;

    let (pending, active) = task_queue.get_status().await;
    println!("任务队列状态: {} 待处理, {} 活跃", pending, active);

    // 示例 3: 异步数据管道
    use futures::stream::{self, StreamExt};

    async fn create_data_pipeline() -> Vec<String> {
        // 简化的数据管道示例
        let data_stream: Vec<String> = (0..100)
            .filter(|&x| x % 3 == 0)  // 过滤3的倍数
            .map(|x| format!("处理后的数据: {}", x))
            .collect();

        data_stream
    }

    let pipeline = create_data_pipeline().await;
    let mut count = 0;

    for result in pipeline.iter() {
        println!("管道输出: {}", result);
        count += 1;
        if count >= 5 {
            break;
        }
    }

    println!();
}

// ===========================================
// 9. Rust 1.85 异步 trait 方法改进
// ===========================================

// Rust 1.85 引入了对异步 trait 方法的重大改进，使得在 trait 中定义和使用异步方法变得更加自然和直观
// 这个改进解决了长期以来异步 trait 方法需要额外语法或第三方库的问题

async fn async_trait_methods() {
    println!("=== Rust 1.85 异步 trait 方法改进 ===");

    // 异步 trait 方法的背景和意义：
    // 1. 简化语法：不再需要 async-trait 宏或其他辅助工具
    // 2. 性能优化：编译器级别的支持减少了运行时开销
    // 3. 类型系统改进：更好的错误信息和类型推断
    // 4. 生态系统标准化：为异步 trait 提供官方支持
    // 5. 代码可读性：更直观的异步方法定义和使用

    // 基本语法：在 trait 中直接使用 async fn
    // 注意：这需要 Rust 1.85+ 版本支持

    // 示例 1: 基本的异步 trait 定义
    trait DatabaseService {
        async fn connect(&self) -> Result<(), String>;
        async fn query(&self, sql: &str) -> Result<Vec<String>, String>;
        async fn execute(&self, sql: &str) -> Result<u64, String>;
        async fn close(&self) -> Result<(), String>;
    }

    // 实现 DatabaseService trait
    struct MockDatabase {
        connected: bool,
    }

    impl DatabaseService for MockDatabase {
        async fn connect(&self) -> Result<(), String> {
            if self.connected {
                Err("数据库已经连接".to_string())
            } else {
                println!("连接到数据库...");
                Ok(())
            }
        }

        async fn query(&self, sql: &str) -> Result<Vec<String>, String> {
            if !self.connected {
                return Err("数据库未连接".to_string());
            }
            println!("执行查询: {}", sql);
            Ok(vec!["结果1".to_string(), "结果2".to_string()])
        }

        async fn execute(&self, sql: &str) -> Result<u64, String> {
            if !self.connected {
                return Err("数据库未连接".to_string());
            }
            println!("执行语句: {}", sql);
            Ok(1) // 返回影响的行数
        }

        async fn close(&self) -> Result<(), String> {
            println!("关闭数据库连接");
            Ok(())
        }
    }

    // 使用异步 trait 方法
    let db = MockDatabase { connected: false };

    match db.connect().await {
        Ok(_) => println!("数据库连接成功"),
        Err(e) => println!("连接失败: {}", e),
    }

    match db.query("SELECT * FROM users").await {
        Ok(results) => println!("查询结果: {:?}", results),
        Err(e) => println!("查询失败: {}", e),
    }

    // 示例 2: 异步迭代器 trait
    trait AsyncDataSource {
        type Item;
        async fn next(&mut self) -> Option<Self::Item>;
        async fn has_more(&self) -> bool;
    }

    struct AsyncCounter {
        current: u32,
        max: u32,
    }

    impl AsyncDataSource for AsyncCounter {
        type Item = u32;

        async fn next(&mut self) -> Option<Self::Item> {
            if self.current >= self.max {
                None
            } else {
                let value = self.current;
                self.current += 1;
                Some(value)
            }
        }

        async fn has_more(&self) -> bool {
            self.current < self.max
        }
    }

    let mut counter = AsyncCounter { current: 0, max: 5 };
    while counter.has_more().await {
        if let Some(value) = counter.next().await {
            println!("计数器值: {}", value);
        }
    }

    // 示例 3: 带有生命周期的异步 trait
    trait AsyncProcessor<'a, T> {
        async fn process(&self, data: &'a [T]) -> Vec<T>
        where
            T: Clone + Send + Sync;
        async fn validate(&self, item: &T) -> bool;
    }

    struct DataValidator;

    impl<'a, T: Clone + Send + Sync + PartialEq> AsyncProcessor<'a, T> for DataValidator {
        async fn process(&self, data: &'a [T]) -> Vec<T> {
            let mut result = Vec::new();
            for item in data {
                if self.validate(item).await {
                    result.push(item.clone());
                }
            }
            result
        }

        async fn validate(&self, item: &T) -> bool {
            // 模拟异步验证过程
            // T::default() 需要 T: Default trait bound，暂时注释
            // item != &T::default()
            true // 简化实现
        }
    }

    let validator = DataValidator;
    let data = vec![1, 0, 2, 0, 3];
    let processed = validator.process(&data).await;
    println!("处理后的数据: {:?}", processed);

    // 示例 4: 异步 builder 模式
    trait AsyncBuilder {
        type Output;
        async fn build(self) -> Self::Output;
        async fn configure(self, config: &str) -> Self;
    }

    struct ServiceBuilder {
        name: String,
        port: u16,
    }

    impl AsyncBuilder for ServiceBuilder {
        type Output = Service;

        async fn build(self) -> Self::Output {
            println!("异步构建服务: {}", self.name);
            Service {
                name: self.name,
                port: self.port,
            }
        }

        async fn configure(mut self, config: &str) -> Self {
            if let Some(port) = config.parse().ok() {
                self.port = port;
            }
            self
        }
    }

    #[derive(Debug)]
    struct Service {
        name: String,
        port: u16,
    }

    let builder = ServiceBuilder {
        name: "Web Service".to_string(),
        port: 8080,
    };

    let service = builder
        .configure("9090").await
        .build().await;
    println!("构建的服务: {:?}", service);

    // 示例 5: 异步工厂模式
    trait AsyncFactory<T> {
        async fn create(&self) -> T;
        async fn create_batch(&self, count: usize) -> Vec<T>;
    }

    struct UserFactory;

    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
    }

    impl AsyncFactory<User> for UserFactory {
        async fn create(&self) -> User {
            User {
                id: 42,
                name: "默认用户".to_string(),
            }
        }

        async fn create_batch(&self, count: usize) -> Vec<User> {
            let mut users = Vec::new();
            for i in 0..count {
                users.push(User {
                    id: i as u32,
                    name: format!("用户{}", i),
                });
            }
            users
        }
    }

    let factory = UserFactory;
    let user = factory.create().await;
    println!("创建的用户: {:?}", user);

    let batch = factory.create_batch(3).await;
    println!("批量创建的用户: {:?}", batch);

    // 示例 6: 异步 trait 对象
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    // 这是 Rust 的当前限制，未来版本可能会改进
    // 参见：https://github.com/rust-lang/rust/issues/91612
    /*
    trait AsyncHandler: Send + Sync {
        async fn handle(&self, message: &str) -> Result<String, String>;
    }

    struct LoggingHandler;

    impl AsyncHandler for LoggingHandler {
        async fn handle(&self, message: &str) -> Result<String, String> {
            println!("记录消息: {}", message);
            Ok("消息已记录".to_string())
        }
    }

    struct NotificationHandler;

    impl AsyncHandler for NotificationHandler {
        async fn handle(&self, message: &str) -> Result<String, String> {
            println!("发送通知: {}", message);
            Ok("通知已发送".to_string())
        }
    }

    // 使用 trait 对象进行动态分发
    let handlers: Vec<Box<dyn AsyncHandler>> = vec![
        Box::new(LoggingHandler),
        Box::new(NotificationHandler),
    ];

    for handler in handlers {
        match handler.handle("测试消息").await {
            Ok(result) => println!("处理结果: {}", result),
            Err(e) => println!("处理失败: {}", e),
        }
    }
    */

    practical_async_trait_examples().await;

    println!("异步 trait 方法改进演示完成");
    println!();
}

async fn practical_async_trait_examples() {
    println!("=== 异步 trait 实际应用示例 ===");

    // 场景 1: HTTP 服务的异步 trait
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    // 这是 Rust 的当前限制，未来版本可能会改进
    mod http_service {
        use std::collections::HashMap;

        #[derive(Debug)]
        pub struct Request {
            pub method: String,
            pub path: String,
            pub headers: HashMap<String, String>,
            pub body: Vec<u8>,
        }

        #[derive(Debug)]
        pub struct Response {
            pub status: u16,
            pub headers: HashMap<String, String>,
            pub body: Vec<u8>,
        }

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait HttpService: Send + Sync {
            async fn handle_request(&self, request: Request) -> Response;
            async fn middleware(&self, request: Request) -> Option<Request>;
        }

        pub struct StaticFileServer;

        impl HttpService for StaticFileServer {
            async fn handle_request(&self, request: Request) -> Response {
                Response {
                    status: 200,
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "text/plain".to_string());
                        headers
                    },
                    body: b"Hello, World!".to_vec(),
                }
            }

            async fn middleware(&self, request: Request) -> Option<Request> {
                // 记录请求日志
                println!("处理请求: {} {}", request.method, request.path);
                Some(request)
            }
        }

        pub struct ApiProxy {
            target_url: String,
        }

        impl HttpService for ApiProxy {
            async fn handle_request(&self, request: Request) -> Response {
                // 模拟代理请求
                println!("代理请求到: {}", self.target_url);
                Response {
                    status: 200,
                    headers: HashMap::new(),
                    body: b"Proxy response".to_vec(),
                }
            }

            async fn middleware(&self, request: Request) -> Option<Request> {
                // 验证请求头
                if request.headers.get("Authorization").is_some() {
                    Some(request)
                } else {
                    None
                }
            }
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleHttpService: Send + Sync {
            fn handle_request_sync(&self, request: Request) -> Response;
        }

        pub struct SimpleStaticFileServer;

        impl SimpleHttpService for SimpleStaticFileServer {
            fn handle_request_sync(&self, _request: Request) -> Response {
                Response {
                    status: 200,
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "text/plain".to_string());
                        headers
                    },
                    body: b"Hello, World!".to_vec(),
                }
            }
        }
    }

    // 场景 2: 数据库连接池的异步 trait
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod database_pool {
        use std::time::Duration;
        use tokio::time::sleep;

        #[derive(Debug, Clone)]
        pub struct DatabaseConnection {
            id: u32,
        }

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait ConnectionPool {
            async fn get_connection(&self) -> Result<DatabaseConnection, String>;
            async fn return_connection(&self, conn: DatabaseConnection) -> Result<(), String>;
            async fn get_pool_size(&self) -> usize;
        }

        pub struct SimpleConnectionPool {
            connections: Vec<DatabaseConnection>,
            max_size: usize,
        }

        impl SimpleConnectionPool {
            pub fn new(max_size: usize) -> Self {
                let mut connections = Vec::new();
                for i in 0..max_size {
                    connections.push(DatabaseConnection { id: i as u32 });
                }
                SimpleConnectionPool {
                    connections,
                    max_size,
                }
            }
        }

        impl ConnectionPool for SimpleConnectionPool {
            async fn get_connection(&self) -> Result<DatabaseConnection, String> {
                if self.connections.is_empty() {
                    Err("连接池已空".to_string())
                } else {
                    // 模拟获取连接的延迟
                    sleep(Duration::from_millis(10)).await;
                    Ok(self.connections[0].clone())
                }
            }

            async fn return_connection(&self, _conn: DatabaseConnection) -> Result<(), String> {
                // 模拟归还连接
                sleep(Duration::from_millis(5)).await;
                Ok(())
            }

            async fn get_pool_size(&self) -> usize {
                self.connections.len()
            }
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub struct SimpleConnectionPool {
            connections: Vec<DatabaseConnection>,
            max_size: usize,
        }

        impl SimpleConnectionPool {
            pub fn new(max_size: usize) -> Self {
                let mut connections = Vec::new();
                for i in 0..max_size {
                    connections.push(DatabaseConnection { id: i as u32 });
                }
                SimpleConnectionPool {
                    connections,
                    max_size,
                }
            }

            pub fn get_connection_sync(&self) -> Result<DatabaseConnection, String> {
                if self.connections.is_empty() {
                    Err("连接池已空".to_string())
                } else {
                    Ok(self.connections[0].clone())
                }
            }

            pub fn get_pool_size_sync(&self) -> usize {
                self.connections.len()
            }
        }
    }

    // 场景 3: 消息队列的异步 trait
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod message_queue {
        use std::collections::VecDeque;
        use tokio::sync::Mutex;

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait MessageQueue<T>: Send + Sync {
            async fn send(&self, message: T) -> Result<(), String>;
            async fn receive(&self) -> Option<T>;
            async fn size(&self) -> usize;
        }

        pub struct InMemoryQueue<T> {
            messages: Mutex<VecDeque<T>>,
        }

        impl<T: Send> InMemoryQueue<T> {
            pub fn new() -> Self {
                InMemoryQueue {
                    messages: Mutex::new(VecDeque::new()),
                }
            }
        }

        impl<T: Send + Clone> MessageQueue<T> for InMemoryQueue<T> {
            async fn send(&self, message: T) -> Result<(), String> {
                let mut messages = self.messages.lock().await;
                messages.push_back(message);
                Ok(())
            }

            async fn receive(&self) -> Option<T> {
                let mut messages = self.messages.lock().await;
                messages.pop_front()
            }

            async fn size(&self) -> usize {
                let messages = self.messages.lock().await;
                messages.len()
            }
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub struct InMemoryQueue<T> {
            messages: Mutex<VecDeque<T>>,
        }

        impl<T: Send> InMemoryQueue<T> {
            pub fn new() -> Self {
                InMemoryQueue {
                    messages: Mutex::new(VecDeque::new()),
                }
            }

            pub async fn send_async(&self, message: T) -> Result<(), String> {
                let mut messages = self.messages.lock().await;
                messages.push_back(message);
                Ok(())
            }

            pub async fn receive_async(&self) -> Option<T> {
                let mut messages = self.messages.lock().await;
                messages.pop_front()
            }

            pub async fn size_async(&self) -> usize {
                let messages = self.messages.lock().await;
                messages.len()
            }
        }
    }

    println!("实际应用示例演示完成");
}

// ===========================================
// 10. Rust 1.86 异步函数生命周期改进
// ===========================================

// Rust 1.86 引入了异步函数生命周期的重要改进，使得异步函数的生命周期处理更加灵活和直观
// 这些改进简化了异步代码中的生命周期标注，提高了代码的可读性和编译器的推断能力

async fn async_function_lifetimes() {
    println!("=== Rust 1.86 异步函数生命周期改进 ===");

    // 异步函数生命周期改进的背景和意义：
    // 1. 简化语法：减少显式生命周期标注的需要
    // 2. 更好的推断：编译器能够更智能地推断异步函数中的生命周期
    // 3. 减少错误：避免常见的生命周期相关编译错误
    // 4. 代码可读性：使异步函数的签名更加简洁清晰
    // 5. 性能优化：优化的生命周期处理可能带来更好的性能

    // 示例 1: 改进的异步函数生命周期推断
    // 在 Rust 1.86 之前，这样的函数可能需要显式生命周期标注
    async fn process_data_ref(data: &[u8]) -> Vec<u8> {
        data.iter().map(|&b| b * 2).collect()
    }

    let data = vec![1, 2, 3, 4, 5];
    let processed = process_data_ref(&data).await;
    println!("处理后的数据: {:?}", processed);

    // 示例 2: 多个引用参数的异步函数
    async fn merge_strings_ref<'a>(left: &'a str, right: &'a str) -> String {
        format!("{}:{}", left, right)
    }

    let result = merge_strings_ref("Hello", "World").await;
    println!("合并结果: {}", result);

    // 示例 3: 异步函数中的结构体引用
    #[derive(Debug)]
    struct Config {
        timeout_ms: u64,
        max_retries: u32,
    }

    async fn apply_config_ref(config: &Config) -> String {
        format!("Config: timeout={}ms, retries={}",
                config.timeout_ms, config.max_retries)
    }

    let config = Config {
        timeout_ms: 5000,
        max_retries: 3,
    };
    let config_str = apply_config_ref(&config).await;
    println!("配置字符串: {}", config_str);

    // 示例 4: 异步闭包的生命周期改进
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = async {
        numbers.iter().map(|&x| x * 2).collect::<Vec<_>>()
    }.await;
    println!("异步闭包结果: {:?}", doubled);

    // 示例 5: 异步函数中的泛型处理器生命周期
    trait AsyncProcessor: Send + Sync {
        fn process<'a>(&'a self, input: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + 'a>>;
    }

    struct StringProcessor;

    impl AsyncProcessor for StringProcessor {
        fn process<'a>(&'a self, input: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + 'a>> {
            Box::pin(async move {
                input.to_uppercase()
            })
        }
    }

    async fn use_processor_ref<T: AsyncProcessor>(processor: &T, input: &str) -> String {
        processor.process(input).await
    }

    let processor = StringProcessor;
    let result = use_processor_ref(&processor, "hello world").await;
    println!("处理器结果: {}", result);

    // 示例 6: 异步迭代器的生命周期改进
    async fn process_stream_ref<T>(stream: &[T], f: impl Fn(&T) -> bool + Send + Sync) -> Vec<T>
    where
        T: Clone + Send + Sync,
    {
        stream.iter().filter(|&item| f(item)).cloned().collect()
    }

    let data = vec![1, 2, 3, 4, 5, 6];
    let filtered = process_stream_ref(&data, |&x| x % 2 == 0).await;
    println!("过滤后的数据: {:?}", filtered);

    // 示例 7: 复杂异步函数的生命周期管理
    async fn complex_async_operation<'a, T: AsyncProcessor>(
        config: &'a Config,
        data: &'a [u8],
        processor: &'a T,
    ) -> Result<String, String> {
        let processed_data = data.iter().map(|&b| b * 2).collect::<Vec<_>>();
        let config_str = apply_config_ref(config).await;
        let processed_input = processor.process(&config_str).await;

        Ok(format!("最终结果: {} (数据: {:?})", processed_input, processed_data))
    }

    let result = complex_async_operation(&config, &data, &processor).await;
    match result {
        Ok(output) => println!("复杂操作结果: {}", output),
        Err(e) => println!("复杂操作失败: {}", e),
    }

    practical_async_lifetime_examples().await;

    println!("异步函数生命周期改进演示完成");
    println!();
}

async fn practical_async_lifetime_examples() {
    println!("=== 异步函数生命周期实际应用示例 ===");

    // 场景 1: HTTP 请求处理中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod http_lifetimes {
        #[derive(Debug)]
        pub struct HttpRequest {
            method: String,
            path: String,
            headers: Vec<(String, String)>,
        }

        #[derive(Debug)]
        pub struct HttpResponse {
            status: u16,
            body: String,
        }

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait HttpClient: Send + Sync {
            async fn send_request(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
        }

        struct MockHttpClient;

        impl HttpClient for MockHttpClient {
            async fn send_request(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
                println!("发送请求: {} {}", request.method, request.path);
                Ok(HttpResponse {
                    status: 200,
                    body: "Mock response".to_string(),
                })
            }
        }

        // 改进的生命周期处理：不再需要显式标注
        async fn process_http_request(
            client: &dyn HttpClient,
            request: &HttpRequest,
        ) -> Result<String, String> {
            let response = client.send_request(request).await?;
            Ok(format!("处理完成: {} - {}", response.status, response.body))
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleHttpClient: Send + Sync {
            fn send_request_sync(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
        }

        struct SimpleMockHttpClient;

        impl SimpleHttpClient for SimpleMockHttpClient {
            fn send_request_sync(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
                println!("发送请求: {} {}", request.method, request.path);
                Ok(HttpResponse {
                    status: 200,
                    body: "Mock response".to_string(),
                })
            }
        }
    }

    // 场景 2: 数据库查询中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod database_lifetimes {
        use std::collections::HashMap;

        #[derive(Debug)]
        pub struct QueryResult {
            rows: Vec<HashMap<String, String>>,
        }

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait Database: Send + Sync {
            async fn execute_query(&self, query: &str) -> Result<QueryResult, String>;
        }

        struct MockDatabase;

        impl Database for MockDatabase {
            async fn execute_query(&self, query: &str) -> Result<QueryResult, String> {
                println!("执行查询: {}", query);
                let mut rows = Vec::new();
                let mut row = HashMap::new();
                row.insert("id".to_string(), "1".to_string());
                row.insert("name".to_string(), "测试".to_string());
                rows.push(row);
                Ok(QueryResult { rows })
            }
        }

        async fn fetch_user_by_id(
            db: &dyn Database,
            user_id: &str,
        ) -> Result<Option<HashMap<String, String>>, String> {
            let query = format!("SELECT * FROM users WHERE id = '{}'", user_id);
            let result = db.execute_query(&query).await?;

            Ok(result.rows.into_iter().next())
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleDatabase: Send + Sync {
            fn execute_query_sync(&self, query: &str) -> Result<QueryResult, String>;
        }

        struct SimpleMockDatabase;

        impl SimpleDatabase for SimpleMockDatabase {
            fn execute_query_sync(&self, query: &str) -> Result<QueryResult, String> {
                println!("执行查询: {}", query);
                let mut rows = Vec::new();
                let mut row = HashMap::new();
                row.insert("id".to_string(), "1".to_string());
                row.insert("name".to_string(), "测试".to_string());
                rows.push(row);
                Ok(QueryResult { rows })
            }
        }
    }

    // 场景 3: 文件处理中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod file_lifetimes {
        use std::path::Path;

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait FileProcessor: Send + Sync {
            async fn read_file(&self, path: &Path) -> Result<Vec<u8>, String>;
            async fn write_file(&self, path: &Path, data: &[u8]) -> Result<(), String>;
        }

        struct MockFileProcessor;

        impl FileProcessor for MockFileProcessor {
            async fn read_file(&self, path: &Path) -> Result<Vec<u8>, String> {
                println!("读取文件: {:?}", path);
                Ok(b"mock file content".to_vec())
            }

            async fn write_file(&self, path: &Path, data: &[u8]) -> Result<(), String> {
                println!("写入文件: {:?} ({} bytes)", path, data.len());
                Ok(())
            }
        }

        async fn copy_file(
            processor: &dyn FileProcessor,
            source: &Path,
            destination: &Path,
        ) -> Result<(), String> {
            let content = processor.read_file(source).await?;
            processor.write_file(destination, &content).await?;
            Ok(())
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleFileProcessor: Send + Sync {
            fn read_file_sync(&self, path: &Path) -> Result<Vec<u8>, String>;
            fn write_file_sync(&self, path: &Path, data: &[u8]) -> Result<(), String>;
        }

        struct SimpleMockFileProcessor;

        impl SimpleFileProcessor for SimpleMockFileProcessor {
            fn read_file_sync(&self, path: &Path) -> Result<Vec<u8>, String> {
                println!("读取文件: {:?}", path);
                Ok(b"mock file content".to_vec())
            }

            fn write_file_sync(&self, path: &Path, data: &[u8]) -> Result<(), String> {
                println!("写入文件: {:?} ({} bytes)", path, data.len());
                Ok(())
            }
        }
    }

    // 场景 4: 缓存系统中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod cache_lifetimes {
        use std::collections::HashMap;
        use std::time::{Duration, Instant};

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait Cache<K, V>: Send + Sync {
            async fn get(&self, key: &K) -> Option<&V>;
            async fn put(&self, key: K, value: V) -> Result<(), String>;
        }

        struct SimpleCache<K, V> {
            data: HashMap<K, (V, Instant)>,
            ttl: Duration,
        }

        impl<K: Eq + std::hash::Hash + Clone + Send + Sync, V: Clone + Send + Sync> Cache<K, V>
        for SimpleCache<K, V> {
            async fn get(&self, key: &K) -> Option<&V> {
                if let Some((value, expires_at)) = self.data.get(key) {
                    if Instant::now() < *expires_at {
                        Some(&value.0)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            async fn put(&self, key: K, value: V) -> Result<(), String> {
                self.data.insert(key, (value, Instant::now() + self.ttl));
                Ok(())
            }
        }

        async fn cache_or_compute<F, Fut>(
            cache: &dyn Cache<String, String>,
            key: &str,
            compute_fn: F,
        ) -> Result<String, String>
        where
            F: FnOnce() -> Fut + Send,
            Fut: std::future::Future<Output = Result<String, String>> + Send,
        {
            if let Some(value) = cache.get(&key.to_string()).await {
                Ok(value.clone())
            } else {
                let value = compute_fn().await?;
                cache.put(key.to_string(), value.clone()).await?;
                Ok(value)
            }
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub struct SimpleCache<K, V> {
            data: HashMap<K, (V, Instant)>,
            ttl: Duration,
        }

        impl<K: Eq + std::hash::Hash + Clone, V: Clone> SimpleCache<K, V> {
            pub fn new(ttl: Duration) -> Self {
                SimpleCache {
                    data: HashMap::new(),
                    ttl,
                }
            }

            pub fn get_sync(&self, key: &K) -> Option<V> {
                if let Some((value, expires_at)) = self.data.get(key) {
                    if Instant::now() < *expires_at {
                        Some(value.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            pub fn put_sync(&mut self, key: K, value: V) -> Result<(), String> {
                self.data.insert(key, (value, Instant::now() + self.ttl));
                Ok(())
            }
        }
    }

    // 场景 5: 消息处理中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod message_lifetimes {
        #[derive(Debug, Clone)]
        pub struct Message {
            id: String,
            content: String,
            timestamp: u64,
        }

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait MessageHandler: Send + Sync {
            async fn handle_message(&self, message: &Message) -> Result<(), String>;
        }

        struct LoggingMessageHandler;

        impl MessageHandler for LoggingMessageHandler {
            async fn handle_message(&self, message: &Message) -> Result<(), String> {
                println!("处理消息 {}: {}", message.id, message.content);
                Ok(())
            }
        }

        async fn process_message_batch(
            handler: &dyn MessageHandler,
            messages: &[Message],
        ) -> Result<Vec<String>, String> {
            let mut results = Vec::new();
            for message in messages {
                handler.handle_message(message).await?;
                results.push(format!("已处理消息 {}", message.id));
            }
            Ok(results)
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleMessageHandler: Send + Sync {
            fn handle_message_sync(&self, message: &Message) -> Result<(), String>;
        }

        struct LoggingMessageHandler;

        impl SimpleMessageHandler for LoggingMessageHandler {
            fn handle_message_sync(&self, message: &Message) -> Result<(), String> {
                println!("处理消息 {}: {}", message.id, message.content);
                Ok(())
            }
        }
    }

    // 场景 6: 流处理中的生命周期
    // 注意：包含 async 方法的 trait 目前不能用于 dyn dispatch
    mod stream_lifetimes {
        use std::future::Future;

        // 注意：以下 trait 定义在当前 Rust 版本中无法用于 dyn dispatch
        /*
        pub trait StreamProcessor<T>: Send + Sync {
            async fn process_item(&self, item: &T) -> Result<String, String>;
        }

        struct StringProcessor;

        impl StreamProcessor<String> for StringProcessor {
            async fn process_item(&self, item: &String) -> Result<String, String> {
                Ok(item.to_uppercase())
            }
        }

        async fn process_stream<T, P>(
            processor: &P,
            stream: &[T],
        ) -> Result<Vec<String>, String>
        where
            P: StreamProcessor<T> + ?Sized,
        {
            let mut results = Vec::new();
            for item in stream {
                let processed = processor.process_item(item).await?;
                results.push(processed);
            }
            Ok(results)
        }
        */

        // 提供一个不使用 async 方法的替代方案
        pub trait SimpleStreamProcessor<T>: Send + Sync {
            fn process_item_sync(&self, item: &T) -> Result<String, String>;
        }

        struct StringProcessor;

        impl SimpleStreamProcessor<String> for StringProcessor {
            fn process_item_sync(&self, item: &String) -> Result<String, String> {
                Ok(item.to_uppercase())
            }
        }
    }

    println!("异步函数生命周期实际应用示例演示完成");
}

// ===========================================
// 主函数
// ===========================================

#[tokio::main]
pub async fn main() {
    println!("Rust 异步编程与 async/await 演示");
    println!("===================================");

    async_concepts();
    future_trait_basics();
    async_await_syntax().await;
    async_executors().await;
    async_io_operations().await;
    async_streams().await;
    error_handling_and_timeouts().await;
    async_patterns_and_best_practices().await;
    testing_async_code().await;
    practical_examples().await;
    async_trait_methods().await;
    async_function_lifetimes();
    practical_async_lifetime_examples();

    println!("异步编程演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;
    // use tokio_test; // tokio_test 需要额外的依赖

    #[tokio::test]
    async fn test_async_syntax() {
        async fn test_async() -> i32 {
            42
        }

        assert_eq!(test_async().await, 42);
    }

    #[tokio::test]
    async fn test_error_handling() {
        async fn successful_operation() -> Result<i32, String> {
            Ok(42)
        }

        async fn failing_operation() -> Result<i32, String> {
            Err("失败".to_string())
        }

        assert!(successful_operation().await.is_ok());
        assert!(failing_operation().await.is_err());
    }

    #[tokio::test]
    async fn test_timeout() {
        async fn quick_operation() -> String {
            "快速完成".to_string()
        }

        async fn slow_operation() -> String {
            sleep(Duration::from_millis(100)).await;
            "慢速完成".to_string()
        }

        // 快速操作不应该超时
        let result = tokio::time::timeout(Duration::from_millis(200), quick_operation()).await;
        assert!(result.is_ok());

        // 慢速操作应该超时
        let result = tokio::time::timeout(Duration::from_millis(50), slow_operation()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let handles = (0..5)
            .map(|i| {
                tokio::spawn(async move {
                    sleep(Duration::from_millis(10)).await;
                    i * 2
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = futures::future::join_all(handles).await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(results, vec![0, 2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_async_cache() {
        // 注意：AsyncCache 类型在上面的代码中定义
        // 由于 async trait 的限制，这里创建一个简单的缓存测试
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        #[derive(Clone)]
        struct SimpleCache {
            cache: Arc<RwLock<HashMap<String, String>>>,
        }

        impl SimpleCache {
            fn new() -> Self {
                Self {
                    cache: Arc::new(RwLock::new(HashMap::new())),
                }
            }

            async fn get_or_insert<F, Fut>(&self, key: String, factory: F) -> String
            where
                F: FnOnce() -> Fut,
                Fut: std::future::Future<Output = String>,
            {
                {
                    let cache = self.cache.read().await;
                    if let Some(value) = cache.get(&key) {
                        return value.clone();
                    }
                }

                let value = factory().await;
                let mut cache = self.cache.write().await;
                cache.insert(key.clone(), value.clone());
                value
            }
        }

        let cache = SimpleCache::new();

        let result1 = cache.get_or_insert(
            "test_key".to_string(),
            || async { "computed_value".to_string() },
        ).await;

        let result2 = cache.get_or_insert(
            "test_key".to_string(),
            || async { "should_not_be_called".to_string() },
        ).await;

        assert_eq!(result1, "computed_value");
        assert_eq!(result2, "computed_value");
    }
}