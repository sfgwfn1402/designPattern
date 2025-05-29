//定义共享接口
pub trait Subject {
    fn request(&self);
}

//实现具体对象
pub struct RealSubject;

impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling request");
    }
}

//实现代理对象
pub struct Proxy{
    //具体对象
    pub real_subject: RealSubject
}
//实现特征Subject
impl Subject for Proxy{
    fn request(&self) {
        self.pre_request();
        self.real_subject.request();
        self.post_request();
    }
}
//代理对象自己的方法
impl Proxy{
    fn pre_request(&self){
        println!("Proxy: Preparing request");
    }
    fn post_request(&self){
        println!("Proxy: Post-processing request");
    }
}