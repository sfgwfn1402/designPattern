///观察者模式（Observer Pattern）可以通过 trait 和组合的方式来实现一对多的依赖通知机制

///定义观察者接口
pub trait Observer  {
    fn update(&self, message: &str);
}
///具体观察者
pub struct ConcreteObserver;
impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("ConcreteObserver received message: {}", message);
    }
}

//-------------------------------------------------------------------
///主题
pub struct Subject {
    /// 观察者列表
    observers: Vec<Box<dyn Observer>>,
}

impl Subject  {
    /// 创建主题
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }
    /// 注册观察者
    pub fn register_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    /// 通知观察者
    pub fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}
//--------------------------------------------------------------------
fn main() {
    // 创建主题
    let mut subject = Subject::new();
    // 创建观察者
    let observer1 = Box::new(ConcreteObserver);
    let observer2 = Box::new(ConcreteObserver);
    // 注册观察者
    subject.register_observer(observer1);
    subject.register_observer(observer2);

    // 通知观察者
    subject.notify_observers("Hello, World!");
}