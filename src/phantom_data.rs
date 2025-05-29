use std::marker::PhantomData;

/// # 一个简单的“智能指针”封装裸指针，并用 PhantomData 标记它“像”拥有 T 类型的数据
/// ptr: *const T 是一个裸指针，不拥有数据，因此不会自动释放。
/// _marker: PhantomData<T> 告诉编译器：“我们希望这个结构体在类型上表现得像是拥有 T 类型的数据”，从而帮助编译器进行借用检查。
/// 这在编写 unsafe 代码时非常有用，比如实现自定义指针、迭代器、生命周期绑定等。
pub struct MyBox2<T> {
    ///是一个 不可变裸指针，指向类型为 T 的数据。
    ///使用它需要在 unsafe 块中进行解引用操作。
    ptr: *const T,
    _marker: PhantomData<T>,
}

impl<T> MyBox2<T> {
    /// 创建一个新的 MyBox，指向给定值的只读指针
    pub fn new(value: &T) -> Self {
        MyBox2 {
            ptr: value,
            _marker: PhantomData,
        }
    }

    /// 解引用操作
    pub fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
}