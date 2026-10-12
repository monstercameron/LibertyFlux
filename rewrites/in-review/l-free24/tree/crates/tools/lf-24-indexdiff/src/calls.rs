//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_008c5fe0::rw_008c5fe0(base as usize as u32, arg)
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_008c6080::rw_008c6080(base as usize as u32, arg)
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    (crate::rewrites::m_0097b2b0::rw_s103_97b2b0(base as *const u8, arg)).to_bits()
}

/// The rewrite of entry 3, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_3(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00d9ec20::rw_s675_00d9ec20(base as usize as u32, arg)
}

/// The rewrite of entry 4, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_4(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00d9ec40::rw_s675_00d9ec40(base as usize as u32, arg)
}

/// The rewrite of entry 5, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_5(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    (crate::rewrites::m_00d9ec50::rw_s675_00d9ec50(base as usize as u32, arg)).to_bits()
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 6] = [
    call_0, call_1, call_2, call_3, call_4, call_5,
];
