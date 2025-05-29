//策略trait
pub trait Strategy {
    fn execute(&self);
}

//具体策略类 A
pub struct ConcreteStrategyA;

impl Strategy for ConcreteStrategyA {
    fn execute(&self) {
        println!("Executing strategy A");
    }
}

//具体策略类 B
pub struct ConcreteStrategyB;

impl Strategy for ConcreteStrategyB {
    fn execute(&self) {
        println!("Executing strategy B");
    }
}

//环境类
pub struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    //创建环境实例
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    //为环境实例设置策略实例
    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>){
        self.strategy = strategy;
    }

    pub fn execute_strategy(&self){
        self.strategy.execute();
    }
}
