//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00604a80::spd_apical_vf2(base as usize as u32, arg); 0 }
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00604a90::spd_apical_vf3(base as usize as u32, arg); 0 }
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00604aa0::spd_apical_vf4(base as usize as u32, arg); 0 }
}

/// The rewrite of entry 3, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_3(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00604ab0::spd_apical_vf5(base as usize as u32, arg); 0 }
}

/// The rewrite of entry 4, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_4(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00648c50::rw_00648c50(base, arg); 0 }
}

/// The rewrite of entry 5, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_5(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    { crate::rewrites::m_00876110::rw_00876110(base, arg); 0 }
}

/// The rewrite of entry 6, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_6(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c0f510::rw_00c0f510(base, f32::from_bits(arg))
}

/// The rewrite of entry 7, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_7(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c0f520::rw_00c0f520(base, f32::from_bits(arg))
}

/// The rewrite of entry 8, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_8(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c62bb0::rw_00c62bb0(base as usize as u32, arg)
}

/// The rewrite of entry 9, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_9(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00c62c70::rw_00c62c70(base as usize as u32, arg)
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 10] = [
    call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8, call_9,
];
