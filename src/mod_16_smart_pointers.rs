// 教程里的类型、字段和方法经常只为演示语法而定义，不一定会被全部读取或调用，所以允许 dead_code；
// 其他 lint（未使用的导入、变量、可变性等）必须保持干净，不要再给模块添加新的 allow。
#![allow(dead_code)]

// Rust 智能指针 (Smart Pointers)
// 深入讲解 Rust 中的智能指针概念，包括 Box、Rc、Arc、RefCell 等
// 智能指针是 Rust 中实现数据结构设计和内存管理的重要工具

// ===========================================
// 1. Rust 智能指针基础概念
// ===========================================

// 智能指针是指实现 Deref 和 Drop trait 的数据结构
// 它们的行为类似于指针但拥有额外的元数据和功能
// 智能指针的核心价值在于提供更灵活的内存管理方式

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn smart_pointer_concepts() {
    println!("=== 智能指针基础概念 ===");

    // 智能指针与普通指针的区别：
    // 1. 普通指针：只引用数据，不拥有所有权
    // 2. 智能指针：拥有数据，并管理数据的生命周期
    // 3. 元数据：包含额外的信息（如引用计数）
    // 4. 行为：实现特定的 trait（Deref、Drop）

    // 智能指针的主要类型：
    // - Box<T>：堆分配，单一所有权
    // - Rc<T>：引用计数，共享不可变所有权
    // - Arc<T>：原子引用计数，线程安全的共享所有权
    // - RefCell<T>：内部可变性，运行时借用检查
    // - String & Vec<T>：也是智能指针的一种

    println!();
}

// ===========================================
// 2. Box<T> 智能指针
// ===========================================

// Box<T> 是最简单的智能指针，提供堆分配功能
// 当需要在编译时不知道大小但需要在运行时确定大小时使用

fn box_smart_pointer() {
    println!("=== Box<T> 智能指针 ===");

    // Box 的主要用途：
    // 1. 堆分配数据：将数据存储在堆上而非栈上
    // 2. 递归类型：处理编译时无法确定大小的递归类型
    // 3. 转移所有权：在需要转移大型数据所有权时

    // 基本的 Box 使用
    let b = Box::new(5); // 在堆上分配一个 i32
    println!("Box 的值: {}", b);
    // 当 b 离开作用域时，堆内存会自动释放

    // Box 用于递归类型
    // Cons List 是函数式编程语言中的经典数据结构
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>), // 使用 Box 使得 List 大小已知
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("递归列表: {:?}", list);

    // Box 用于 trait 对象
    // 当需要在运行时使用不同的类型实现时
    trait Draw {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }

    impl Draw for Circle {
        fn draw(&self) {
            println!("绘制圆形，半径: {}", self.radius);
        }
    }

    struct Square {
        side: f64,
    }

    impl Draw for Square {
        fn draw(&self) {
            println!("绘制正方形，边长: {}", self.side);
        }
    }

    // Box<dyn Draw> 可以存储任何实现 Draw trait 的类型
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];

    for shape in shapes {
        shape.draw();
    }

    // Box 的性能特点：
    // 1. 内存分配：在堆上分配，比栈分配稍慢
    // 2. 内存释放：自动释放，无需手动管理
    // 3. 访问速度：通过指针访问，与直接访问几乎相同
    // 4. 零成本抽象：运行时没有额外开销

    println!();
}

// ===========================================
// 3. Deref trait 与强制解引用
// ===========================================

// Deref trait 用于自定义解引用行为
// Rust 会在很多情况下自动进行解引用强制转换

fn deref_trait() {
    println!("=== Deref trait 与强制解引用 ===");

    // Deref trait 的核心概念：
    // 1. 解引用：使用 * 运算符访问指针指向的值
    // 2. 强制解引用：Rust 自动将引用转换为其他类型的引用
    // 3. 方法调用：在调用方法时会自动进行解引用

    // 自定义 MyBox 类型实现 Deref
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用 * 运算符

    // 强制解引用的工作原理
    let m = MyBox::new(String::from("Rust"));
    // Rust 会自动将 &MyBox<String> 转换为 &String
    // 然后将 &String 转换为 &str
    let hello: &str = &m;
    println!("强制解引用: {}", hello);

    // Deref 的使用场景：
    // 1. 智能指针：让智能指针像普通引用一样使用
    // 2. 构建器模式：创建流畅的 API
    // 3. 包装器类型：透明地包装底层类型
    // 4. 零成本抽象：提供便利而不牺牲性能

    // DerefMut trait 用于可变解引用
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    let mut z = MyBox::new(42);
    *z = 24; // 使用 DerefMut 进行可变访问
    println!("修改后的值: {}", *z);

    println!();
}

// ===========================================
// 4. Drop trait 与清理逻辑
// ===========================================

// Drop trait 允许我们在值离开作用域时执行自定义代码
// 这是 Rust 中 RAII (Resource Acquisition Is Initialization) 模式的核心

