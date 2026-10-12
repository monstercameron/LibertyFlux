//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_008993c0::rw_008993c0(base as usize as u32)
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_0094eea0::rw_0094eea0(base)
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_009bb5f0::rw_009bb5f0(base as usize as u32)
}

/// The rewrite of entry 3, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_3(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    u32::from(crate::rewrites::m_00a73240::rw_00a73240(base as usize as u32, 0, 0, 0))
}

/// The rewrite of entry 4, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_4(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00ab5ce0::rw_00ab5ce0(base as usize as u32)
}

/// The rewrite of entry 5, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_5(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00b18280::rw_00B18280(base as usize as u32)
}

/// The rewrite of entry 6, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_6(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00b79180::rw_00b79180(0, base as usize as u32)
}

/// The rewrite of entry 7, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_7(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00be4d40::rw_00be4d40(0, base as usize as u32)
}

/// The rewrite of entry 8, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_8(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00be4d50::rw_00be4d50(0, base as usize as u32)
}

/// The rewrite of entry 9, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_9(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00be4d60::rw_00be4d60(0, base as usize as u32)
}

/// The rewrite of entry 10, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_10(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00be4d70::rw_00be4d70(0, base as usize as u32)
}

/// The rewrite of entry 11, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_11(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00beac00::rw_00beac00(base as usize as u32)
}

/// The rewrite of entry 12, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_12(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c30c40::rw_00c30c40(base)
}

/// The rewrite of entry 13, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_13(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00d39d00::rw_00d39d00(0, base as usize as u32)
}

/// The rewrite of entry 14, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_14(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00d4f790::rw_00d4f790(0, base as usize as u32)
}

/// The rewrite of entry 15, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_15(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00d4f7a0::rw_00d4f7a0(0, base as usize as u32)
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 16] = [
    call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8, call_9, call_10, call_11, call_12, call_13, call_14, call_15,
];
