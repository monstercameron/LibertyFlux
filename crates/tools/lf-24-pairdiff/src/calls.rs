//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_006036b0::rm_ptfx_manager_set_words_4b0_4b4(base as usize as u32, arg, arg2)
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00891540::rw_00891540(base as usize as u32, arg, arg2)
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_008b1080::rw_008B1080(base as usize as u32, arg, arg2)
}

/// The rewrite of entry 3, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_3(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00b4fdc0::rw_00b4fdc0(base as usize as u32, arg, arg2)
}

/// The rewrite of entry 4, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_4(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c08c40::rw_00c08c40(base as usize as u32, arg, arg2)
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 5] = [call_0, call_1, call_2, call_3, call_4];