fn drop_trait() {
    println!("=== Drop trait 与清理逻辑 ===");

    // Drop trait 的作用：
    // 1. 资源清理：释放文件句柄、网络连接等
    // 2. 内存管理：智能指针使用 Drop 来释放内存
    // 3. 日志记录：在对象销毁时记录日志
    // 4. 状态更新：在对象销毁时更新程序状态

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("清理 CustomSmartPointer，数据: {}", self.data);
        }
    }

    let _c = CustomSmartPointer {
        data: String::from("我的数据"),
    };

    println!("创建 CustomSmartPointer");
    // c 离开作用域时，drop 方法会被自动调用

    // 手动调用 drop 的注意事项
    let d = CustomSmartPointer {
        data: String::from("重要数据"),
    };

    println!("创建另一个 CustomSmartPointer");
    drop(d); // 手动调用 drop，提前清理资源
    println!("手动调用 drop 后");

    // Drop 的使用场景：
    // 1. 文件句柄：确保文件在使用后被正确关闭
    // 2. 数据库连接：确保连接在使用后被正确关闭
    // 3. 锁：确保锁在使用后被正确释放
    // 4. 内存分配：智能指针使用 Drop 来释放内存

    // Drop trait 的限制：
    // 1. 不能在 trait 定义中使用 Drop
    // 2. 泛型参数的 Drop 需要额外约束
    // 3. 循环引用可能导致内存泄漏
    // 4. 不能在 drop 方法中再次 drop 同一个对象

    println!();
}

// ===========================================
// 5. Rc<T> 引用计数智能指针
// ===========================================

// Rc<T> (Reference Counting) 提供共享所有权
// 允许多个所有者共享同一个数据，但数据是不可变的

fn rc_smart_pointer() {
    println!("=== Rc<T> 引用计数智能指针 ===");

    // Rc 的核心概念：
    // 1. 共享所有权：多个所有者可以共享同一份数据
    // 2. 不可变数据：数据本身是不可变的，确保线程安全
    // 3. 引用计数：跟踪有多少所有者拥有数据
    // 4. 自动释放：当引用计数为 0 时自动释放内存

    use std::rc::Rc;

    let a = Rc::new(String::from("共享数据"));

    println!("引用计数: {}", Rc::strong_count(&a)); // 1

    let _b = Rc::clone(&a); // 增加引用计数
    println!("引用计数: {}", Rc::strong_count(&a)); // 2

    {
        let _c = Rc::clone(&a); // 增加引用计数
        println!("引用计数: {}", Rc::strong_count(&a)); // 3
        // c 离开作用域
    }

    println!("引用计数: {}", Rc::strong_count(&a)); // 2

    // Rc 的典型使用场景
    // 图结构中的节点共享
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }

    let node1 = Rc::new(Node {
        value: 1,
        children: Vec::new(),
    });

    let node2 = Rc::new(Node {
        value: 2,
        children: vec![Rc::clone(&node1)], // node2 拥有 node1 的引用
    });

    let node3 = Rc::new(Node {
        value: 3,
        children: vec![Rc::clone(&node1)], // node3 也拥有 node1 的引用
    });

    println!("图结构: node1 被 node{} 和 node{} 共享，强引用计数 = {}", node2.value, node3.value, Rc::strong_count(&node1));

    // Rc 的限制：
    // 1. 不可变性：只能共享不可变数据
    // 2. 线程安全：不是线程安全的，不能跨线程使用
    // 3. 循环引用：可能导致内存泄漏

    // Rc 与 Box 的对比：
    // - Box: 单一所有权，性能更高
    // - Rc: 共享所有权，适合共享数据

    println!();
}

// ===========================================
// 6. RefCell<T> 内部可变性
// ===========================================

// RefCell<T> 提供内部可变性，允许在不可变引用中修改数据
// 通过运行时借用检查来确保内存安全

