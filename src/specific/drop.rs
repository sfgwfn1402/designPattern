pub struct MyResource {
    pub(crate) name: String,
}

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

pub fn main() {
    {
        let _resource1 = MyResource { name: "Resource 1".to_string() };
        let _resource2 = MyResource { name: "Resource 2".to_string() };
        println!("Exiting inner scope...");
    }

    println!("Exited outer scope. Resources have been dropped.");
}