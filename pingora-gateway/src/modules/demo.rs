use pingora_proxy::Session;
use crate::gateway::module::{Module, ModuleId, ModuleInfo};

struct Gizmo {}

impl Gizmo {
    pub fn new() -> Self {
        Gizmo {}
    }
}

///
/// register module
impl Module for Gizmo {
    fn module(&self) -> ModuleInfo {
        ModuleInfo {
            id: ModuleId(String::from("foo.gizmo")),
            new: Box::new(move || Box::new(Gizmo::new())),
        }
    }

    fn serve_http(&self, _session: Option<&mut Session>) -> bool {
        if _session.is_none() {
            return false;
        }
        println!("serve http ok");
        false
    }
}

#[cfg(test)]
mod test {
    use std::ptr::null_mut;
    use linked_hash_map::LinkedHashMap;
    use pingora_cache::cache_control::Cacheable::No;
    use crate::gateway;
    use crate::gateway::module;
    use crate::modules::demo::Gizmo;

    #[test]
    pub fn test_register_gizmo() {
        gateway::register_module(Gizmo::new());
    }

    #[test]
    pub fn test_modules() {
        gateway::register_module(Gizmo::new());
        for x in gateway::modules() {
            println!("module -> {x}")
        }
    }

    #[test]
    pub fn test_get_module() {
        gateway::register_module(Gizmo::new());
        let m = gateway::get_module("foo.gizmo");
        match m {
            Some(val) => {
                let module = val.module();
                println!("success:{:?}->{:?}", module.id.namespace(), module.id.name())
            }
            None => {
                println!("failed")
            }
        }
    }

    #[test]
    pub fn test_get_module_name() {
        gateway::register_module(Gizmo::new());
        let m = gateway::get_module("foo.gizmo");
        println!("get module name : {:?}", gateway::get_module_name(m.unwrap()));
    }


    #[test]
    pub fn test_serve_http() {
        gateway::register_module(Gizmo::new());
        for x in gateway::modules() {
            if let Some(serve_http) = gateway::get_module(x.as_str()) {
                serve_http.provision();
                serve_http.validate();
                serve_http.cleanup();
                serve_http.serve_http(None);
            }
        }
    }

    #[test]
    pub fn test_map() {
        let mut map = LinkedHashMap::new();
        map.insert(3, "Three");
        map.insert(1, "One");
        map.insert(2, "Two");

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }
    }
}
