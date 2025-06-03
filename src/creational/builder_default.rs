use std::time::Duration;
/*
✨ 特点总结
    特性	    说明
    Default	    提供字段默认值，避免重复代码
    Option<T>	表示字段是否被用户显式设置
    .build()	合并默认值与自定义值
    线程安全	 所有字段都实现了 Send + Sync（如果需要）
*/
// 定义一个配置结构体
#[derive(Debug)]
pub struct Config {
    timeout: Duration,
    retries: u32,
    enable_logging: bool,
    max_connections: usize,
}

// 为 Config 实现 Default trait
impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retries: 3,
            enable_logging: true,
            max_connections: 100,
        }
    }
}

// Builder 结构体，包含和 Config 相同的字段
#[derive(Default)]
pub struct ConfigBuilder {
    timeout: Option<Duration>,
    retries: Option<u32>,
    enable_logging: Option<bool>,
    max_connections: Option<usize>,
}

// Builder 的方法
/// 链式设置字段
impl ConfigBuilder {
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn enable_logging(mut self, enable_logging: bool) -> Self {
        self.enable_logging = Some(enable_logging);
        self
    }

    pub fn max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    // 构建最终的 Config 对象
    pub fn build(self) -> Config {
        Config {
            timeout: self.timeout.unwrap_or_default(),
            retries: self.retries.unwrap_or_default(),
            enable_logging: self.enable_logging.unwrap_or_default(),
            max_connections: self.max_connections.unwrap_or_default(),
        }
    }
}

fn main() {
    // 使用 Builder 构造 Config，默认值 + 自定义部分字段
    let config = ConfigBuilder::default()
        .timeout(Duration::from_secs(10))
        .retries(5)
        .enable_logging(false)
        .build();

    println!("Config: {:?}", config);
}