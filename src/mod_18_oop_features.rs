// Rust 面向对象特性
// 深入讲解 Rust 中实现面向对象编程的各种模式和技术
// 基于《The Rust Programming Language》第 19 章内容



// ===========================================
#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_macros, deprecated)]

// Rust 面向对象特性教程
// ===========================================

// Rust 虽然不是传统的面向对象语言，但提供了强大的机制来实现 OOP 概念
// 本章探讨如何在 Rust 中实现 OOP 的核心特性：封装、继承、多态

fn oop_concepts() {
    println!("=== Rust 中的面向对象编程概念 ===");

    // 传统 OOP 语言的四个核心特性：
    // 1. 封装 (Encapsulation)：隐藏实现细节，提供公共接口
    // 2. 继承 (Inheritance)：代码复用和类型层次结构
    // 3. 多态 (Polymorphism)：同一接口，不同实现
    // 4. 抽象 (Abstraction)：定义通用接口，隐藏具体实现

    // Rust 对这些特性的支持：
    // - 封装：通过 pub 关键字和模块系统实现
    // - 继承：通过 trait 组合和默认实现实现
    // - 多态：通过 trait 对象和泛型实现
    // - 抽象：通过 trait 和抽象类型实现

    // Rust 的 OOP 实现理念：
    // - 组合优于继承 (Composition over inheritance)
    // - 基于特征的系统比传统的类继承更灵活
    // - 零成本抽象 (Zero-cost abstractions)
    // - 编译时多态和运行时多态的选择

    println!("Rust 通过 trait、结构体和 trait 对象实现了独特的 OOP 模式");
    println!();
}

// ===========================================
// 2. 封装 (Encapsulation)
// ===========================================

// 封装是 OOP 的核心概念，Rust 通过可见性控制实现封装
// 默认情况下，所有项都是私有的，只有标记为 pub 的项才对外可见

fn encapsulation() {
    println!("=== 封装 ===");

    // 创建一个封装的图形库
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(4.0, 6.0);

    println!("圆形面积: {}", circle.area());
    println!("矩形面积: {}", rectangle.area());

    // 封装的优势：
    // 1. 实现细节隐藏，接口清晰
    // 2. 防止外部代码破坏内部状态
    // 3. 可以修改内部实现而不影响外部代码
    // 4. 提供数据验证和约束

    // 封装的最佳实践：
    // - 只暴露必要的公共接口
    // - 内部实现细节保持私有
    // - 使用构造函数确保对象状态的合法性
    // - 提供验证和约束机制

    println!("通过封装，我们确保了图形对象的状态始终是有效的");
    println!();
}

// 封装的图形结构体
// 所有字段都是私有的，只能通过公共方法访问
#[derive(Debug, Clone)]
struct Circle {
    radius: f64,      // 半径私有，确保不会设置为负值
    center: (f64, f64), // 圆心坐标私有
}

impl Circle {
    // 关联函数作为构造函数
    // 验证输入参数，确保对象状态有效
    pub fn new(radius: f64) -> Self {
        assert!(radius > 0.0, "半径必须大于 0");
        Circle {
            radius,
            center: (0.0, 0.0),
        }
    }

    // 带圆心的构造函数
    pub fn with_center(radius: f64, center: (f64, f64)) -> Self {
        assert!(radius > 0.0, "半径必须大于 0");
        Circle { radius, center }
    }

    // 计算面积 - 公共接口
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // 计算周长 - 公共接口
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    // 安全的半径设置方法
    pub fn set_radius(&mut self, radius: f64) {
        if radius > 0.0 {
            self.radius = radius;
        } else {
            panic!("半径必须大于 0");
        }
    }

    // 获取半径 - 只读访问
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // 内部辅助方法 - 私有
    fn validate_radius(radius: f64) -> bool {
        radius > 0.0
    }
}