fn refcell_smart_pointer() {
    println!("=== RefCell<T> 内部可变性 ===");

    // RefCell 的核心概念：
    // 1. 内部可变性：即使在不可变引用下也能修改数据
    // 2. 运行时检查：借用规则在运行时检查，而非编译时
    // 3. panic 机制：违反借用规则时会导致 panic
    // 4. 单线程限制：只能在单线程环境中使用

    use std::cell::RefCell;

    struct Messenger {
        messages: RefCell<Vec<String>>,
    }

    impl Messenger {
        fn new() -> Self {
            Messenger {
                messages: RefCell::new(Vec::new()),
            }
        }

        fn send_message(&self, message: &str) {
            // 即使 self 是不可变引用，也能修改 messages
            self.messages.borrow_mut().push(message.to_string());
        }

        fn get_messages(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    let messenger = Messenger::new();
    messenger.send_message("Hello");
    messenger.send_message("World");

    let messages = messenger.get_messages();
    println!("消息: {:?}", messages);

    // RefCell 的借用规则
    let cell = RefCell::new(5);

    {
        // 获取不可变引用
        let r1 = cell.borrow();
        println!("不可变借用: {}", r1);

        // 不能在已有不可变借用时获取可变借用
        // let r2 = cell.borrow_mut(); // 这会导致运行时 panic
    }

    {
        // 获取可变引用
        let mut r2 = cell.borrow_mut();
        *r2 = 10;
        println!("修改后的值: {}", r2);
    }

    // RefCell 的使用场景：
    // 1. Mock 对象：在测试中修改对象状态
    // 2. 观察者模式：内部维护观察者列表
    // 3. 缓存机制：内部缓存计算结果
    // 4. 配置对象：运行时修改配置

    // RefCell 的注意事项：
    // 1. 运行时开销：借用检查需要运行时成本
    // 2. panic 风险：错误的借用会导致 panic
    // 3. 单线程限制：不能在多线程中使用
    // 4. 调试困难：运行时错误难以调试

    println!();
}

// ===========================================
// 7. Weak<T> 弱引用
// ===========================================

// Weak<T> 是弱引用，不会增加引用计数
// 用于解决循环引用问题

fn weak_reference() {
    println!("=== Weak<T> 弱引用 ===");

    // Weak 的核心概念：
    // 1. 弱引用：不拥有数据，不会阻止数据被释放
    // 2. 循环引用解决：避免因循环引用导致的内存泄漏
    // 3. 升级为强引用：可以尝试升级为 Rc<T>
    // 4. 可能无效：数据可能已经被释放

    use std::rc::{Rc, Weak};

    // 循环引用的例子
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Option<Weak<Node>>, // 使用 Weak 避免循环引用
        children: Vec<Rc<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Node> {
            Rc::new(Node {
                value,
                parent: None,
                children: Vec::new(),
            })
        }

        fn add_child(self: &Rc<Self>, child: Rc<Node>) {
            let _child_weak = Rc::downgrade(&child); // 创建弱引用
            let mut children = self.children.clone();
            children.push(child);
        }

        fn get_parent(&self) -> Option<Rc<Node>> {
            self.parent.as_ref().and_then(|weak| weak.upgrade())
        }
    }

    let root = Node::new(1);
    let child = Node::new(2);

    root.add_child(child.clone());

    // 设置父节点的弱引用
    if let Some(node_data) = Rc::get_mut(&mut child.clone()) {
        node_data.parent = Some(Rc::downgrade(&root));
    }

    println!("创建了父子节点关系");

    // Weak 的使用场景：
    // 1. 缓存系统：缓存项不阻止原始数据被释放
    // 2. 观察者模式：观察者不阻止被观察者被释放
    // 3. 图结构：双向引用但不造成循环
    // 4. 事件监听：监听器不阻止事件源被释放

    // Weak 的 API：
    let weak_ref: Weak<i32> = Weak::new();

    // 尝试升级为强引用
    match weak_ref.upgrade() {
        Some(strong) => println!("升级成功: {:?}", strong),
        None => println!("升级失败，数据已被释放"),
    }

    // Weak 的特点：
    // 1. 不增加引用计数：不会阻止数据被释放
    // 2. 可能为空：指向的数据可能已经被释放
    // 3. 需要升级：使用前需要升级为强引用
    // 4. 线程安全：与 Rc 一样不是线程安全的

    println!();
}

// ===========================================
// 8. Arc<T> 原子引用计数
// ===========================================

// Arc<T> (Atomic Reference Counting) 是线程安全的引用计数智能指针
// 用于在多线程环境中共享数据

fn arc_smart_pointer() {
    println!("=== Arc<T> 原子引用计数 ===");

    // Arc 的核心概念：
    // 1. 原子操作：使用原子操作进行引用计数
    // 2. 线程安全：可以安全地在多线程间共享
    // 3. 不可变数据：共享的数据是不可变的
    // 4. 性能考虑：比 Rc 有更高的性能开销

    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {}: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Arc 的使用场景：
    // 1. 多线程共享数据：在线程间共享不可变数据
    // 2. 配置对象：共享配置信息
    // 3. 全局状态：共享的全局状态
    // 4. 不可变缓存：共享的缓存数据

    // Arc 与 Rc 的对比：
    // - Arc: 线程安全，性能开销较高
    // - Rc: 单线程，性能开销较低

    // Arc + Mutex：实现可变的共享数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    // Arc 的最佳实践：
    // 1. 尽量使用不可变数据：减少锁的使用
    // 2. 合理设计数据结构：减少锁争用
    // 3. 考虑性能影响：原子操作的开销
    // 4. 错误处理：处理可能的锁获取失败

    println!();
}

// ===========================================
// 9. 智能指针的组合使用
// ===========================================

// 智能指针经常需要组合使用来解决复杂的问题
// 理解不同智能指针的组合使用模式

fn smart_pointer_combinations() {
    println!("=== 智能指针的组合使用 ===");

    // 组合模式 1: Arc + RefCell
    // 线程安全的内部可变性
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct SharedState {
        data: Arc<Mutex<Vec<String>>>,
    }

    impl SharedState {
        fn new() -> Self {
            SharedState {
                data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn add_data(&self, item: String) {
            let mut data = self.data.lock().unwrap();
            data.push(item);
        }

        fn get_data(&self) -> Vec<String> {
            self.data.lock().unwrap().clone()
        }
    }

    let state = SharedState::new();
    state.add_data("项目 1".to_string());
    state.add_data("项目 2".to_string());

    println!("共享状态: {:?}", state.get_data());

    // 组合模式 2: Rc + Weak
    // 解决循环引用问题
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct GraphNode {
        id: String,
        neighbors: Vec<Rc<GraphNode>>,
        back_edges: Vec<Weak<GraphNode>>,
    }

    impl GraphNode {
        fn new(id: String) -> Rc<Self> {
            Rc::new(Self {
                id,
                neighbors: Vec::new(),
                back_edges: Vec::new(),
            })
        }

        fn add_neighbor(self: &Rc<Self>, neighbor: Rc<Self>) {
            let self_weak = Rc::downgrade(self);
            if let Some(neighbor_data) = Rc::get_mut(&mut neighbor.clone()) {
                neighbor_data.back_edges.push(self_weak);
            }
            if let Some(self_data) = Rc::get_mut(&mut self.clone()) {
                self_data.neighbors.push(neighbor);
            }
        }
    }

    let node1 = GraphNode::new("A".to_string());
    let node2 = GraphNode::new("B".to_string());

    node1.add_neighbor(node2.clone());

    println!("图节点: {:?}", node1);

    // 组合模式 3: Box + trait 对象
    // 动态分发和类型擦除
    trait Animal {
        fn make_sound(&self);
    }

    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    impl Animal for Dog {
        fn make_sound(&self) {
            println!("{}: 汪汪!", self.name);
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) {
            println!("{}: 喵喵!", self.name);
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "旺财".to_string(),
        }),
        Box::new(Cat {
            name: "咪咪".to_string(),
        }),
    ];

    for animal in animals {
        animal.make_sound();
    }

    // 组合使用的最佳实践：
    // 1. 明确需求：根据具体需求选择合适的组合
    // 2. 性能考虑：考虑组合对性能的影响
    // 3. 复杂度管理：避免过度复杂的组合
    // 4. 测试策略：为复杂的组合编写充分的测试

    println!();
}

// ===========================================
// 10. 实际应用示例
// ===========================================

fn practical_examples() {
    println!("=== 实际应用示例 ===");

    // 示例 1: 缓存系统
    #[derive(Debug)]
    struct Cache<K, V> {
        data: Arc<Mutex<std::collections::HashMap<K, V>>>,
        hits: Arc<Mutex<u64>>,
        misses: Arc<Mutex<u64>>,
    }

    impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
        fn new() -> Self {
            Self {
                data: Arc::new(Mutex::new(std::collections::HashMap::new())),
                hits: Arc::new(Mutex::new(0)),
                misses: Arc::new(Mutex::new(0)),
            }
        }

        fn get(&self, key: &K) -> Option<V> {
            let data = self.data.lock().unwrap();
            if let Some(value) = data.get(key) {
                *self.hits.lock().unwrap() += 1;
                Some(value.clone())
            } else {
                *self.misses.lock().unwrap() += 1;
                None
            }
        }

        fn put(&self, key: K, value: V) {
            let mut data = self.data.lock().unwrap();
            data.insert(key, value);
        }

        fn stats(&self) -> (u64, u64) {
            (*self.hits.lock().unwrap(), *self.misses.lock().unwrap())
        }
    }

    let cache = Cache::new();
    cache.put("key1".to_string(), "value1".to_string());
    cache.put("key2".to_string(), "value2".to_string());

    println!("缓存命中: {:?}", cache.get(&"key1".to_string()));
    println!("缓存未命中: {:?}", cache.get(&"key3".to_string()));
    println!("缓存统计: {:?}", cache.stats());

    // 示例 2: 观察者模式
    struct Subject {
        observers: Vec<Box<dyn Observer>>,
        state: String,
    }

    impl std::fmt::Debug for Subject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Subject")
                .field("observers", &format!("{} observers", self.observers.len()))
                .field("state", &self.state)
                .finish()
        }
    }

    trait Observer {
        fn update(&self, subject: &Subject);
    }

    impl Subject {
        fn new(state: String) -> Self {
            Self {
                observers: Vec::new(),
                state,
            }
        }

        fn add_observer(&mut self, observer: Box<dyn Observer>) {
            self.observers.push(observer);
        }

        fn set_state(&mut self, state: String) {
            self.state = state;
            self.notify_observers();
        }

        fn notify_observers(&self) {
            for observer in &self.observers {
                observer.update(self);
            }
        }
    }

    struct ConsoleObserver;

    impl Observer for ConsoleObserver {
        fn update(&self, subject: &Subject) {
            println!("控制台观察者: 状态变更为 {}", subject.state);
        }
    }

    struct LoggingObserver {
        log_file: String,
    }

    impl Observer for LoggingObserver {
        fn update(&self, subject: &Subject) {
            println!(
                "日志观察器 [{}]: 状态变更为 {}",
                self.log_file, subject.state
            );
        }
    }

    let mut subject = Subject::new("初始状态".to_string());
    subject.add_observer(Box::new(ConsoleObserver));
    subject.add_observer(Box::new(LoggingObserver {
        log_file: "app.log".to_string(),
    }));

    subject.set_state("新状态".to_string());

    // 示例 3: 内存池
    #[derive(Debug)]
    struct MemoryPool {
        available: Rc<RefCell<Vec<usize>>>,
        used: Rc<RefCell<Vec<usize>>>,
        pool_size: usize,
    }

    impl MemoryPool {
        fn new(pool_size: usize) -> Self {
            let available = (0..pool_size).collect();
            Self {
                available: Rc::new(RefCell::new(available)),
                used: Rc::new(RefCell::new(Vec::new())),
                pool_size,
            }
        }

        fn allocate(&self) -> Option<usize> {
            let mut available = self.available.borrow_mut();
            let mut used = self.used.borrow_mut();

            available.pop().map(|addr| {
                used.push(addr);
                addr
            })
        }

        fn deallocate(&self, addr: usize) {
            let mut available = self.available.borrow_mut();
            let mut used = self.used.borrow_mut();

            if let Some(pos) = used.iter().position(|&x| x == addr) {
                used.remove(pos);
                available.push(addr);
            }
        }

        fn stats(&self) -> (usize, usize) {
            (self.available.borrow().len(), self.used.borrow().len())
        }
    }

    let pool = MemoryPool::new(10);
    let addr1 = pool.allocate();
    let addr2 = pool.allocate();

    println!("分配的地址: {:?}, {:?}", addr1, addr2);
    println!("内存池统计: {:?}", pool.stats());

    if let Some(addr) = addr1 {
        pool.deallocate(addr);
    }

    println!("释放后的统计: {:?}", pool.stats());

    println!();
}

