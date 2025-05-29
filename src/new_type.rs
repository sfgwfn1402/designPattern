// 使用 Newtype 模式定义一个安全的 ID 类型
/// 应用场景
/// - 1. 类型安全增强
/// 避免不同业务id之间混淆，例如将UserId和OrderId混用
/// - 2. 输入验证封装
/// 构造时确保数据合法（如非零、范围限制等），避免无效状态传播。
/// - 3. 领域建模（Domain Modeling）
/// 在业务逻辑中为原始类型赋予明确语义，使代码更具可读性和意图表达能力。
/// 示例：代替 u32，使用 UserId 表示用户唯一标识符。
/// - 4. 封装实现细节
/// 如果将来需要更换底层存储类型（如从 u32 改成 String 或 UUID），只需修改 Newtype 的定义，不影响调用方。
pub struct UserId(u32);

impl UserId {
    // 创建一个新的 UserId 实例
    pub fn new(id: u32) -> Self {
        if id > 0 {
            UserId(id)
        } else {
            panic!("ID must be greater than 0");
        }
    }

    // 获取底层的 u32 值
    pub fn get(&self) -> u32 {
        self.0
    }
}

fn main() {
    // 创建合法的 UserId
    let user_id = UserId::new(42);
    println!("User ID: {}", user_id.get());

    // 尝试创建非法的 UserId（会导致 panic）
    // let invalid_user_id = UserId::new(0); // 这将触发 panic
}