// 矩形结构体 - 展示封装的另一种形式
#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 构造函数
    pub fn new(width: f64, height: f64) -> Self {
        assert!(width > 0.0 && height > 0.0, "宽度和高度必须大于 0");
        Rectangle { width, height }
    }

    // 计算面积
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    // 计算周长
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // 判断是否为正方形
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    // 调整大小，保持宽高比
    pub fn resize(&mut self, scale: f64) {
        assert!(scale > 0.0, "缩放比例必须大于 0");
        self.width *= scale;
        self.height *= scale;
    }
}

// ===========================================
// 3. 继承的替代方案 (Inheritance Alternatives)
// ===========================================

// Rust 没有传统的类继承，但提供了更灵活的机制来实现代码复用
// 1. Trait 默认实现 (Default trait implementations)
// 2. 组合 (Composition)
// 3. Trait 继承 (Trait inheritance)

fn inheritance_alternatives() {
    println!("=== 继承的替代方案 ===");

    // 示例 1：使用 trait 默认实现
    let animal = Dog::new("Buddy");
    animal.make_sound();
    println!("动物信息: {}", animal.info());

    // 示例 2：使用组合模式
    let gui_button = GuiButton {
        widget: Widget { x: 10, y: 20, width: 100, height: 30 },
        text: "点击我".to_string(),
        color: (255, 0, 0),
    };
    gui_button.draw();

    // 示例 3：trait 继承
    let writer = ConsoleWriter;
    writer.write("Hello, World!".to_string());
    writer.log("这是一条日志信息".to_string());

    println!("Rust 通过 trait 和组合提供了比传统继承更灵活的代码复用机制");
    println!();
}

// Animal trait - 定义动物的基本行为
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // 默认实现，可以被覆盖
    fn make_sound(&self) {
        println!("{} 发出声音: {}", self.name(), self.sound());
    }

    // 另一个默认实现
    fn info(&self) -> String {
        format!("这是一个 {}，它会 {}", self.name(), self.sound())
    }
}

// Dog 结构体 - 实现 Animal trait
struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: &str) -> Self {
        Dog {
            name: name.to_string(),
        }
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn sound(&self) -> &str {
        "汪汪"
    }

    // 覆盖默认实现
    fn make_sound(&self) {
        println!("{} 欢快地叫着: {}!", self.name(), self.sound());
    }
}

// Cat 结构体 - 另一个 Animal 实现
struct Cat {
    name: String,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        Cat {
            name: name.to_string(),
        }
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn sound(&self) -> &str {
        "喵喵"
    }
}

// 组合模式示例
// 基础组件
#[derive(Debug)]
struct Widget {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Widget {
    pub fn draw(&self) {
        println!("绘制组件: 位置({}, {}), 尺寸 {}x{}",
                 self.x, self.y, self.width, self.height);
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.width &&
        y >= self.y && y <= self.y + self.height
    }
}

// 按钮组件 - 通过组合扩展 Widget
#[derive(Debug)]
struct GuiButton {
    widget: Widget,  // 组合 Widget
    text: String,
    color: (u8, u8, u8),
}

impl GuiButton {
    pub fn new(x: i32, y: i32, width: i32, height: i32, text: &str) -> Self {
        GuiButton {
            widget: Widget { x, y, width, height },
            text: text.to_string(),
            color: (0, 0, 255), // 默认蓝色
        }
    }

    pub fn draw(&self) {
        println!("绘制按钮: '{}'", self.text);
        self.widget.draw();
        println!("按钮颜色: RGB({}, {}, {})", self.color.0, self.color.1, self.color.2);
    }

