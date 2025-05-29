///装饰器模式（Decorator Pattern）可以通过 trait 和结构体组合的方式来实现动态添加行为
///基础特性
pub trait Component {
    fn operation(&self);
}

//实现一个基础装饰器
pub struct ConcreteComponent;
impl Component for ConcreteComponent {
    fn operation(&self) {
        println!("ConcreteComponent operation");
    }
}
//-----------------------------------------------------

///定义装饰器特性
pub trait Decorator: Component {}

//-----------------------------------------------------

///实现具体的装饰器
pub struct ConcreteDecoratorA<T: Component>  {
    ///该组件被装饰，这是一个接口类型变量
    component : T,
}

impl<T: Component> ConcreteDecoratorA<T> {
    pub fn new(component: T) -> Self {
        ConcreteDecoratorA { component }
    }

    fn added_behavior(&self) {
        println!("ConcreteDecoratorA added behavior");
    }
}

impl<T: Component> Component for ConcreteDecoratorA<T> {
    fn operation(&self) {
        //先调用被装饰的组件
        self.component.operation();
        //再调用自己的行为
        self.added_behavior();
    }
}
//--------------------------------------------------------------------------------------------------
fn main() {
    let component = ConcreteComponent;
    let decorated_component = ConcreteDecoratorA::new(component);
    decorated_component.operation();
}