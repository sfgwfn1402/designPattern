///如果你正在构建一个模块化系统，推荐将“单例”逻辑封装在模块中，并通过函数暴露接口，而不是直接暴露结构体
/// 
mod config {
    use std::sync::{Once, Mutex};
    use std::collections::HashMap;

    static INIT: Once = Once::new();
    static mut CONFIG_MAP: Option<Mutex<HashMap<String, String>>> = None;

    fn get_config_map() -> &'static Mutex<HashMap<String, String>> {
        unsafe {
            INIT.call_once(|| {
                CONFIG_MAP = Some(Mutex::new(HashMap::new()));
            });
            CONFIG_MAP.as_ref().unwrap()
        }
    }

    pub fn set(key: &str, value: &str) {
        let mut map = get_config_map().lock().unwrap();
        map.insert(key.to_string(), value.to_string());
    }

    pub fn get(key: &str) -> Option<String> {
        let map = get_config_map().lock().unwrap();
        map.get(key).cloned()
    }
}