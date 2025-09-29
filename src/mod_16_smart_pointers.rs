// Rust 智能指针 (Smart Pointers)
// 深入讲解 Rust 中的智能指针概念，包括 Box、Rc、Arc、RefCell 等
// 智能指针是 Rust 中实现数据结构设计和内存管理的重要工具

// ===========================================
// 1. 智能指针基础概念
// ===========================================

// 智能指针是指实现 Deref 和 Drop trait 的数据结构
// 它们的行为类似于指针但拥有额外的元数据和功能
// 智能指针的核心价值在于提供更灵活的内存管理方式

use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};
use std::collections::LinkedList;

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

    let c = CustomSmartPointer {
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

    let b = Rc::clone(&a); // 增加引用计数
    println!("引用计数: {}", Rc::strong_count(&a)); // 2

    {
        let c = Rc::clone(&a); // 增加引用计数
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

    println!("图结构: node1 被多个节点共享");

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
            let child_weak = Rc::downgrade(&child); // 创建弱引用
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
    if let Some(mut node_data) = Rc::get_mut(&mut child.clone()) {
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

    let data = Arc::new(vec
![1, 2, 3, 4, 5]);

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
            if let Some(mut neighbor_data) = Rc::get_mut(&mut neighbor.clone()) {
                neighbor_data.back_edges.push(self_weak);
            }
            if let Some(mut self_data) = Rc::get_mut(&mut self.clone()) {
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
        Box::new(Dog { name: "旺财".to_string() }),
        Box::new(Cat { name: "咪咪".to_string() }),
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
            println!("日志观察器 [{}]: 状态变更为 {}", self.log_file, subject.state);
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
        let b = Rc::clone(&a);
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

        let handle = thread::spawn(move || {
            *data_clone
        });

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
}