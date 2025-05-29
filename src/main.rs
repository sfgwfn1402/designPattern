mod adapter;
mod flyweight;
mod proxy;
mod responsibility_chain;
mod strategy;
mod template_method;
mod singleton;
mod singleton2;
mod objectpoll;
mod bridge;
mod new_type;
mod deref_coercion;
mod interior_mutability;
mod drop;
mod phantom_data;
mod builder_default;
mod panic_result;
mod decorator;
mod observer;
mod command;
mod prototype;
mod factory_method;


use crate::factory_method::{Button, ButtonFactory, WindowsButtonFactory, WebButtonFactory};
use crate::prototype::{Circle, Prototype};
use crate::command::{Command, Light, LightOnCommand, RemoteControl};
use crate::observer::{Subject, Observer, ConcreteObserver};
use crate::decorator::{Component, ConcreteComponent, ConcreteDecoratorA};
use crate::panic_result::{divide, safe_divide};
use crate::builder_default::{Config, ConfigBuilder};
use crate::phantom_data::MyBox2;
use crate::drop::MyResource;
use crate::interior_mutability::{MyData, SharedCounter, ReadWriteData};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

// use bridge::{Circle, DirectXRenderer, OpenGLRenderer, Renderer};

use crate::deref_coercion::MyBox;
use crate::deref_coercion::say_hello;
use crate::new_type::{UserId};
use crate::objectpoll::{Connection, ConnectionPool};
use crate::singleton2::Singleton2;
use crate::singleton::Singleton;
use crate::adapter::{LegacyRectangle, RectangleAdapter, Shape};
use crate::flyweight::FlyweightFactory;
// use crate::proxy::{Proxy, RealSubject, Subject};
use crate::responsibility_chain::{ConcreteHandlerA, ConcreteHandlerB, Handler, Request};
use crate::strategy::{ConcreteStrategyA, ConcreteStrategyB, Context};
use crate::template_method::{AbstractClass, ConcreteClassA, ConcreteClassB};

