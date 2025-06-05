use std::{any::Any, fmt};
/// #111
/// 通过克隆原型对象创建新对象 可用 Clone trait 实现.
/// 原型模式（Prototype Pattern） 是一种创建型设计模式，它通过克隆一个已有的对象来创建新对象，
/// 而不是通过构造函数显式创建。这种模式适用于对象的创建成本较高、需要复制已有状态等场景.

/// 定义一个结构体，表示图形
#[derive(Clone, Debug)] // 自动生成 clone() 方法，这是原型模式的核心。
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Circle at ({}, {}) with radius {}",
            self.x, self.y, self.radius
        )
    }
}
//-------------------原型接口---------------------------------
///定义一个抽象的原型接口（trait)，虽然不是必须的，但可以统一行为
pub trait Prototype {
    /// 克隆自身
    fn clone_box(&self) -> Box<dyn Prototype>;

    /// 返回模拟动态类型的接口
    fn as_any(&self) -> &dyn Any;
}

impl Prototype for Circle {
    ///clone_box 是为了支持 trait object 的动态克隆（因为 trait 不能直接 impl Clone）。
    /// 第一层Box: 为了把具体类型擦除为 trait object
    fn clone_box(&self) -> Box<dyn Prototype> { //Box<dyn Prototype> 可以用于统一处理不同类型的原型对象（多态克隆）。
        //第二层Box: 返回新的 Box<dyn Prototype>，是为了克隆整个 trait object
        Box::new(self.clone()) // 使用标准库的 Clone trait 进行克隆
    }

    /// as_any 用于运行时类型检查和转换。
    fn as_any(&self) -> &dyn Any {
        self
    }
}
//-----------------------main方法--------------------------------
fn main() {
    // 创建一个原型对象
    let original_circle = Circle {
        x: 10,
        y: 20,
        radius: 5,
    };
    // 克隆出一个新的对象
    let cloned_circle = original_circle.clone();

    // 打印原始对象和克隆对象
    println!("Original Circle: {}", original_circle);
    println!("Cloned Circle: {}", cloned_circle);

    //使用 trait object 动态克隆
    //第一层Box：为了把具体类型擦除为 trait object
    let proto = Box::new(original_circle);
    let another = proto.clone_box();
    // let another = original_circle.clone_box();

    //检查是否为Circle类型
    if let Some(circle) = another.as_any().downcast_ref::<Circle>() {
        println!("Another Circle: {}", circle);
    }
}