    pub fn click(&self) {
        println!("按钮 '{}' 被点击了!", self.text);
    }
}

// Trait 继承示例
// 基础 Writer trait
trait Writer {
    fn write(&self, content: String);
}

// Logger trait 继承 Writer
trait Logger: Writer {
    fn log(&self, message: String);
}

// ConsoleWriter 实现 Logger
struct ConsoleWriter;

impl Writer for ConsoleWriter {
    fn write(&self, content: String) {
        println!("写入: {}", content);
    }
}

impl Logger for ConsoleWriter {
    fn log(&self, message: String) {
        println!("日志: {}", message);
    }
}

// ===========================================
// 4. 多态与 Trait 对象 (Polymorphism and Trait Objects)
// ===========================================

// 多态允许使用统一的接口处理不同类型的数据
// Rust 通过 trait 对象实现运行时多态，通过泛型实现编译时多态

fn polymorphism() {
    println!("=== 多态与 Trait 对象 ===");

    // 运行时多态：使用 trait 对象
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Buddy")),
        Box::new(Cat::new("Whiskers")),
    ];

    println!("动物园中的动物:");
    for animal in animals {
        animal.make_sound();
    }

    // 编译时多态：使用泛型
    let dog = Dog::new("Max");
    let cat = Cat::new("Luna");

    print_animal_info(&dog);
    print_animal_info(&cat);

    // 动态分发 vs 静态分发
    let mut screen = Screen::new();
    screen.add_component(Box::new(DrawableButton::new(100, 40, "Submit")));
    screen.add_component(Box::new(DrawableTextField::new(200, 30, "输入内容")));
    screen.run();

    println!("多态让代码更加灵活和可扩展");
    println!();
}

// 泛型函数实现编译时多态
// 在编译时为每个类型生成专门的代码
fn print_animal_info<T: Animal>(animal: &T) {
    println!("动物信息: {}", animal.info());
}

// Draw trait 用于 GUI 组件
trait Draw {
    fn draw(&self);
}

// Button 组件实现 Draw
#[derive(Debug)]
struct DrawableButton {
    width: u32,
    height: u32,
    label: String,
}

impl DrawableButton {
    pub fn new(width: u32, height: u32, label: &str) -> Self {
        DrawableButton {
            width,
            height,
            label: label.to_string(),
        }
    }
}

impl Draw for DrawableButton {
    fn draw(&self) {
        println!("绘制按钮: {} ({}x{})", self.label, self.width, self.height);
    }
}

// TextField 组件实现 Draw
#[derive(Debug)]
struct DrawableTextField {
    width: u32,
    height: u32,
    placeholder: String,
}

impl DrawableTextField {
    pub fn new(width: u32, height: u32, placeholder: &str) -> Self {
        DrawableTextField {
            width,
            height,
            placeholder: placeholder.to_string(),
        }
    }
}

impl Draw for DrawableTextField {
    fn draw(&self) {
        println!("绘制文本框: {} ({}x{})", self.placeholder, self.width, self.height);
    }
}

// Screen 包含多个可绘制的组件
// 使用 trait 对象 Box<dyn Draw> 实现动态分发
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        println!("运行屏幕渲染:");
        for component in &self.components {
            component.draw();
        }
    }
}

// ===========================================
// 5. 对象安全 (Object Safety)
// ===========================================

// Trait 对象要求 trait 是对象安全的 (object safe)
// 对象安全的 trait 才能被用作 trait 对象

fn object_safety() {
    println!("=== 对象安全 ===");

    // 展示对象安全的 trait
    let notifier = EmailNotifier;
    send_notification(&notifier, "重要通知".to_string());

    // 展示非对象安全的 trait 的替代方案
    let factory = AnimalFactory;
    let dog = factory.create_animal("Buddy");
    dog.make_sound();

    println!("理解对象安全对于正确使用 trait 对象很重要");
    println!();
}

// 对象安全的 trait
// 不包含泛型参数、Self 类型或返回 Self 类型的方法
trait Notifier {
    fn send(&self, message: String);

    // 这个方法是对象安全的
    fn send_twice(&self, message: String) {
        self.send(message.clone());
        self.send(message);
    }
}

// 非对象安全的 trait
// 包含返回 Self 类型的方法
trait CloneFactory {
    fn create(&self) -> Self;
}

// 对象安全的 Notifier 实现
struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn send(&self, message: String) {
        println!("发送邮件通知: {}", message);
    }
}

