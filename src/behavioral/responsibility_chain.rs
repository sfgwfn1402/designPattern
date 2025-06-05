//请求结构体
pub struct Request{
    pub content : String
}
//处理者trait
pub trait Handler{
    fn set_next(&mut self, handler : Box<dyn Handler>);
    fn handle_request(&self, request: &Request);
}

//具体处理者A
pub struct ConcreteHandlerA{
    pub next_handler : Option<Box<dyn Handler>>
}

impl Handler for ConcreteHandlerA {
    //设置下一个处理者
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next_handler = Some(handler);
    }

    //处理请求
    fn handle_request(&self, request: &Request) {
        if request.content.contains("A") {
            println!("Handled by ConcreteHandlerA");
        }else if let Some(ref handler) = self.next_handler{
            handler.handle_request(request);
        }else {
            println!("No handler can handle the request");
        }
    }
}

//具体处理者B
pub struct ConcreteHandlerB{
    pub next_handler : Option<Box<dyn Handler>>
}

impl Handler for ConcreteHandlerB {
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next_handler = Some(handler);
    }

    fn handle_request(&self, request: &Request) {
        if request.content.contains("B") {
            println!("Handled by ConcreteHandlerB");
        }else if let Some(ref handler) = self.next_handler{
            handler.handle_request(request);
        }else{
            println!("No handler can handle the request");
        }
    }
}