fn main() {
    //工厂方法模式-------------------------------------------------------
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

    //原型模式----------------------------------------------------------
    // 创建一个原型对象
    // let original_circle = prototype::Circle {
    //     x: 10,
    //     y: 20,
    //     radius: 5,
    // };
    // // 克隆出一个新的对象
    // let cloned_circle = original_circle.clone();

    // // 打印原始对象和克隆对象
    // println!("Original Circle: {}", original_circle);
    // println!("Cloned Circle: {}", cloned_circle);

    // //使用 trait object 动态克隆
    // let proto = Box::new(original_circle);
    // let another = proto.clone_box();
    // // let another = original_circle.clone_box();

    // //检查是否为Circle类型
    // if let Some(circle) = another.as_any().downcast_ref::<prototype::Circle>() {
    //     println!("Another Circle: {}", circle);
    // }

    //命令模式----------------------------------------------------------
    //创建被调用对象
    //创建灯对象
    // let light = Arc::new(Mutex::new(Light::new()));

    // //创建打开灯的命令对象
    // let light_on_command = LightOnCommand::new(light.clone());

    // //创建遥控器对象
    // let mut remote_control = RemoteControl::new();

    // //设置遥控器的命令
    // remote_control.set_command(Box::new(light_on_command));

    // //按下按钮，执行命令
    // remote_control.press_button();

    // //撤销按钮，执行撤销命令
    // remote_control.undo_button();

    //观察者模式--------------------------------------------------------
       // 创建主题
    // let mut subject = observer::Subject::new();
    // // 创建观察者
    // let observer1 = Box::new(ConcreteObserver);
    // let observer2 = Box::new(ConcreteObserver);
    // // 注册观察者
    // subject.register_observer(observer1);
    // subject.register_observer(observer2);

    // // 通知观察者
    // subject.notify_observers("Hello, World!");


    //装饰器模式--------------------------------------------------------
    // let component = ConcreteComponent;
    // let decorated_component = ConcreteDecoratorA::new(component);
    // decorated_component.operation();

    //panic 和 result ------------------------------------------------
    // let result = divide(10, 0); // 这会 panic!
    // println!("Result: {}", result);

    // match safe_divide(10, 0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
    // Builder 模式 --------------------------------------------------
    // 使用 Builder 构造 Config，默认值 + 自定义部分字段
    // let config = ConfigBuilder::default()
    //     .timeout(Duration::from_secs(10))
    //     .retries(5)
    //     .enable_logging(false)
    //     .build();

    // println!("Config: {:?}", config);

    //Send 和 Sync 是两个特殊的 marker trait（标记 trait），
    //它们是 Rust 独有的线程安全机制，用于控制类型是否可以跨线程安全传递或共享。
    //Send	如果一个类型 T: Send，那么它可以安全地在线程间转移所有权（即可以通过 channel 发送到另一个线程）
    //Sync	如果一个类型 T: Sync，那么它可以被多个线程同时安全地共享引用（即 &T 可以跨线程传递）
    //注：移动是语法行为，而Send是语义承诺。Rust 要求只有实现了 Send 的类型才能被安全地 move 到另一个线程中，以保障线程安全。
    //场景1：Send示例 - 跨线程传递数据
    // let data = vec![1, 2, 3];
    // thread::spawn(move || {
    //     println!("Data in thread: {:?}", data);
    // }).join().unwrap();

    //场景2：Sync示例 - 多线程共享数据
    // let data = Arc::new(vec![1, 2, 3]);
    // let mut handles = vec![];
    // for _ in 0..3 {
    //     let data_clone = data.clone();
    //     let handle = thread::spawn(move || {
    //         println!("Data in thread: {:?}", data_clone);
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    

    //一个简单的“智能指针”封装裸指针，并用 PhantomData 标记它“像”拥有 T 类型的数据
    // let val = 42;
    // let boxed = MyBox2::new(&val);

    // println!("Value: {}", boxed.get()); // 输出 42
    
    //Drop----------------------------------------------------------
    // {
    //     let _resource1 = MyResource { name: "Resource 1".to_string() };
    //     let _resource2 = MyResource { name: "Resource 2".to_string() };
    //     println!("Exiting inner scope...");
    // }

    // println!("Exited outer scope. Resources have been dropped.");

    //Interior Mutability（内部可变性）是 Rust 独有的设计模式----------------
    // RefCell 示例
    // let data = MyData::new(5);
    // data.increment();
    // println!("RefCell value: {}", data.get_value()); // 输出 6

    // // Mutex 示例（多线程）
    // let counter = SharedCounter::new(0);
    // let counter_clone = SharedCounter {
    //     count: counter.count.clone(),
    // };
    // //查看两个结构对象的字段count指向的值的地址是否是同一个
    // assert_eq!(Arc::<Mutex<i32>>::as_ptr(&counter.count), Arc::<Mutex<i32>>::as_ptr(&counter_clone.count));
    // assert_eq!(Arc::as_ptr(&counter.count), Arc::as_ptr(&counter_clone.count));

    
    // let handle = thread::spawn(move || {
    //     for _ in 0..5 {
    //         counter_clone.increment();
    //     }
    // });

    // handle.join().unwrap();
    // println!("Mutex value: {}", counter.get_count()); // 输出 5

    // // RwLock 示例
    // let rw_data = ReadWriteData::new("hello");
    // {
    //     let read1 = rw_data.read_data();
    //     println!("Read 1: {}", read1); // hello
    // }

    // rw_data.write_data("world");

    // {
    //     let read2 = rw_data.read_data();
    //     println!("Read 2: {}", read2); // world
    // }
    //使用 Deref coercion（解引用强制转换） 的完整示例代码，演示如何通过实现 Deref trait 来实现智能指针的透明访问：
    // let x = 5;
    // let y = MyBox::new(x);

    // // 使用 Deref coercion 访问内部值
    // assert_eq!(5, *y); // *y 相当于 *(y.deref())

    // // 智能指针透明访问
    // let name = MyBox::new(String::from("Alice"));
    // say_hello(&name); // MyBox<String> 被自动转换为 &String，再转换为 &str


    //NewType 模式 --------------------------------------------------
    // 使用元组结构体包装基础类型以增加类型安全性
    // let user_id = UserId::new(42);
    // println!("User ID: {}", user_id.get());

    //桥接模式 --------------------------------------------------
    //  创建OpenGL渲染器、创建DirectX渲染器
    // let opengl_renderer = Box::new(OpenGLRenderer);
    // let directx_renderer : Box<dyn Renderer + Send + Sync> = Box::new(DirectXRenderer);

    // // 创建圆形
    // let circle1 = Circle::new(opengl_renderer, 5.0);
    // let circle2 = Circle::new(directx_renderer, 10.0);
    
    // //  绘制圆形
    // circle1.draw();
    // circle2.draw();

    //object poll -------------------------------------------------
    // 创建包含 3 个连接的连接池（线程安全）
    // let pool = ConnectionPool::new(3);

    // // 模拟多个线程获取/释放连接
    // let mut handles = vec![];

    // for _ in 0..5 {
    //     let pool_clone = Arc::clone(&pool);
    //     let handle = thread::spawn(move || {
    //         if let Some(conn) = pool_clone.get_connection() {
    //             conn.connect();
    //             // 模拟使用连接
    //             thread::sleep(std::time::Duration::from_millis(100));
    //             pool_clone.release_connection(conn);
    //         } else {
    //             println!("No available connection!");
    //         }
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // 单例模式 --------------------------------------------------
    // let instance2 = Singleton2::get_instance();
    // {
    //     let locked = instance2.lock().unwrap();
    //     println!("Value: {}", locked.value);
    // }
    // 单例模式 --------------------------------------------------
    // let instance = Singleton::get_instance();
    // instance.do_something();

    // // 再次获取，验证是否是同一个实例
    // let another_instance = Singleton::get_instance();
    // println!(
    //     "Same instance? {}",
    //     std::ptr::eq(instance, another_instance)
    // );
    //适配器模式 --------------------------------------------------
    // let legacy_rect = LegacyRectangle {
    //     x: 10,
    //     y: 20,
    //     width: 100,
    //     height: 50,
    // };

    // let adapter = RectangleAdapter {
    //     legacy_rect: legacy_rect,
    // };

    // let shapes : Vec<&dyn Shape> = vec![&adapter];

    // for shape in shapes {
    //     shape.draw();
    // }
    // //享元模式--------------------------------------------------
    // let mut factory = FlyweightFactory { flyweights: vec![] };

    // //获取享元对象
    // factory.handle("state1");
    // factory.handle("state1");
    // factory.handle("state3");

    // //创建具体子类对象
    // let class_a = ConcreteClassA;
    // let class_b = ConcreteClassB;

    // //调用模板方法
    // class_a.template_method();
    // class_b.template_method();

    // //代理模式 --------------------------------------------------
    // let proxy = Proxy { real_subject: RealSubject };
    // proxy.request();

    // //策略模式 --------------------------------------------------
    // let strategy_a = Box::new(ConcreteStrategyA);
    // let strategy_b = Box::new(ConcreteStrategyB);

    // let mut context = Context::new(strategy_a);
    // context.execute_strategy();

    // context.set_strategy(strategy_b);
    // context.execute_strategy();

    // //职责链模式 --------------------------------------------------
    // let mut handler_a = ConcreteHandlerA { next_handler: None };
    // let mut handler_b = ConcreteHandlerB { next_handler: None };
    // //1
    // handler_a.set_next(Box::new(handler_b));
    // let request_a = Request { content: String::from("Request A") };
    // handler_a.handle_request(&request_a);

    // let request_b = Request { content: String::from("Request B") };
    // handler_a.handle_request(&request_b);

    // let request_c = Request { content: String::from("Request C") };
    // handler_a.handle_request(&request_c);
}
