//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00ab6ce0::rw_00ab6ce0()
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00ac8b60::rw_00ac8b60()
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00e61630::rw_00e61630()
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 3] = [call_0, call_1, call_2];
