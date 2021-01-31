//! Gum bindings for Rust
//!
//! Gum provides a number of utilities for instrumenting binary applications,
//! and traditionally is consumed via the JavaScript API known as GumJS.
//! This crate aims to provide a complete interface to the instrumentation
//! API provided by Gum, rather than GumJS (s.t. these bindings exclude the `Java` and `ObjC`
//! modules).
//!
//! # Quick Start
//! First, ensure that your platform is supported by Gum. You can find a listing of
//! development kits on the [Frida releases page](https://github.com/frida/frida/releases).
//! To get started using Gum, you need to obtain a global [`Gum`] object; this is required
//! to safely ensure that Gum has been properly initialized as required. Next, you are
//! free to use any available APIs, such as the [`stalker::Stalker`]:
//! ```
//! use frida_gum::{Gum, stalker::{Stalker, Transformer, NoneEventSink}};
//! use lazy_static::lazy_static;
//!
//! lazy_static! {
//!     static ref GUM: Gum = Gum::obtain();
//! }
//!
//! fn main() {
//!     let mut stalker = Stalker::new(&GUM);
//!
//!     let transformer = Transformer::from_callback(&GUM, |basic_block, _output| unsafe {
//!         for instr in basic_block {
//!             instr.keep();
//!         }
//!     });
//!
//!     unsafe {
//!         stalker.follow_me::<NoneEventSink>(transformer, None);
//!         stalker.unfollow_me();
//!     }
//! }
//! ```

#![deny(warnings)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::missing_safety_doc)]

extern crate num;
#[allow(unused_imports)]
#[macro_use]
extern crate num_derive;

use std::os::raw::c_void;

pub mod stalker;

pub mod interceptor;

mod module;
pub use module::*;

mod cpu_context;
pub use cpu_context::*;

mod memory_range;
pub use memory_range::*;

/// Context required for instantiation of all structures under the Gum namespace.
pub struct Gum;

impl Gum {
    /// Obtain a Gum handle, ensuring that the runtime is properly initialized. This may
    /// be called as many times as needed, and results in a no-op if the Gum runtime is
    /// already initialized.
    pub fn obtain() -> Gum {
        unsafe { frida_gum_sys::gum_init_embedded() };
        Gum {}
    }
}

impl Drop for Gum {
    fn drop(&mut self) {
        unsafe { frida_gum_sys::gum_deinit_embedded() };
    }
}

pub struct NativePointer(pub *mut c_void);
