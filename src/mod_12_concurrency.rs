#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Rust 并发编程（Concurrency）
// 深入讲解线程、通道、共享状态、原子操作等并发编程特性

use std::sync::Arc;

// ===========================================
// 1. 线程基础 (Thread Basics)
// ===========================================

// 线程是并发编程的基础单位，Rust 提供了轻量级的操作系统线程支持
// Rust 的线程模型基于 1:1 的绿色线程映射，每个 Rust 线程对应一个操作系统线程
// 这种设计提供了良好的性能和可预测的行为，同时避免了某些语言中的绿色线程陷阱

fn thread_basics() {
    println!("=== 线程基础 ===");

    // 创建新线程
    // thread::spawn 创建一个新线程，返回一个 JoinHandle
    // 闭包中的代码会在新线程中异步执行
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        println!("新线程开始运行");
        thread::sleep(Duration::from_millis(100));
        println!("新线程结束运行");
    });

    println!("主线程继续运行");

    // 等待线程完成
    // join() 方法阻塞当前线程，直到目标线程完成执行
    // 这是线程同步的基本机制，确保主线程等待子线程完成
    handle.join().unwrap();
    println!("主线程继续执行");

    // 带参数的线程 - move 闭包
    // move 关键字强制闭包获取其捕获变量的所有权
    // 这是必要的，因为新线程可能比创建它的线程更长寿
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("线程接收向量: {:?}", v);
        // v 的所有权被移动到新线程中
        // 这样确保了数据的安全访问，避免了数据竞争
    });

    handle.join().unwrap();

    // 线程命名和配置
    // 使用 Builder 模式配置线程的属性
    // 线程名称在调试和日志记录时很有用
    let builder = thread::Builder::new().name("my_worker".to_string());
    let handle = builder
        .spawn(|| {
            println!("命名线程: {:?}", thread::current().name());
        })
        .unwrap();

    handle.join().unwrap();

    // 获取线程标识
    // 每个线程都有唯一的 ID，可以用于线程识别和调试
    let main_id = thread::current().id();
    println!("主线程 ID: {:?}", main_id);

    let handle = thread::spawn(|| {
        println!("工作线程 ID: {:?}", thread::current().id());
    });

    handle.join().unwrap();

    // 线程管理的最佳实践：
    // 1. 总是使用 move 闭包来避免借用检查问题
    // 2. 合理使用 join() 确保线程完成，避免僵尸线程
    // 3. 为线程设置有意义的名称，便于调试
    // 4. 注意线程的创建开销，避免创建过多短命线程
    // 5. 使用线程池来管理大量并发任务

    // Rust 线程的优势：
    // 1. 内存安全：编译时保证没有数据竞争
    // 2. 零成本抽象：没有运行时开销
    // 3. 可组合性：与其他并发原语良好集成
    // 4. 错误处理：通过 Result 处理线程创建失败

    println!();
}

// ===========================================
// 2. 通道 (Channels)
// ===========================================

// 通道是 Rust 中线程间通信的主要机制，实现了消息传递并发模型
// "Do not communicate by sharing memory; instead, share memory by communicating"
// 这是 Go 语言的格言，也是 Rust 并发设计的核心哲学

fn channels() {
    println!("=== 通道 ===");

    // mpsc: Multiple Producer, Single Consumer
    // 创建一个多生产者、单消费者的通道
    use std::sync::mpsc;
    use std::thread;

    // 创建通道
    // channel() 返回发送端 (tx) 和接收端 (rx)
    // 发送端可以克隆以支持多个发送者
    let (tx, rx) = mpsc::channel();

    // 发送端移动到新线程
    // send() 方法转移数据的所有权到通道中
    // 这样确保了数据的安全传输，避免了数据竞争
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        println!("发送完成");
        // val 的所有权已被转移，不能再使用
    });

    // 接收消息
    // recv() 方法阻塞等待消息到达
    // 当通道关闭且没有消息时，返回错误
    let received = rx.recv().unwrap();
    println!("收到: {}", received);

    // 发送多个值
    // 发送端可以连续发送多个消息
    // 接收端可以逐个接收这些消息
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
        // 当发送端被丢弃时，通道会自动关闭
    });

    // 接收多个值
    // rx 实现了 Iterator trait，可以用于循环
    // 当通道关闭且没有消息时，循环会自动结束
    for received in rx {
        println!("收到: {}", received);
    }

    // 多个发送者（多生产者模式）
    // 通过克隆发送端，多个线程可以发送消息到同一个通道
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("sender 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("sender 2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // 重要：必须丢弃原始的发送端，否则通道永远不会关闭
    drop(tx);

    // 等待发送线程完成
    handle1.join().unwrap();
    handle2.join().unwrap();

    // 所有发送者完成后，接收循环会自动结束
    for received in rx {
        println!("收到: {}", received);
    }

    // 通道的类型和特性：
    // 1. mpsc::channel<T>: 多生产者，单消费者
    // 2. mpsc::sync_channel<T>: 同步通道，有缓冲区限制
    // 3. oneshot::channel<T>: 一次性通道，只能发送一次

    // 通道的最佳实践：
    // 1. 使用 move 闭包发送数据的所有权
    // 2. 合理处理 send() 和 recv() 的错误
    // 3. 使用 drop() 显式关闭通道
    // 4. 考虑使用同步通道来控制流量
    // 5. 使用通道组合而不是复杂的共享状态

    // 通道与共享状态的比较：
    // 优势：避免数据竞争、消息边界清晰、错误传播容易
    // 劣势：需要复制数据、可能有性能开销、设计复杂

    println!();
}

// ===========================================
// 3. 共享状态并发 (Shared State Concurrency)
// ===========================================

// 虽然消息传递是推荐的并发模型，但有时共享状态更合适或更高效
// Rust 通过 Mutex 和 Arc 提供了安全的共享状态并发机制
// 这些类型在编译时保证了内存安全，避免了数据竞争

fn shared_state_concurrency() {
    println!("=== 共享状态并发 ===");

    // Mutex: Mutual Exclusion（互斥锁）
    // Mutex 确保同一时间只有一个线程可以访问数据
    // 通过 lock() 方法获取锁，返回 MutexGuard
    // MutexGuard 实现了 Drop trait，离开作用域时自动释放锁
    use std::sync::Mutex;
    use std::thread;

    // 使用 Mutex 保护共享数据
    let counter = Mutex::new(0);
    let mut handles = vec![];

    // 注意：下面的代码有一个错误！
    // counter.clone() 不会工作，因为 Mutex 没有实现 Clone
    // 这是 Rust 编译器在保护我们免于并发错误
    // 我们需要使用 Arc 来共享 Mutex
    let counter = Arc::new(counter);
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    // 使用 Arc 实现多线程共享
    // Arc: Atomic Reference Counting（原子引用计数）
    // Arc 允许多个线程共享同一个数据的所有权
    // Arc 的引用计数是原子操作，保证了线程安全
    use std::sync::Arc;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁
            // lock() 可能阻塞，直到其他线程释放锁
            // lock() 返回 Result<MutexGuard, PoisonError<T>>
            // unwrap() 处理可能的错误（通常不应该发生）
            let mut num = counter.lock().unwrap();
            *num += 1;
            // 当 num 离开作用域时，锁会自动释放
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc 计数: {}", *counter.lock().unwrap());

    // Mutex 的关键特性：
    // 1. 互斥性：同一时间只能有一个线程访问数据
    // 2. 中毒保护：如果持有锁的线程 panic， Mutex 会中毒
    // 3. 自动释放：MutexGuard 实现了 Drop，确保锁总是被释放
    // 4. 错误处理：lock() 返回 Result，可以处理中毒情况

    // Arc 的使用场景：
    // 1. 多线程需要共享数据的所有权
    // 2. 只读访问比写入访问更频繁
    // 3. 需要在线程间传递复杂的数据结构

    // Mutex + Arc 的组合模式：
    // 1. Arc 提供共享所有权
    // 2. Mutex 提供互斥访问
    // 3. 一起使用实现线程安全的共享状态

    // 共享状态的最佳实践：
    // 1. 保持临界区尽可能小（减少锁的持有时间）
    // 2. 避免在持有锁时进行阻塞操作
    // 3. 按一致的顺序获取多个锁以避免死锁
    // 4. 使用 try_lock() 来避免阻塞，如果可能的话
    // 5. 考虑使用 RwLock 进行读写分离

    // 共享状态的陷阱：
    // 1. 死锁：多个线程相互等待对方释放锁
    // 2. 饥饿：某个线程永远无法获得锁
    // 3. 性能问题：过多的锁争用会影响性能
    // 4. 复杂性：共享状态代码难以推理和调试

    println!();
}

