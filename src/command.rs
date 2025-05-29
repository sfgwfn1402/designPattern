use std::{str, sync::{Arc, Mutex}};

/// 命令模式（Command Pattern）是一种行为设计模式，它将请求封装为对象，从而使你能够将具有不同请求的对象参数化其他对象。
/// 这种模式允许将操作排队、记录日志或支持撤销操作等功能。

/// 定义 Command trait
/// 定义一个通用的 Command trait，包含执行和撤销方法
/// 定义了所有命令必须实现的行为（如 execute() 和 undo()）。
pub trait Command {
    /// 执行命令
    fn execute(&self);
    /// 撤销命令
    fn undo(&self);
}

//--------------------------------------------------------------------------------------------------
//实现具体的命令：打开灯的命令

/// 具体的接收者对象：灯  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
/// 真正执行操作的对象。
pub struct Light{
    is_on: bool,
}

impl Light {
    /// 创建一个新的 Light 实例
    pub fn new() -> Self {
        Light { is_on: false }
    }
    /// 打开灯
    pub fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is on");
    }
    /// 关闭灯
    pub fn turn_off(&mut self) {
        self.is_on = false;
        println!("Light is off");
    }
}
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

///打开灯的具体命令>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
/// 封装了对接收者（Light）的操作。
pub struct LightOnCommand {
    light : Arc<Mutex<Light>>,
}
impl LightOnCommand {
    pub fn new(light: Arc<Mutex<Light>>) -> Self {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn execute(&self) {
        let this = self.light.lock();
        match this {
            Ok(mut t) => t.turn_on(),
            Err(e) => {
                // 如果锁被污染（比如之前有线程 panic），可以尝试继续使用数据
                println!("打开灯异常: {:?}", e);
                // 获取锁的所有权
                let mut light = e.into_inner();
                light.turn_on();
            }
        }
    }

    fn undo(&self) {
        match self.light.lock()  {
            Ok(mut light) => light.turn_off(),
            Err(e) => {
                // 如果锁被污染（比如之前有线程 panic），可以尝试继续使用数据
                println!("关闭灯异常");
                // 获取锁的所有权
                let mut light = e.into_inner();
                light.turn_off();
            },
        }
    }
}
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
//--------------------------------------------------------------------------------------------------
/// 调用者：遥控器
/// 不关心具体命令如何执行，只负责调用 execute() 或 undo()。
pub struct RemoteControl{
    /// 命令对象
    command : Option<Box<dyn Command>>,
}

impl RemoteControl  {
    ///创建遥控器
    pub fn new() -> Self  {
        RemoteControl {
            command : None, //命令置为None
        }
    }
    ///为遥控器 设置命令
    pub fn set_command(&mut self, command : Box<dyn Command>) {
        self.command = Some(command);
    }
    ///按下按钮，执行命令
    pub fn press_button(self : &Self) {
        if let Some(command) = &self.command {
            command.execute();
        }
    }
    ///撤销按钮，执行撤销命令
    pub fn undo_button(self : &Self) {
        if let Some(ref command) = &self.command {
            command.undo();
        }
    }

}

fn main() {
    //创建被调用对象
    //创建灯对象
    let light = Arc::new(Mutex::new(Light::new()));

    //创建打开灯的命令对象
    let light_on_command = LightOnCommand::new(light.clone());

    //创建遥控器对象
    let mut remote_control = RemoteControl::new();

    //设置遥控器的命令
    remote_control.set_command(Box::new(light_on_command));

    //按下按钮，执行命令
    remote_control.press_button();

    //撤销按钮，执行撤销命令
    remote_control.undo_button();
}