// ===========================================
// 11. 取出 Rc / Arc 里的值：unwrap_or_clone（Rust 1.76）
// ===========================================

// 场景：手里有一个 `Rc<T>`（或 `Arc<T>`），最终需要的是一个拥有所有权的 `T`。
// - 如果你是**唯一的持有者**（强引用计数为 1），最省的做法是直接把 `T` 取出来，不用克隆
// - 如果还有别的持有者，就只能克隆一份，别的持有者不受影响
// 过去要手写这个套路：`Rc::try_unwrap(rc).unwrap_or_else(|rc| (*rc).clone())`。
// 1.76 起标准库直接提供了 `Rc::unwrap_or_clone(rc)` 和 `Arc::unwrap_or_clone(arc)`（要求 `T: Clone`）。
//
// 和 `Rc::make_mut` 的分工：
// - 需要**原地修改**共享数据：用 `Rc::make_mut(&mut rc)`，它在共享时才克隆（写时复制）
// - 需要拿到一个**有所有权的 T**（比如要把它移动给别的函数）：用 `unwrap_or_clone`

use std::cell::Cell;

// 带克隆计数的文档类型：每克隆一次，共享的计数器加一，方便观察“到底克隆了没有”
#[derive(Debug)]
struct Document {
    text: String,
    clone_counter: Rc<Cell<u32>>,
}

