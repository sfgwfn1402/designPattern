//!  工厂方法模式
//! ```
//! 工厂方法模式（Factory Method Pattern） 是一种创建型设计模式，它通过定义一个用于创建对象的接口（trait 或函数），
//! 但让子类决定实例化哪一个类。这种模式将对象的创建延迟到实现该接口的具体类型中。
//! 例：将以“不同类型的按钮”为例，展示如何通过工厂方法创建不同的 UI 控件。
//! ```
//!

//-----------定义产品trait(抽象接口)-----------------------------------------------------------------------------

/// 按钮 trait
pub trait Button {
    ///渲染按钮
    /// # 参数
    /// - `self`： 自身引用
    /// # 返回值
    /// 无
    fn render(&self);

    ///点击按钮
    fn on_click(&self);
}

//-----------实现具体产品（Windows 和 Web 按钮）-----------------------------------------------------------------------------
/// Windows 风格按钮
pub struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Rendering a Windows button");
    }
    fn on_click(&self) {
        println!("Windows button clicked");
    }
}

/// Web 风格按钮
pub struct WebButton;

impl Button for WebButton {
    fn render(&self) {
        println!("Rendering a Web button");
    }
    fn on_click(&self) {
        println!("Web button clicked");
    }
}
//-----------定义工厂trait(工厂方法接口)-----------------------------------------------------------------------------
/// 工厂trait: 定义创建按钮的方法
pub trait ButtonFactory {
    /// 工厂方法：由子类实现，返回某种按钮
    fn create_button(&self) -> Box<dyn Button>;
}

///创建Windows风格按钮的工厂
pub struct WindowsButtonFactory;

impl ButtonFactory for WindowsButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}
///创建Web风格按钮的工厂
pub struct WebButtonFactory;
impl ButtonFactory for WebButtonFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WebButton)
    }
}
//-------------------------------
fn main() {
    //根据平台选择工厂
    let os = "windows";
    let factory: Box<dyn ButtonFactory> = match os {
        "windows" => Box::new(WindowsButtonFactory),
        "web" => Box::new(WebButtonFactory),
        _ => panic!("unknown os"),
    };
    //通过工厂方法创建按钮
    let button = factory.create_button();

    //使用按钮
    button.render();
    button.on_click();
}
