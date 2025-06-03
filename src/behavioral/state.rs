//! State
//! 状态模式（State Pattern） 通常用于实现对象的行为随着其内部状态的变化而变化。
//! Rust 的枚举（enum）结合 trait 可以非常自然地表达这种状态切换机制。
//! 场景说明：
//! 我们以一个 投票系统 为例，其中投票流程分为三个状态：
//!     NotVoted：尚未投票
//!     Voted：已经投票
//!     Locked：投票已锁定，无法更改
//! 每个状态下有不同的行为，比如只有在 NotVoted 状态下可以投票。

//-----------------定义状态trait 和 枚举---------------------------------------------------------------------------------
// 定义状态的 trait
trait VotingState {
    // 投票
    fn vote(self: Box<Self>) -> Box<dyn VotingState>;
    // 状态
    fn status(&self);
}

// 定义状态枚举
/// 未投票状态
struct NotVoted;
/// 已投票状态
struct Voted;
/// 锁定状态
struct Locked;

/// 未投票状态 实现 VotingState trait
/// 实现投票行为，并返回已投票状态
impl VotingState for NotVoted {
    /// 投票操作：当前状态可以投票，投票后转为 Voted 状态
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("You voted successfully!");
        Box::new(Voted)
    }

    fn status(&self) {
        println!("Current state: NotVoted");
    }
}

/// 已投票状态 实现 VotingState trait
/// 实现投票行为，并返回锁定状态
impl VotingState for Voted {
    /// 投票操作：当前状态已经投票，无法再次投票，转为 Locked 状态
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("You have already voted.");
        self
    }
    fn status(&self) {
        println!("Current state: Voted");
    }
}

/// 锁定状态 实现 VotingState trait
impl VotingState for Locked {
    /// 投票操作：当前状态已锁定，无法投票
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("Voting is locked, you cannot vote anymore.");
        self
    }

    fn status(&self) {
        println!("Current state: Locked");
    }
}
//-----------------定义上下文结构体---------------------------------------------------------------------
/// 投票机器
pub struct VotingMachine {
    state: Box<dyn VotingState>,
}

impl VotingMachine {
    /// 创建投票机器
    pub fn new() -> Self {
        VotingMachine {
            state: Box::new(NotVoted),
        }
    }
    /// 投票操作
    pub fn vote(&mut self) {
        // 调用当前状态的投票方法，并更新状态
        // 因为在调用 vote 方法时, 需要移动，所以这里用了获取所有权的方式
        let old_state = std::mem::replace(&mut self.state, Box::new(NotVoted)); // 临时占位
        let new_state = old_state.vote(); // 调用 vote 获取新状态
        self.state = new_state; // 更新为新状态
    }

    /// 获取当前状态
    pub fn status(&self) {
        self.state.status();
    }
}
//--------------------------------------------------------------------------------------------------
fn main() {
    // 创建投票机器
    let mut machine = VotingMachine::new();

    // 检查初始状态
    machine.status();

    // 投票
    machine.vote();

    // 再次投票，应该提示已经投票
    machine.vote();

    // 尝试在已投票状态下再次投票，应该转为 Locked 状态
    machine.vote();
    machine.status();
}
