use super::*;

mod bindings;
mod delay_load;
mod factory_cache;
mod generic_factory;
mod heap;
mod ref_count;
mod sha1;
mod waiter;
mod weak_ref_count;

pub use bindings::*;
pub use delay_load::*;
pub use factory_cache::*;
pub use generic_factory::*;
pub use heap::*;
pub use ref_count::*;
pub use sha1::*;
pub use waiter::*;
pub use weak_ref_count::*;

// This is a workaround since 1.48 does not include `bool::then_some`.
pub fn then_some<T>(value: bool, t: T) -> Option<T> {
    if value {
        Some(t)
    } else {
        None
    }
}

// This is a workaround since 1.48 does not include `bool::then`.
pub fn then<T, F: FnOnce() -> T>(value: bool, f: F) -> Option<T> {
    if value {
        Some(f())
    } else {
        None
    }
}

pub fn wide_trim_end(mut wide: &[u16]) -> &[u16] {
    while let Some(last) = wide.last() {
        match last {
            32 | 9..=13 => wide = &wide[..wide.len() - 1],
            _ => break,
        }
    }
    wide
}

#[doc(hidden)]
#[macro_export]
macro_rules! interface_hierarchy {
    ($child:ty, $parent:ty) => {
        impl ::std::convert::From<$child> for $parent {
            fn from(value: $child) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&$child> for &$parent {
            fn from(value: &$child) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&$child> for $parent {
            fn from(value: &$child) -> Self {
                unsafe { ::std::mem::transmute(::std::clone::Clone::clone(value)) }
            }
        }
    };
    ($child:ty, $first:ty, $($rest:ty),+) => {
        $crate::imp::interface_hierarchy!($child, $first);
        $crate::imp::interface_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use interface_hierarchy;

#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern "system" {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(windows, not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = "windows")]
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(not(windows), not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[doc(hidden)]
pub use crate::link;