impl Clone for Document {
    fn clone(&self) -> Self {
        self.clone_counter.set(self.clone_counter.get() + 1);
        Document { text: self.text.clone(), clone_counter: Rc::clone(&self.clone_counter) }
    }
}

fn rc_unwrap_or_clone() {
    println!("=== 取出 Rc / Arc 里的值：unwrap_or_clone（Rust 1.76） ===");

    let clones = Rc::new(Cell::new(0));

    // 唯一持有者：直接取出，没有克隆
    let sole = Rc::new(Document { text: "草稿".to_string(), clone_counter: Rc::clone(&clones) });
    let owned = Rc::unwrap_or_clone(sole);
    println!("唯一持有者取出 {:?}，克隆次数 = {}", owned.text, clones.get());

    // 还有别的持有者：克隆一份，原来的那份依旧存在
    let shared = Rc::new(Document { text: "共享稿".to_string(), clone_counter: Rc::clone(&clones) });
    let other_holder = Rc::clone(&shared);
    let copy = Rc::unwrap_or_clone(shared);
    println!(
        "有其他持有者时取出 {:?}，克隆次数 = {}，另一个持有者仍然持有 {:?}",
        copy.text,
        clones.get(),
        other_holder.text
    );

    // Arc 版本用法相同，适合跨线程共享的配置：最后一个持有者取走时不用克隆
    let config = std::sync::Arc::new(vec![1, 2, 3]);
    let taken = std::sync::Arc::unwrap_or_clone(config);
    println!("Arc 的唯一持有者取出: {taken:?}");

    // 最佳实践：
    // 1. 需要所有权的值又不确定自己是不是最后一个持有者时，用 unwrap_or_clone，不要无条件 `(*rc).clone()`
    // 2. 只是想修改共享数据，用 `Rc::make_mut`（写时复制）
    println!();
}

// ===========================================
// 12. 在每条退出路径上执行清理：DropGuard（Rust 1.100）
// ===========================================

// `Drop` trait 保证：值离开作用域时一定会被清理——不管是正常走到末尾、`return`、用 `?` 提前返回，还是 panic 展开
// （本章前面的“Drop trait”一节讲的就是这个）。
// 但如果只是想“函数退出时无论如何都要执行这一小段收尾代码”，专门定义一个类型再给它实现 `Drop` 就太重了。
// 1.100 稳定了 `std::mem::DropGuard`：把“一个值 + 一个闭包”打包，离开作用域时把值交给闭包。
// 其他语言里类似的东西叫 `defer`（Go）、`finally`（Java）、`ScopeGuard`（C++ 与 Rust 的第三方 crate）。
//
// 用法要点：
// - `DropGuard::new(值, |值| 收尾代码)`：创建守卫。守卫被丢弃时，闭包以**值的所有权**被调用
// - 守卫实现了 `Deref` / `DerefMut`，被包着的值照常使用；你改成什么样，闭包里看到的就是什么样
// - `DropGuard::dismiss(守卫)`：解除守卫，取回内部的值，**不**运行闭包。这是实现“事务式回滚”的关键：
//   先动手修改，全部成功就 dismiss，中途失败（`?` 提前返回或 panic）就让守卫生效，把状态回滚
// - `dismiss` 是关联函数而不是方法（写 `DropGuard::dismiss(g)`，不写 `g.dismiss()`），
//   这样不会和被包装的值自己的同名方法冲突（`Box::leak`、`Rc::try_unwrap` 也是同样的命名风格）
// - 官方文档特别说明：想解除守卫，优先用 `dismiss`，不要用 `mem::forget`——
//   `dismiss` 会把闭包捕获的资源正常释放，`forget` 会让它们泄漏
//
// 两个典型场景：
// - 事务式回滚：批量修改一个集合，中途出错就恢复原样（见下面的 `parse_all_or_nothing`）
// - 收尾清理：不管从哪条路径退出，都要还原一个临时状态，比如“正在处理”的标记（见 `process_with_busy_flag`）
//
// 一个常见的坑：守卫必须绑定到**有名字**的变量。写成 `let _ = DropGuard::new(..);` 时，
// 值没有被任何变量接住，会在这一行结束时立刻被丢弃，清理代码就提前执行了；
// 写成 `let _cleanup = DropGuard::new(..);`（带下划线前缀的普通变量）才会一直活到作用域末尾。

