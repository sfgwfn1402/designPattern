//! Iterator trait 是标准库提供的一个非常核心的 trait，用于实现对集合类型的遍历功能。
//!
//!

pub struct MyIterator<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> MyIterator<'a> {
    pub fn new(data: &'a [i32]) -> Self {
        MyIterator { data, index: 0 }
    }
}

impl<'a> Iterator for MyIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut iter = MyIterator::new(&numbers);
    while let Some(number) = iter.next() {
        println!("Number: {}", number);
    }
}
