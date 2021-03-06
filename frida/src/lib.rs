/*
 * Copyright (C) 2020-2021 meme <keegan@sdf.org>
 *
 * Licence: wxWindows Library Licence, Version 3.1
 */

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![deny(warnings)]

use frida_sys;

/// Context required for instantiation of all structures under the Frida namespace.
pub struct Frida;

impl Frida {
    /// Obtain a Frida handle, ensuring that the runtime is properly initialized. This may
    /// be called as many times as needed, and results in a no-op if the Frida runtime is
    /// already initialized.
    pub unsafe fn obtain() -> Frida {
        frida_sys::frida_init();
        Frida {}
    }
}

impl Drop for Frida {
    fn drop(&mut self) {
        unsafe { frida_sys::frida_deinit() };
    }
}