// ===========================================
// 4. 原子操作 (Atomic Operations)
// ===========================================

// 原子操作是并发编程中的基础构建块，它们提供了无锁的并发访问
// 原子操作在单个 CPU 指令中完成，不会被其他线程中断
// 这是实现更复杂并发原语（如 Mutex）的基础

fn atomic_operations() {
    println!("=== 原子操作 ===");

    // 原子类型提供无锁的并发访问
    // 它们通过特殊的 CPU 指令确保操作的原子性
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    // 原子计数器 - 无锁并发
    // fetch_add 是原子操作，多个线程可以安全地递增计数器
    // 这比使用 Mutex 更高效，特别是在高并发场景下
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("原子计数: {}", counter.load(Ordering::SeqCst));

    // 比较交换操作 (Compare-and-Swap)
    // 这是实现无锁算法的核心操作
    // 它读取当前值，如果与期望值相等，则设置为新值
    let value = AtomicUsize::new(5);

    let old_value = value.compare_exchange(
        5,                // 期望值 (expected)
        10,               // 新值 (new)
        Ordering::SeqCst, // 成功时的内存顺序
        Ordering::SeqCst, // 失败时的内存顺序
    );

    match old_value {
        Ok(v) => println!("比较交换成功，旧值: {}", v),
        Err(v) => println!("比较交换失败，当前值: {}", v),
    }

    println!("最终值: {}", value.load(Ordering::SeqCst));

    // 原子布尔值 - 用于状态标志
    // 原子布尔常用于线程间的状态同步
    use std::sync::atomic::AtomicBool;
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100));
        flag_clone.store(true, Ordering::SeqCst);
        println!("设置标志位");
    });

    // 轮询等待状态变化
    while !flag.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(10));
    }
    println!("检测到标志位变化");

    handle.join().unwrap();

    // 内存顺序 (Memory Ordering)
    // Ordering 参数控制原子操作的内存可见性保证
    // SeqCst: Sequential Consistency - 最强保证，但性能开销最大
    // Acquire/Release: 用于获取和释放语义
    // Relaxed: 最弱保证，只保证操作的原子性

    // 常用的原子操作：
    // 1. load() - 原子读取
    // 2. store() - 原子写入
    // 3. fetch_add() - 原子加法
    // 4. fetch_sub() - 原子减法
    // 5. swap() - 原子交换
    // 6. compare_exchange() - 原子比较交换
    // 7. compare_exchange_weak() - 弱比较交换（可能在循环中使用）

    // 原子操作的优势：
    // 1. 无锁：避免了锁的开销和争用
    // 2. 高效：在适当的情况下比 Mutex 更快
    // 3. 组合性：可以构建更复杂的无锁算法
    // 4. 死锁安全：不会发生死锁

    // 原子操作的注意事项：
    // 1. 复杂性：无锁算法难以正确实现
    // 2. ABA 问题：需要额外的机制来避免
    // 3. 内存顺序：需要理解内存模型以保证正确性
    // 4. 适用性：不是所有场景都适合使用原子操作

    // 原子操作 vs Mutex：
    // 原子操作：高性能、复杂、适用于简单操作
    // Mutex：简单、安全、适用于复杂操作

    println!();
}

// ===========================================
// 5. 条件变量 (Condition Variables)
// ===========================================

// 条件变量（Condition Variables）是并发编程中用于线程间同步的高级原语
// 它允许线程在特定条件不满足时进入睡眠状态，直到被其他线程唤醒
// 这种机制避免了忙等待（busy waiting）带来的 CPU 资源浪费
//
// 条件变量的核心概念：
// - 条件检查：线程在等待前必须检查条件
// - 原子性：条件检查和等待必须是原子操作
// - 通知机制：线程可以通知等待的线程条件已改变
// - 虚假唤醒：线程可能在未被通知的情况下被唤醒，需要重新检查条件

fn condition_variables() {
    println!("=== 条件变量 ===");

    // 基本条件变量模式
    // 条件变量通常与互斥锁一起使用，确保条件检查的原子性
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // 生产者线程：设置条件并通知等待者
    thread::spawn(move || {
        println!("生产者线程：开始处理任务");
        // 模拟一些工作
        thread::sleep(Duration::from_millis(100));

        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true; // 设置条件为 true
        println!("生产者线程：条件已满足，通知等待者");
        cvar.notify_one(); // 唤醒一个等待的线程
    });

    // 消费者线程：等待条件满足
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    // 经典的条件变量使用模式：while 循环检查条件
    // 为什么使用 while 而不是 if？
    // 1. 防止虚假唤醒（spurious wakeup）
    // 2. 确保条件在等待期间未被其他线程改变
    // 3. 提供更强的安全性保证
    while !*started {
        println!("消费者线程：等待条件满足...");
        // wait() 方法会自动释放锁，让其他线程可以修改条件
        // 被唤醒时，wait() 会重新获取锁并返回新的 MutexGuard
        started = cvar.wait(started).unwrap();
    }
    println!("消费者线程：条件已满足，继续执行！");

    // 条件变量的实际应用：生产者-消费者模式
    println!("\n--- 生产者-消费者模式示例 ---");
    producer_consumer_example();

    // 条件变量的注意事项和最佳实践：
    // 1. 始终在持有锁的情况下调用 wait() 和 notify()
    // 2. 使用 while 循环检查条件，而不是 if
    // 3. 避免在持有锁的情况下执行耗时操作
    // 4. 注意死锁的可能性
    // 5. 考虑使用 notify_all() 而不是 notify_one() 当多个线程等待同一条件

    println!();
}

