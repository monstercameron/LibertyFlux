//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `crate::instances::INSTANCES`. The object is at `base`;
//! `arg` and `arg2` are the routine's own parameters, where it takes them. Results are widened to a word.

/// The rewrite of entry 0, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_0(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_005d5a60::rw_005d5a60(base as usize as u32)
}

/// The rewrite of entry 1, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_1(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_005e1370::rw_005e1370(base as usize as u32, 0, 0)
}

/// The rewrite of entry 2, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_2(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_008fb770::rw_008fb770(base as usize as u32)
}

/// The rewrite of entry 3, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_3(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_0097c0a0::rw_s103_97c0a0(base as *const u8)
}

/// The rewrite of entry 4, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_4(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_0097c0b0::rw_s103_97c0b0(base as *const u8)
}

/// The rewrite of entry 5, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_5(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00a4a9d0::rw_00a4a9d0(base as usize as u32, arg)
}

/// The rewrite of entry 6, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_6(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    crate::rewrites::m_00de1ab0::rw_00de1ab0(base as usize as u32)
}

/// The rewrite of entry 7, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_7(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    u32::from(crate::rewrites::m_00de51b0::rw_de51b0(base as usize as u32))
}

/// The rewrite of entry 8, called with the object at `base`.
#[allow(unused_variables)]
pub fn call_8(base: *mut u8, arg: u32, arg2: u32) -> u32 {
    u32::from(crate::rewrites::m_00de51f0::rw_de51f0(base as usize as u32))
}

/// The call table, in the order of the instance table.
pub const CALLS: [fn(*mut u8, u32, u32) -> u32; 9] = [
    call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8,
];
