use std::sync::Once;
/// ------------------------------------------------------------------
/// 线程安全的单例实现
/// 线程安全性说明
/// Once 是标准库中的机制，保证初始化代码只执行一次，即使在多线程环境下也能安全运行。
/// 使用 unsafe 是因为我们需要手动管理静态可变状态，但通过 Once 保证了访问的安全性。
/// 返回的是 'static 生命周期的引用，表示该实例存活于整个程序运行期间。
/// ------------------------------------------------------------------

// 单例结构体
pub struct Singleton {
    value: String,
}

// 静态变量用于保存单例实例
static mut SINGLETON_INSTANCE: Option<Singleton> = None;
static INIT: Once = Once::new();

impl Singleton {
    // 获取单例实例的引用
    pub fn get_instance() -> &'static Self {
        unsafe {
            INIT.call_once(|| {
                SINGLETON_INSTANCE = Some(Singleton {
                    value: String::from("Singleton Instance"),
                });
            });

            SINGLETON_INSTANCE.as_ref().unwrap()
        }
    }

    // 示例方法
    pub fn do_something(&self) {
        println!("Singleton is doing something. Value: {}", self.value);
    }
}

fn main() {
    // 获取单例实例并调用方法
    let instance = Singleton::get_instance();
    instance.do_something();

    // 再次获取，验证是否是同一个实例
    let another_instance = Singleton::get_instance();
    println!(
        "Same instance? {}",
        std::ptr::eq(instance, another_instance)
    );
}