// 生产者-消费者模式的完整实现
// 这是条件变量最经典的应用场景之一
fn producer_consumer_example() {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    // 共享缓冲区：包含队列、互斥锁和条件变量
    let buffer = Arc::new(Mutex::new(Vec::<i32>::new()));
    let not_empty = Arc::new(Condvar::new()); // 缓冲区非空条件
    let not_full = Arc::new(Condvar::new()); // 缓冲区未满条件
    const MAX_SIZE: usize = 5;

    // 生产者线程
    let producer_buffer = buffer.clone();
    let producer_not_empty = not_empty.clone();
    let producer_not_full = not_full.clone();
    thread::spawn(move || {
        for i in 1..=10 {
            let mut buffer = producer_buffer.lock().unwrap();

            // 等待缓冲区有空间
            while buffer.len() >= MAX_SIZE {
                println!("生产者：缓冲区已满，等待消费者...");
                buffer = producer_not_full.wait(buffer).unwrap();
            }

            // 生产数据
            buffer.push(i);
            println!("生产者：生产了数据 {}", i);

            // 通知消费者缓冲区非空
            producer_not_empty.notify_one();

            // 释放锁，模拟生产时间
            drop(buffer);
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 消费者线程
    let consumer_buffer = buffer.clone();
    let consumer_not_empty = not_empty.clone();
    let consumer_not_full = not_full.clone();
    thread::spawn(move || {
        for _ in 1..=10 {
            let mut buffer = consumer_buffer.lock().unwrap();

            // 等待缓冲区有数据
            while buffer.is_empty() {
                println!("消费者：缓冲区为空，等待生产者...");
                buffer = consumer_not_empty.wait(buffer).unwrap();
            }

            // 消费数据
            let item = buffer.remove(0);
            println!("消费者：消费了数据 {}", item);

            // 通知生产者缓冲区未满
            consumer_not_full.notify_one();

            // 释放锁，模拟消费时间
            drop(buffer);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 等待线程完成
    thread::sleep(Duration::from_secs(2));
}

// ===========================================
// 6. 读写锁 (RwLock)
// ===========================================

// 读写锁（RwLock - Read-Write Lock）是一种特殊的锁，允许多个读取者或单个写入者
// 这种锁适用于读多写少的场景，可以显著提高并发性能
//
// RwLock 的核心特性：
// - 读取锁（Read Lock）：多个线程可以同时持有读取锁
// - 写入锁（Write Lock）：同一时间只有一个线程可以持有写入锁
// - 锁升级：读取锁不能直接升级为写入锁（可能导致死锁）
// - 锁降级：写入锁可以降级为读取锁
//
// RwLock 的工作原理：
// 1. 当没有写入者时，多个读取者可以同时访问数据
// 2. 当有写入者时，所有其他访问（包括读取）都会被阻塞
// 3. 写入者优先级通常高于读取者，防止写入饥饿

fn rwlock_usage() {
    println!("=== 读写锁 ===");

    use std::sync::RwLock;
    use std::thread;
    use std::time::Duration;

    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 创建多个读取线程 - 演示并发读取
    // 读取锁允许多个线程同时读取数据
    // 这是 RwLock 相对于 Mutex 的主要优势
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // 获取读取锁 - read() 方法返回 RwLockReadGuard
            let num = data.read().unwrap();
            println!("读取线程 {} 开始读取数据: {}", i, *num);

            // 模拟读取操作 - 可以并发进行
            thread::sleep(Duration::from_millis(100));

            println!("读取线程 {} 完成读取", i);
            // RwLockReadGuard 离开作用域时自动释放读取锁
        });
        handles.push(handle);
    }

    // 创建写入线程 - 演示独占写入
    // 写入锁确保同一时间只有一个线程可以修改数据
    for i in 0..2 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // 获取写入锁 - write() 方法返回 RwLockWriteGuard
            let mut num = data.write().unwrap();
            println!("写入线程 {} 获取写入锁，当前值: {}", i, *num);

            // 模拟写入操作 - 独占进行
            *num += 1;
            thread::sleep(Duration::from_millis(50));

            println!("写入线程 {} 完成写入，新值: {}", i, *num);
            // RwLockWriteGuard 离开作用域时自动释放写入锁
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *data.read().unwrap());

    // RwLock 的实际应用：缓存系统
    println!("\n--- 缓存系统示例 ---");
    cache_system_example();

    // RwLock 的适用场景和注意事项：
    // 适用场景：
    // 1. 读多写少的数据结构
    // 2. 需要高并发读取的系统
    // 3. 配置信息、统计数据等
    // 4. 缓存系统
    //
    // 注意事项：
    // 1. 写入者可能被读取者饿死（写入饥饿）
    // 2. 不能直接升级读取锁为写入锁
    // 3. 锁的争用会影响性能
    // 4. 死锁风险：不当的锁顺序会导致死锁

    println!();
}

// 缓存系统示例：RwLock 的实际应用
// 演示如何使用 RwLock 实现一个简单的缓存系统
fn cache_system_example() {
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    // 缓存数据结构
    #[derive(Debug)]
    struct Cache<K, V> {
        data: RwLock<HashMap<K, V>>,
        hits: RwLock<u64>,
        misses: RwLock<u64>,
    }

    impl<K, V> Cache<K, V>
    where
        K: std::hash::Hash + Eq + Clone + std::fmt::Display,
        V: Clone + std::fmt::Display,
    {
        fn new() -> Self {
            Cache {
                data: RwLock::new(HashMap::new()),
                hits: RwLock::new(0),
                misses: RwLock::new(0),
            }
        }

        // 读取缓存 - 使用读取锁
        fn get(&self, key: &K) -> Option<V> {
            let data = self.data.read().unwrap();
            match data.get(key) {
                Some(value) => {
                    *self.hits.write().unwrap() += 1;
                    Some(value.clone())
                }
                None => {
                    *self.misses.write().unwrap() += 1;
                    None
                }
            }
        }

        // 写入缓存 - 使用写入锁
        fn put(&self, key: K, value: V) {
            let mut data = self.data.write().unwrap();
            data.insert(key, value);
        }

        // 获取命中率统计 - 使用读取锁
        fn stats(&self) -> (u64, u64, f64) {
            let hits = *self.hits.read().unwrap();
            let misses = *self.misses.read().unwrap();
            let total = hits + misses;
            let hit_rate = if total > 0 {
                hits as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            (hits, misses, hit_rate)
        }
    }

    let cache = Arc::new(Cache::new());
    let mut handles = vec![];

    // 初始化缓存
    cache.put("key1".to_string(), "value1".to_string());
    cache.put("key2".to_string(), "value2".to_string());

    // 读取线程 - 模拟高并发读取
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let key = format!("key{}", (j % 3) + 1); // key1, key2, key3
                match cache.get(&key) {
                    Some(value) => println!("读取线程 {} 找到 {}: {}", i, key, value),
                    None => println!("读取线程 {} 未找到 {}", i, key),
                }
                thread::sleep(Duration::from_millis(20));
            }
        });
        handles.push(handle);
    }

    // 写入线程 - 模拟偶尔的缓存更新
    for i in 0..2 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let key = format!("key{}", j + 1);
                let value = format!("new_value_{}_{}", i, j);
                cache.put(key.clone(), value);
                println!("写入线程 {} 更新 {}", i, key);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 显示缓存统计信息
    let (hits, misses, hit_rate) = cache.stats();
    println!(
        "缓存统计: 命中={}, 未命中={}, 命中率={:.2}%",
        hits, misses, hit_rate
    );
}

// ===========================================
// 7. Barrier 屏障
// ===========================================

// Barrier（屏障）是并发编程中的同步原语，用于让一组线程在某个点等待
// 直到所有线程都到达该点后，才能继续执行
// 这种机制在并行算法中很常见，特别是在分阶段处理数据时
//
// Barrier 的核心概念：
// - 等待点：线程在 Barrier 处等待其他线程
// - 集体同步：所有线程到达后才能继续
// - 可重用：Barrier 可以被重复使用
// - 阶段划分：将计算划分为明确的阶段
//
// Barrier 的工作原理：
// 1. 创建 Barrier 时指定需要等待的线程数量
// 2. 每个线程调用 wait() 方法时会阻塞
// 3. 当指定数量的线程都调用了 wait() 后，所有线程被唤醒
// 4. Barrier 重置，可以用于下一轮同步