// 为非对象安全 trait 提供替代方案
trait AnimalCreator {
    fn create_animal(&self, name: &str) -> Box<dyn Animal>;
}

struct AnimalFactory;

impl AnimalCreator for AnimalFactory {
    fn create_animal(&self, name: &str) -> Box<dyn Animal> {
        Box::new(Dog::new(name))
    }
}

// 使用对象安全的 trait 对象
fn send_notification(notifier: &dyn Notifier, message: String) {
    notifier.send(message);
}

// ===========================================
// 6. 状态模式 (State Pattern)
// ===========================================

// 状态模式是一种行为设计模式，允许对象在内部状态改变时改变其行为
// Rust 可以通过 trait 对象优雅地实现状态模式

fn state_pattern() {
    println!("=== 状态模式 ===");

    let mut post = Post::new();

    // 草稿状态
    let _ = post.add_text("我在学习 Rust");
    println!("内容: {}", post.content());

    // 审核状态
    post.request_review();
    println!("内容: {}", post.content());

    // 发布状态
    post.approve();
    println!("内容: {}", post.content());

    // 尝试在已发布状态下添加内容（应该失败）
    let result = post.add_text("更多内容");
    println!("尝试添加内容: {:?}", result);

    println!("状态模式让状态转换更加安全和清晰");
    println!();
}

// 状态 trait 定义状态的行为
trait State {
    // 内容是否可以被评论
    fn is_review_allowed(&self) -> bool;

    // 添加文本的行为
    fn add_text(&self, post: &mut Post, text: &str) -> Result<(), String>;

    // 请求审核
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // 批准发布
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

// 草稿状态
struct Draft {}

impl State for Draft {
    fn is_review_allowed(&self) -> bool {
        false
    }

    fn add_text(&self, post: &mut Post, text: &str) -> Result<(), String> {
        post.content.push_str(text);
        Ok(())
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // 草稿状态不能直接批准
    }
}

// 待审核状态
struct PendingReview {}

impl State for PendingReview {
    fn is_review_allowed(&self) -> bool {
        true
    }

    fn add_text(&self, _post: &mut Post, _text: &str) -> Result<(), String> {
        Err("待审核状态下不能修改内容".to_string())
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // 已经在审核状态
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 已发布状态
struct Published {}

impl State for Published {
    fn is_review_allowed(&self) -> bool {
        false
    }

    fn add_text(&self, _post: &mut Post, _text: &str) -> Result<(), String> {
        Err("已发布的文章不能修改".to_string())
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // 已发布的文章不能重新审核
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // 已经是发布状态
    }
}

// Post 结构体使用状态模式
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), String> {
        if let Some(state) = self.state.take() {
            let result = state.add_text(self, text);
            self.state = Some(state);
            result
        } else {
            Err("文章状态无效".to_string())
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }

    pub fn is_review_allowed(&self) -> bool {
        if let Some(ref state) = self.state {
            state.is_review_allowed()
        } else {
            false
        }
    }
}

// ===========================================
// 7. 策略模式 (Strategy Pattern)
// ===========================================

// 策略模式定义了一系列算法，并将每个算法封装起来
// 使它们可以相互替换，且算法的变化不会影响使用算法的客户端

fn strategy_pattern() {
    println!("=== 策略模式 ===");

    let text = "Hello World";

    // 使用不同的策略
    let strategies: Vec<Box<dyn Formatter>> = vec![
        Box::new(UpperCaseFormatter),
        Box::new(LowerCaseFormatter),
        Box::new(CapitalizeFormatter),
    ];

    for strategy in strategies {
        let formatted = format_text(text, &*strategy);
        println!("格式化结果: {}", formatted);
    }

    println!("策略模式使算法可以独立于使用它们的客户端而变化");
    println!();
}

// 策略 trait
trait Formatter {
    fn format(&self, text: &str) -> String;
}

// 大写格式化策略
struct UpperCaseFormatter;

impl Formatter for UpperCaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_uppercase()
    }
}

// 小写格式化策略
struct LowerCaseFormatter;