use std::mem::DropGuard;
use std::num::ParseIntError;

// 全部解析成功才保留；只要有一个失败，`nums` 就保持调用之前的样子
fn parse_all_or_nothing(nums: &mut Vec<u32>, texts: &[&str]) -> Result<(), ParseIntError> {
    let original_len = nums.len();
    // 守卫包着 `&mut Vec<u32>`；闭包拿到它，失败时把中途推进去的元素截掉
    let mut nums = DropGuard::new(nums, move |nums| nums.truncate(original_len));

    for text in texts {
        nums.push(text.parse()?); // `?` 提前返回时守卫被丢弃，闭包执行，完成回滚
    }

    // 全部成功：解除守卫，闭包不会执行。取回的 `&mut Vec` 不再需要，直接丢弃
    DropGuard::dismiss(nums);
    Ok(())
}

// 收尾清理：不管从哪条路径退出，都要把“处理中”标记还原
fn process_with_busy_flag(busy: &Cell<bool>, input: &str) -> Result<u32, String> {
    busy.set(true);
    // 闭包里用不到内部的值，所以守卫里放一个 `()`
    let _cleanup = DropGuard::new((), |()| busy.set(false));

    if input.is_empty() {
        return Err("输入为空".to_string()); // 提前返回：守卫照样执行
    }
    input.parse::<u32>().map_err(|error| error.to_string()) // 正常返回 / 解析失败：同样执行
}

fn drop_guard_demo() {
    println!("=== 在每条退出路径上执行清理：DropGuard（Rust 1.100） ===");

    // ---------- 事务式回滚 ----------
    let mut numbers = vec![1, 2];
    let first = parse_all_or_nothing(&mut numbers, &["3", "4"]);
    println!("全部成功: {first:?}，numbers = {numbers:?}");
    let second = parse_all_or_nothing(&mut numbers, &["5", "x", "6"]);
    println!("中途失败: 出错 = {}，numbers = {numbers:?}（回滚到了这次调用之前）", second.is_err());

    // ---------- 收尾清理：每条退出路径都会执行 ----------
    let busy = Cell::new(false);
    for input in ["42", "", "abc"] {
        let result = process_with_busy_flag(&busy, input);
        println!("处理 {input:?} -> {result:?}，退出之后 busy = {}", busy.get());
    }

    // ---------- Deref / DerefMut：守卫里的值照常使用 ----------
    let log = RefCell::new(Vec::new());
    {
        let mut guarded = DropGuard::new(String::from("草稿"), |text| log.borrow_mut().push(format!("保存: {text}")));
        guarded.push_str("（已修改）"); // 通过 DerefMut 直接调用 String 的方法
        println!("守卫里的值: {}", *guarded);
    } // 守卫在这里被丢弃，闭包拿到的是修改之后的值
    println!("离开作用域之后的日志: {:?}", log.borrow());

    // ---------- dismiss：取回值，闭包不执行 ----------
    let guard = DropGuard::new(String::from("不需要保存"), |text| log.borrow_mut().push(format!("保存: {text}")));
    let value = DropGuard::dismiss(guard);
    println!("dismiss 取回 {value:?}，日志条数仍是 {}", log.borrow().len());

    // 最佳实践：
    // 1. “先改、后确认”的批量操作，用 `DropGuard` 包住被修改的对象，成功后 `dismiss`，失败（含 `?` 和 panic）自动回滚
    // 2. 守卫一定要绑定到有名字的变量（`_cleanup`），不要写成 `let _ = ..`
    // 3. 需要对资源有完整的生命周期管理（有状态、要复用）时，仍然应该定义专门的类型并实现 `Drop`；
    //    `DropGuard` 适合一次性的、写在函数里的收尾逻辑
    println!();
}

// ===========================================
// 13. 就地变换智能指针里的值：Box::map、Box::take、Rc::map、Arc::map（Rust 1.100）
// ===========================================

// 场景：手里有一个 `Box<T>`，想得到一个 `Box<U>`。
// 以前只能写 `Box::new(f(*boxed))`：把值从盒子里搬出来、算出新值、再**重新申请一块堆内存**装它，旧的那块被释放——
// 一次释放加一次分配。1.100 稳定了一组关联函数，把“尽量复用原来那块堆内存”变成了标准做法：
// - `Box::map(盒子, |值| 新值)`：值按**所有权**交给闭包，返回 `Box<U>`。
//   `T` 和 `U` 的大小、对齐都相同时，直接在原来那块堆内存上放新值，不分配也不释放；不同就释放旧的、分配新的
// - `Box::take(盒子)`：取走值，但**保留堆内存**，返回 `(T, Box<MaybeUninit<T>>)`；
//   配合 `Box::write(空盒子, 新值)`（1.87）可以在同一块堆内存上反复“取出—处理—放回”
// - `Rc::map(rc, |值: &T| 新值)` / `Arc::map(arc, |值: &T| 新值)`：闭包拿到的是 `&T`（引用），不是 `T`——
//   因为 Rc / Arc 可能还有别的持有者，值不能被搬走。返回新的 `Rc<U>` / `Arc<U>`；
//   只有当它是唯一持有者、并且 `T` 与 `U` 的布局相同时才会复用分配，否则就是新分配一份
//
// 为什么都写成关联函数（`Box::map(b, f)`，而不是 `b.map(f)`）？
// 因为这些智能指针会自动解引用：如果 `T` 自己也有一个叫 `map` 的方法（比如 `Option::map`、`Iterator::map`），
// `b.map(f)` 就分不清调的是谁。这是标准库里一贯的做法（`Rc::clone`、`Box::leak` 也一样）。
//
// 典型场景：流水线里把 `Box<Request>` 逐级转换成 `Box<Response>`；
// 配置放在 `Arc<Config>` 里共享，给某个子系统派生出一个只含名字的 `Arc<String>`。
// 注意：只是想看一眼数据、不需要变换时，不要用 map——借用 `&T` 就够了。