fn barrier_usage() {
    println!("=== Barrier 屏障 ===");

    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    // 创建 Barrier，指定需要等待 3 个线程
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    // 创建 3 个工作线程，演示 Barrier 的同步效果
    for i in 0..3 {
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            println!("线程 {} 开始第一阶段工作", i);

            // 模拟第一阶段的工作
            thread::sleep(Duration::from_millis(100 * (i + 1)));
            println!("线程 {} 完成第一阶段，到达 Barrier", i);

            // 等待其他线程
            // wait() 方法返回一个 BarrierWaitResult，表示是否是最后一个到达的线程
            let is_leader = barrier.wait().is_leader();
            if is_leader {
                println!("线程 {} 是最后一个到达的", i);
            }

            println!("线程 {} 开始第二阶段工作", i);

            // 模拟第二阶段的工作
            thread::sleep(Duration::from_millis(50));
            println!("线程 {} 完成第二阶段", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Barrier 的实际应用：并行数据处理
    println!("\n--- 并行数据处理示例 ---");
    parallel_data_processing();

    // Barrier 的适用场景：
    // 1. 并行算法的同步点（如矩阵运算）
    // 2. 分阶段的数据处理流水线
    // 3. 需要定期同步的并行计算
    // 4. 游戏中的回合制同步
    // 5. 模拟程序中的时间步进
    //
    // Barrier 的优势：
    // 1. 提供了清晰的同步点
    // 2. 避免忙等待，使用系统级的阻塞
    // 3. 可以重复使用
    // 4. 简化了复杂的同步逻辑
    //
    // 注意事项：
    // 1. 确保正确设置线程数量
    // 2. 避免在 Barrier 处执行耗时操作
    // 3. 注意线程异常可能导致的死锁
    // 4. 考虑使用超时机制避免无限等待

    println!();
}

// 并行数据处理示例：Barrier 的实际应用
// 演示如何使用 Barrier 实现分阶段的并行数据处理
fn parallel_data_processing() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    // 模拟数据：将一个大数组分成多个部分并行处理
    let data: Vec<i32> = (1..=100).collect();
    let num_threads = 4;
    let chunk_size = data.len() / num_threads;

    // 创建 Barrier 用于阶段同步
    let barrier = Arc::new(Barrier::new(num_threads));
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut handles = vec![];

    // 创建工作线程
    for i in 0..num_threads {
        let barrier = barrier.clone();
        let results = results.clone();
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            data.len()
        } else {
            (i + 1) * chunk_size
        };
        let chunk = data[start..end].to_vec();

        let handle = thread::spawn(move || {
            // 第一阶段：计算局部和
            println!("线程 {} 开始处理数据块 [{}, {})", i, start, end);
            let local_sum: i32 = chunk.iter().sum();
            let local_avg = local_sum as f64 / chunk.len() as f64;

            println!(
                "线程 {} 局部计算完成：sum={}, avg={:.2}",
                i, local_sum, local_avg
            );

            // 等待所有线程完成第一阶段
            barrier.wait();
            println!("线程 {} 开始第二阶段", i);

            // 第二阶段：处理全局结果（这里只是示例）
            let processed_result = format!("Thread {}: sum={}, avg={:.2}", i, local_sum, local_avg);

            // 等待所有线程完成第二阶段
            barrier.wait();
            println!("线程 {} 开始第三阶段", i);

            // 第三阶段：汇总结果
            results.lock().unwrap().push(processed_result);

            println!("线程 {} 完成", i);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 显示结果
    let final_results = results.lock().unwrap();
    println!("\n并行处理结果：");
    for result in final_results.iter() {
        println!("  {}", result);
    }
}

// ===========================================
// 8. 并发模式 (Concurrency Patterns)
// ===========================================

// 并发模式是经过验证的解决特定并发问题的设计方案
// 这些模式提供了标准化的方法来处理常见的并发挑战
// 理解这些模式对于构建高效、安全的并发系统至关重要
//
// 常见的并发模式：
// 1. 生产者-消费者模式（Producer-Consumer）
// 2. 工作线程池模式（Worker Pool）
// 3. 读写模式（Read-Write Lock）
// 4. 期货和承诺模式（Future-Promise）
// 5. 监视器模式（Monitor）
// 6. 主从模式（Master-Worker）
// 7. 管道模式（Pipeline）
// 8. 事件循环模式（Event Loop）

fn concurrency_patterns() {
    println!("=== 并发模式 ===");

    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 模式1：生产者-消费者模式
    // 这是最经典的并发模式之一，用于解耦数据的产生和处理
    // 生产者线程生成数据，消费者线程处理数据，通过通道进行通信
    println!("\n--- 生产者-消费者模式 ---");
    producer_consumer_pattern();

    // 模式2：工作线程池模式
    // 创建固定数量的工作线程来处理任务队列，避免频繁创建销毁线程
    // 这种模式适用于需要处理大量短期任务的场景
    println!("\n--- 工作线程池模式 ---");
    worker_pool_pattern();

    // 模式3：主从模式（Master-Worker）
    // 主线程负责分发任务，工作线程负责执行任务
    // 这种模式适用于并行计算和任务分发
    println!("\n--- 主从模式 ---");
    master_worker_pattern();

    // 并发模式的选择原则：
    // 1. 任务特性：CPU密集型 vs IO密集型
    // 2. 数据流：生产者和消费者的关系
    // 3. 资源限制：线程数量、内存使用
    // 4. 错误处理：如何处理失败的线程
    // 5. 可扩展性：系统的扩展需求
    //
    // Rust 中的并发优势：
    // 1. 类型安全：编译时保证并发安全
    // 2. 零成本抽象：无运行时开销
    // 3. 所有权系统：避免数据竞争
    // 4. 错误处理：强制的错误处理机制

    println!();
}

// 生产者-消费者模式的实现
fn producer_consumer_pattern() {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 创建多生产者、单消费者通道
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // 启动多个生产者线程
    let producer_handles: Vec<_> = (0..3)
        .map(|producer_id| {
            let tx = tx.clone();
            thread::spawn(move || {
                for item_id in 0..5 {
                    let item = format!("生产者{}-商品{}", producer_id, item_id);
                    println!("生产者{}：生产商品 {}", producer_id, item_id);

                    // 发送商品到通道
                    tx.send(item).unwrap();

                    // 模拟生产时间
                    thread::sleep(Duration::from_millis(50));
                }
                println!("生产者{}：完成生产", producer_id);
            })
        })
        .collect();

    // 启动消费者线程
    let consumer_handle = {
        let rx = Arc::clone(&rx);
        thread::spawn(move || {
            let mut consumed_count = 0;
            let total_items = 15; // 3个生产者 × 5个商品

            loop {
                match rx.lock().unwrap().recv() {
                    Ok(item) => {
                        println!("消费者：处理 {}", item);
                        consumed_count += 1;

                        if consumed_count >= total_items {
                            println!("消费者：处理完所有商品");
                            break;
                        }
                    }
                    Err(_) => {
                        println!("消费者：通道已关闭");
                        break;
                    }
                }

                // 模拟处理时间
                thread::sleep(Duration::from_millis(30));
            }
        })
    };

    // 等待所有生产者完成
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // 关闭通道，通知消费者结束
    drop(tx);

    // 等待消费者完成
    consumer_handle.join().unwrap();
}

// 工作线程池模式的实现
fn worker_pool_pattern() {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 任务定义
    #[derive(Debug)]
    enum Task {
        Compute(i32, i32), // 计算任务
        Sleep(u64),        // 休眠任务
        Print(String),     // 打印任务
    }

    // 任务结果
    #[derive(Debug)]
    struct TaskResult {
        task_id: u32,
        worker_id: usize,
        result: String,
    }

    // 创建任务通道和结果通道
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let task_rx = Arc::new(Mutex::new(task_rx));
    let result_tx = Arc::new(Mutex::new(result_tx));

    // 创建工作线程池
    let num_workers = 3;
    let worker_handles: Vec<_> = (0..num_workers)
        .map(|worker_id| {
            let task_rx = Arc::clone(&task_rx);
            let result_tx = Arc::clone(&result_tx);

            thread::spawn(move || {
                println!("工作线程 {}：启动", worker_id);

                while let Ok(task) = task_rx.lock().unwrap().recv() {
                    let result = match task {
                        Task::Compute(a, b) => {
                            thread::sleep(Duration::from_millis(50));
                            format!("计算结果: {} + {} = {}", a, b, a + b)
                        }
                        Task::Sleep(ms) => {
                            thread::sleep(Duration::from_millis(ms));
                            format!("休眠了 {} 毫秒", ms)
                        }
                        Task::Print(msg) => {
                            println!("工作线程 {}：{}", worker_id, msg);
                            format!("打印消息: {}", msg)
                        }
                    };

                    let task_result = TaskResult {
                        task_id: worker_id as u32,
                        worker_id,
                        result,
                    };

                    result_tx.lock().unwrap().send(task_result).unwrap();
                }

                println!("工作线程 {}：退出", worker_id);
            })
        })
        .collect();

    // 分发任务
    let tasks = vec![
        Task::Compute(10, 20),
        Task::Print("Hello from pool!".to_string()),
        Task::Sleep(100),
        Task::Compute(5, 3),
        Task::Print("Processing complete".to_string()),
        Task::Compute(100, 200),
    ];

    for (task_id, task) in tasks.into_iter().enumerate() {
        println!("任务分发器：发送任务 {}", task_id);
        task_tx.send(task).unwrap();
        thread::sleep(Duration::from_millis(20));
    }

    // 关闭任务通道，让工作线程优雅退出
    drop(task_tx);

    // 等待所有工作线程完成
    for handle in worker_handles {
        handle.join().unwrap();
    }

    // 关闭结果通道
    drop(result_tx);

    // 收集并显示结果
    println!("工作线程池结果：");
    for result in result_rx.try_iter() {
        println!("  工作线程 {}: {}", result.worker_id, result.result);
    }
}

// 主从模式的实现
fn master_worker_pattern() {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 并行矩阵乘法示例
    #[derive(Debug)]
    struct Matrix {
        data: Vec<Vec<f64>>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        fn new(rows: usize, cols: usize) -> Self {
            Matrix {
                data: vec![vec![0.0; cols]; rows],
                rows,
                cols,
            }
        }

        fn identity(size: usize) -> Self {
            let mut matrix = Matrix::new(size, size);
            for i in 0..size {
                matrix.data[i][i] = 1.0;
            }
            matrix
        }
    }

    // 主线程：任务分发
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let task_rx = Arc::new(Mutex::new(task_rx));
    let result_tx = Arc::new(Mutex::new(result_tx));

    // 创建工作线程
    let num_workers = 4;
    let worker_handles: Vec<_> = (0..num_workers)
        .map(|worker_id| {
            let task_rx = Arc::clone(&task_rx);
            let result_tx = Arc::clone(&result_tx);

            thread::spawn(move || {
                while let Ok((row, col)) = task_rx.lock().unwrap().recv() {
                    // 模拟矩阵乘法计算
                    let result = (row as f64) * (col as f64);
                    thread::sleep(Duration::from_millis(10));

                    result_tx
                        .lock()
                        .unwrap()
                        .send((worker_id, row, col, result))
                        .unwrap();
                }
            })
        })
        .collect();

    // 主线程：分发任务
    let matrix_size = 6;
    for row in 0..matrix_size {
        for col in 0..matrix_size {
            task_tx.send((row, col)).unwrap();
        }
    }

    // 关闭任务通道
    drop(task_tx);

    // 等待工作线程完成
    for handle in worker_handles {
        handle.join().unwrap();
    }

    // 关闭结果通道
    drop(result_tx);

    // 收集结果
    let mut results = Vec::new();
    for result in result_rx.try_iter() {
        results.push(result);
    }

    // 显示结果
    println!("主从模式计算结果：");
    println!("工作线程分配：");
    for worker_id in 0..num_workers {
        let worker_results: Vec<_> = results
            .iter()
            .filter(|(wid, _, _, _)| *wid == worker_id)
            .collect();
        println!(
            "  工作线程 {} 处理了 {} 个任务",
            worker_id,
            worker_results.len()
        );
    }
}

// ===========================================
// 9. 并发数据结构 (Concurrent Data Structures)
// ===========================================

// 并发数据结构是专门为多线程环境设计的数据结构
// 它们内部集成了同步机制，确保在并发访问时的线程安全性
// 使用并发数据结构可以避免手动管理锁，减少出错的可能性
//
// 常见的并发数据结构类型：
// 1. 并发映射（Concurrent Map）：线程安全的键值存储
// 2. 并发队列（Concurrent Queue）：线程安全的队列
// 3. 并发集合（Concurrent Set）：线程安全的集合
// 4. 原子计数器（Atomic Counter）：线程安全的计数器
// 5. 并发优先队列（Concurrent Priority Queue）：线程安全的优先队列
//
// 并发数据结构的设计原则：
// - 细粒度锁：减少锁的争用
// - 无锁设计：使用原子操作
// - 乐观并发：假设冲突很少发生
// - 读写分离：分离读写操作以提高并发度

fn concurrent_data_structures() {
    println!("=== 并发数据结构 ===");

    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // 示例1：并发哈希表（Concurrent HashMap）
    // 使用单个互斥锁保护整个哈希表，简单但扩展性有限
    println!("\n--- 并发哈希表 ---");
    concurrent_hashmap_example();

    // 示例2：分段锁哈希表（Segmented HashMap）
    // 将哈希表分成多个段，每个段有自己的锁，提高并发度
    println!("\n--- 分段锁哈希表 ---");
    segmented_hashmap_example();

    // 示例3：并发队列（Concurrent Queue）
    // 线程安全的队列，支持多生产者和多消费者
    println!("\n--- 并发队列 ---");
    concurrent_queue_example();

    // 并发数据结构的选择原则：
    // 1. 访问模式：读多写少 vs 写多读少
    // 2. 数据规模：小数据集 vs 大数据集
    // 3. 并发级别：少量线程 vs 大量线程
    // 4. 性能要求：延迟敏感 vs 吞吐量敏感
    // 5. 一致性要求：强一致性 vs 最终一致性
    //
    // Rust 并发数据结构的优势：
    // 1. 类型安全：编译时保证线程安全
    // 2. 零成本抽象：无运行时开销
    // 3. 内存安全：避免数据竞争和内存泄漏
    // 4. 组合性：可以与其他并发原语自由组合

    println!();
}

// 并发哈希表的实现
fn concurrent_hashmap_example() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // 简单的并发哈希表实现
    #[derive(Debug)]
    struct ConcurrentHashMap<K, V> {
        data: Mutex<HashMap<K, V>>,
        stats: Mutex<HashStats>,
    }

    #[derive(Debug, Clone)]
    struct HashStats {
        puts: u64,
        gets: u64,
        removes: u64,
        hits: u64,
        misses: u64,
    }

    impl HashStats {
        fn new() -> Self {
            HashStats {
                puts: 0,
                gets: 0,
                removes: 0,
                hits: 0,
                misses: 0,
            }
        }

        fn hit_rate(&self) -> f64 {
            let total = self.hits + self.misses;
            if total == 0 {
                0.0
            } else {
                self.hits as f64 / total as f64 * 100.0
            }
        }
    }

    impl<K, V> ConcurrentHashMap<K, V>
    where
        K: std::hash::Hash + Eq + Clone + std::fmt::Display,
        V: Clone + std::fmt::Display,
    {
        fn new() -> Self {
            ConcurrentHashMap {
                data: Mutex::new(HashMap::new()),
                stats: Mutex::new(HashStats::new()),
            }
        }

        fn insert(&self, key: K, value: V) {
            let mut data = self.data.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();
            data.insert(key, value);
            stats.puts += 1;
        }

        fn get(&self, key: &K) -> Option<V> {
            let data = self.data.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();
            stats.gets += 1;
            match data.get(key) {
                Some(value) => {
                    stats.hits += 1;
                    Some(value.clone())
                }
                None => {
                    stats.misses += 1;
                    None
                }
            }
        }

        fn remove(&self, key: &K) -> Option<V> {
            let mut data = self.data.lock().unwrap();
            let mut stats = self.stats.lock().unwrap();
            stats.removes += 1;
            data.remove(key)
        }

        fn stats(&self) -> HashStats {
            self.stats.lock().unwrap().clone()
        }
    }

    let map = Arc::new(ConcurrentHashMap::new());
    let mut handles = vec![];

    // 插入线程
    for i in 0..5 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let key = format!("key_{}_{}", i, j);
                let value = i * 10 + j;
                map.insert(key.clone(), value);
                println!("插入线程 {}：{} = {}", i, key, value);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // 读取线程
    for i in 0..3 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let key = format!("key_{}_{}", i % 3, j);
                match map.get(&key) {
                    Some(value) => println!("读取线程 {}：{} = {}", i, key, value),
                    None => println!("读取线程 {}：{} 未找到", i, key),
                }
                thread::sleep(Duration::from_millis(15));
            }
        });
        handles.push(handle);
    }

    // 删除线程
    for i in 0..2 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            for j in 0..2 {
                let key = format!("key_{}_{}", i, j);
                if let Some(value) = map.remove(&key) {
                    println!("删除线程 {}：删除 {} = {}", i, key, value);
                }
                thread::sleep(Duration::from_millis(20));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_map = map.data.lock().unwrap();
    let stats = map.stats();
    println!("最终映射: {:?}", *final_map);
    println!("统计信息: {:?}", stats);
    println!("命中率: {:.2}%", stats.hit_rate());
}

