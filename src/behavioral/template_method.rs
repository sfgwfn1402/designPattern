//定义模板方法的抽象类
pub trait AbstractClass {
    //模板方法
    fn template_method(&self) {
        self.step1();
        self.step2();
        self.step3();
    }
    //定义算法的具体步骤，由子类实现
    fn step1(&self);
    fn step2(&self);
    fn step3(&self);
}

//具体实现模板方法的子类
pub struct ConcreteClassA;

impl AbstractClass for ConcreteClassA {
    fn step1(&self) {
        println!("ConcreteClassA: Step 1");
    }

    fn step2(&self) {
        println!("ConcreteClassA: Step 2")
    }

    fn step3(&self) {
        println!("ConcreteClassA: Step 3");
    }
}

pub struct ConcreteClassB;

impl AbstractClass for ConcreteClassB {
    fn step1(&self) {
        println!("ConcreteClassB: Step 1")
    }

    fn step2(&self) {
        println!("ConcreteClassB: Step 2")
    }

    fn step3(&self) {
        println!("ConcreteClassB: Step 3")
    }
}