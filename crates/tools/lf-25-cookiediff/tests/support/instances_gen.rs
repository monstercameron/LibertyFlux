//! The cookie-seal instances, generated from the verified rewrites.
//!
//! Each entry gives a rewrite's export name and address, the vtable constant it
//! writes, and the width of its value store.

use lf_25_cookiediff::rewrites;
use lf_core::object_cookie::ValueWidth;

/// One verified rewrite of the batch.
#[allow(dead_code)]
pub struct CookieInstance {
    /// The export name of the rewrite.
    pub name: &'static str,
    /// The image address of the rewrite, for the report.
    pub address: u32,
    /// The vtable word the rewrite leaves in the object.
    pub vtable: u32,
    /// The width of the value store.
    pub width: ValueWidth,
    /// Calls the rewrite with the object's address and the value.
    pub call: fn(u32, u32) -> u32,
}

fn call_fn_008dbba0(this: u32, value: u32) -> u32 {
    rewrites::fn_008dbba0::rw_008dbba0(this, value)
}

fn call_fn_008dbc20(this: u32, value: u32) -> u32 {
    rewrites::fn_008dbc20::rw_008dbc20(this, value)
}

fn call_fn_008dbc90(this: u32, value: u32) -> u32 {
    rewrites::fn_008dbc90::rw_008dbc90(this, value)
}

fn call_fn_008dbcc0(this: u32, value: u32) -> u32 {
    rewrites::fn_008dbcc0::rw_008dbcc0(this, value)
}

/// The instances, in address order.
pub static INSTANCES: [CookieInstance; 4] = [
    CookieInstance {
        name: "rw_008dbba0",
        address: 0x008DBBA0,
        vtable: 0x00E81990,
        width: ValueWidth::Word,
        call: call_fn_008dbba0,
    },
    CookieInstance {
        name: "rw_008dbc20",
        address: 0x008DBC20,
        vtable: 0x00E81AA8,
        width: ValueWidth::Word,
        call: call_fn_008dbc20,
    },
    CookieInstance {
        name: "rw_008dbc90",
        address: 0x008DBC90,
        vtable: 0x00E81AC4,
        width: ValueWidth::Byte,
        call: call_fn_008dbc90,
    },
    CookieInstance {
        name: "rw_008dbcc0",
        address: 0x008DBCC0,
        vtable: 0x00E81AE0,
        width: ValueWidth::Word,
        call: call_fn_008dbcc0,
    },
];
