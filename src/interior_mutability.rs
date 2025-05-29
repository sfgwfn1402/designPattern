use std::cell::RefCell; // 单线程内部可变性
use std::sync::{Arc, Mutex, RwLock}; // 多线程支持
use std::thread;

// 1. 使用 RefCell（单线程）
/// 单线程下的可变数据封装，使用 `RefCell` 实现运行时借用检查。
pub struct MyData {
    value: RefCell<i32>,
}

impl MyData {
    pub fn new(val: i32) -> Self {
        MyData {
            value: RefCell::new(val),
        }
    }

    pub fn increment(&self) {
        let mut v = self.value.borrow_mut();
        *v += 1;
    }

    pub fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

// 2. 使用 Mutex（多线程互斥访问）
/// 多线程下基于 Mutex 的共享计数器，适用于并发递增场景。
pub struct SharedCounter {
    pub count: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(val: i32) -> Self {
        SharedCounter {
            count: Arc::new(Mutex::new(val)),
        }
    }

    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    pub fn get_count(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}

// 3. 使用 RwLock（多线程读写控制）
/// 支持多线程读写分离的字符串容器，使用 RwLock 实现。
pub struct ReadWriteData {
    data: RwLock<String>,
}

impl ReadWriteData {
    pub fn new(s: &str) -> Self {
        ReadWriteData {
            data: RwLock::new(s.to_string()),
        }
    }

    pub fn read_data(&self) -> String {
        let guard = self.data.read();
        if let Ok(guard) = guard {
            guard.clone()
        } else {
            "".to_string()
        }
    }

    pub fn write_data(&self, new_val: &str) {
        let mut guard = self.data.write();
        match guard {
            Ok(ref mut g) => **g = new_val.to_string(),
            Err(_) => (),
        }
    }
}

pub fn main() {
    // RefCell 示例
    let data = MyData::new(5);
    data.increment();
    println!("RefCell value: {}", data.get_value()); // 输出 6

    // Mutex 示例（多线程）
    let counter = SharedCounter::new(0);
    let counter_clone = SharedCounter {
        count: counter.count.clone(),
    };
    //查看两个结构对象的字段count的地址是否是同一个
    println!("{:p} == {:p}", &counter.count, &counter_clone.count);

    let handle = thread::spawn(move || {
        for _ in 0..5 {
            counter_clone.increment();
        }
    });

    handle.join().unwrap();
    println!("Mutex value: {}", counter.get_count()); // 输出 5

    // RwLock 示例
    let rw_data = ReadWriteData::new("hello");
    {
        let read1 = rw_data.read_data();
        println!("Read 1: {}", read1); // hello
    }

    rw_data.write_data("world");

    {
        let read2 = rw_data.read_data();
        println!("Read 2: {}", read2); // world
    }
}