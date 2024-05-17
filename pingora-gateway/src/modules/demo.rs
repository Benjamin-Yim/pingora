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
}

#[cfg(test)]
mod test {
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
                println!("获取成功:{:?}->{:?}", module.id.namespace(), module.id.name())
            }
            None => {
                println!("获取失败")
            }
        }
    }

    #[test]
    pub fn test_get_module_name() {
        gateway::register_module(Gizmo::new());
        let m = gateway::get_module("foo.gizmo");
        println!("get module name : {:?}", gateway::get_module_name(m.unwrap()));
    }
}
