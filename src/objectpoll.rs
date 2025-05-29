use std::sync::{Arc, Mutex};
use std::thread;

// 要复用的对象
#[derive(Debug)]
pub struct Connection {
    id: u32,
}

impl Connection {
    pub fn new(id: u32) -> Self {
        println!("Creating connection {}", id);
        Connection { id }
    }

    pub fn connect(&self) {
        println!("Connection {} is connecting...", self.id);
    }
}

// 对象池定义
pub struct ConnectionPool {
    connections: Mutex<Vec<Connection>>,
}

impl ConnectionPool {
    // 创建新的连接池
    pub fn new(size: usize) -> Arc<Self> {
        let mut connections = Vec::with_capacity(size);
        for i in 0..size {
            connections.push(Connection::new(i as u32));
        }

        Arc::new(Self {
            connections: Mutex::new(connections),
        })
    }

    // 获取连接（这里简单弹出最后一个）
    pub fn get_connection(&self) -> Option<Connection> {
        self.connections.lock().unwrap().pop()
    }

    // 归还连接
    pub fn release_connection(&self, conn: Connection) {
        self.connections.lock().unwrap().push(conn);
    }
}

fn main() {
    // 创建包含 3 个连接的连接池（线程安全）
    let pool = ConnectionPool::new(3);

    // 模拟多个线程获取/释放连接
    let mut handles = vec![];

    for _ in 0..5 {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            if let Some(conn) = pool_clone.get_connection() {
                conn.connect();
                // 模拟使用连接
                thread::sleep(std::time::Duration::from_millis(100));
                pool_clone.release_connection(conn);
            } else {
                println!("No available connection!");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}