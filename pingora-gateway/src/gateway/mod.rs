use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use pingora_cache::cache_control::Cacheable::No;
use crate::gateway::module::Module;

pub mod http_proxy;
mod module;

lazy_static::lazy_static! {
    static ref MODULES:Mutex<HashMap<String,Arc<dyn Module+Send + Sync>>> = Mutex::new(HashMap::new());
}


pub fn register_module<T>(module: T) where T: Module + Send + Sync + 'static {
    let m = module.module();
    if m.id.name() == None {
        panic!("module ID missing")
    }
    if let Some(val) = m.id.name() {
        if val == "admin" {
            panic!("module ID '{val}' is reserved")
        }
    }
    let mut modules = MODULES.lock().unwrap();
    let key = module.module().id.0;
    if modules.contains_key(&key) {
        panic!("module already registered: {key}", )
    }
    modules.insert(module.module().id.0, Arc::new(module));
}

pub fn get_module(name: String) -> Option<Arc<dyn Module + Send + Sync>> {
    let mut modules = MODULES.lock().unwrap();
    if !modules.contains_key(&name) {
        panic!("module not registered: {name}")
    }
    if let Some(value) = modules.get(&name) {
        return Some(value.clone());
    }
    None
}


// GetModuleName returns a module's name (the last label of its ID)
// from an instance of its value. If the value is not a module, an
// empty string will be returned.
pub fn get_module_name(instance: Arc<dyn Module + Send + Sync>) -> Option<String> {
    if let Some(value) = instance.clone().module().id.name() {
        return Some(value.to_string());
    }
    None
}


// GetModuleID returns a module's ID from an instance of its value.
// If the value is not a module, an empty string will be returned.
pub fn get_module_id(instance: Arc<dyn Module + Send + Sync>) -> Option<String> {
    Some(instance.clone().module().id.0.clone())
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test() {
        println!("Hello")
    }
}