impl Formatter for LowerCaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

// 首字母大写格式化策略
struct CapitalizeFormatter;

impl Formatter for CapitalizeFormatter {
    fn format(&self, text: &str) -> String {
        if let Some(first_char) = text.chars().next() {
            let mut result = String::new();
            result.push(first_char.to_uppercase().next().unwrap());
            result.push_str(&text[1..]);
            result
        } else {
            text.to_string()
        }
    }
}

// 使用策略的上下文
fn format_text(text: &str, formatter: &dyn Formatter) -> String {
    formatter.format(text)
}

// ===========================================
// 8. 建造者模式 (Builder Pattern)
// ===========================================

// 建造者模式将复杂对象的构建与其表示分离
// 使得同样的构建过程可以创建不同的表示

fn builder_pattern() {
    println!("=== 建造者模式 ===");

    // 使用建造者模式创建用户
    let user = User::builder()
        .name("张三")
        .email("zhangsan@example.com")
        .age(25)
        .build()
        .unwrap();

    println!("创建的用户: {:?}", user);

    println!("建造者模式使复杂对象的创建更加清晰和灵活");
    println!();
}

// 用户结构体
#[derive(Debug, Clone)]
struct User {
    name: String,
    email: String,
    age: u32,
    address: Option<String>,
    phone: Option<String>,
}

impl User {
    // 建造者
    pub fn builder() -> UserBuilder {
        UserBuilder::new()
    }
}

// 用户建造者
#[derive(Debug)]
struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    address: Option<String>,
    phone: Option<String>,
}

impl UserBuilder {
    pub fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
            address: None,
            phone: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_string());
        self
    }

    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_string());
        self
    }

    pub fn build(self) -> Result<User, String> {
        if self.name.is_none() {
            return Err("用户名是必需的".to_string());
        }
        if self.email.is_none() {
            return Err("邮箱是必需的".to_string());
        }

        Ok(User {
            name: self.name.unwrap(),
            email: self.email.unwrap(),
            age: self.age.unwrap_or(0),
            address: self.address,
            phone: self.phone,
        })
    }
}

// ===========================================
// 主函数
// ===========================================

pub fn main() {
    println!("Rust 面向对象编程模式演示");
    println!("=======================");
    println!();

    oop_concepts();
    encapsulation();
    inheritance_alternatives();
    polymorphism();
    object_safety();
    state_pattern();
    strategy_pattern();
    builder_pattern();

    println!("面向对象编程模式演示完成！");
}

// ===========================================
// 测试函数
// ===========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_encapsulation() {
        let circle = Circle::new(5.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
        assert_eq!(circle.radius(), 5.0);
    }

    #[test]
    fn test_animal_trait() {
        let dog = Dog::new("Buddy");
        assert_eq!(dog.name(), "Buddy");
        assert_eq!(dog.sound(), "汪汪");
    }

    #[test]
    fn test_polymorphism() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog::new("Max")),
            Box::new(Cat::new("Luna")),
        ];

        for animal in animals {
            animal.make_sound();
        }
    }

    #[test]
    fn test_state_pattern() {
        let mut post = Post::new();
        post.add_text("测试内容").unwrap();
        assert!(!post.is_review_allowed());

        post.request_review();
        assert!(post.is_review_allowed());

        post.approve();
        assert!(!post.is_review_allowed());
    }

    #[test]
    fn test_strategy_pattern() {
        let text = "hello world";
        let formatter = UpperCaseFormatter;
        assert_eq!(formatter.format(text), "HELLO WORLD");
    }

    #[test]
    fn test_builder_pattern() {
        let user = User::builder()
            .name("测试用户")
            .email("test@example.com")
            .build()
            .unwrap();

        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_screen_components() {
        let mut screen = Screen::new();
        screen.add_component(Box::new(DrawableButton::new(100, 50, "测试按钮")));
        screen.add_component(Box::new(DrawableTextField::new(200, 30, "占位符")));

        assert_eq!(screen.components.len(), 2);
    }
}