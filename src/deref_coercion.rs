use std::ops::Deref;
/// 使用 Deref coercion（解引用强制转换） 的完整示例代码，演示如何通过实现 Deref trait 来实现智能指针的透明访问：
/**
 * Deref coercion 是 Rust 中非常独特的特性，它允许你自定义类型像指针一样被解引用。
 *   常用于封装智能指针、共享引用等场景。
 *   编译器会自动插入 .deref() 调用，使代码更简洁。
 *   配合 Drop、DerefMut 等 trait，可以构建安全高效的资源管理抽象。
 */
// 定义一个智能指针结构体
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 创建一个新的 MyBox 实例
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

// 为 MyBox 实现 Deref Trait，使得可以像指针一样访问内部值
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 示例函数，接受一个字符串引用
pub fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // 使用 Deref coercion 访问内部值
    assert_eq!(5, *y); // *y 相当于 *(y.deref())

    // 智能指针透明访问
    let name = MyBox::new(String::from("Alice"));
    say_hello(&name); // MyBox<String> 被自动转换为 &String，再转换为 &str
}