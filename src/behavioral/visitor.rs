//! Visitor Pattern in Rust - 扩展 VotingMachine 的行为而不修改结构
//! 
// 前置状态 trait 和结构体（与你现有代码一致）
trait VotingState: Send + Sync {
    fn vote(self: Box<Self>) -> Box<dyn VotingState>;
    fn status(&self);
    fn accept(&self, visitor: &mut dyn StateVisitor);
}

#[derive(Debug)]
struct NotVoted;

#[derive(Debug)]
struct Voted;

#[derive(Debug)]
struct Locked;

impl VotingState for NotVoted {
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("You voted successfully!");
        Box::new(Voted)
    }

    fn status(&self) {
        println!("Current state: NotVoted");
    }

    fn accept(&self, visitor: &mut dyn StateVisitor) {
        visitor.visit_not_voted(self);
    }
}

impl VotingState for Voted {
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("You have already voted.");
        Box::new(Locked)
    }

    fn status(&self) {
        println!("Current state: Voted");
    }

    fn accept(&self, visitor: &mut dyn StateVisitor) {
        visitor.visit_voted(self);
    }
}

impl VotingState for Locked {
    fn vote(self: Box<Self>) -> Box<dyn VotingState> {
        println!("Voting is locked, you cannot vote anymore.");
        self
    }

    fn status(&self) {
        println!("Current state: Locked");
    }

    fn accept(&self, visitor: &mut dyn StateVisitor) {
        visitor.visit_locked(self);
    }
}

// ======================
// 定义 Visitor trait
// ======================

trait StateVisitor {
    fn visit_not_voted(&mut self, state: &NotVoted);
    fn visit_voted(&mut self, state: &Voted);
    fn visit_locked(&mut self, state: &Locked);
}

// ======================
// 实现具体 Visitor
// ======================

// 示例 1：日志访问者
struct LoggingVisitor;

impl StateVisitor for LoggingVisitor {
    fn visit_not_voted(&mut self, _: &NotVoted) {
        println!("[Log] Current state is NotVoted");
    }

    fn visit_voted(&mut self, _: &Voted) {
        println!("[Log] Current state is Voted");
    }

    fn visit_locked(&mut self, _: &Locked) {
        println!("[Log] Current state is Locked");
    }
}

// 示例 2：审计访问者
struct AuditVisitor;

impl StateVisitor for AuditVisitor {
    fn visit_not_voted(&mut self, _: &NotVoted) {
        println!("[Audit] NotVoted: Ready to vote");
    }

    fn visit_voted(&mut self, _: &Voted) {
        println!("[Audit] Voted: Already casted");
    }

    fn visit_locked(&mut self, _: &Locked) {
        println!("[Audit] Locked: Vote finalized");
    }
}

// ======================
// 修改 VotingMachine 支持 Visitor
// ======================

pub struct VotingMachine {
    state: Box<dyn VotingState>,
}

impl VotingMachine {
    pub fn new() -> Self {
        VotingMachine {
            state: Box::new(NotVoted),
        }
    }

    pub fn vote(&mut self) {
        let old_state = std::mem::replace(&mut self.state, Box::new(NotVoted));
        let new_state = old_state.vote();
        self.state = new_state;
    }

    pub fn status(&self) {
        self.state.status();
    }

    // 新增方法：接受访问者
    pub fn accept(&self, visitor: &mut dyn StateVisitor) {
        self.state.accept(visitor);
    }
}

// ======================
// 使用示例
// ======================

fn main() {
    let mut machine = VotingMachine::new();

    machine.status(); // Current state: NotVoted
    machine.accept(&mut LoggingVisitor);   // [Log] Current state is NotVoted
    machine.accept(&mut AuditVisitor);     // [Audit] NotVoted: Ready to vote

    machine.vote();
    machine.status(); // Current state: Voted
    machine.accept(&mut LoggingVisitor);   // [Log] Current state is Voted
    machine.accept(&mut AuditVisitor);     // [Audit] Voted: Already casted

    machine.vote();
    machine.status(); // Current state: Locked
    machine.accept(&mut LoggingVisitor);   // [Log] Current state is Locked
    machine.accept(&mut AuditVisitor);     // [Audit] Locked: Vote finalized
}