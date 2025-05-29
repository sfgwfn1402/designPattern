use std::sync::{Once, Mutex};

pub struct Singleton2 {
    pub value: String,
}

static mut SINGLETON_INSTANCE: Option<Mutex<Singleton2>> = None;
static INIT: Once = Once::new();

impl Singleton2 {
    pub fn get_instance() -> &'static Mutex<Singleton2> {
        unsafe {
            INIT.call_once(|| {
                SINGLETON_INSTANCE = Some(Mutex::new(Singleton2 {
                    value: String::from("Singleton Instance"),
                }));
            });

            SINGLETON_INSTANCE.as_ref().unwrap()
        }
    }
}