// 分段锁哈希表的实现
fn segmented_hashmap_example() {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // 分段锁哈希表：提高并发度的实现
    #[derive(Debug)]
    struct SegmentedHashMap<K, V> {
        segments: Vec<Mutex<HashMap<K, V>>>,
        num_segments: usize,
    }

    impl<K, V> SegmentedHashMap<K, V>
    where
        K: Hash + Eq + Clone,
        V: Clone,
    {
        fn new(num_segments: usize) -> Self {
            let mut segments = Vec::new();
            for _ in 0..num_segments {
                segments.push(Mutex::new(HashMap::new()));
            }
            SegmentedHashMap {
                segments,
                num_segments,
            }
        }

        fn get_segment(&self, key: &K) -> usize {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() as usize) % self.num_segments
        }

        fn insert(&self, key: K, value: V) {
            let segment = self.get_segment(&key);
            let mut seg = self.segments[segment].lock().unwrap();
            seg.insert(key, value);
        }

        fn get(&self, key: &K) -> Option<V> {
            let segment = self.get_segment(key);
            let seg = self.segments[segment].lock().unwrap();
            seg.get(key).cloned()
        }

        fn remove(&self, key: &K) -> Option<V> {
            let segment = self.get_segment(key);
            let mut seg = self.segments[segment].lock().unwrap();
            seg.remove(key)
        }
    }

    // 注意：这个示例需要添加 twox_hash 依赖
    // 这里我们使用简化的分段逻辑
    println!("分段锁哈希表：使用多个锁提高并发度");
    println!("（实际实现需要外部依赖，这里仅演示概念）");
}