fn smart_pointer_map() {
    println!("=== 就地变换智能指针里的值：Box::map、Box::take、Rc::map、Arc::map（Rust 1.100） ===");

    // ---------- Box::map：值的所有权交给闭包，返回 Box<U> ----------
    let boxed_text: Box<String> = Box::new(String::from("rust 1.100"));
    let boxed_len: Box<usize> = Box::map(boxed_text, |text| text.len()); // String → usize，布局不同，会重新分配
    println!("Box::map 把 Box<String> 变成 Box<usize>: {boxed_len}");

    // u64 → i64：大小、对齐都相同，可以直接复用原来那块堆内存
    let boxed_unsigned = Box::new(41_u64);
    let address_before: *const u64 = Box::as_ptr(&boxed_unsigned);
    let boxed_signed: Box<i64> = Box::map(boxed_unsigned, |value| value as i64 + 1);
    let address_after: *const i64 = Box::as_ptr(&boxed_signed);
    println!(
        "u64 → i64 之后: {boxed_signed}，堆地址是否相同: {}（复用是实现细节，文档只承诺“尽可能复用”）",
        address_before.cast::<i64>() == address_after
    );

    // ---------- Box::take：取走值，保留堆内存 ----------
    let boxed = Box::new(String::from("第一份"));
    // first 是取出来的 String；empty_box 是 `Box<MaybeUninit<String>>`：堆内存还在，只是里面没有值
    let (first, empty_box) = Box::take(boxed);
    let boxed = Box::write(empty_box, String::from("第二份")); // 在同一块堆内存上放新值
    println!("take 取出了 {first:?}，同一块内存里换成了 {boxed:?}");

    // ---------- Rc::map / Arc::map：闭包拿到的是 &T ----------
    #[derive(Debug)]
    struct Config {
        name: String,
        retries: u32,
    }
    let config = Rc::new(Config { name: "api-server".to_string(), retries: 3 });
    let other_holder = Rc::clone(&config);
    // 派生出只含名字的 Rc<String>：闭包拿到 &Config，返回的必须是自己拥有数据的新值（不能借用 &Config 里的东西）
    let name_only: Rc<String> = Rc::map(config, |config| config.name.clone());
    println!("Rc::map 派生出 {name_only:?}；另一个持有者仍然持有 {other_holder:?}");

    let numbers = Arc::new(vec![1, 2, 3]);
    let total: Arc<i32> = Arc::map(numbers, |numbers| numbers.iter().sum());
    println!("Arc::map 求和得到 {total}");

    // 小结：
    // 1. `Box::map(b, f)`：把 `Box<T>` 变成 `Box<U>`，能复用堆内存就复用；比 `Box::new(f(*b))` 少一次分配和释放
    // 2. `Box::take(b)` + `Box::write(空盒子, 新值)`：循环里反复“取出—处理—放回”，整个过程只分配一次
    // 3. `Rc::map` / `Arc::map` 的闭包拿到的是 `&T`，返回值要自己拥有数据；原来的其他持有者不受影响
    // 4. 都是关联函数：写 `Box::map(b, f)`，不写 `b.map(f)`，避免和被包装类型自己的 `map` 方法混淆
    println!();
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 智能指针演示");
    println!("=================");

    smart_pointer_concepts();
    box_smart_pointer();
    deref_trait();
    drop_trait();
    rc_smart_pointer();
    refcell_smart_pointer();
    weak_reference();
    arc_smart_pointer();
    smart_pointer_combinations();
    practical_examples();
    rc_unwrap_or_clone();
    drop_guard_demo();
    smart_pointer_map();

    println!("智能指针演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_usage() {
        let b = Box::new(5);
        assert_eq!(*b, 5);
    }

    #[test]
    fn test_rc_reference_counting() {
        let a = Rc::new(5);
        let _b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn test_refcell_interior_mutability() {
        let cell = RefCell::new(5);
        *cell.borrow_mut() = 10;
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_weak_reference() {
        let rc = Rc::new(5);
        let weak = Rc::downgrade(&rc);
        assert_eq!(weak.upgrade(), Some(rc));
    }

    #[test]
    fn test_arc_thread_safety() {
        use std::thread;

        let data = Arc::new(42);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || *data_clone);

        assert_eq!(handle.join().unwrap(), 42);
    }

    #[test]
    fn test_cache_functionality() {
        // 简化的缓存测试
        use std::collections::HashMap;

        let mut cache = HashMap::new();
        cache.insert("key1".to_string(), "value1".to_string());

        assert_eq!(cache.get("key1"), Some(&"value1".to_string()));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_memory_pool() {
        // 简化的内存池测试
        struct SimplePool {
            size: usize,
            used: usize,
        }

        impl SimplePool {
            fn new(size: usize) -> Self {
                SimplePool { size, used: 0 }
            }

            fn allocate(&mut self) -> Option<usize> {
                if self.used < self.size {
                    let addr = self.used;
                    self.used += 1;
                    Some(addr)
                } else {
                    None
                }
            }

            fn stats(&self) -> (usize, usize) {
                (self.size - self.used, self.used)
            }
        }

        let mut pool = SimplePool::new(5);
        let addr1 = pool.allocate();
        let addr2 = pool.allocate();

        assert!(addr1.is_some());
        assert!(addr2.is_some());

        let (available, used) = pool.stats();
        assert_eq!(available + used, 5);
    }

    #[test]
    fn test_observer_pattern() {
        // 简化的观察者模式测试
        struct SimpleSubject {
            state: String,
        }

        impl SimpleSubject {
            fn new(state: String) -> Self {
                SimpleSubject { state }
            }

            fn set_state(&mut self, new_state: String) {
                self.state = new_state;
            }
        }

        let mut subject = SimpleSubject::new("test".to_string());
        subject.set_state("updated".to_string());
        assert_eq!(subject.state, "updated".to_string());
    }

    #[test]
    fn test_rc_unwrap_or_clone() {
        use std::cell::Cell;

        let clones = Rc::new(Cell::new(0));
        // 唯一持有者：直接取出，不克隆
        let sole = Rc::new(Document { text: "草稿".to_string(), clone_counter: Rc::clone(&clones) });
        let owned = Rc::unwrap_or_clone(sole);
        assert_eq!(owned.text, "草稿");
        assert_eq!(clones.get(), 0);

        // 还有其他持有者：克隆一份，另一个持有者不受影响
        let shared = Rc::new(Document { text: "共享稿".to_string(), clone_counter: Rc::clone(&clones) });
        let other = Rc::clone(&shared);
        let copy = Rc::unwrap_or_clone(shared);
        assert_eq!(copy.text, "共享稿");
        assert_eq!(clones.get(), 1);
        assert_eq!(Rc::strong_count(&other), 1); // shared 被消耗后，只剩 other 一个强引用
        assert_eq!(other.text, "共享稿");

        // Arc 同理
        let arc = std::sync::Arc::new(vec![1, 2, 3]);
        let other_arc = std::sync::Arc::clone(&arc);
        assert_eq!(std::sync::Arc::unwrap_or_clone(arc), [1, 2, 3]);
        assert_eq!(*other_arc, [1, 2, 3]);
    }

    #[test]
    fn test_drop_guard() {
        use std::panic::{AssertUnwindSafe, catch_unwind};

        // 事务式回滚：全部成功才保留
        let mut numbers = vec![1, 2];
        assert_eq!(parse_all_or_nothing(&mut numbers, &["3", "4"]), Ok(()));
        assert_eq!(numbers, [1, 2, 3, 4]);
        assert!(parse_all_or_nothing(&mut numbers, &["5", "x", "6"]).is_err());
        assert_eq!(numbers, [1, 2, 3, 4]);

        // 收尾清理：每条退出路径之后标记都被还原
        let busy = Cell::new(false);
        assert_eq!(process_with_busy_flag(&busy, "42"), Ok(42));
        assert!(!busy.get());
        assert!(process_with_busy_flag(&busy, "").is_err());
        assert!(!busy.get());

        // panic 展开时守卫同样会执行
        let flag = Cell::new(0);
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let _guard = DropGuard::new((), |()| flag.set(1));
            panic!("模拟中途 panic");
        }));
        assert!(outcome.is_err());
        assert_eq!(flag.get(), 1);

        // dismiss 不执行闭包；而 `let _ = ..` 会让守卫在这一行结束时立刻被丢弃
        let ran = Cell::new(0);
        let guard = DropGuard::new(5, |_| ran.set(ran.get() + 1));
        assert_eq!(DropGuard::dismiss(guard), 5);
        assert_eq!(ran.get(), 0);
        let _ = DropGuard::new(5, |_| ran.set(ran.get() + 1));
        assert_eq!(ran.get(), 1);
    }

    #[test]
    fn test_smart_pointer_map() {
        // Box::map：值的所有权交给闭包，类型可以变
        let boxed_len: Box<usize> = Box::map(Box::new(String::from("rust")), |text| text.len());
        assert_eq!(*boxed_len, 4);

        // Box::take + Box::write：同一块堆内存先后放两个值
        let (first, empty) = Box::take(Box::new(String::from("甲")));
        let second = Box::write(empty, String::from("乙"));
        assert_eq!((first.as_str(), second.as_str()), ("甲", "乙"));

        // Rc::map：闭包拿到 &T，原来的另一个持有者不受影响
        let shared = Rc::new(String::from("api"));
        let other_holder = Rc::clone(&shared);
        let length: Rc<usize> = Rc::map(shared, |text| text.len());
        assert_eq!(*length, 3);
        assert_eq!(*other_holder, "api");
        assert_eq!(Rc::strong_count(&other_holder), 1); // map 消耗了传进去的那份引用

        // Arc::map 同理
        let total: Arc<i32> = Arc::map(Arc::new(vec![1, 2, 3]), |numbers| numbers.iter().sum());
        assert_eq!(*total, 6);
    }
}
