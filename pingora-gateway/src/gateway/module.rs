// ModuleID is a string that uniquely identifies a Caddy module. A
// module ID is lightly structured. It consists of dot-separated
// labels which form a simple hierarchy from left to right. The last
// label is the module name, and the labels before that constitute
// the namespace (or scope).
//
// Thus, a module ID has the form: <namespace>.<name>
//
// An ID with no dot has the empty namespace, which is appropriate
// for app modules (these are "top-level" modules that Caddy core
// loads and runs).
//
// Module IDs should be lowercase and use underscores (_) instead of
// spaces.
//
// Examples of valid IDs:
// - http
// - http.handlers.file_server
// - caddy.logging.encoders.json
pub struct ModuleId(pub String);


// ModuleInfo represents a registered Caddy module.
pub struct ModuleInfo {
    // ID is the "full name" of the module. It
    // must be unique and properly namespaced.
    pub id: ModuleId,

    // New returns a pointer to a new, empty
    // instance of the module's type. This
    // method must not have any side-effects,
    // and no other initialization should
    // occur within it. Any initialization
    // of the returned value should be done
    // in a Provision() method (see the
    // Provisioner interface).
    pub new: Box<fn() -> Box<dyn Module>>,
}


// Module is a type that is used as a Caddy module. In
// addition to this interface, most modules will implement
// some interface expected by their host module in order
// to be useful. To learn which interface(s) to implement,
// see the documentation for the host module. At a bare
// minimum, this interface, when implemented, only provides
// the module's ID and constructor function.
//
// Modules will often implement additional interfaces
// including Provisioner, Validator, and CleanerUpper.
// If a module implements these interfaces, their
// methods are called during the module's lifespan.
//
// When a module is loaded by a host module, the following
// happens: 1) ModuleInfo.New() is called to get a new
// instance of the module. 2) The module's configuration is
// unmarshaled into that instance. 3) If the module is a
// Provisioner, the Provision() method is called. 4) If the
// module is a Validator, the Validate() method is called.
// 5) The module will probably be type-asserted from
// 'any' to some other, more useful interface expected
// by the host module. For example, HTTP handler modules are
// type-asserted as caddyhttp.MiddlewareHandler values.
// 6) When a module's containing Context is canceled, if it is
// a CleanerUpper, its Cleanup() method is called.
pub trait Module {
    // This method indicates that the type is a Caddy
    // module. The returned ModuleInfo must have both
    // a name and a constructor function. This method
    // must not have any side-effects.
    fn module(&self) -> ModuleInfo;
}

impl ModuleId {
    pub fn namespace(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    pub fn name(&self) -> Option<&str> {
        if self.0.len() == 0 {
            return None;
        }
        self.0.rsplit(".").last()
    }
}