// 并发队列的实现
fn concurrent_queue_example() {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    // 有界并发队列：使用条件变量实现阻塞操作
    #[derive(Debug)]
    struct ConcurrentQueue<T> {
        data: Mutex<Vec<T>>,
        not_empty: Condvar,
        not_full: Condvar,
        capacity: usize,
    }

    impl<T> ConcurrentQueue<T> {
        fn new(capacity: usize) -> Self {
            ConcurrentQueue {
                data: Mutex::new(Vec::new()),
                not_empty: Condvar::new(),
                not_full: Condvar::new(),
                capacity,
            }
        }

        fn push(&self, item: T) {
            let mut data = self.data.lock().unwrap();
            while data.len() >= self.capacity {
                data = self.not_full.wait(data).unwrap();
            }
            data.push(item);
            self.not_empty.notify_one();
        }

        fn pop(&self) -> T {
            let mut data = self.data.lock().unwrap();
            while data.is_empty() {
                data = self.not_empty.wait(data).unwrap();
            }
            let item = data.remove(0);
            self.not_full.notify_one();
            item
        }

        fn try_pop(&self) -> Option<T> {
            let mut data = self.data.lock().unwrap();
            data.pop()
        }
    }

    let queue = Arc::new(ConcurrentQueue::new(5));
    let mut handles = vec![];

    // 生产者线程
    for i in 0..3 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let item = format!("P{}-{}", i, j);
                println!("生产者 {}：生产 {}", i, item);
                queue.push(item);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // 消费者线程
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 0..7 {
                let item = queue.pop();
                println!("消费者 {}：消费 {}", i, item);
                thread::sleep(Duration::from_millis(80));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ===========================================
// 10. Send 和 Sync Trait (Send and Sync Traits)
// ===========================================

// Send 和 Sync 是 Rust 中用于标记线程安全性的特殊 trait
// 它们是自动 trait，编译器会根据类型定义自动实现
// 理解这两个 trait 对于构建安全的并发程序至关重要

fn send_and_sync_traits() {
    println!("=== Send 和 Sync Trait ===");

    // Send trait：允许在线程间转移所有权
    // 实现 Send 的类型可以安全地移动到另一个线程
    // 大多数类型都实现了 Send，除了包含裸指针的类型

    // 基本类型都实现了 Send
    use std::thread;

    // 示例：使用 Send trait 的类型
    fn send_example<T: Send + 'static>(data: T) {
        thread::spawn(move || {
            println!("在新线程中使用数据: {:?}", std::any::type_name::<T>());
            // data 的所有权被移动到新线程
        });
    }

    send_example(42i32);
    send_example(String::from("hello"));
    send_example(vec![1, 2, 3]);

    // Sync trait：允许多线程安全地共享引用
    // 实现 Sync 的类型的引用可以在多个线程间安全共享
    // 如果 &T 是 Send，那么 T 就是 Sync

    // 示例：使用 Sync trait 的类型
    fn sync_example<T: Sync + std::fmt::Debug>(data: &T) {
        // 演示 Sync 类型可以在多线程间安全共享引用
        println!("共享数据: {:?}", data);
        // 实际使用中应该用 Arc 来共享数据
    }

    let num = 42;
    sync_example(&num);

    // 手动实现 Send 和 Sync
    // 对于自定义类型，通常不需要手动实现
    // 但有时需要为包含原始指针的类型实现

    use std::ptr;

    // 自定义的线程安全指针包装器
    #[derive(Debug)]
    struct SafePtr<T> {
        ptr: *mut T,
    }

    // 手动实现 Send - 确保指针可以安全地在线程间转移
    unsafe impl<T> Send for SafePtr<T> {}

    // 手动实现 Sync - 确保指针的引用可以安全地在线程间共享
    unsafe impl<T> Sync for SafePtr<T> {}

    impl<T> SafePtr<T> {
        fn new(value: &mut T) -> Self {
            SafePtr {
                ptr: value as *mut T,
            }
        }

        // 注意：这个例子仅用于演示，实际使用需要额外的同步机制
        fn as_ref(&self) -> Option<&T> {
            if self.ptr.is_null() {
                None
            } else {
                unsafe { Some(&*self.ptr) }
            }
        }
    }

    // Send 和 Sync 的最佳实践：
    // 1. 优先使用标准库中的线程安全类型
    // 2. 使用 Arc + Mutex 模式来包装需要共享的可变数据
    // 3. 在实现自定义 Send/Sync 时要格外小心
    // 4. 注意生命周期和引用的有效性
    // 5. 使用原子操作来避免锁的开销

    // 常见的 Send/Sync 模式：
    // 1. Arc<T>: 当 T: Send + Sync 时，Arc<T> 也是 Send + Sync
    // 2. Mutex<T>: 当 T: Send 时，Mutex<T> 也是 Send + Sync
    // 3. 原子类型：如 AtomicUsize 都是 Send + Sync
    // 4. 不可变数据：不可变引用通常是 Send + Sync

    // 不实现 Send/Sync 的类型：
    // 1. Rc<T>: 引用计数不是线程安全的
    // 2. Cell<T>: 内部可变性不是线程安全的
    // 3. RefCell<T>: 运行时借用检查不是线程安全的
    // 4. 裸指针：需要手动确保线程安全

    println!();
}

// ===========================================
// 11. 无锁编程 (Lock-Free Programming)
// ===========================================

// 无锁编程是并发编程的高级技术，它不使用传统的锁机制
// 而是依赖原子操作来实现线程安全的并发访问
// 无锁编程可以提供更好的性能和可扩展性，但也更加复杂

fn lock_free_programming() {
    println!("=== 无锁编程 ===");

    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
    use std::thread;

    // 示例1：无锁计数器
    #[derive(Debug)]
    struct LockFreeCounter {
        count: AtomicUsize,
    }

    impl LockFreeCounter {
        fn new() -> Self {
            LockFreeCounter {
                count: AtomicUsize::new(0),
            }
        }

        fn increment(&self) -> usize {
            // 使用 fetch_add 原子地增加计数器
            self.count.fetch_add(1, Ordering::SeqCst)
        }

        fn get(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }
    }

    let counter = Arc::new(LockFreeCounter::new());
    let mut handles = vec![];

    // 创建多个线程并发增加计数器
    for i in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let prev = counter.increment();
                if i == 0 && prev % 100 == 0 {
                    println!("线程 0：计数器增加到 {}", prev + 1);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("无锁计数器最终值: {}", counter.get());

    // 示例2：简化的无锁数据结构（使用原子操作）
    // 注意：完整的无锁栈实现非常复杂，涉及内存回收和ABA问题
    // 这里我们展示一个简化的概念，实际应用中建议使用成熟的库
    
    println!("无锁栈演示：");
    println!("注意：完整的无锁栈实现非常复杂，涉及内存安全问题");
    println!("实际应用中建议使用 crossbeam 等成熟的无锁数据结构库");
    
    // 使用原子操作实现简单的无锁计数器集合
    use std::sync::atomic::AtomicI32;
    
    #[derive(Debug)]
    struct AtomicArray {
        data: Vec<AtomicI32>,
    }
    
    impl AtomicArray {
        fn new(size: usize) -> Self {
            let mut data = Vec::with_capacity(size);
            for _ in 0..size {
                data.push(AtomicI32::new(0));
            }
            AtomicArray { data }
        }
        
        fn increment(&self, index: usize) -> i32 {
            if index < self.data.len() {
                self.data[index].fetch_add(1, Ordering::SeqCst)
            } else {
                -1
            }
        }
        
        fn get(&self, index: usize) -> i32 {
            if index < self.data.len() {
                self.data[index].load(Ordering::SeqCst)
            } else {
                -1
            }
        }
    }
    
    let atomic_array = Arc::new(AtomicArray::new(5));
    let mut handles = vec![];
    
    // 多个线程并发修改数组
    for thread_id in 0..3 {
        let array = Arc::clone(&atomic_array);
        let handle = thread::spawn(move || {
            for i in 0..5 {
                let prev = array.increment(i);
                println!("线程 {} 增加索引 {}: {} -> {}", thread_id, i, prev, prev + 1);
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数组状态:");
    for i in 0..5 {
        println!("  索引 {}: {}", i, atomic_array.get(i));
    }

    // 无锁编程的优势和挑战：
    // 优势：
    // 1. 性能：避免了锁的开销和争用
    // 2. 可扩展性：在多核系统上表现更好
    // 3. 死锁安全：不会发生死锁
    // 4. 公平性：没有线程饥饿问题
    //
    // 挑战：
    // 1. 复杂性：算法难以理解和实现
    // 2. ABA 问题：需要额外的机制来处理
    // 3. 内存回收：难以安全地释放内存
    // 4. 调试困难：并发错误难以重现
    // 5. 正确性证明：需要严格的数学证明

    // 无锁编程的最佳实践：
    // 1. 优先使用标准库中的无锁数据结构
    // 2. 从简单的无锁算法开始
    // 3. 使用适当的内存顺序
    // 4. 彻底测试并发行为
    // 5. 考虑使用垃圾回收机制

    println!();
}

// ===========================================
// 12. 并发错误处理 (Concurrency Error Handling)
// ===========================================

// 并发程序中的错误处理比单线程程序更加复杂
// 需要考虑线程间的错误传播、资源清理和状态恢复
// Rust 的错误处理机制在并发环境中同样适用，但需要额外的考虑

fn concurrency_error_handling() {
    println!("=== 并发错误处理 ===");

    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 示例1：线程间的错误传播
    #[derive(Debug)]
    enum WorkerError {
        Io(String),
        Network(String),
        Parse(String),
        Timeout,
    }

    // 使用 Result 传递错误
    fn worker_with_error_handling(id: usize) -> Result<String, WorkerError> {
        // 模拟可能失败的操作
        if id % 3 == 0 {
            return Err(WorkerError::Io(format!("Worker {} 遇到 I/O 错误", id)));
        }

        if id % 5 == 0 {
            return Err(WorkerError::Network(format!("Worker {} 遇到网络错误", id)));
        }

        // 模拟工作
        thread::sleep(Duration::from_millis(10));
        Ok(format!("Worker {} 完成任务", id))
    }

    // 创建多个工作线程，收集所有结果和错误
    let mut handles = vec![];
    let (result_tx, result_rx) = mpsc::channel();

    for i in 0..10 {
        let tx = result_tx.clone();
        let handle = thread::spawn(move || match worker_with_error_handling(i) {
            Ok(result) => tx.send(Ok(result)).unwrap(),
            Err(error) => tx.send(Err(error)).unwrap(),
        });
        handles.push(handle);
    }

    // 关闭发送端
    drop(result_tx);

    // 收集结果
    let mut successes = vec![];
    let mut failures = vec![];

    for result in result_rx {
        match result {
            Ok(success) => successes.push(success),
            Err(failure) => failures.push(failure),
        }
    }

    println!("成功的任务: {}", successes.len());
    for success in successes {
        println!("  {}", success);
    }

    println!("失败的任务: {}", failures.len());
    for failure in failures {
        println!("  {:?}", failure);
    }

    // 示例2：恐慌传播和恢复
    let (panic_tx, panic_rx) = mpsc::channel();

    // 可能发生恐慌的工作线程
    let panic_handle = thread::spawn(move || {
        panic_tx.send("线程开始".to_string()).unwrap();

        // 模拟可能发生的恐慌
        if true {
            panic!("这是一个故意的恐慌！");
        }

        panic_tx.send("线程正常结束".to_string()).unwrap();
    });

    // 捕获恐慌
    let panic_result = panic_handle.join();
    match panic_result {
        Ok(_) => println!("线程正常完成"),
        Err(_) => {
            println!("检测到线程恐慌");
            // 可以在这里进行恢复操作
        }
    }

    // 示例3：资源清理和超时处理
    fn worker_with_timeout(timeout_ms: u64) -> Result<String, String> {
        let (work_tx, work_rx) = mpsc::channel();

        // 创建工作线程
        let handle = thread::spawn(move || {
            // 模拟耗时工作
            thread::sleep(Duration::from_millis(timeout_ms / 2));
            work_tx.send("工作完成".to_string()).unwrap();
        });

        // 设置超时
        match work_rx.recv_timeout(Duration::from_millis(timeout_ms)) {
            Ok(result) => Ok(result),
            Err(_) => {
                // 超时，取消工作线程
                // 注意：Rust 中无法直接取消线程，需要使用其他机制
                Err("操作超时".to_string())
            }
        }
    }

    match worker_with_timeout(100) {
        Ok(result) => println!("工作成功: {}", result),
        Err(error) => println!("工作失败: {}", error),
    }

    // 示例4：使用 JoinHandle 捕获线程结果
    let (success_tx, success_rx) = mpsc::channel();
    let (error_tx, error_rx) = mpsc::channel();

    let worker_handle = thread::spawn(move || {
        // 模拟可能失败的工作
        if true {
            error_tx.send("工作失败".to_string()).unwrap();
        } else {
            success_tx.send("工作成功".to_string()).unwrap();
        }
    });

    // 等待线程完成
    let _ = worker_handle.join();

    // 检查结果
    if let Ok(success_msg) = success_rx.try_recv() {
        println!("成功: {}", success_msg);
    }

    if let Ok(error_msg) = error_rx.try_recv() {
        println!("错误: {}", error_msg);
    }

    // 并发错误处理的最佳实践：
    // 1. 使用 Result 类型明确表示可能的错误
    // 2. 实现适当的错误恢复机制
    // 3. 处理线程恐慌，避免程序崩溃
    // 4. 实现超时机制，防止无限等待
    // 5. 确保资源的正确清理
    // 6. 记录错误信息以便调试
    // 7. 实现重试机制处理临时性错误
    // 8. 使用断路器模式防止级联失败

    println!();
}

// ===========================================
// 13. 并发测试 (Concurrency Testing)
// ===========================================

// 并发测试是确保并发程序正确性的重要手段
// 由于并发错误的非确定性，需要专门的测试策略和工具
// Rust 提供了一些内置的并发测试支持，同时也可以使用外部工具

fn concurrency_testing() {
    println!("=== 并发测试 ===");

    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };
    use std::thread;
    use std::time::Duration;

    // 示例1：基本的并发测试
    #[derive(Debug)]
    struct ConcurrentCounter {
        value: Mutex<u32>,
    }

    impl ConcurrentCounter {
        fn new() -> Self {
            ConcurrentCounter {
                value: Mutex::new(0),
            }
        }

        fn increment(&self) -> u32 {
            let mut value = self.value.lock().unwrap();
            *value += 1;
            *value
        }

        fn get(&self) -> u32 {
            *self.value.lock().unwrap()
        }
    }

    // 并发测试函数
    fn test_concurrent_counter() {
        let counter = Arc::new(ConcurrentCounter::new());
        let mut handles = vec![];

        // 创建多个线程并发修改计数器
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    counter.increment();
                }
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 验证最终结果
        let final_value = counter.get();
        assert_eq!(final_value, 10000, "并发计数器测试失败");
        println!("并发计数器测试通过: 最终值 = {}", final_value);
    }

    test_concurrent_counter();

    // 示例2：测试死锁检测
    #[derive(Debug)]
    struct ResourceA {
        data: Mutex<u32>,
    }

    #[derive(Debug)]
    struct ResourceB {
        data: Mutex<u32>,
    }

    fn test_deadlock_prevention() {
        let a = Arc::new(ResourceA {
            data: Mutex::new(0),
        });
        let b = Arc::new(ResourceB {
            data: Mutex::new(0),
        });

        let a1 = Arc::clone(&a);
        let b1 = Arc::clone(&b);
        let a2 = Arc::clone(&a);
        let b2 = Arc::clone(&b);

        // 死锁预防：两个线程都按照相同的顺序获取锁 (a -> b)
        // 这样可以避免死锁的发生
        
        // 线程1：按照 a -> b 的顺序获取锁
        let handle1 = thread::spawn(move || {
            let _guard1 = a1.data.lock().unwrap();
            println!("线程1 获取了 ResourceA");
            thread::sleep(Duration::from_millis(10));
            let _guard2 = b1.data.lock().unwrap();
            println!("线程1 获取了 ResourceB");
            println!("线程1 完成操作");
        });

        // 线程2：也按照 a -> b 的顺序获取锁（避免死锁）
        let handle2 = thread::spawn(move || {
            // 稍微延迟，让线程1先开始
            thread::sleep(Duration::from_millis(5));
            let _guard1 = a2.data.lock().unwrap();
            println!("线程2 获取了 ResourceA");
            thread::sleep(Duration::from_millis(10));
            let _guard2 = b2.data.lock().unwrap();
            println!("线程2 获取了 ResourceB");
            println!("线程2 完成操作");
        });

        // 等待线程完成
        handle1.join().unwrap();
        handle2.join().unwrap();

        println!("死锁预防测试通过：通过统一的锁获取顺序避免了死锁");
    }

    test_deadlock_prevention();

    // 示例3：性能基准测试
    fn benchmark_mutex_vs_atomic() {
        const NUM_THREADS: usize = 4;
        const ITERATIONS: usize = 100_000;

        // 测试 Mutex 性能
        let start_time = std::time::Instant::now();
        let mutex_counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..NUM_THREADS {
            let counter = Arc::clone(&mutex_counter);
            let handle = thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mutex_time = start_time.elapsed();
        let mutex_result = *mutex_counter.lock().unwrap();

        // 测试 Atomic 性能
        let start_time = std::time::Instant::now();
        let atomic_counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..NUM_THREADS {
            let counter = Arc::clone(&atomic_counter);
            let handle = thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let atomic_time = start_time.elapsed();
        let atomic_result = atomic_counter.load(Ordering::Relaxed);

        println!("性能测试结果:");
        println!("  Mutex 计数器: 值={}, 时间={:?}", mutex_result, mutex_time);
        println!(
            "  Atomic 计数器: 值={}, 时间={:?}",
            atomic_result, atomic_time
        );
        println!(
            "  性能提升: {:.2}x",
            mutex_time.as_nanos() as f64 / atomic_time.as_nanos() as f64
        );
    }

    benchmark_mutex_vs_atomic();

    // 并发测试的最佳实践：
    // 1. 编写明确的测试用例覆盖不同的并发场景
    // 2. 使用断言验证并发操作的最终结果
    // 3. 测试极端情况和边界条件
    // 4. 实现性能基准测试
    // 5. 使用压力测试发现潜在的并发问题
    // 6. 考虑使用专门的并发测试工具
    // 7. 实现测试的随机性以增加发现错误的可能性
    // 8. 确保测试的可重复性

    // Rust 并发测试工具：
    // 1. 内置的测试框架支持多线程测试
    // 2. cargo test 可以运行并发测试
    // 3. criterion crate 用于性能基准测试
    // 4. loom crate 用于并发算法的模型检查
    // 5. rayon crate 用于数据并行测试

    println!();
}

// ===========================================
// 14. 并发示例程序 (Concurrency Example Program)
// ===========================================

fn concurrency_example_program() {
    println!("=== 并发示例程序 ===");

    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // 并发任务调度器
    #[derive(Debug)]
    enum Task {
        Compute(u32, u32), // 计算 a + b
        Sleep(u64),        // 休眠指定毫秒
        Print(String),     // 打印消息
    }

    #[derive(Debug)]
    struct TaskResult {
        task_id: u32,
        result: String,
    }

    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    let result_tx = Arc::new(Mutex::new(result_tx));

    // 任务分发器
    let dispatcher = thread::spawn(move || {
        let mut task_id = 0;

        // 创建一些示例任务
        let tasks = vec![
            Task::Compute(10, 20),
            Task::Sleep(100),
            Task::Print("Hello from task!".to_string()),
            Task::Compute(5, 7),
            Task::Sleep(50),
            Task::Print("Another message".to_string()),
            Task::Compute(100, 200),
        ];

        for task in tasks {
            let worker_tx: mpsc::Sender<Task> = task_tx.clone();
            let worker_result_tx = Arc::clone(&result_tx);
            let current_task_id = task_id;

            thread::spawn(move || {
                let result = match task {
                    Task::Compute(a, b) => format!("{} + {} = {}", a, b, a + b),
                    Task::Sleep(ms) => {
                        thread::sleep(Duration::from_millis(ms));
                        format!("休眠了 {} 毫秒", ms)
                    }
                    Task::Print(msg) => format!("打印: {}", msg),
                };

                let task_result = TaskResult {
                    task_id: current_task_id,
                    result,
                };

                worker_result_tx.lock().unwrap().send(task_result).unwrap();
            });

            task_id += 1;
            thread::sleep(Duration::from_millis(20));
        }
    });

    // 结果收集器
    let collector = thread::spawn(move || {
        let mut results = Vec::new();

        // 收集结果
        for _ in 0..7 {
            // 我们知道有7个任务
            if let Ok(result) = result_rx.recv() {
                results.push(result);
            }
        }

        results.sort_by_key(|r| r.task_id);
        results
    });

    // 等待所有任务完成
    dispatcher.join().unwrap();
    let results = collector.join().unwrap();

    println!("=== 并发任务调度结果 ===");
    for result in results {
        println!("任务 {}: {}", result.task_id, result.result);
    }

    // 并发网页爬虫示例
    #[derive(Debug, Clone)]
    struct WebPage {
        url: String,
        content: String,
        links: Vec<String>,
    }

    fn fetch_page(url: &str) -> WebPage {
        // 模拟网络请求
        thread::sleep(Duration::from_millis(50));

        let content = format!("这是 {} 页面的内容", url);
        let links = match url {
            "http://example.com" => vec![
                "http://example.com/about".to_string(),
                "http://example.com/contact".to_string(),
            ],
            "http://example.com/about" => vec!["http://example.com/team".to_string()],
            _ => vec![],
        };

        WebPage {
            url: url.to_string(),
            content,
            links,
        }
    }

    let start_url = "http://example.com";
    let (url_tx, url_rx) = mpsc::channel::<String>();
    let (page_tx, page_rx) = mpsc::channel();
    let url_tx = Arc::new(Mutex::new(url_tx));

    // 已访问的 URL 集合
    let visited = Arc::new(Mutex::new(std::collections::HashSet::new()));

    // 工作线程
    let mut worker_handles = vec![];
    let url_rx = Arc::new(Mutex::new(url_rx));

    for i in 0..3 {
        let page_tx = page_tx.clone();
        let url_rx = Arc::clone(&url_rx);
        let visited = Arc::clone(&visited);

        let handle = thread::spawn(move || {
            while let Ok(url) = url_rx.lock().unwrap().recv() {
                // 检查是否已访问
                {
                    let mut visited = visited.lock().unwrap();
                    if visited.contains(&url) {
                        continue;
                    }
                    visited.insert(url.clone());
                }

                println!("爬虫 {} 抓取: {}", i, url);
                let page = fetch_page(&url);

                // 发送页面内容
                page_tx.send(page.clone()).unwrap();

                // 发送新的 URL
                for link in page.links {
                    // Note: In a real implementation, we would need to handle this properly
                    // For now, we'll just print the links
                    println!("发现链接: {}", link);
                }
            }
        });

        worker_handles.push(handle);
    }

    // 开始爬取
    url_tx.lock().unwrap().send(start_url.to_string()).unwrap();

    // 等待一段时间后关闭 URL 通道
    thread::sleep(Duration::from_millis(500));
    drop(url_tx);

    // 等待工作线程完成
    for handle in worker_handles {
        handle.join().unwrap();
    }

    // 关闭页面通道
    drop(page_tx);

    // 收集结果
    let mut pages = Vec::new();
    while let Ok(page) = page_rx.try_recv() {
        pages.push(page);
    }

    println!("\n=== 并发网页爬虫结果 ===");
    for page in &pages {
        println!("URL: {}", page.url);
        println!("  内容: {}", page.content);
        println!("  链接: {:?}", page.links);
    }

    println!("总共抓取了 {} 个页面", pages.len());

    println!();
}

// ===========================================
// 15. Rust 1.91: Sync Exclusive
// ===========================================

// Rust 1.91 引入了 std::sync::Exclusive<T>
// 这是一个同步原语，提供对 T 的独占访问，相当于编译时的 Mutex
// 它实现了 Sync，即使 T 只是 Send

pub fn sync_exclusive() {
    println!("=== Rust 1.91: Sync Exclusive ===");

    // std::sync::Exclusive 是一个包装器
    // 它只允许拥有 Exclusive<T> 的所有者访问 T
    // 因为它拥有 T，所以它可以在线程间安全传递（实现了 Sync）
    
    // 模拟 Rust 1.91 的 Exclusive (如果标准库尚未提供，这里是概念演示)
    // 注意：在实际 Rust 1.91 中，这位于 std::sync::Exclusive
    struct Exclusive<T: ?Sized> {
        inner: T,
    }
    
    impl<T> Exclusive<T> {
        pub const fn new(t: T) -> Exclusive<T> {
            Exclusive { inner: t }
        }
        
        pub fn get_mut(&mut self) -> &mut T {
            &mut self.inner
        }
    }
    
    // Exclusive 实现了 Sync，只要 T 是 Sized
    // 这是因为要访问 inner，必须拥有 Exclusive 的 &mut 引用
    // 而 &mut 引用本身就是互斥的
    unsafe impl<T: ?Sized> Sync for Exclusive<T> {}
    
    let mut ex = Exclusive::new(42);
    
    // 独占访问
    let val = ex.get_mut();
    *val += 1;
    
    println!("Exclusive 值: {}", ex.inner);
    
    println!("Exclusive<T> 的主要用途是适配既有的 Send 类型为 Sync，用于特定并发场景。");
    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 并发编程演示");
    println!("=================");

    thread_basics();
    channels();
    shared_state_concurrency();
    atomic_operations();
    condition_variables();
    rwlock_usage();
    barrier_usage();
    concurrency_patterns();
    concurrent_data_structures();
    send_and_sync_traits();
    lock_free_programming();
    concurrency_error_handling();
    concurrency_testing();
    concurrency_example_program();

    println!("并发编程演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::thread;

        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_mutex_counter() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_channel_communication() {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(42).unwrap();
        });

        let received = rx.recv().unwrap();
        assert_eq!(received, 42);
    }

    #[test]
    fn test_concurrent_hash_map() {
        use std::collections::HashMap;
        use std::sync::{Arc, Mutex};
        use std::thread;

        #[derive(Debug)]
        struct ConcurrentHashMap<K, V> {
            data: Mutex<HashMap<K, V>>,
        }

        impl<K, V> ConcurrentHashMap<K, V>
        where
            K: std::hash::Hash + Eq + Clone,
            V: Clone + PartialEq,
        {
            fn new() -> Self {
                ConcurrentHashMap {
                    data: Mutex::new(HashMap::new()),
                }
            }

            fn insert(&self, key: K, value: V) {
                let mut data = self.data.lock().unwrap();
                data.insert(key, value);
            }

            fn get(&self, key: &K) -> Option<V> {
                let data = self.data.lock().unwrap();
                data.get(key).cloned()
            }
        }

        let map = Arc::new(ConcurrentHashMap::new());
        let mut handles = vec![];

        for i in 0..5 {
            let map = Arc::clone(&map);
            let handle = thread::spawn(move || {
                map.insert(format!("key_{}", i), i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(map.get(&"key_0".to_string()), Some(0));
        assert_eq!(map.get(&"key_3".to_string()), Some(3));
        assert_eq!(map.get(&"key_5".to_string()), None);
    }
}
