//定义享元接口
pub trait Flyweight {
    /**
     * 操作
     */
    fn operation(&self, extrinsic_state: &str);
}

//实现具体享元
pub struct ConcreteFlyweight {
    intrinsic_state: String,
}

impl Flyweight for ConcreteFlyweight {
    fn operation(&self, extrinsic_state: &str) {
        println!("Intrinsic state:{}, Extrinsic state:{}", self.intrinsic_state, extrinsic_state);
    }
}

//定义享元工厂
pub struct FlyweightFactory {
    pub(crate) flyweights: Vec<ConcreteFlyweight>,
}

impl FlyweightFactory {
    ///处理数据
    pub(crate) fn handle(&mut self, s: &str) ->Option<&ConcreteFlyweight> {
        //不可变借用
        if let Some(i)
            = self.flyweights.iter().position(|x| x.intrinsic_state == s ) {
            let option = self.flyweights.get(i);
            return option;
        }
        let flyweight2 = ConcreteFlyweight {
            intrinsic_state: s.to_string(),
        };
        self.flyweights.push(flyweight2);//可变借用
        None
    }
    /**
     * 获取值
     */
    pub fn get_flyweight(&mut self, intrinsic_state: &str) -> Option<&ConcreteFlyweight> {
        //不可变借用
        if let Some(flyweight)
            = self.flyweights.iter().find(|x| x.intrinsic_state == intrinsic_state) {
            return Some(flyweight);
        }
        None
    }

    /**
     * 向集合中加入成员元素
     */
    pub fn push_flyweight(&mut self, intrinsic_state: String) {
        let flyweight2 = ConcreteFlyweight {
            intrinsic_state: intrinsic_state,
        };
        self.flyweights.push(flyweight2);//可变借用
    }
}