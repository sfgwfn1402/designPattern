//! **Memento 模式（备忘录模式）**用于保存和恢复对象的状态，同时不破坏封装性。
//! 通常的做法是通过一个结构体来保存目标对象的“快照”，并在需要时还原它。
//----定义Editor和Memento结构体--------------------------------------------------
/// 编辑器结构体，包含文本内容。
pub struct Editor {
    content: String,
}

///备忘录结构体，用于保存编辑器的状态。
#[derive(Clone)]
pub struct Memento {
    content: String,
}
//----实现Editor的方法---------------------
impl Editor {
    /// 创建一个新的编辑器实例。
    pub fn new() -> Self {
        Editor {
            content : String::new(),
        }
    }
    /// 设置编辑器的内容。
    pub fn set_content(&mut self, content : &str) {
        self.content = content.to_string();
    }

    /// 获取编辑器的内容。
    pub fn get_content(&self) -> &str {
        &self.content
    }
    /// 创建一个备忘录，保存当前编辑器的状态。
    pub fn save(&self) -> Memento {
        Memento {
            content: self.content.clone(),
        }
    }
    /// 恢复编辑器的状态。
    pub fn restore(&mut self, memento: &Memento) {
        self.content = memento.content.clone();
    }
}

fn main() {
    let mut editor = Editor::new();

    editor.set_content("第一次输入");
    println!("当前内容: {}", editor.get_content());

    // 保存当前状态
    let memento = editor.save();

    editor.set_content("第二次输入");
    println!("当前内容: {}", editor.get_content());

    // 恢复到之前的状态
    editor.restore(&memento);
    println!("恢复到之前的状态: {}", editor.get_content());
}