use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use crate::gateway::module::Module;

pub mod http_proxy;
mod module;

lazy_static::lazy_static! {
    static ref MODULES:Mutex<HashMap<String,Box<dyn Module+Send + Sync>>> = Mutex::new(HashMap::new());
}


pub fn register_module<T>(module: T) where T: Module + Send + Sync +'static{
    let mut modules = MODULES.lock().unwrap();
    modules.insert(module.module().id.0, Box::new(module));
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {
        println!("Hello")
    }
}
