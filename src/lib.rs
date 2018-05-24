extern crate lxc_sys;
#[macro_use]
extern crate bitflags;

mod attach;
mod console;
mod container;
#[macro_use]
mod ffi;
mod flags;
pub mod log;
mod migrate;

pub use self::container::Container;
pub use self::flags::{AttchFlags, CloneFlags, CreateFlags};
pub use self::log::Log;

pub use ::lxc_sys::lxc_conf as Conf;
pub use ::lxc_sys::lxc_lock as Lock;
pub use ::lxc_sys::lxc_snapshot as Snapshot;

pub fn version() -> String {
    let version = unsafe {
        ::std::ffi::CStr::from_ptr(::lxc_sys::lxc_get_version())
    };

    version.to_str()
        .unwrap()
        .to_string()
}

pub fn wait_states() -> Vec<String> {
    let size = unsafe {
        ::lxc_sys::lxc_get_wait_states(::std::ptr::null_mut())
    };

    let mut states = Vec::new();
    states.resize(size as usize, ::std::ptr::null());

    unsafe {
        ::lxc_sys::lxc_get_wait_states(states.as_mut_ptr())
    };

    states.iter()
        .map(|e| self::ffi::to_string(*e))
        .collect()
}

pub fn get_global_config_item(key: &str) -> Result<String, ()> {
    let value = unsafe {
        ::lxc_sys::lxc_get_global_config_item(self::ffi::to_cstr(key))
    };

    if value == ::std::ptr::null() {
        Err(())
    } else {
        Ok(self::ffi::to_string(value))
    }
}

#[cfg(feature = "v2_0")]
pub fn config_item_is_supported(key: &str) -> bool {
    unsafe {
        ::lxc_sys::lxc_config_item_is_supported(self::ffi::to_cstr(key))
    }
}
